use tonic::{Request, Response, Status};
use pigeon_auth::auth_proto::{LoginRequest, LoginResponse};
use pigeon_auth::auth_proto::auth_server::Auth;

#[derive(Debug)]
pub(crate) struct AuthService;

#[tonic::async_trait]
impl Auth for AuthService {
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginResponse>, Status> {
        todo!()
    }
}
