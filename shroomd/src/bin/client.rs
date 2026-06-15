use std::env;

use shroomd::proto::shroom_client::ShroomClient;
use shroomd::proto::HealthCheckRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().expect("Failed to read .env file");

    let address = env::var("CLIENT_ADDRESS")?;
    let port = env::var("CLIENT_PORT")?;
    let url = format!("https://{}:{}", address, port);

    let mut client = ShroomClient::connect(url).await?;
    let response = client.health_check(HealthCheckRequest {}).await?;
    println!("{}", response.get_ref().healthy);
    Ok(())
}
