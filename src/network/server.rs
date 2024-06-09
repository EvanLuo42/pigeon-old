use async_trait::async_trait;
use tokio::net::TcpListener;
use tracing::error;
use xactor::{Actor, Addr, Context, Handler, message};
use crate::ecs::game::Game;
use crate::network::session::{ListenPacket, Session};
use crate::error::ServerError;

pub struct TcpServer {
    sessions: Vec<Addr<Session>>,
    game: Game,
}

impl TcpServer {
    pub fn new() -> TcpServer {
        TcpServer {
            sessions: vec![],
            game: Game::new()
        }
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
            tokio::spawn(async move {
                let message = ListenPacket {
                    tcp_server
                };
                if let Err(e) = _session.call(message).await {
                    error!("Error when handling request from {}: {}", addr.to_string(), e);
                }
            });
            self.sessions.push(session);
        }
        Ok(())
    }
}