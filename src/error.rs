use std::io;
use bincode::error::{DecodeError, EncodeError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("IO error")]
    IoError(#[from] io::Error),

    #[error("Decode error: {0}")]
    Decode(#[from] DecodeError),

    #[error("Encode error: {0}")]
    Encode(#[from] EncodeError),

    #[error("Wrong magic number: expected: 47382, actual: {0}")]
    Magic(u16),

    #[error("Handler not exist: {0}")]
    HandlerNotExist(u8),
    
    #[error("Empty request from {0}")]
    EmptyRequest(String)
}