// ============================================================================
// FastOS - Filesystem Module
// ============================================================================
// Módulo del sistema de archivos
//
// Author: Eddi Andreé Salazar Matos 🇵🇪
// ============================================================================

pub mod vfs;

pub use vfs::{Vfs, VfsNode, NodeType, FileOps};
