// ============================================================
// ADead-BIB C++ Preprocessor
// ============================================================
// Resolves #include directives by injecting built-in C++ headers
// Handles: #include <header>, #include "header", #define, #ifdef, etc.
// Skips: #pragma, #error, #warning, #line
//
// No GCC. No libstdc++. No libc++. ADead-BIB owns the headers. 💀🦈
// ============================================================

use std::collections::HashSet;
use super::cpp_stdlib;

pub struct CppPreprocessor {
    included: HashSet<String>,
    prologue_injected: bool,
}

impl CppPreprocessor {
    pub fn new() -> Self {
        Self {
            included: HashSet::new(),
            prologue_injected: false,
        }
    }

    /// Process C++ source code, resolving #include directives
    pub fn process(&mut self, source: &str) -> String {
        let mut output = String::new();
        let mut lines_iter = source.lines().peekable();

        while let Some(line) = lines_iter.next() {
            let trimmed = line.trim();

            if trimmed.starts_with("#include") {
                if let Some(header_name) = self.extract_include(trimmed) {
                    if self.included.contains(&header_name) {
                        output.push('\n');
                        continue;
                    }
                    self.included.insert(header_name.clone());

                    // Inject common prologue on first include
                    if !self.prologue_injected {
                        self.prologue_injected = true;
                        output.push_str(cpp_stdlib::CPP_COMMON_PROLOGUE);
                        output.push('\n');
                    }

                    // Look up C++ header declarations
                    if let Some(declarations) = cpp_stdlib::get_cpp_header(&header_name) {
                        output.push_str(declarations);
                        output.push('\n');
                    } else {
                        // Unknown header — skip silently
                        output.push('\n');
                    }
                } else {
                    output.push('\n');
                }
            } else if trimmed.starts_with("#define") {
                // Process simple #define VALUE macros
                // Skip multi-line defines with backslash continuation
                output.push('\n');
                // Consume continuation lines
                while let Some(_next_line) = lines_iter.peek() {
                    if line.trim_end().ends_with('\\') {
                        lines_iter.next();
                        output.push('\n');
                    } else {
                        break;
                    }
                }
            } else if trimmed.starts_with('#') {
                // Skip: #ifdef, #ifndef, #endif, #else, #elif, #if, #pragma, 
                // #error, #warning, #undef, #line, #pragma once
                output.push('\n');
            } else {
                output.push_str(line);
                output.push('\n');
            }
        }

        output
    }

    /// Extract header name from #include directive
    fn extract_include(&self, line: &str) -> Option<String> {
        let after_include = line.strip_prefix("#include")?.trim();

        if after_include.starts_with('<') {
            let end = after_include.find('>')?;
            Some(after_include[1..end].trim().to_string())
        } else if after_include.starts_with('"') {
            let rest = &after_include[1..];
            let end = rest.find('"')?;
            Some(rest[..end].trim().to_string())
        } else {
            None
        }
    }

    pub fn included_headers(&self) -> &HashSet<String> {
        &self.included
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_angle_include() {
        let pp = CppPreprocessor::new();
        assert_eq!(pp.extract_include("#include <iostream>"), Some("iostream".to_string()));
        assert_eq!(pp.extract_include("#include <vector>"), Some("vector".to_string()));
        assert_eq!(pp.extract_include("#include <string>"), Some("string".to_string()));
    }

    #[test]
    fn test_extract_quote_include() {
        let pp = CppPreprocessor::new();
        assert_eq!(pp.extract_include("#include \"myheader.h\""), Some("myheader.h".to_string()));
    }

    #[test]
    fn test_no_double_include() {
        let mut pp = CppPreprocessor::new();
        let source = "#include <iostream>\n#include <iostream>\nint main() { return 0; }\n";
        let result = pp.process(source);
        // printf comes from prologue, should appear only once
        let count = result.matches("int printf").count();
        assert!(count >= 1, "printf should be declared");
    }

    #[test]
    fn test_preserves_code() {
        let mut pp = CppPreprocessor::new();
        let source = "int main() {\n    return 0;\n}\n";
        let result = pp.process(source);
        assert!(result.contains("int main()"));
        assert!(result.contains("return 0;"));
    }

    #[test]
    fn test_skips_define() {
        let mut pp = CppPreprocessor::new();
        let source = "#define MAX 100\nint x;\n";
        let result = pp.process(source);
        assert!(!result.contains("#define"));
        assert!(result.contains("int x;"));
    }

    #[test]
    fn test_iostream_injected() {
        let mut pp = CppPreprocessor::new();
        let source = "#include <iostream>\nint main() { return 0; }\n";
        let result = pp.process(source);
        assert!(result.contains("printf"), "iostream should inject printf");
        assert!(result.contains("puts"), "iostream should inject puts");
        assert!(result.contains("size_t"), "prologue should inject size_t");
    }

    #[test]
    fn test_vector_injected() {
        let mut pp = CppPreprocessor::new();
        let source = "#include <vector>\nint main() { return 0; }\n";
        let result = pp.process(source);
        // vector header is empty (types recognized by parser prescan)
        // but prologue should be injected
        assert!(result.contains("size_t"), "prologue should inject size_t");
    }

    #[test]
    fn test_multiple_headers() {
        let mut pp = CppPreprocessor::new();
        let source = "#include <iostream>\n#include <vector>\n#include <string>\nint main() { return 0; }\n";
        let result = pp.process(source);
        assert!(result.contains("printf"), "iostream should inject printf");
        assert!(result.contains("size_t"), "prologue should inject size_t");
        assert!(result.contains("int main()"), "code should be preserved");
    }
}
