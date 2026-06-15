use std::env;

use shroomd::proto::shroom_client::ShroomClient;
use shroomd::proto::HealthCheckRequest;
use evdev::Device;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let address = env::var("CLIENT_ADDRESS")?;
    let port = env::var("CLIENT_PORT")?;
    let url = format!("http://{}:{}", address, port);

    let mut client = ShroomClient::connect(url).await?;
    let response = client.health_check(HealthCheckRequest {}).await?;
    println!("{}", response.get_ref().healthy);

    // Collect command line arguments
    // TODO list all /dev/input/event*
    // loop over all and choose the one where CLIENT_EXPECTED_DEVICE_NAME if there is none then
    // exit with error
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- /dev/input/eventX");
        std::process::exit(1);
    }

    // Target the specific device path provided
    let device_path = &args[1];
    let mut device = Device::open(device_path)?;

    // Print device name and configuration metadata
    println!("Device name: {}", device.name().unwrap_or("Unknown"));
    println!("Listening for physical events... Press Ctrl+C to exit.\n");

    // Continuously pull hardware event buffers from the kernel
    loop {
        for event in device.fetch_events()? {
            println!(
                "Time: {:?}, Type: {:?}, Code: {}, Value: {}",
                event.timestamp(),
                event.event_type(),
                event.code(),
                event.value()
            );
        }
    }
    Ok(())
}
