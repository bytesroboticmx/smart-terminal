//! Tema y colores de la interfaz.

/// Tema visual de la terminal.
#[derive(Debug, Clone)]
pub struct Theme {
    pub prompt_color: String,
    pub error_color: String,
    pub success_color: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            prompt_color: "cyan".to_string(),
            error_color: "red".to_string(),
            success_color: "green".to_string(),
        }
    }
}
