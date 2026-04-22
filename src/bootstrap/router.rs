use axum::{
    routing::{get, post},
    Router,
};

use crate::bootstrap::state::AppState;
use crate::handlers::{health, email};

pub fn build(state: AppState) -> Router {
    Router::new().
    // health check endpoint
    route("/health-check", get(health::check)).
    // email sending endpoint
    route("/email", post(email::send)).
    // add shared application state
    with_state(state)
}   