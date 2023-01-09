use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub async fn write_report<P: AsRef<Path>>(error: &anyhow::Error, report_path: &Option<P>) {
    if let Some(path) = report_path {
        let write_result = write_report_impl(error, path).await;
        if let Err(e) = write_result {
            print_stderr(&e);
            print_stderr(error);
        }
    } else {
        print_stderr(error);
    }
}

pub async fn write_report_impl<P: AsRef<Path>>(
    error: &anyhow::Error,
    report_path: &P,
) -> anyhow::Result<()> {
    let mut report = File::create(report_path).await?;
    let payload = format!("{:?}", error);
    report.write_all(payload.as_bytes()).await?;
    Ok(())
}

fn print_stderr(error: &anyhow::Error) {
    eprintln!("{:?}", error);
}
