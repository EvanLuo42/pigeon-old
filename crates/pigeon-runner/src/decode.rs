use actix_rt::net::TcpStream;
use tokio::io::AsyncReadExt;

use pigeon_proto::errors::ErrorCode;

pub async fn read(stream: &mut TcpStream) -> Result<(u32, Vec<u8>), ErrorCode> {
    let proto_id = stream.read_u32()
        .await
        .map_err(|_| ErrorCode::UnsupportedProto)?;
    let length = stream.read_u32()
        .await
        .map_err(|_| ErrorCode::UnsupportedProto)?;
    let mut buffer = vec![0; length as usize];
    stream.read_exact(&mut buffer)
        .await
        .map_err(|_| ErrorCode::UnsupportedProto)?;
    Ok((proto_id, buffer))
}