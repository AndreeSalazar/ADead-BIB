// ============================================================
// ADead-BIB — C++ Language Driver (Stub)
// ============================================================
// Future: Full C++17/20 compilation pipeline
//   Phase 0: Preprocessor
//   Phase 1: Lexer
//   Phase 2: Parser (C++ AST)
//   Phase 3: Name Resolution + Overload Resolution
//   Phase 4: Template Instantiation
//   Phase 5: UB Detection
//   Phase 6: IR Generation
//   Phase 7: Code Gen (x86-64)
//   Phase 8: PE/ELF Output
// ============================================================

use crate::cli::term;

pub fn compile_cpp_file(
    input_file: &str,
    output_file: &str,
    step_mode: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("  ADead-BIB C++ Compiler v1.0 (preview)");
    println!("   Source: {}", input_file);
    println!("   Target: {}", output_file);

    if step_mode {
        println!();
        println!("{}", term::phase_bar(0, "Preprocessor", "C++"));
        println!("{}", term::info("C++ preprocessor: not yet implemented"));
        println!();
        println!("{}", term::phase_bar(1, "Lexer", "C++"));
        println!("{}", term::info("C++ lexer: not yet implemented"));
        println!();
        println!("{}", term::phase_bar(2, "Parser", "C++"));
        println!("{}", term::info("C++ parser: not yet implemented"));
    }

    Err("C++ compilation not yet implemented — use 'cc' for C".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpp_driver_returns_error() {
        let result = compile_cpp_file("test.cpp", "test.exe", false);
        assert!(result.is_err());
    }
}
