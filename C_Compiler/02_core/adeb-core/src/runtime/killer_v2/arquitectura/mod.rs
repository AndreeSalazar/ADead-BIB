// ============================================================================
// RESPONSABILIDAD 1: ARQUITECTURA — Infraestructura Determinista
// ============================================================================
//
// Módulos de infraestructura que garantizan comportamiento predecible
// en memoria, stack, lifetimes y ABI.
//
// Principio: Todo costo es O(1) o calculable en compile-time.
// ============================================================================

pub mod memoria_continua;
pub mod stack_management;
pub mod lifetime_determinista;
pub mod abi_calling_convention;
