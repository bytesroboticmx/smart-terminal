//! Modelo de comandos.

/// Comando que puede ejecutar la terminal.
#[derive(Debug, Clone)]
pub enum Command {
    /// Comando interno de la terminal.
    Builtin(String),
    /// Programa externo con sus argumentos.
    External { program: String, args: Vec<String> },
    /// Petición en lenguaje natural para el motor de IA.
    Ai(String),
}

/// Resultado de ejecutar un comando.
#[derive(Debug, Clone, Default)]
pub struct ExecutionResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
}
