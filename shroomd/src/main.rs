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
        job::run_process("remote", "true", &[]).await;
    });

    let _ = tokio::join!(j1, j2, j3);
}
