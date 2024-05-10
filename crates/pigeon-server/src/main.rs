use actix::Actor;
use tokio::net::TcpListener;

use crate::actors::game::{GameActor, Login};

mod actors;

#[actix::main]
async fn main() {
    let game = GameActor::new().start();
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await.expect("Failed to create TcpListener");
    while let Ok((tcp_stream, _addr)) = listener.accept().await {
        // TODO: Error handling
        game.try_send(Login { tcp_stream }).unwrap();
    }
}
