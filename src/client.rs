use user_center::user_client::UserClient;
use user_center::LoginRequest;

pub mod user_center {
    tonic::include_proto!("usercenter");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = UserClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(LoginRequest {
        name: "Tonic".into(),
        password: "Tonic".into(),
    });

    let response = client.login(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
