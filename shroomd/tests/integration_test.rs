use std::net::TcpListener;
use std::process::Command;
use std::time::Duration;

use shroomd::proto::shroom_client::ShroomClient;
use shroomd::proto::HealthCheckRequest;

fn free_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    port
}

#[tokio::test]
async fn health_check_works() {
    let port = free_port();

    tokio::spawn(async move {
        shroomd::run_server("127.0.0.1".to_string(), port).await.unwrap();
    });

    tokio::time::sleep(Duration::from_millis(200)).await;

    let addr = format!("http://127.0.0.1:{}", port);
    let mut client = ShroomClient::connect(addr).await.unwrap();
    let response = client.health_check(HealthCheckRequest {}).await.unwrap();
    assert!(response.get_ref().healthy);
}

#[test]
fn server_client_health_check() {
    let port = free_port();

    let mut server = Command::new(env!("CARGO_BIN_EXE_shroomd-server"))
        .arg(port.to_string())
        .spawn()
        .expect("failed to start server");

    std::thread::sleep(Duration::from_millis(500));

    let output = Command::new(env!("CARGO_BIN_EXE_shroomd-client"))
        .arg(format!("http://127.0.0.1:{}", port))
        .output()
        .expect("failed to run client");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "true");

    // Check that server is still running
    assert!(
        server.try_wait().unwrap().is_none(),
        "server exited unexpectedly"
    );

    drop(server);
}
