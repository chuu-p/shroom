use std::env;

use shroomd::proto::shroom_client::ShroomClient;
use shroomd::proto::HealthCheckRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "http://0.0.0.0:50051".to_string());

    let mut client = ShroomClient::connect(addr).await?;
    let response = client.health_check(HealthCheckRequest {}).await?;
    println!("{}", response.get_ref().healthy);
    Ok(())
}
