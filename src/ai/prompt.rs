//! Prompts utilizados por el motor de IA.

/// Prompt de sistema de la terminal.
pub const SYSTEM_PROMPT: &str = "\
Eres el motor de una terminal inteligente. \
Traduce las peticiones del usuario en comandos seguros y explica brevemente qué hacen.";

/// Construye el prompt para generar un comando.
pub fn command_prompt(_description: &str) -> String {
    todo!("construir el prompt para generar un comando")
}
