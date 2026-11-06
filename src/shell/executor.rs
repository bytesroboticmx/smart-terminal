//! Ejecución de comandos.

use crate::error::Result;
use crate::shell::command::{Command, ExecutionResult};

/// Ejecutor de comandos.
#[derive(Debug, Default)]
pub struct Executor;

impl Executor {
    pub fn new() -> Self {
        Self
    }

    /// Ejecuta un comando y devuelve su resultado.
    pub fn execute(&self, _command: &Command) -> Result<ExecutionResult> {
        todo!("ejecutar un comando")
    }
}
