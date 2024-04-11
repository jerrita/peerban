use std::time::Instant;

use anyhow::Result;
use log::{debug, info, warn};

use crate::backend::Backend;
use crate::conf::PeerBanConfig;
use crate::peer::BannedPeer;
use crate::rules::{Rule, RuleType};
use crate::rules::preload::PREDEFINED_RULES;

struct Statistic {
    pub torrents: u64,
    pub peers: u64,
    pub banned: u64,
    pub released: u64,
}

pub struct Daemon {
    backend: Box<dyn Backend>,
    conf: PeerBanConfig,
    banned: Vec<BannedPeer>,
    rules: Vec<Rule>,
}

impl Daemon {
    pub fn new(backend: Box<dyn Backend>, conf: PeerBanConfig) -> Self {
        let mut rules = PREDEFINED_RULES.clone();
        if conf.block_progress_fallback {
            rules.push(Rule {
                class: RuleType::ProgressProbe,
                value: conf.block_progress_fallback_threshold.into(),
            });
        }
        if conf.block_excessive_clients {
            rules.push(Rule {
                class: RuleType::ExcessiveProbe,
                value: conf.block_excessive_clients_threshold.into(),
            });
        }
        Daemon {
            backend,
            conf,
            banned: Vec::new(),
            rules,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        info!("Backend: {}", self.backend.describe().await?);
        info!("[interval] scan: {}s, block: {}s", self.conf.scan_time, self.conf.block_time);
        let mut stat = Statistic {
            torrents: 0,
            peers: 0,
            banned: 0,
            released: 0,
        };
        loop {
            let mut flag = false;
            let torrents = self.backend.get_uploading_torrents().await?;
            stat.torrents = torrents.len() as u64;
            stat.peers = 0;
            for torrent in torrents {
                debug!("Torrent: {}({})", torrent.name, torrent.hash);
                let peers = self.backend.get_peers(&torrent.hash).await?;
                stat.peers += peers.len() as u64;
                for peer in peers {
                    if self.banned.iter().any(|banned| banned.peer == peer) {
                        warn!("Peer {}({}) is already banned.", peer.address, peer.id);
                        continue;
                    }

                    for rule in &self.rules {
                        if rule.match_peer(&peer, torrent.size) {
                            flag = true;
                            self.backend.ban_peer(&peer).await?;
                            info!("Banned {:?} {:?}.", peer, rule);
                            self.banned.push(BannedPeer {
                                peer,
                                time: Instant::now(),
                                rule: rule.class.clone(),
                                torrent: torrent.clone(),
                            });
                            stat.banned += 1;

                            break;
                        }
                    }
                }

                // Remove expired banned peers
                let now = Instant::now();
                self.banned.retain(|banned| {
                    if now.duration_since(banned.time).as_secs() > self.conf.block_time {
                        flag = true;
                        stat.released += 1;
                        info!("Released {}({}) {:?}.", banned.peer.address, banned.peer.id, banned.rule);
                        false
                    } else {
                        true
                    }
                });
            }
            if flag {
                info!("[active] torrents: {}, peers: {}, banned: {}, released: {}", stat.torrents, stat.peers, stat.banned, stat.released);
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(self.conf.scan_time)).await;
        }
    }
}