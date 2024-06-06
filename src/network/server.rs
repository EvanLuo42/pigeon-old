use tokio::net::{TcpListener};
use tracing::{error, info};

use crate::error::ServerError;
use crate::network::session::Session;

pub struct TcpServer {
    listener: TcpListener
}

impl TcpServer {
    pub async fn new(addr: String) -> Result<TcpServer, ServerError> {
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
                let addr = socket.local_addr().unwrap().to_string();
                let session = Session::new(socket);
                if let Err(e) = session.handle().await {
                    error!("Error when handling request from {}: {}", addr, e)
                }
            });
        }
    }
}
