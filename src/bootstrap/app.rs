use anyhow::Result;

use crate::bootstrap::{logger, router, server, shutdown, state::AppState};
use crate::config::settings::Settings;

pub async fn run() -> Result<()> {
    // load configuration settings
    let settings = Settings::new()?;

    // initialize logger
    logger::init(&settings);

    // shared application state
    let state = AppState::new(settings.clone());

    // build axum router
    let app = router::build(state);
    
    // run http server
    server::start(app, &settings).await?;

    Ok(())
}