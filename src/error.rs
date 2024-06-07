use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("IO error")]
    IoError(#[from] io::Error),

    #[error("ProtoBuf decode error: {0}")]
    ProtoBufDecode(#[from] prost::DecodeError),

    #[error("ProtoBuf encode error: {0}")]
    ProtoBufEncode(#[from] prost::EncodeError),

    #[error("Wrong magic number: expected: {0}, actual: {1}")]
    Magic(u32, u32),

    #[error("Handler not exist: {0}")]
    HandlerNotExist(u32),

    #[error("Manager {0} not exist")]
    ManagerNotExist(String),

    #[error("Player from {0} disconnected")]
    Disconnected(String),
    
    #[error("Player {0} not exist")]
    PlayerNotExist(String)
}