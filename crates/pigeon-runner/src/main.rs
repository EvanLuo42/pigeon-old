use dotenv::dotenv;
use tracing::info;
use tracing_subscriber::EnvFilter;
use crate::network::Server;

mod network;

#[actix_rt::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Launching server...");
    let server = Server::new().await;
    server.run().await;
}
