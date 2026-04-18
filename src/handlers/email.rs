use axum::{
    extract::State,
    Json,
};
use serde::{Serialize, Deserialize};


use crate::bootstrap::state::AppState;

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
    _state: State<AppState>,
    Json(payload): Json<EmailRequest>,
) -> Json<EmailResponse> {
   println!(
        "sending email to: {}, subject: {}, body: {}",
        payload.to, payload.subject, payload.body
    );
    Json(EmailResponse {
        message: "Email sent successfully".into(),
        success: true,
    })
}