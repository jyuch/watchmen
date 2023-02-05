use serde::Deserialize;
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

fn f() -> bool {
    false
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub watchmen: WatchmenConfig,
    pub execute: ExecuteConfig,
    pub log: Option<LogConfig>,
    pub mail: Option<MailConfig>,
}

#[derive(Debug, Deserialize)]
pub struct WatchmenConfig {
    pub id: String,
    pub passthrough_exit_code: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ExecuteConfig {
    pub executable: PathBuf,
    pub current_dir: Option<String>,
    pub param: Option<Vec<String>>,
    pub env: Option<Vec<EnvConfig>>,
}

#[derive(Debug, Deserialize)]
pub struct EnvConfig {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    pub base_dir: PathBuf,
    #[serde(default = "f")]
    pub remain_only_exists: bool,
}

#[derive(Debug, Deserialize)]
pub struct MailConfig {
    #[serde(default = "f")]
    pub insecure: bool,
    pub server: String,
    pub port: u16,
    pub from: String,
    pub to: Vec<String>,
}

pub async fn read_config<P: AsRef<Path>>(path: P) -> anyhow::Result<Config> {
    let mut f = File::open(path).await?;
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer).await?;

    let config = toml::de::from_slice(&buffer)?;
    Ok(config)
}
