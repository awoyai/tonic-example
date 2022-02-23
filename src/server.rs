use tonic::{transport::Server, Request, Response, Status};

use user_center::user_server::{User, UserServer};
use user_center::{LoginReply, LoginRequest};

pub mod user_center {
    tonic::include_proto!("usercenter");
}

#[derive(Debug, Default)]
pub struct MyUser {}

#[tonic::async_trait]
impl User for MyUser {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginReply>, Status> {
        println!("Got a request: {:?}", request);
        let req = request.into_inner();
        let reply = user_center::LoginReply {
            message: format!("Hello {}!, your password is {}", req.name, req.password).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let user = MyUser::default();

    Server::builder()
        .add_service(UserServer::new(user))
        .serve(addr)
        .await?;

    Ok(())
}