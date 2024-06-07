use async_trait::async_trait;
use enum_primitive::{enum_from_primitive, FromPrimitive};

use crate::error::ServerError;
use crate::handlers::movement::MovementHandler;
use crate::network::session::Session;

pub mod movement;

#[async_trait]
pub trait Handler: Send + Sync {
    async fn handle(&self, socket: Session) -> Result<(), ServerError>;
}

enum_from_primitive! {
    pub enum Handlers {
        Movement = 1,
    }
}

pub struct HandlerFactory;

impl HandlerFactory {
    pub fn from_id(id: u32) -> Result<Box<dyn Handler>, ServerError> {
        match Handlers::from_u8(id as u8).ok_or(ServerError::HandlerNotExist(id))? {
            Handlers::Movement => Ok(Box::new(MovementHandler::new()))
        }
    }
}