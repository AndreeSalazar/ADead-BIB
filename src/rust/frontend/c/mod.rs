// ============================================================
// ADead-BIB C Frontend
// ============================================================
// C99/C11 → ADead-BIB IR pipeline
//
// C Source → CLexer → CParser → CTranslationUnit → CToIR → Program
//
// El C entra aquí, ADead-BIB IR sale.
// Sin GCC. Sin LLVM. Sin Clang. Solo ADead-BIB.
// ============================================================

pub mod c_lexer;
pub mod c_ast;
pub mod c_parser;
pub mod c_to_ir;

pub use c_to_ir::compile_c_to_program;
