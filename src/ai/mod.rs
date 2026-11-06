//! Motor de IA: cliente, intención, planificación, seguridad y prompts.

pub mod client;
pub mod intent;
pub mod planner;
pub mod prompt;
pub mod safety;

pub use client::LlmClient;
pub use intent::Intent;
pub use planner::Plan;
pub use safety::SafetyVerdict;

/// Fachada del motor de IA.
#[derive(Debug, Default)]
pub struct AiEngine;
