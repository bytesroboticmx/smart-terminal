//! Historial de comandos.

/// Historial acotado de entradas.
#[derive(Debug, Default)]
pub struct History {
    entries: Vec<String>,
    max_size: usize,
}

impl History {
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_size,
        }
    }

    /// Añade una entrada al historial.
    pub fn push(&mut self, _entry: String) {
        todo!("registrar una entrada en el historial")
    }

    /// Devuelve las entradas registradas.
    pub fn entries(&self) -> &[String] {
        &self.entries
    }

    /// Capacidad máxima del historial.
    pub fn max_size(&self) -> usize {
        self.max_size
    }
}
