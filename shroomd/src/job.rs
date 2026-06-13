use std::future::Future;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

pub async fn run_process(name: &str, program: &str, args: &[&str]) {
    loop {
        println!("[{}] spawning: {} {}", name, program, args.join(" "));
        let mut child = match Command::new(program)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(c) => c,
            Err(e) => {
                eprintln!("[{}] failed to spawn: {}", name, e);
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                continue;
            }
        };

        let stdout = child.stdout.take().expect("no stdout");
        let stderr = child.stderr.take().expect("no stderr");

        let n1 = name.to_string();
        let n2 = name.to_string();

        let out = tokio::spawn(async move {
            let mut reader = BufReader::new(stdout);
            let mut line = String::new();
            while reader.read_line(&mut line).await.unwrap_or(0) > 0 {
                print!("[{}] {}", n1, line);
                line.clear();
            }
        });

        let err = tokio::spawn(async move {
            let mut reader = BufReader::new(stderr);
            let mut line = String::new();
            while reader.read_line(&mut line).await.unwrap_or(0) > 0 {
                eprint!("[{}] {}", n2, line);
                line.clear();
            }
        });

        let status = child.wait().await;
        println!("[{}] exited with: {:?}", name, status);

        let _ = out.await;
        let _ = err.await;

        println!("[{}] restarting in 1 second...", name);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

pub async fn run_loop<F, Fut>(name: &str, f: F)
where
    F: Fn() -> Fut,
    Fut: Future<Output = ()>,
{
    loop {
        println!("[{}] starting...", name);
        f().await;
        println!("[{}] exited, restarting in 1 second...", name);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
