// ============================================================
// FastOS — UEFI Boot Application (Fase 8)
// ============================================================
// UEFI boot path: GOP framebuffer + memory map + kernel load.
// This module defines UEFI types, protocols, and the boot flow.
// Compiled as a PE32+ DLL (UEFI application subsystem).
// ============================================================

pub mod types;
pub mod gop;
pub mod memory_map;
pub mod boot;
