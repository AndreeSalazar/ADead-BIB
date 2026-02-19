// ============================================================
// ADead-BIB Format — Binary Intermediate Binary
// ============================================================
// Universal intermediate format between compiler and target.
// Independent of OS, executable format, and (partially) ISA.
//
// Equivalent to: LLVM bitcode, COFF .obj, WebAssembly module
// But designed for ADead ecosystem.
//
// Can be translated to: PE, ELF, Mach-O, FsOS
// ============================================================

pub mod format;
pub mod reader;
pub mod writer;
pub mod builder;
