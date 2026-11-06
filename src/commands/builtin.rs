//! Comandos internos de la terminal.

/// Comandos gestionados por la propia terminal.
#[derive(Debug, Clone)]
pub enum BuiltinCommand {
    Help,
    Clear,
    History,
    Config,
    Exit,
}

/// Reconoce un comando interno por su nombre.
pub fn parse(_name: &str) -> Option<BuiltinCommand> {
    todo!("reconocer comandos internos")
}
