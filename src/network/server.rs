use std::collections::HashMap;
use async_trait::async_trait;
use tokio::net::TcpListener;
use tracing::error;
use xactor::{Actor, Addr, Context, Handler, message};
use crate::ecs::game::{Game, InitializeGame};
use crate::network::session::{ListenPacket, Session};
use crate::error::ServerError;

pub struct TcpServer {
    sessions: HashMap<String, Addr<Session>>,
    game: Option<Addr<Game>>,
}

impl TcpServer {
    pub async fn new() -> Result<Addr<TcpServer>, ServerError> {
        let tcp_server = TcpServer {
            sessions: HashMap::new(),
            game: None,
        }.start().await?;
        let game = Game::new().start().await?;
        tcp_server.call(InitializeServer {
            game: game.clone()
        }).await?;
        game.call(InitializeGame {
            tcp_server: tcp_server.clone()
        }).await?;
        Ok(tcp_server)
    }
}

impl Actor for TcpServer {
}

#[message]
pub struct InitializeServer {
    pub game: Addr<Game>
}

#[async_trait]
impl Handler<InitializeServer> for TcpServer {
    async fn handle(&mut self, ctx: &mut Context<Self>, msg: InitializeServer) {
        self.game = Some(msg.game)
    }
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
                    game: game.unwrap(),
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