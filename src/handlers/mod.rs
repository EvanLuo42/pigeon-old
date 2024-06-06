use async_trait::async_trait;
use enum_primitive::{enum_from_primitive, FromPrimitive};
use tokio::net::TcpStream;
use crate::error::ServerError;
use crate::handlers::chat::ChatHandler;

pub mod chat;

#[async_trait]
pub trait Handler: Send + Sync {
    async fn handle(&self, socket: TcpStream) -> Result<(), ServerError>;
}

enum_from_primitive! {
    pub enum Handlers {
        Chat = 0
    }
}

pub struct HandlerFactory;

impl HandlerFactory {
    pub fn from_id(id: u8) -> Result<Box<dyn Handler>, ServerError> {
        match Handlers::from_u8(id).ok_or(ServerError::HandlerNotExist(id))? {
            Handlers::Chat => Ok(Box::new(ChatHandler::new()))
        }
    }
}