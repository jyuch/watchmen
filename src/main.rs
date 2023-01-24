use clap::Parser;
use std::path::PathBuf;
use watchmen::crash_report::write_report;
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

#[tokio::main]
async fn main() {
    let opt = Cli::parse();

    let config = config::read_config(&opt.config).await;
    match config {
        Ok(config) => {
            let result = execute(&config).await;
            match result {
                Ok(execute_result) => {
                    if execute_result.exit_status.code().unwrap_or(0) != 0 {
                        if let Some(mail_config) = &config.mail {
                            let mail_result =
                                mail::notify(&config.watchmen.id, mail_config, &execute_result)
                                    .await;

                            if let Err(e) = mail_result {
                                write_report(&e, &config.watchmen.crash_report).await;
                                std::process::exit(opt.exit_code_on_error);
                            }
                        }
                    }

                    if config.watchmen.passthrough_exit_code.unwrap_or(false) {
                        std::process::exit(execute_result.exit_status.code().unwrap_or(0));
                    }
                }
                Err(e) => {
                    write_report(&e, &config.watchmen.crash_report).await;
                    std::process::exit(opt.exit_code_on_error);
                }
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(opt.exit_code_on_error);
        }
    }
}
