mod console;
mod data;
mod game;
mod models;
mod vision;
mod api;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Spawn server in a separate task
    tokio::spawn(async {
        api::run_server().await;
    });

    if let Err(e) = console::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
