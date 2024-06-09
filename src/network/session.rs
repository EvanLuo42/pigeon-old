use async_trait::async_trait;
use tokio::net::TcpStream;
use xactor::{Actor, Addr, Context, Handler, message};
use crate::network::server::TcpServer;

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
    pub tcp_server: Addr<TcpServer>
}

#[async_trait]
impl Handler<ListenPacket> for Session {
    async fn handle(&mut self, ctx: &mut Context<Self>, msg: ListenPacket) {
        todo!()
    }
}