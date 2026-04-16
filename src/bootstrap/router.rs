use axum::{
    routing::{get, post},
    Router,
};

use crate::bootstrap::state::AppState;
use crate::handlers::{health_check, email};

pub fn build(state: AppState) -> Router {
    Router::new().
    // health check endpoint
    route("/health", get(health_check)).
    // email sending endpoint
    route("/email", post(email)).
    // add shared application state
    with_state(state)
}   