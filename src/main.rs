//! Punto de entrada del binario.

use smart_terminal::app;

fn main() {
    if let Err(error) = app::run() {
        eprintln!("smart-terminal: {error:?}");
        std::process::exit(1);
    }
}
