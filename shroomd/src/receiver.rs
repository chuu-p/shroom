use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpListener;
use tokio::process::Command;

pub async fn run() {
    let listener = TcpListener::bind("0.0.0.0:5000")
        .await
        .expect("failed to bind port 5000");
    println!("[receiver] listening on port 5000");

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                println!("[receiver] connection from {}", addr);
                tokio::spawn(handle_client(stream));
            }
            Err(e) => {
                eprintln!("[receiver] accept error: {}", e);
            }
        }
    }
}

async fn handle_client(stream: tokio::net::TcpStream) {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    loop {
        line.clear();
        match reader.read_line(&mut line).await {
            Ok(0) => break,
            Ok(_) => {
                if let Some((code, value)) = parse_event(&line) {
                    let ydotool = match value {
                        1 => format!("{}:1", code),
                        0 => format!("{}:0", code),
                        _ => continue,
                    };
                    if let Err(e) = Command::new("ydotool")
                        .args(["key", &ydotool])
                        .spawn()
                    {
                        eprintln!("[receiver] ydotool error: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("[receiver] read error: {}", e);
                break;
            }
        }
    }
}

fn parse_event(line: &str) -> Option<(u32, u8)> {
    let code_start = line.find("code ")?;
    let rest_after_code = &line[code_start + 5..];
    let code_end = rest_after_code.find(|c: char| !c.is_ascii_digit())?;
    let code: u32 = rest_after_code[..code_end].parse().ok()?;

    let value_start = line.find("value ")?;
    let rest_after_value = &line[value_start + 6..];
    let value_end = rest_after_value.find(|c: char| !c.is_ascii_digit())?;
    let value: u8 = rest_after_value[..value_end].parse().ok()?;

    if value > 2 {
        return None;
    }

    Some((code, value))
}
