use std::sync::Arc;
use prost::Message;
use tokio::net::{TcpListener, TcpStream};
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
                if let Err(e) = Self::on_request(socket).await {
                    error!("Error when handling request: {}", e);
                }
            });
        }
    }
    
    async fn on_request(socket: TcpStream) -> Result<(), ServerError> {
        let socket = Arc::new(RwLock::new(socket));
        let buf = read_by_len(socket.clone()).await?;
        let player_packet = Login::decode(buf)?;
        if player_packet.magic != 4739283 {
            return Err(Magic(4739283, player_packet.magic))
        }
        if let Err(e) = Session::new(socket.clone(), player_packet.username.clone()).handle().await {
            get_manager::<PlayerManager>().await?.logout(player_packet.username).await?;
            return Err(e)
        }
        Ok(())
    }
}
