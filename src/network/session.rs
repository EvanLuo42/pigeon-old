use async_trait::async_trait;
use tokio::net::TcpStream;
use xactor::{Actor, Addr, Context, Handler, message};
use crate::ecs::game::Game;
use crate::network::server::TcpServer;
use crate::protos::read_by_len;

pub struct Session {
    tcp_stream: TcpStream
}

impl Session {
    pub fn new(tcp_stream: TcpStream) -> Session {
        Session {
            tcp_stream
        }
    }
}

impl Actor for Session {
}

#[message]
pub struct ListenPacket {
    pub tcp_server: Addr<TcpServer>,
    pub game: Addr<Game>
}

#[async_trait]
impl Handler<ListenPacket> for Session {
    async fn handle(&mut self, ctx: &mut Context<Self>, msg: ListenPacket) {
        loop {
            let buf = read_by_len(&mut self.tcp_stream).await;
            
        }
    }
}