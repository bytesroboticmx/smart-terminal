//! Lectura de entrada interactiva.

use crate::error::Result;

/// Lector de entrada del usuario.
#[derive(Debug, Default)]
pub struct InputReader;

impl InputReader {
    pub fn new() -> Self {
        Self
    }

    /// Lee una línea introducida por el usuario.
    pub fn read_line(&self) -> Result<String> {
        todo!("leer una línea de entrada")
    }
}
