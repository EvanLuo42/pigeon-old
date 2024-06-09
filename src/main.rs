#[macro_use] extern crate enum_primitive;

use std::env;

use anyhow::Result;
use tracing_subscriber::EnvFilter;
use xactor::Actor;
use crate::network::server::{ListenSession, TcpServer};

mod error;
mod protos;
mod network;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let server = TcpServer::new().start().await?;
    server.call(ListenSession {
        host: "0.0.0.0:8080".into()
    }).await??;
    Ok(())
}