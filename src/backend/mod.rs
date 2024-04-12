use anyhow::Result;
use async_trait::async_trait;

use crate::peer::Peer;
use crate::torrent::Torrent;

pub mod qb;

#[async_trait]
pub trait Backend {
    async fn describe(&mut self) -> Result<String>;
    async fn get_uploading_torrents(&self) -> Result<Vec<Torrent>>;
    async fn get_peers(&self, hash: &str) -> Result<Vec<Peer>>;
    async fn ban_clear(&self) -> Result<()>;
    async fn ban_peer(&self, peer: &Peer) -> Result<()>;
}