use std::time::Instant;

use crate::rules::RuleType;
use crate::torrent::Torrent;

#[derive(Debug)]
pub struct Peer {
    pub address: String,
    pub id: String,
    pub name: String,
    pub download_speed: u64,
    pub downloaded: u64,
    pub upload_speed: u64,
    pub uploaded: u64,
    pub progress: f64,
}

pub struct BannedPeer {
    pub rule: RuleType,
    pub peer: Peer,
    pub time: Instant,
    pub torrent: Torrent,  // for snapshot
}

impl PartialEq for Peer {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}

impl PartialEq for BannedPeer {
    fn eq(&self, other: &Self) -> bool {
        self.peer == other.peer
    }
}