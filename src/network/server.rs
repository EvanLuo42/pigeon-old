use std::sync::Arc;
use prost::Message;
use tokio::net::{TcpListener};
use tokio::sync::RwLock;
use tracing::{error, info};

use crate::error::ServerError;
use crate::error::ServerError::Magic;
use crate::managers::get_manager;
use crate::managers::player::PlayerManager;
use crate::network::session::Session;
use crate::protos::common::Login;
use crate::protos::read_by_len;

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
                let socket = Arc::new(RwLock::new(socket));
                let buf = read_by_len(socket.clone()).await;
                if let Ok(buf) = buf {
                    let player_packet = Login::decode(buf);
                    if let Ok(player_packet) = player_packet {
                        if player_packet.magic != 4739283 {
                            return Err(Magic(4739283, player_packet.magic))
                        }
                        if let Err(e) = Session::new(socket.clone(), player_packet.username.clone()).handle().await {
                            if let Err(e) = get_manager::<PlayerManager>().await?.logout(player_packet.username).await {
                                error!("Error when handling request from {}: {}", addr, e);
                            }
                            error!("Error when handling request from {}: {}", addr, e);
                        }
                    }
                }
                Ok(())
            });
        }
    }
}
