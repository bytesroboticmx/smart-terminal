//! Cliente del proveedor de modelos de lenguaje.

use crate::error::Result;

/// Interfaz mínima que debe implementar un proveedor de IA.
pub trait LlmClient {
    /// Envía un prompt y devuelve la respuesta del modelo.
    fn complete(&self, prompt: &str) -> Result<String>;
}

/// Cliente HTTP genérico. La implementación se añadirá más adelante.
#[derive(Debug, Default)]
pub struct HttpLlmClient;
