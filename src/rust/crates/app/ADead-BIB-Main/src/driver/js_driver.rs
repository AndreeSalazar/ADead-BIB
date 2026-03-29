// ============================================================
// ADead-BIB — JavaScript Language Driver (Stub)
// ============================================================
// Future: JavaScript/ES2024 compilation pipeline
//   Phase 0: Lexer (ES2024 tokens)
//   Phase 1: Parser (JS AST)
//   Phase 2: Scope Analysis
//   Phase 3: Type Inference (optional)
//   Phase 4: IR Generation
//   Phase 5: Code Gen (x86-64 JIT or bytecode)
//   Phase 6: Output (native or bytecode)
// ============================================================

use crate::cli::term;

pub fn compile_js_file(
    input_file: &str,
    output_file: &str,
    step_mode: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("  ADead-BIB JavaScript Compiler v1.0 (preview)");
    println!("   Source: {}", input_file);
    println!("   Target: {}", output_file);

    if step_mode {
        println!();
        println!("{}", term::phase_bar(0, "Lexer", "JS"));
        println!("{}", term::info("JS lexer: not yet implemented"));
        println!();
        println!("{}", term::phase_bar(1, "Parser", "JS"));
        println!("{}", term::info("JS parser: not yet implemented"));
    }

    Err("JavaScript compilation not yet implemented — use 'cc' for C".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_js_driver_returns_error() {
        let result = compile_js_file("test.js", "test.exe", false);
        assert!(result.is_err());
    }
}
