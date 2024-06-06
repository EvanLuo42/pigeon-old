#[macro_use] extern crate enum_primitive;

use anyhow::Result;

use crate::network::server::TcpServer;

mod network;
mod error;
mod handlers;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let server = TcpServer::new("127.0.0.1:8080").await?;
    server.listen().await?;
    Ok(())
}