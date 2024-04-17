use std::env;
use std::io::Error;
use std::net::SocketAddr;
use actix::Actor;
use actix_rt::net::TcpListener;
use tracing::{error, info};
use pigeon_logic::test::{Print, TestActor};
use pigeon_proto::errors::ErrorCode;
use pigeon_proto::main::MessageWrapper;
use pigeon_proto::test::Test;
use crate::decode::read;

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
        while let Ok((mut stream, addr)) = self.listener.accept().await {
            info!("Received a request from {}", addr);

            let (proto_id, buffer) = match read(&mut stream).await {
                Ok(read) => read,
                Err(e) => {
                    error!("{:?}", e);
                    continue
                }
            };
            
            match proto_id {
                0 => {
                    let addr = TestActor.start();
                    addr.send(Print(buffer)).await.unwrap();
                },
                _ => {
                    error!("{:?}", ErrorCode::UnsupportedProto);
                    continue
                }
            };
        }
    }
}
