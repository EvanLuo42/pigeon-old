use std::sync::Arc;
use bytes::BytesMut;
use prost::Message;
use tokio::io::{AsyncReadExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tracing::{debug};

use crate::error::ServerError;
use crate::error::ServerError::{EmptyRequest, Magic};
use crate::handlers::{HandlerFactory};
use crate::protos::common::{Login, Route};

#[derive(Clone)]
pub struct Session {
    pub socket: Arc<Mutex<TcpStream>>
}

impl Session {
    pub fn new(socket: Arc<Mutex<TcpStream>>) -> Session {
        Session {
            socket
        }
    }

    pub async fn handle(self) -> Result<(), ServerError> {
        let socket = self.socket.clone();
        let mut socket = socket.lock().await;
        let addr = socket.local_addr().unwrap().to_string();
        debug!("Created session with {}", addr);
        // TODO: Calculate size
        let mut buf = BytesMut::zeroed(1);
        let len = socket.read_exact(&mut buf).await?;
        if len == 0 {
            return Err(EmptyRequest(addr))
        }
        let packet = Login::decode(buf)?;
        if packet.magic != 4739283 {
            return Err(Magic(4739283, packet.magic))
        }

        let player_handler = HandlerFactory::from_id(0)?;
        player_handler.handle(self.clone()).await?;
        loop {
            // TODO: Calculate size
            let buf = BytesMut::zeroed(1);
            if len == 0 {
                return Err(EmptyRequest(addr))
            }
            let packet = Route::decode(buf)?;
            let handler = HandlerFactory::from_id(packet.handler)?;
            handler.handle(self.clone()).await?;
        }
    }
}