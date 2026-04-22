use anyhow::{Result, anyhow};
use lettre::{
    message::header::ContentType,
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport,
    AsyncTransport,
    Message,
    Tokio1Executor,
};

use crate::bootstrap::state::AppState;
use crate::handlers::email::EmailRequest;

pub async fn send_email(
    state: &AppState,
    req: EmailRequest,
) -> Result<()> {
    // Implementation for sending email
    let email = &state.settings.email;
    let smtp = &email.smtp;

    // build email message
    let email = Message::builder()
        .from(format!("{}<{}>", email.from_name, email.from_email).parse()?)
        .to(req.to.parse()?)
        .subject(req.subject)
        .header(ContentType::TEXT_HTML)
        .body(req.body.to_string())?;

    // create SMTP transport
    let mailer = if smtp.username.is_empty(){
        AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&smtp.host)
            .port(smtp.port)
            .build()
    } else {
        let creds = Credentials::new(
            smtp.username.clone(), 
            smtp.password.clone(),
        );

        AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp.host)?
            .credentials(creds)
            .port(smtp.port)
            .build()
    };

    // send email
    mailer.send(email)
        .await
        .map_err(|e| anyhow!("smtp send email failed: {}",e))?;

    Ok(())
}