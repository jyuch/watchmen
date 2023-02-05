use crate::config::MailConfig;
use crate::execute::ExecuteResult;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

pub async fn notify(
    id: &String,
    config: &MailConfig,
    execute_result: &ExecuteResult,
) -> anyhow::Result<()> {
    let mut email = Message::builder().from(config.from.parse()?);

    for it in &config.to {
        email = email.to(it.parse()?)
    }
    let email = email
        .subject(format!("Execution failed - {id}"))
        .body(format!(
            "id: {}\r\npid: {}\r\ndate: {}\r\nexit code: {}",
            id,
            execute_result.pid,
            execute_result.start_date,
            execute_result.code(),
        ))?;

    let mailer: AsyncSmtpTransport<Tokio1Executor> = if config.insecure {
        AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&config.server)
            .port(config.port)
            .build()
    } else {
        AsyncSmtpTransport::<Tokio1Executor>::relay(&config.server)?
            .port(config.port)
            .build()
    };

    mailer.send(email).await?;
    Ok(())
}
