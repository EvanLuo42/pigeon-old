use async_trait::async_trait;

use crate::error::ServerError;
use crate::handlers::Handler;
use crate::managers::get_manager;
use crate::managers::player::PlayerManager;
use crate::network::session::Session;

pub struct PlayerHandler {
    
}

impl PlayerHandler {
    pub fn new() -> PlayerHandler {
        PlayerHandler {}
    }
}

#[async_trait]
impl Handler for PlayerHandler {
    async fn handle(&self, session: Session) -> Result<(), ServerError> {
        let manager = get_manager::<PlayerManager>().await?;
        Err(ServerError::EmptyRequest("111".into()))
    }
}