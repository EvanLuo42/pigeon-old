use std::sync::Arc;
use std::time::Duration;
use prost::Message;
use tokio::net::TcpStream;
use tokio::sync::RwLock;
use tokio::time::interval;
use tracing::{debug};
use crate::error::ServerError;

use crate::error::ServerError::Magic;
use crate::handlers::{HandlerFactory};
use crate::managers::get_manager;
use crate::managers::player::PlayerManager;
use crate::protos::common::{Route};
use crate::protos::read_by_len;

#[derive(Clone, Debug)]
pub struct Session {
    pub socket: Arc<RwLock<TcpStream>>,
    pub username: String
}

impl Session {
    pub fn new(socket: Arc<RwLock<TcpStream>>, username: String) -> Session {
        Session {
            socket,
            username
        }
    }

    pub async fn handle(self) -> Result<(), ServerError> {
        let socket = self.socket.clone();
        let addr = socket.read().await.local_addr().unwrap().to_string();
        debug!("Created session with {}", addr);

        let manager = get_manager::<PlayerManager>().await?;
        manager.login(self.username.clone(), self.clone()).await?;
        let mut interval = interval(Duration::from_secs(5));
        interval.tick().await;
        loop {
            let buf = read_by_len(socket.clone()).await?;
            let packet = Route::decode(buf)?;
            if packet.magic != 2948374 {
                return Err(Magic(2948374, packet.magic))?
            }
            let handler = HandlerFactory::from_id(packet.handler)?;
            handler.handle(self.clone()).await?;
        }
    }
}