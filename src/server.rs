use tonic::{transport::Server, Request, Response, Status};

use trading::trade_server::{Trade, TradeServer};
use trading::{TradeReply, TradeRequest};
use user_center::user_server::{User, UserServer};
use user_center::{LoginReply, LoginRequest};

pub mod user_center {
    tonic::include_proto!("usercenter");
}

#[derive(Debug, Default)]
pub struct MyUser {}

#[tonic::async_trait]
impl User for MyUser {
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginReply>, Status> {
        println!("Got a request: {:?}", request);
        let req = request.into_inner();
        let reply = user_center::LoginReply {
            message: format!("Hello {}!, your password is {}", req.name, req.password).into(),
        };

        Ok(Response::new(reply))
    }
}

pub mod trading {
    tonic::include_proto!("trading");
}

#[derive(Debug, Default)]
pub struct Trader {}

#[tonic::async_trait]
impl Trade for Trader {
    async fn trade(&self, request: Request<TradeRequest>) -> Result<Response<TradeReply>, Status> {
        println!("request: {:?}", request);
        let req = request.into_inner();
        let reply = trading::TradeReply {
            message: format!(
                "trading successful!, from: {}, to: {}, amount: {}",
                req.from, req.to, req.amount,
            )
            .into(),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let user = MyUser::default();

    Server::builder()
        .accept_http1(true)
        .add_service(UserServer::new(user))
        .add_service(TradeServer::new(Trader::default()))
        .serve(addr)
        .await?;

    Ok(())
}
