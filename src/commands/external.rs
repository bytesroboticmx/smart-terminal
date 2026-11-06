//! Comandos externos del sistema.

use crate::error::Result;
use crate::shell::command::ExecutionResult;

/// Programa externo y sus argumentos.
#[derive(Debug, Clone)]
pub struct ExternalCommand {
    pub program: String,
    pub args: Vec<String>,
}

/// Ejecuta un programa externo.
pub fn execute(_command: &ExternalCommand) -> Result<ExecutionResult> {
    todo!("ejecutar un proceso externo")
}
