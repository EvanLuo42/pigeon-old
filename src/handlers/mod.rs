use enum_primitive::{enum_from_primitive, FromPrimitive};
use crate::error::ServerError;
use crate::handlers::chat::ChatHandler;

pub mod chat;

pub trait Handler {

}

enum_from_primitive! {
    pub enum Handlers {
        Chat = 0
    }
}

pub struct HandlerFactory;

impl HandlerFactory {
    pub fn from_id(id: u8) -> Result<Box<dyn Handler>, ServerError> {
        match Handlers::from_u8(id).ok_or(ServerError::Handler(id))? {
            Handlers::Chat => Ok(Box::new(ChatHandler::new()))
        }
    }
}