//! Prompt visual.

/// Prompt mostrado al usuario.
#[derive(Debug, Clone)]
pub struct Prompt {
    pub text: String,
}

impl Default for Prompt {
    fn default() -> Self {
        Self {
            text: "❯ ".to_string(),
        }
    }
}

impl Prompt {
    /// Devuelve el texto listo para mostrar.
    pub fn render(&self) -> String {
        self.text.clone()
    }
}
