use bytes::BytesMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::error;

use crate::error::ServerError;
use crate::error::ServerError::{EmptyRequest, Magic};
use crate::handlers::HandlerFactory;
use crate::network::packet::Packet;

pub struct Session {
    socket: TcpStream
}

impl Session {
    pub fn new(socket: TcpStream) -> Session {
        Session {
            socket
        }
    }

    pub async fn handle(mut self) -> Result<(), ServerError> {
        let addr = self.socket.local_addr().unwrap().to_string();
        let mut buf = BytesMut::with_capacity(1024);
        let len = self.socket.read_buf(&mut buf).await?;
        if len == 0 {
            return Err(EmptyRequest(addr))
        }
        if let Ok(packet) = Packet::decode(&buf) {
            if packet.magic != 47382 {
                return Err(Magic(packet.magic))
            }
            let socket = self.socket;
            let handler = HandlerFactory::from_id(packet.handler)?;
            tokio::spawn(async move {
                match handler.handle(socket).await {
                    Ok((p, mut stream)) => {
                        if let Some(packet) = p {
                            let encoded = packet.encode();
                            if let Err(ref e) = encoded {
                                error!("Error when handling request from {}: {}", addr, e)
                            }
                            stream.write_all(&encoded.unwrap()).await
                                .unwrap_or_else(|e| error!("Error when handling request from {}: {}", addr, e));
                        }
                    }
                    Err(e) =>
                        error!("Error when handling request from {}: {}", addr, e)
                }
            });
            buf.clear();
        }
        Ok(())
    }
}