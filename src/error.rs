use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("IO error: {0}")]
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
    
    #[error("Player {0} not exist")]
    PlayerNotExist(String),
    
    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error)
}