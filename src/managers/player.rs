use std::collections::HashMap;

use tokio::sync::RwLock;

use crate::managers::Manager;
use crate::network::session::Session;

pub struct PlayerManager {
    players: RwLock<HashMap<String, Session>>
}

impl PlayerManager {
    pub async fn login(&self, username: String, session: Session) {
        self.players.write().await.insert(username, session);
    }
}

impl Manager for PlayerManager {
    fn init() -> Self where Self: Sized {
        PlayerManager {
            players: RwLock::new(HashMap::new())
        }
    }
}