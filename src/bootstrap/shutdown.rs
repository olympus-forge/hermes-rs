use tracing::info;

pub async fn signal(){
    let ctrl_c = async {
        tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C handler");
    };
    
    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::{signal, SignalKind};
        
        let mut sigterm = signal(SignalKind::terminate())
        .expect("failed to install SIGTERM handler");
    
       sigterm.recv().await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>(); // never completes on non-unix

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C, shutting down");
        },
        _ = terminate => {
            info!("Received SIGTERM, shutting down");
        }
    }

    info!("Shutdown signal received, exiting");
}