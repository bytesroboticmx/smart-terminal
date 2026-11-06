//! Registro de plugins.

/// Interfaz mínima de un plugin.
pub trait Plugin: Send + Sync {
    /// Nombre identificativo del plugin.
    fn name(&self) -> &str;

    /// Hook invocado antes de procesar un comando.
    fn on_command(&self, _command: &str) {}
}

/// Registro de plugins cargados.
#[derive(Default)]
pub struct PluginRegistry {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginRegistry {
    /// Registra un nuevo plugin.
    pub fn register(&mut self, _plugin: Box<dyn Plugin>) {
        todo!("registrar un plugin")
    }

    /// Devuelve los plugins registrados.
    pub fn plugins(&self) -> &[Box<dyn Plugin>] {
        &self.plugins
    }
}
