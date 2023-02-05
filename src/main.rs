use anyhow::Context;
use clap::Parser;
use std::path::PathBuf;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
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

    /// Log output directory.
    #[clap(long)]
    log: Option<PathBuf>,

    /// Exit code when occurs error
    #[clap(long, default_value = "-1")]
    exit_code_on_error: i32,
}

fn main() {
    let opt = Cli::parse();

    let exit_code = {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(main_internal(&opt))
    };

    std::process::exit(exit_code);
}

async fn main_internal(opt: &Cli) -> i32 {
    let mut layers = vec![];
    let mut guards = vec![];

    {
        let (log_stdout, guard) = tracing_appender::non_blocking(std::io::stdout());
        let log = tracing_subscriber::fmt::layer().with_writer(log_stdout);
        layers.push(log);
        guards.push(guard);
    }

    if let Some(log) = &opt.log {
        let file_appender = tracing_appender::rolling::daily(log, "watchmen.log");
        let (log_file, guard) = tracing_appender::non_blocking(file_appender);
        let log = tracing_subscriber::fmt::layer()
            .with_writer(log_file)
            .with_ansi(false);
        layers.push(log);
        guards.push(guard);
    }

    tracing_subscriber::registry().with(layers).init();

    let config = config::read_config(&opt.config)
        .await
        .with_context(|| format!("Failed to read configuration file {:?}", &opt.config));

    match config {
        Ok(config) => match main_impl(&config).await {
            Ok(i) => i,
            Err(e) => {
                tracing::error!("{e:?}");
                opt.exit_code_on_error
            }
        },
        Err(e) => {
            tracing::error!("{e:?}");
            opt.exit_code_on_error
        }
    }
}

async fn main_impl(config: &Config) -> anyhow::Result<i32> {
    tracing::info!("Process start {:?}", config.execute.executable);

    let result = execute(config)
        .await
        .context("Failed to execute application")?;

    tracing::info!("Process exit with {}", result.code());

    if result.code() != 0 {
        if let Some(mail_config) = &config.mail {
            mail::notify(&config.watchmen.id, mail_config, &result)
                .await
                .context("Failed to send email")?;
        }
    }
    Ok(result.code())
}
