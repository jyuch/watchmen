use crate::config::Config;
use anyhow::Context as _;
use chrono::Local;
use std::process::{ExitStatus, Stdio};
use tokio::fs::File;
use tokio::process::Command;
use tokio::{io, join};

pub struct ExecuteResult {
    pub pid: u32,
    pub start_date: String,
    pub exit_status: ExitStatus,
}

impl ExecuteResult {
    pub fn code(&self) -> i32 {
        self.exit_status.code().unwrap_or(0)
    }
}

pub async fn execute(config: &Config) -> anyhow::Result<ExecuteResult> {
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

    if config.execute.log_dir.is_some() {
        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    } else {
        cmd.stdout(Stdio::null()).stderr(Stdio::null());
    }

    let mut p = cmd.spawn()?;

    let start_date = Local::now();
    let start_date = start_date.format("%Y%m%dT%H%M%S%z").to_string();
    let pid = p.id().unwrap_or(0);

    if let Some(log_dir) = &config.execute.log_dir {
        let log_stdout_name = format!("{start_date}-{pid}-stdout.log");
        let log_stderr_name = format!("{start_date}-{pid}-stderr.log");

        let mut stdout = p.stdout.take().context("Failed to attach stdout")?;
        let mut stderr = p.stderr.take().context("Failed to attach stderr")?;

        let mut log_stdout = File::create(log_dir.join(log_stdout_name)).await?;
        let mut log_stderr = File::create(log_dir.join(log_stderr_name)).await?;

        let o = io::copy(&mut stdout, &mut log_stdout);
        let e = io::copy(&mut stderr, &mut log_stderr);

        let (_, _) = join!(o, e);
    }

    let exit_status = p.wait().await?;
    Ok(ExecuteResult {
        pid,
        start_date,
        exit_status,
    })
}
