mod console;
mod data;
mod game;
mod models;

fn main() {
    if let Err(e) = console::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
