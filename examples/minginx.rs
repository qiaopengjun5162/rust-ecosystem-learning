use std::sync::Arc;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::{
    io,
    net::{TcpListener, TcpStream},
};
use tracing::{info, level_filters::LevelFilter, warn};
use tracing_subscriber::{Layer as _, fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    listen_addr: String,
    upstream_addr: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // proxy client traffic to upstream
    let layer = Layer::new().pretty().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let config = resolve_config();
    let config = Arc::new(config);
    info!("Upstream: {}", config.upstream_addr);
    info!("Listen: {}", config.listen_addr);

    let listener = TcpListener::bind(&config.listen_addr).await?;
    loop {
        let (client, addr) = listener.accept().await?;
        // let cloned_config = config.clone(); 阅读代码 方便使用 Arc::clone
        let cloned_config = Arc::clone(&config);
        info!("Accept connection from {}", addr);
        tokio::spawn(async move {
            let upstream = TcpStream::connect(&cloned_config.upstream_addr).await?;
            proxy(client, upstream).await?;
            Ok::<(), anyhow::Error>(())
        });
    }
}

async fn proxy(mut client: TcpStream, mut upstream: TcpStream) -> Result<()> {
    let (mut client_rd, mut client_wr) = client.split();
    let (mut upstream_rd, mut upstream_wr) = upstream.split();

    let client_to_upstream = io::copy(&mut client_rd, &mut upstream_wr);

    let upstream_to_client = io::copy(&mut upstream_rd, &mut client_wr);

    if let Err(e) = tokio::try_join!(client_to_upstream, upstream_to_client) {
        warn!("Error in proxy: {}", e)
    }
    Ok(())
}

fn resolve_config() -> Config {
    Config {
        listen_addr: "0.0.0.0:8081".to_string(),
        upstream_addr: "0.0.0.0:8080".to_string(),
    }
}
