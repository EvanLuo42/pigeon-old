use async_trait::async_trait;
use tokio::net::TcpStream;
use crate::error::ServerError;
use crate::handlers::Handler;

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
        todo!()
    }
}