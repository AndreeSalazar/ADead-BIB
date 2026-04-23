// ============================================================
// ADead-BIB Bridge — ASM-BIB COFF .obj Importer
// ============================================================
// Reads COFF object files produced by ASM-BIB and extracts:
//   - .text section (machine code)
//   - .data section (initialized data)
//   - Symbol table (exported/external functions)
//   - Relocations (REL32 for calls, ADDR32NB for data)
//   - .pdata/.xdata (SEH unwind info)
//
// This allows ADead-BIB (C/C++ compiler) to link against
// ASM-BIB assembled functions seamlessly.
//
// Architecture: MASM Ring 1-3 (drivers + userland)
// ============================================================

pub mod coff_reader;
pub mod linker;

pub use coff_reader::CoffObject;
pub use linker::BridgeLinker;
