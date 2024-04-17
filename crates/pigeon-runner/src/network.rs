use std::env;
use std::net::SocketAddr;
use actix_rt::net::TcpListener;
use tracing::info;

pub struct Server {
    listener: TcpListener
}

impl Server {
    pub async fn new() -> Server {
        let addr = env::var("HOST_ADDRESS")
            .unwrap_or(String::from("127.0.0.1:8080"));
        let addr: SocketAddr = addr.parse().expect("Invalid host address!");
        info!("Server launched on {}", addr);
        Server {
            listener: TcpListener::bind(addr).await.unwrap(),
        }
    }

    pub async fn run(&self) {
        while let Ok((stream, addr)) = self.listener.accept().await {
            info!("Received a request from {}", addr);
        }
    }
}
