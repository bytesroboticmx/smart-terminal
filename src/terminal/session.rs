//! Estado de la sesión interactiva.

use std::collections::HashMap;
use std::path::PathBuf;

/// Estado compartido durante una sesión de terminal.
#[derive(Debug, Clone)]
pub struct SessionState {
    pub cwd: PathBuf,
    pub env: HashMap<String, String>,
    pub running: bool,
}

impl Default for SessionState {
    fn default() -> Self {
        Self {
            cwd: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            env: std::env::vars().collect(),
            running: true,
        }
    }
}
