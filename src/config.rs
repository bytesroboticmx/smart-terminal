//! Configuración de la terminal.

use crate::error::Result;

/// Configuración general.
#[derive(Debug, Clone, Default)]
pub struct Config {
    pub shell: ShellConfig,
    pub ai: AiConfig,
    pub ui: UiConfig,
}

/// Configuración del shell.
#[derive(Debug, Clone)]
pub struct ShellConfig {
    pub history_size: usize,
    pub timeout_seconds: u64,
    pub confirm_dangerous_commands: bool,
}

/// Configuración del motor de IA.
#[derive(Debug, Clone, Default)]
pub struct AiConfig {
    pub enabled: bool,
    pub provider: String,
    pub model: String,
    pub api_key_env: String,
}

/// Configuración visual.
#[derive(Debug, Clone)]
pub struct UiConfig {
    pub theme: String,
    pub show_banner: bool,
}

impl Default for ShellConfig {
    fn default() -> Self {
        Self {
            history_size: 1_000,
            timeout_seconds: 30,
            confirm_dangerous_commands: true,
        }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            theme: "default".to_string(),
            show_banner: true,
        }
    }
}

impl Config {
    /// Carga la configuración desde un archivo o desde los valores por defecto.
    pub fn load(_path: Option<&str>) -> Result<Self> {
        todo!("cargar configuración")
    }
}
