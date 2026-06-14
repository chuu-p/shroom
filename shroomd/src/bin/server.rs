use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port: u16 = env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(50051);

    shroomd::run_server(port).await
}
