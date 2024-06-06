use crate::managers::get_manager;
use async_trait::async_trait;
use tokio::net::TcpStream;
use crate::error::ServerError;
use crate::handlers::Handler;
use crate::managers::chat::ChatManager;

pub struct ChatHandler {
    
}

impl ChatHandler {
    pub fn new() -> ChatHandler {
        ChatHandler {}
    }
}

#[async_trait]
impl Handler for ChatHandler {
    async fn handle(&self, socket: TcpStream) -> Result<(), ServerError> {
        let manager = get_manager::<ChatManager>().await?;
        manager.print();
        Ok(())
    }
}