// ============================================================
// JsDead-BIB — JavaScript Frontend for ADead-BIB
// ============================================================
// JS → ASM directo. Sin Node.js. Sin V8. Sin runtime.
//
// Pipeline: JS Source → JsLexer → JsParser → JsAST → JsToIR → Program
//
// Features:
//   - Implícitamente estricto — "Respetar Bits"
//   - == / != blocked → must use === / !==
//   - No implicit type coercion
//   - No GC — same heap as C
//   - No JIT — already ASM
//   - console.log → native print
//   - Classes → Structs + methods (same as C++)
//   - Type annotations: let x: int = 5
//   - Arrays homogéneos obligatorios
//
// Supported JS features:
//   - let/const/var with type annotations
//   - function declarations
//   - class declarations (constructor, methods, properties)
//   - if/else, for, while, do-while, switch
//   - Arrow functions (basic)
//   - Template literals (basic)
//   - Strict equality (===, !==)
//   - Bitwise operators
//   - for-of, for-in
//   - try/catch/finally (basic)
//
// Blocked (filosofía):
//   - eval() → UB por definición
//   - with → deprecated
//   - == / != → use === / !==
//   - Implicit type coercion → respeta los bits
//   - typeof null === "object" → bug histórico no existe
//
// Sin Node.js. Sin V8. Sin GC. Sin runtime de Google.
// Brendan Eich creó JS en 1995 — JsDead-BIB lo compila a ASM.
// Lima, Perú 🇵🇪 💀🦈
// ============================================================

pub mod js_ast;
pub mod js_lexer;
pub mod js_parser;
pub mod js_to_ir;

pub use js_to_ir::compile_js_to_program;
