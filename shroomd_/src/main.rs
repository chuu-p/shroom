mod job;
mod receiver;

#[tokio::main]
async fn main() {
    println!("shroomd starting...");

    let j1 = tokio::spawn(async {
        job::run_process("ydotoold", "sudo", &["ydotoold"]).await;
    });

    let j2 = tokio::spawn(async {
        job::run_loop("receiver", receiver::run).await;
    });

    let j3 = tokio::spawn(async {
        job::run_process(
            "remote",
            "ssh",
            &[
                "-R",
                "9999:localhost:5000",
                "100.100.204.26",
                "evtest /dev/input/event3 | nc localhost 9999",
            ],
        )
        .await;
    });

    let _ = tokio::join!(j1, j2, j3);
}
