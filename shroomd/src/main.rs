mod job;

#[tokio::main]
async fn main() {
    println!("shroomd starting...");

    let j1 = tokio::spawn(async {
        job::run_process("ydotoold", "true", &[]).await;
    });

    let j2 = tokio::spawn(async {
        job::run_process("receiver", "true", &[]).await;
    });

    let j3 = tokio::spawn(async {
        job::run_process("remote", "true", &[]).await;
    });

    let _ = tokio::join!(j1, j2, j3);
}
