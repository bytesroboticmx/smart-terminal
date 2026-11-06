//! Errores de la aplicación.

/// Error general de smart-terminal.
#[derive(Debug)]
pub enum Error {
    /// Error de configuración.
    Config(String),
    /// Error de entrada/salida.
    Io(std::io::Error),
    /// Error del motor de IA.
    Ai(String),
    /// Error al ejecutar un comando.
    Execution(String),
    /// Funcionalidad pendiente de implementar.
    NotImplemented(&'static str),
}

/// Alias de resultado usado en toda la aplicación.
pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Config(message) => write!(f, "configuración: {message}"),
            Self::Io(error) => write!(f, "E/S: {error}"),
            Self::Ai(message) => write!(f, "IA: {message}"),
            Self::Execution(message) => write!(f, "ejecución: {message}"),
            Self::NotImplemented(what) => write!(f, "pendiente de implementar: {what}"),
        }
    }
}

impl std::error::Error for Error {}
