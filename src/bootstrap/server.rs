use anyhow::Result;
use axum::Router;
use tokio::net::TcpListener;
use tracing::info;

use crate::bootstrap::shutdown;
use crate::config::settings::Settings;

pub async fn start(app: Router, setting: &Settings) -> Result<()> {
    let addr = format!(
        "{}:{}", setting.app.app_host, setting.app.app_port
    );

    let listener = TcpListener::bind(&addr).await?;

    info!(
        service = %setting.app.app_name,
        version = %setting.app.app_version,
        host = %setting.app.app_host,
        port = setting.app.app_port,
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
