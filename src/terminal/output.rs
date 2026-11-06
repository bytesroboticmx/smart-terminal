//! Renderizado de salida.

use crate::error::Result;

/// Escritor de salida de la terminal.
#[derive(Debug, Default)]
pub struct OutputWriter;

impl OutputWriter {
    pub fn new() -> Self {
        Self
    }

    /// Escribe una línea en la salida.
    pub fn write_line(&self, _text: &str) -> Result<()> {
        todo!("renderizar una línea de salida")
    }
}
