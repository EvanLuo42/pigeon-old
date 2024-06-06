use tokio::net::{TcpListener, ToSocketAddrs};
use tracing::info;

use crate::error::ServerError;
use crate::network::session::Session;

pub struct TcpServer {
    listener: TcpListener
}

impl TcpServer {
    pub async fn new(addr: &str) -> Result<TcpServer, ServerError> {
        info!("Creating TcpServer on {}...", addr);
        Ok(
            TcpServer {
                listener: TcpListener::bind(addr).await?,
            }
        )
    }

    pub async fn listen(&self) -> Result<(), ServerError> {
        info!("Listening incoming requests...");
        loop {
            let (socket, _addr) = self.listener.accept().await?;
            tokio::spawn(async move {
                let mut session = Session::new(socket);
                session.handle().await.unwrap();
            });
        }
    }
}
