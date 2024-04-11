#[derive(Debug, Clone)]
pub struct Torrent {
    pub name: String,
    pub hash: String,
    pub size: u64,
}