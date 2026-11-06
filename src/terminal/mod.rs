//! Capa de terminal: entrada, salida y estado de sesión.

pub mod input;
pub mod output;
pub mod session;

pub use input::InputReader;
pub use output::OutputWriter;
pub use session::SessionState;
