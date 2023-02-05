use clap::Parser;
use std::path::PathBuf;
use watchmen::config::Config;
use watchmen::execute::execute;
use watchmen::{config, mail};

#[derive(Parser, Debug)]
#[clap(bin_name = "watchmen")]
#[clap(version, about)]
struct Cli {
    /// Configuration file
    #[clap(long)]
    config: PathBuf,

    /// Exit code when occurs error
    #[clap(long, default_value = "-1")]
    exit_code_on_error: i32,
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let exit_code = rt.block_on(main_internal());
    std::process::exit(exit_code);
}

async fn main_internal() -> i32 {
    let opt = Cli::parse();
    let config = config::read_config(&opt.config).await;
    match config {
        Ok(config) => match main_impl(&config).await {
            Ok(i) => i,
            Err(e) => {
                eprintln!("{e}");
                opt.exit_code_on_error
            }
        },
        Err(e) => {
            eprintln!("{e}");
            opt.exit_code_on_error
        }
    }
}

async fn main_impl(config: &Config) -> anyhow::Result<i32> {
    let result = execute(config).await?;
    if result.code() != 0 {
        if let Some(mail_config) = &config.mail {
            mail::notify(&config.watchmen.id, mail_config, &result).await?;
        }
    }
    Ok(result.code())
}
