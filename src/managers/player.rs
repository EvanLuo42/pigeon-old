use std::collections::HashMap;

use tokio::sync::RwLock;
use tracing::debug;
use crate::error::ServerError;

use crate::managers::{get_manager, Manager};
use crate::network::session::Session;

pub struct PlayerManager {
    pub players: RwLock<HashMap<String, Session>>
}

impl PlayerManager {
    pub async fn login(&self, username: String, session: Session) -> Result<(), ServerError> {
        self.players.write().await.insert(username.clone(), session.clone());
        debug!("Player {} from {} login", username, session.socket.read().await.local_addr()?);
        Ok(())
    }

    pub async fn logout(&self, username: String) -> Result<(), ServerError> {
        self.players.write().await.remove(&username)
            .map(|_| ())
            .ok_or(ServerError::PlayerNotExist(username))
    }
}

impl Manager for PlayerManager {
    fn init() -> Self where Self: Sized {
        PlayerManager {
            players: RwLock::new(HashMap::new())
        }
    }
}