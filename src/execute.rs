use crate::config::Config;
use crate::error::ExecuteError;
use std::process::{ExitStatus, Stdio};
use tokio::fs::File;
use tokio::process::Command;
use tokio::{io, join};

pub async fn execute(config: &Config) -> anyhow::Result<ExitStatus> {
    let mut cmd = Command::new(&config.execute.executable);

    if let Some(param) = &config.execute.param {
        cmd.args(param);
    }

    if let Some(current_dir) = &config.execute.current_dir {
        cmd.current_dir(current_dir);
    }

    if let Some(env) = &config.execute.env {
        for it in env {
            cmd.env(&it.key, &it.value);
        }
    }

    let mut p = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;

    let mut stdout = p.stdout.take().ok_or(ExecuteError::BrokenStdioPipeError)?;
    let mut stderr = p.stderr.take().ok_or(ExecuteError::BrokenStdioPipeError)?;

    let mut log_stdout = File::create("stdout.txt").await?;
    let mut log_stderr = File::create("stderr.txt").await?;

    let o = io::copy(&mut stdout, &mut log_stdout);
    let e = io::copy(&mut stderr, &mut log_stderr);

    let (_, _) = join!(o, e);

    let exit = p.wait().await?;
    Ok(exit)
}
