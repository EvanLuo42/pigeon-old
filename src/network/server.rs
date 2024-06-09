use std::collections::HashMap;
use async_trait::async_trait;
use tokio::net::TcpListener;
use tracing::error;
use xactor::{Actor, Addr, Context, Handler, message};
use crate::ecs::game::Game;
use crate::network::session::{ListenPacket, Session};
use crate::error::ServerError;

pub struct TcpServer {
    sessions: HashMap<String, Addr<Session>>,
    game: Addr<Game>,
}

impl TcpServer {
    pub async fn new() -> Result<TcpServer, ServerError> {
       Ok(
           TcpServer {
               sessions: HashMap::new(),
               game: Game::new().start().await?
           }
       )
    }
}

impl Actor for TcpServer {
}

#[message(result = "Result<(), ServerError>")]
pub struct ListenSession {
    pub host: String
}

#[async_trait]
impl Handler<ListenSession> for TcpServer {
    async fn handle(&mut self, ctx: &mut Context<Self>, msg: ListenSession) -> Result<(), ServerError> {
        let listener = TcpListener::bind(msg.host).await?;
        while let Ok((stream, addr)) = listener.accept().await {
            let session = Session::new(stream).start().await?;
            let _session = session.clone();
            let tcp_server = ctx.address();
            let game = self.game.clone();
            tokio::spawn(async move {
                let message = ListenPacket {
                    tcp_server: tcp_server.clone(),
                    game,
                };
                if let Err(e) = _session.call(message).await {
                    tcp_server.call(TerminateSession { ip: addr.to_string() }).await.unwrap();
                    error!("Error when handling request from {}: {}", addr.to_string(), e);
                }
            });
            self.sessions.insert(addr.to_string(), session);
        }
        Ok(())
    }
}

#[message]
pub struct TerminateSession {
    ip: String
}

#[async_trait]
impl Handler<TerminateSession> for TcpServer {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: TerminateSession) {
        self.sessions.remove(&msg.ip);
    }
}