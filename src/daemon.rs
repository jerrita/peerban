use std::time::Instant;

use anyhow::Result;
use log::{debug, info, warn};

use crate::backend::Backend;
use crate::peer::BannedPeer;
use crate::rules::preload::PREDEFINED_RULES;
use crate::rules::Rule;

struct Statistic {
    pub torrents: u64,
    pub peers: u64,
    pub banned: u64,
}

pub struct Daemon {
    backend: Box<dyn Backend>,
    banned: Vec<BannedPeer>,
    rules: Vec<Rule>,
    scan_time: u64,
    clear: bool,
}

impl Daemon {
    pub fn new(backend: Box<dyn Backend>, scan: u64, clear: bool) -> Self {
        let rules = PREDEFINED_RULES.clone();
        Daemon {
            backend,
            banned: Vec::new(),
            rules,
            scan_time: scan,
            clear,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        info!("Backend: {}", self.backend.describe().await?);
        info!("[interval] scan: {}s", self.scan_time);
        let mut stat = Statistic {
            torrents: 0,
            peers: 0,
            banned: 0,
        };
        if self.clear {
            self.backend.ban_clear().await?;
            info!("[start] jail cleared.");
        }
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
            }
            if flag {
                info!("[active] torrents: {}, peers: {}, banned: {}", stat.torrents, stat.peers, stat.banned);
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(self.scan_time)).await;
        }
    }
}