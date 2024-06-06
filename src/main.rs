#[macro_use] extern crate enum_primitive;

use std::env;

use anyhow::Result;
use tracing_subscriber::EnvFilter;

use crate::network::server::TcpServer;

mod network;
mod error;
mod handlers;
mod managers;
mod protos;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    managers::init().await?;
    let server = TcpServer::new(
        env::var("ADDR").unwrap_or(String::from("127.0.0.1:8080"))).await?;
    server.listen().await?;
    Ok(())
}