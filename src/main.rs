use hermes_rs::bootstrap::app;

#[tokio::main]
async fn main() {
    if let Err(e) = app::run().await {
        eprintln!("Application error: {e}");
        std::process::exit(1);
    }
}