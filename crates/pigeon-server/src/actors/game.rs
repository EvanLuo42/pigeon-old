use actix::{Actor, Context, Handler, Message};
use tokio::net::TcpStream;

use crate::actors::player::PlayerActor;

pub struct GameActor {
    players: Vec<PlayerActor>
}

impl GameActor {
    pub fn new() -> Self {
        Self {
            players: Vec::new()
        }
    }
}

impl Actor for GameActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Login {
    pub(crate) tcp_stream: TcpStream
}

impl Handler<Login> for GameActor {
    type Result = ();

    fn handle(&mut self, msg: Login, _ctx: &mut Self::Context) -> Self::Result {
        println!("{}", msg.tcp_stream.local_addr().unwrap().ip());
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Logout;
