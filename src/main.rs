use clap::Parser;
use std::path::PathBuf;
use watchmen::config;
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

    let config = config::read_config(&opt.config).await.unwrap();
    println!("{:?}", config);

    execute(&config).await.unwrap();
}
