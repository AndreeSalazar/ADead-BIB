// ============================================================================
// RESPONSABILIDAD 2: MONOLITO — Ejecución Determinista
// ============================================================================
//
// Módulos de ejecución que garantizan resultados reproducibles
// en matemáticas, scheduling y manejo de errores.
//
// Principio: Mismo input → mismo output, siempre. Sin excepciones.
// ============================================================================

pub mod ejecucion_matematica;
pub mod scheduler_determinista;
pub mod error_como_valor;
