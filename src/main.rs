use std::env;

use anyhow::Result;
use tracing_subscriber::EnvFilter;
use xactor::Actor;
use crate::network::server::{ListenSession, TcpServer};

mod error;
mod protos;
mod network;
mod ecs;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let server = TcpServer::new().start().await?;
    server.call(ListenSession {
        host: env::var("ADDR").unwrap_or("127.0.0.1:8080".into())
    }).await??;
    Ok(())
}