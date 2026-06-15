use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().expect("Failed to read .env file");

    let address = env::var("SERVER_ADDRESS")?;
    let port = env::var("SERVER_PORT")?.parse()?;

    shroomd::run_server(address, port).await
}
