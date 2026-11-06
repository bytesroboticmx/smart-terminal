//! Orquestación de la aplicación.

use crate::config::Config;
use crate::error::Result;

/// Inicia la terminal inteligente.
pub fn run() -> Result<()> {
    let _config = Config::default();

    // TODO:
    // 1. Cargar la configuración.
    // 2. Inicializar la UI y la sesión.
    // 3. Arrancar el bucle de lectura, interpretación y ejecución.
    // 4. Integrar el motor de IA.
    todo!("implementar el arranque de la aplicación")
}
