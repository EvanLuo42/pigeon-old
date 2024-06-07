use async_trait::async_trait;

use crate::error::ServerError;
use crate::handlers::Handler;
use crate::network::session::Session;

pub struct MovementHandler {

}

impl MovementHandler {
    pub fn new() -> MovementHandler {
        MovementHandler {}
    }
}

#[async_trait]
impl Handler for MovementHandler {
    async fn handle(&self, _session: Session) -> Result<(), ServerError> {
        Ok(())
    }
}