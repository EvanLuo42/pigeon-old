use pigeon_auth::auth_proto::auth_client::AuthClient;

#[tokio::main]
async fn main() {
    let mut client = AuthClient::connect("").await.unwrap();
}
