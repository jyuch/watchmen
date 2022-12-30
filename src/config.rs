use serde::Deserialize;
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub execute: Execute,
}

#[derive(Debug, Deserialize)]
pub struct Execute {
    pub executable: PathBuf,
}

pub async fn read_config<P: AsRef<Path>>(path: P) -> anyhow::Result<Config> {
    let mut f = File::open(path).await?;
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer).await?;

    let config = toml::de::from_slice(&buffer)?;
    Ok(config)
}
