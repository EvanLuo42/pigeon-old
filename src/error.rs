use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("IO error")]
    IoError(#[from] io::Error),

    #[error("MsgPack decode error: {0}")]
    MsgPackDecode(#[from] rmp_serde::decode::Error),

    #[error("MsgPack encode error: {0}")]
    MsgPackEncode(#[from] rmp_serde::encode::Error),

    #[error("Wrong magic number: expected: 0x1234, actual: {0}")]
    Magic(u16),

    #[error("Handler not exist: {0}")]
    HandlerNotExist(u8),
    
    #[error("Empty request from {0}")]
    EmptyRequest(String),

    #[error("Manager {0} not exist")]
    ManagerNotExist(String),
}