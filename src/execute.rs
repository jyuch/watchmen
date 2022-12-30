use crate::config::Config;
use std::process::Stdio;
use tokio::fs::File;
use tokio::process::Command;
use tokio::{io, join};

pub async fn execute(config: &Config) -> anyhow::Result<()> {
    let mut p = Command::new(&config.execute.executable)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut stdout = p.stdout.take().unwrap();
    let mut stderr = p.stderr.take().unwrap();

    let mut log_stdout = File::create("stdout.txt").await.unwrap();
    let mut log_stderr = File::create("stderr.txt").await.unwrap();

    let o = io::copy(&mut stdout, &mut log_stdout);
    let e = io::copy(&mut stderr, &mut log_stderr);

    let (_, _) = join!(o, e);

    let result = p.wait().await;
    println!("{:?}", result);

    Ok(())
}
