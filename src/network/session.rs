use bytes::BytesMut;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

use crate::error::ServerError;
use crate::error::ServerError::Magic;
use crate::handlers::HandlerFactory;
use crate::network::packet::RequestPacket;

pub struct Session {
    socket: TcpStream
}

impl Session {
    pub fn new(socket: TcpStream) -> Session {
        Session {
            socket
        }
    }

    pub async fn handle(&mut self) -> Result<(), ServerError> {
        let mut buf = BytesMut::with_capacity(1024);
        loop {
            let n = self.socket.read_buf(&mut buf).await?;

            if n == 0 {
                break;
            }

            if let Ok(packet) = RequestPacket::decode(&buf) {
                if packet.magic != 47382 {
                    return Err(Magic(packet.magic))
                }
                let handler = HandlerFactory::from_id(packet.handler)?;
                buf.clear();
            }
        }
        Ok(())
    }
}