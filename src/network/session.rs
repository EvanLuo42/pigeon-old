use bytes::BytesMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::{debug, error};

use crate::error::ServerError;
use crate::error::ServerError::{EmptyRequest, Magic};
use crate::handlers::HandlerFactory;
use crate::network::packet::{ErrorPacket, Packet, ResponsePacket, RoutePacket};

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
        debug!("Created session with {}", addr);
        let mut buf = BytesMut::with_capacity(1024);
        let len = self.socket.read_buf(&mut buf).await?;
        if len == 0 {
            return Err(EmptyRequest(addr))
        }

        let packet = RoutePacket::decode(&buf)?;
        if packet.magic != 1234 {
            return Err(Magic(packet.magic))
        }
        let mut socket = self.socket;
        let handler = HandlerFactory::from_id(packet.handler)?;
        tokio::spawn(async move {
            if let Err(e) = handler.handle(&socket).await {
                error!("Error when handling request from {}: {}", addr, e);
                let error = ErrorPacket {
                    error: e.to_string(),
                };
                let encoded_error = error.encode();
                if let Err(e) = encoded_error {
                    error!("{}", e);
                    return
                }
                let encoded_error = encoded_error.unwrap();
                let response = ResponsePacket {
                    pack_id: 0,
                    length: encoded_error.len() as u32,
                };
                let encoded_response = response.encode();
                if let Err(e) = encoded_response {
                    error!("{}", e);
                    return
                }
                let encoded_response = encoded_response.unwrap();
                println!("{}", encoded_response.len());
                socket.write_all(&encoded_response).await.unwrap_or_else(|e| error!("{}", e));
                socket.write_all(&encoded_error).await.unwrap_or_else(|e| error!("{}", e));
            }
        });
        Ok(())
    }
}