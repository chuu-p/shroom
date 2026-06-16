use std::env;
use std::path::Path;

use evdev::Device;
use shroomd::proto::shroom_client::ShroomClient;
use shroomd::proto::{HealthCheckRequest, KeyEvent, KeyState};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let address = env::var("CLIENT_ADDRESS")?;
    let port = env::var("CLIENT_PORT")?;
    let url = format!("http://{}:{}", address, port);

    let mut client = ShroomClient::connect(url).await?;
    let resp = client.health_check(HealthCheckRequest {}).await?;
    println!("{}", resp.get_ref().healthy);

    let device_path = resolve_device()?;
    let mut device = Device::open(&device_path)?;
    println!(
        "Device: {} ({})",
        device.name().unwrap_or("Unknown"),
        device_path.display()
    );

    let (tx, rx) = mpsc::channel::<KeyEvent>(256);

    std::thread::spawn(move || {
        loop {
            match device.fetch_events() {
                Ok(events) => {
                    for event in events {
                        let state = if event.value() != 0 {
                            KeyState::KeyOn as i32
                        } else {
                            KeyState::KeyOff as i32
                        };
                        let key_event = KeyEvent {
                            key_code: event.code().to_string(),
                            state,
                        };
                        if tx.blocking_send(key_event).is_err() {
                            return;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("evdev error: {}", e);
                    return;
                }
            }
        }
    });

    client.stream_keys(ReceiverStream::new(rx)).await?;
    Ok(())
}

fn resolve_device() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        return Ok(Path::new(&args[1]).to_path_buf());
    }

    let expected = env::var("CLIENT_EXPECTED_DEVICE_NAME").ok();

    for entry in std::fs::read_dir("/dev/input")? {
        let entry = entry?;
        let path = entry.path();
        let fname = path.file_name().unwrap().to_string_lossy().to_string();
        if !fname.starts_with("event") {
            continue;
        }

        if let Some(ref expected_name) = expected {
            if let Ok(dev) = Device::open(&path) {
                if dev.name() == Some(expected_name) {
                    return Ok(path);
                }
            }
        } else {
            return Ok(path);
        }
    }

    if let Some(name) = expected {
        Err(format!("Device with name '{}' not found in /dev/input", name).into())
    } else {
        Err("No input device found in /dev/input".into())
    }
}
