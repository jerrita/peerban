use lazy_static::lazy_static;

use crate::rules::Rule;

lazy_static!(
    pub static ref PREDEFINED_RULES: Vec<Rule> = vec![
        "idStartsWith@-XL",
        "idStartsWith@-SD",
        "idStartsWith@-XF",
        "idStartsWith@-QD",
        "idStartsWith@-BN",
        "idStartsWith@-DL",
        "idStartsWith@-TS",
        "idStartsWith@-FG",
        "idStartsWith@-TT",
        "idStartsWith@-NX",
        "idStartsWith@-SP",
        "idStartsWith@-GT0002",
        "idStartsWith@-GT0003",
        "idStartsWith@-DT",
        "idStartsWith@-HP",
        "idContains@cacao",

        "nameStartsWith@-XL",
        "nameContains@Xunlei",
        "nameStartsWith@TaiPei-Torrent",
        "nameStartsWith@Xfplay",
        "nameStartsWith@BitSpirit",
        "nameContains@FlashGet",
        "nameContains@TuDou",
        "nameContains@TorrentStorm",
        "nameContains@QQDownload",
        "nameContains@github.com/anacrolix/torrent",
        "nameStartsWith@qBittorrent/3.3.15",
        "nameStartsWith@dt/torrent",
        "nameStartsWith@hp/torrent",
        "nameStartsWith@DT",
        "nameStartsWith@go.torrent.dev",
        "nameStartsWith@github.com/thank423/trafficConsume",

        "progressProbe@0.08",  // 进度倒退检测
        "excessiveProbe@1.5",  // 超量下载检测
    ].iter().map(|&s| Rule::from(s)).collect();
);