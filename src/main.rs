mod bootstrap;
mod config;
mod domain;
mod errors;
mod handlers;
mod providers;
mod repository;
mod services;

#[tokio::main]
async fn main() {
    if let Err(e) = bootstrap::app::run().await {
        eprintln!("Application error: {e}");
        std::process::exit(1);
    }
}