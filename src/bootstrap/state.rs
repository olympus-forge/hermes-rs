use std::sync::Arc;

use crate::config::settings::Settings;

#[derive(Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,
}

impl AppState {
    pub fn new(settings: Settings) -> Self {
        Self { 
            settings: Arc::new(settings), 
        }
    }
}