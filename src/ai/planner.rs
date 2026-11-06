//! Planificación de acciones a partir de una intención.

use crate::ai::intent::Intent;
use crate::error::Result;

/// Plan de acciones propuesto por la IA.
#[derive(Debug, Clone, Default)]
pub struct Plan {
    pub steps: Vec<String>,
}

/// Construye un plan para la intención indicada.
pub fn plan(_intent: &Intent) -> Result<Plan> {
    todo!("construir un plan a partir de la intención")
}
