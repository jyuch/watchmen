use std::process::{ExitStatus, Stdio};

use anyhow::Context as _;
use chrono::Local;
use tokio::fs::{remove_file, File};
use tokio::process::Command;
use tokio::{io, join};

use crate::config::Config;

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

    if config.log.is_some() {
        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    } else {
        cmd.stdout(Stdio::null()).stderr(Stdio::null());
    }

    let mut p = cmd.spawn()?;

    let start_date = Local::now();
    let start_date = start_date.format("%Y%m%dT%H%M%S%z").to_string();
    let pid = p.id().unwrap_or(0);

    if let Some(log_config) = &config.log {
        let log_stdout_name = format!("{start_date}-{pid}-stdout.log");
        let log_stderr_name = format!("{start_date}-{pid}-stderr.log");

        let log_stdout_path = log_config.base_dir.join(log_stdout_name);
        let log_stderr_path = log_config.base_dir.join(log_stderr_name);

        let (o, e) = {
            let mut stdout = p.stdout.take().context("Failed to attach stdout")?;
            let mut stderr = p.stderr.take().context("Failed to attach stderr")?;

            let mut log_stdout = File::create(&log_stdout_path).await?;
            let mut log_stderr = File::create(&log_stderr_path).await?;

            let o = io::copy(&mut stdout, &mut log_stdout);
            let e = io::copy(&mut stderr, &mut log_stderr);

            join!(o, e)
        };

        if log_config.remain_only_exists && o? == 0 {
            remove_file(log_stdout_path).await?;
        }

        if log_config.remain_only_exists && e? == 0 {
            remove_file(log_stderr_path).await?;
        }
    }

    let exit_status = p.wait().await?;
    Ok(ExecuteResult {
        pid,
        start_date,
        exit_status,
    })
}
