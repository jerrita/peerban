use clap::Parser;
use log::{error, info, warn};

use crate::backend::qb::QBitBackend;
use crate::daemon::Daemon;

mod backend;
mod torrent;
mod peer;
mod rules;
mod daemon;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = "qb")]
    backend: String,
    #[arg(short, long, default_value = "http://127.0.0.1:8080")]
    endpoint: String,
    #[arg(short, long, default_value = "admin:admin")]
    auth: String,
    #[arg(short, long, default_value = "5", help = "Scan interval in seconds.")]
    scan: u64,
    #[arg(short, long, default_value = "false", help = "Clear all bans before start.")]
    clear: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let args = Args::parse();
    if args.backend != "qb" {
        panic!("Invalid backend, only qb is supported now.");
    }

    info!("PeerBan/{} started.", env!("CARGO_PKG_VERSION"));

    let qb = QBitBackend::new(args.endpoint, args.auth);
    let mut daemon = Daemon::new(Box::new(qb), args.scan, args.clear);
    loop {
        match daemon.run().await {
            Ok(_) => (),
            Err(e) => error!("Error: {}", e),
        };
        warn!("Restarting daemon in 5 seconds...");
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}