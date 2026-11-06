//! Validación de seguridad de comandos.

/// Veredicto de seguridad sobre un comando.
#[derive(Debug, Clone)]
pub enum SafetyVerdict {
    /// El comando es seguro.
    Allow,
    /// El comando requiere confirmación.
    Confirm(String),
    /// El comando debe bloquearse.
    Block(String),
}

/// Analiza un comando antes de ejecutarlo.
pub fn check(_command: &str) -> SafetyVerdict {
    todo!("analizar la seguridad del comando")
}
