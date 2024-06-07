use std::sync::Arc;

use bytes::BytesMut;
use prost::Message;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::RwLock;

use crate::error::ServerError;
use crate::protos::common::Length;

pub mod common {
    include!(concat!(env!("OUT_DIR"), "/common.rs"));
}

pub async fn read_by_len(socket: Arc<RwLock<TcpStream>>) -> Result<BytesMut, ServerError> {
    let mut socket = socket.write().await;

    let mut buf = BytesMut::zeroed(2);
    socket.read_exact(&mut buf).await?;

    let mut buf = BytesMut::zeroed(Length::decode(buf)?.length as usize);
    socket.read_exact(&mut buf).await?;
    Ok(buf)
}