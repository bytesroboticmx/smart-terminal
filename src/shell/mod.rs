//! Modelo del shell y ejecución de comandos.

pub mod command;
pub mod executor;
pub mod history;
pub mod parser;

pub use command::{Command, ExecutionResult};
pub use executor::Executor;
pub use history::History;
pub use parser::parse;
