use clap::Parser;
use std::path::PathBuf;
use watchmen::config;
use watchmen::crash_report::write_report;
use watchmen::execute::execute;

#[derive(Parser, Debug)]
#[clap(bin_name = "watchmen")]
#[clap(version, about)]
struct Cli {
    /// Configuration file.
    #[clap(long)]
    config: PathBuf,
}

#[tokio::main]
async fn main() {
    let opt = Cli::parse();

    let config = config::read_config(&opt.config).await;
    match config {
        Ok(config) => {
            let result = execute(&config).await;
            match result {
                Ok(_exit) => {}
                Err(e) => {
                    write_report(&e, &config.watchmen.crash_report).await;
                }
            }
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
