use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

use crate::backend::Backend;
use crate::peer::Peer;
use crate::torrent::Torrent;

const UPLOADING_STATUS: &str = "stalledDL|metaDL|forcedMetaDL|downloading|forcedDL|uploading|forcedUP";

pub struct QBitBackend {
    endpoint: String,
    username: String,
    password: String,
    cookie: Option<String>,
    login: bool,
}

impl QBitBackend {
    pub(crate) fn new(ep: String, auth: String) -> Self {
        if !ep.starts_with("http") {
            panic!("Invalid endpoint, must start with http or https.");
        }
        let mut split = auth.split(':');
        if split.clone().count() != 2 {
            panic!("Invalid qb auth string, use username:password format.");
        }
        QBitBackend {
            endpoint: format!("{}/api/v2", ep.trim_end_matches('/')),
            username: split.next().unwrap().to_string(),
            password: split.next().unwrap().to_string(),
            cookie: None,
            login: false,
        }
    }
}

#[async_trait]
impl Backend for QBitBackend {
    async fn describe(&mut self) -> Result<String> {
        let client = reqwest::Client::new();
        if !self.login {
            let mut form = HashMap::new();
            form.insert("username", self.username.clone());
            form.insert("password", self.password.clone());
            let resp = client
                .post(&format!("{}/auth/login", self.endpoint))
                .form(&form)
                .send()
                .await?;
            if resp.status().is_success() {
                let cookie = resp.headers().get("set-cookie").unwrap().to_str().unwrap();
                self.cookie = Some(cookie.split(';').next().unwrap().to_string());
                self.login = true;
            } else {
                Err(anyhow::anyhow!("Failed to login to QBitBackend."))?;
            }
        }
        let resp = client
            .get(&format!("{}/app/version", self.endpoint))
            .header("Cookie", self.cookie.clone().unwrap())
            .send()
            .await?;
        let version = resp.text().await?;
        Ok(format!("QBittorrent: {} - {}", self.endpoint.clone(), version))
    }

    async fn get_uploading_torrents(&self) -> Result<Vec<Torrent>> {
        let mut form = HashMap::new();
        form.insert("filter", UPLOADING_STATUS);
        form.insert("sort", "upspeed");
        form.insert("reverse", "true");
        let resp = reqwest::Client::new()
            .post(&format!("{}/torrents/info", self.endpoint))
            .header("Cookie", self.cookie.clone().unwrap())
            .form(&form)
            .send()
            .await?
            .json::<Vec<HashMap<String, Value>>>()
            .await?;
        Ok(resp.iter().map(|t| Torrent {
            name: t.get("name").unwrap().as_str().unwrap().to_string(),
            hash: t.get("hash").unwrap().as_str().unwrap().to_string(),
            size: t.get("size").unwrap().as_u64().unwrap(),
            tracker: t.get("tracker").unwrap().as_str().unwrap().to_string(),
        }).collect())
    }

    async fn get_peers(&self, hash: &str) -> Result<Vec<Peer>> {
        let mut form = HashMap::new();
        form.insert("hash", hash);
        let resp = reqwest::Client::new()
            .post(&format!("{}/sync/torrentPeers", self.endpoint))
            .header("Cookie", self.cookie.clone().unwrap())
            .form(&form)
            .send()
            .await?
            .json::<HashMap<String, Value>>()
            .await?;
        Ok(resp.get("peers").ok_or(
            anyhow::anyhow!("Failed to get peers for torrent {}.", hash)
        )?.as_object().unwrap().iter().map(|(k, v)| Peer {
            address: k.clone(),
            id: v.get("peer_id_client").unwrap().as_str().unwrap().to_string(),
            name: v.get("client").unwrap().as_str().unwrap().to_string(),
            download_speed: v.get("dl_speed").unwrap().as_u64().unwrap(),
            downloaded: v.get("downloaded").unwrap().as_u64().unwrap(),
            upload_speed: v.get("up_speed").unwrap().as_u64().unwrap(),
            uploaded: v.get("uploaded").unwrap().as_u64().unwrap(),
            progress: v.get("progress").unwrap().as_f64().unwrap(),
        }).collect())
    }

    async fn ban_peer(&self, peer: &Peer) -> Result<()> {
        let mut form = HashMap::new();
        form.insert("peers", peer.address.clone());
        reqwest::Client::new()
            .post(&format!("{}/transfer/banPeers", self.endpoint))
            .header("Cookie", self.cookie.clone().unwrap())
            .form(&form)
            .send()
            .await?;
        Ok(())
    }

    async fn ban_clear(&self) -> Result<()> {
        let mut form = HashMap::new();
        form.insert("json", serde_json::json!({"banned_IPs": ""}).to_string());
        reqwest::Client::new()
            .post(&format!("{}/app/setPreferences", self.endpoint))
            .header("Cookie", self.cookie.clone().unwrap())
            .form(&form)
            .send()
            .await?;
        Ok(())
    }
}