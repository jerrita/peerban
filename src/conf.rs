pub struct PeerBanConfig {
    pub scan_time: u64,
    pub block_time: u64,

    // 进度倒退检测
    pub block_progress_fallback: bool,
    pub block_progress_fallback_threshold: f64,

    // 超量下载检测
    pub block_excessive_clients: bool,
    pub block_excessive_clients_threshold: f64,
}

impl Default for PeerBanConfig {
    fn default() -> Self {
        PeerBanConfig {
            scan_time: 3,
            block_time: 24 * 60 * 60,
            block_progress_fallback: true,
            block_progress_fallback_threshold: 0.08,
            block_excessive_clients: true,
            block_excessive_clients_threshold: 1.5,
        }
    }
}