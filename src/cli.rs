//! Argumentos de línea de comandos.

use crate::config::Config;
use crate::error::Result;

/// Argumentos aceptados por el binario.
#[derive(Debug, Default)]
pub struct CliArgs {
    pub config_path: Option<String>,
    pub command: Option<String>,
}

/// Parsea los argumentos de línea de comandos.
pub fn parse() -> CliArgs {
    todo!("parsear argumentos de línea de comandos")
}

/// Carga la configuración a partir de los argumentos.
pub fn load_config(args: &CliArgs) -> Result<Config> {
    Config::load(args.config_path.as_deref())
}
