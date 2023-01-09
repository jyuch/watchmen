use serde::Deserialize;
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub watchmen: WatchmenConfig,
    pub execute: ExecuteConfig,
}

#[derive(Debug, Deserialize)]
pub struct WatchmenConfig {
    pub crash_report: Option<PathBuf>,
    pub passthrough_exit_code: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ExecuteConfig {
    pub executable: PathBuf,
    pub current_dir: Option<String>,
    pub param: Option<Vec<String>>,
    pub env: Option<Vec<EnvConfig>>,
    pub log_dir: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
pub struct EnvConfig {
    pub key: String,
    pub value: String,
}

pub async fn read_config<P: AsRef<Path>>(path: P) -> anyhow::Result<Config> {
    let mut f = File::open(path).await?;
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer).await?;

    let config = toml::de::from_slice(&buffer)?;
    Ok(config)
}
