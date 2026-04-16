use anyhow::Result;
use axum::Router;
use axum::serve::Listener;
use tokio::net::TcpListener;
use tracing::info;

use crate::bootstrap::shutdown;
use crate::config::settings::Settings;

pub async fn start(app: Router, setting: &Settings) -> Result<()> {
    let addr = format!(
        "{}:{}", setting.server.host, setting.server.port
    );

    let listener = TcpListener::bind(&addr).await?;

    info!(
        service = %setting.app.app_name,
        version = %setting.app.version,
        host = %setting.server.host,
        port = setting.server.port,
        "Starting http server"
    );

    axum::serve(
        listener, 
        app
    )
    .with_graceful_shutdown(shutdown::signal())
    .await?;
    
    Ok(())
}
