use axum::{
    extract::State,
    Json,
};
use serde::{Serialize, Deserialize};

use crate::bootstrap::state::AppState;
use crate::services::{email_service};

#[derive(Debug, Deserialize)]
pub struct EmailRequest {
   pub to: String,
   pub subject: String,
   pub body: String,
}

#[derive(Debug, Serialize)]
pub struct EmailResponse {
    pub message: String,
    pub success: bool,
}

pub async fn send(
    state: State<AppState>,
    Json(payload): Json<EmailRequest>,
) -> Json<EmailResponse> {
   println!(
        "sending email to: {}, subject: {}, body: {}",
        payload.to, payload.subject, payload.body
    );

    let result  = email_service::send_email(&state, payload).await;

    match result {
        Ok(_) => Json(EmailResponse {
            message: "Email sent successfully".into(),
            success: true,
        }),
        Err(_) => Json(EmailResponse {
            message: "Failed to send email".into(),
            success: false,
        }),
    }
}