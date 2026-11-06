//! Detección de intenciones.

/// Intención detectada en la entrada del usuario.
#[derive(Debug, Clone)]
pub enum Intent {
    /// Conversación general.
    Chat,
    /// Explicación de un comando o concepto.
    Explain(String),
    /// Generación de un comando a partir de una descripción.
    GenerateCommand { description: String },
    /// Ayuda para corregir un comando que falló.
    FixError { command: String, error: String },
}
