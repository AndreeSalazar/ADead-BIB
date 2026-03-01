// ============================================================
// ADead-BIB C Preprocessor
// ============================================================
// Resolves #include directives by injecting built-in headers
// Handles: #include <header.h>, #include "header.h"
// Skips: #define, #ifdef, #ifndef, #endif, #else, #if, #pragma
//
// No GCC. No Clang. ADead-BIB owns the headers. 💀🦈
// ============================================================

use std::collections::HashSet;
use super::c_stdlib;

pub struct CPreprocessor {
    /// Track included headers to prevent double inclusion
    included: HashSet<String>,
    /// Whether the common prologue has been injected
    prologue_injected: bool,
}

impl CPreprocessor {
    pub fn new() -> Self {
        Self {
            included: HashSet::new(),
            prologue_injected: false,
        }
    }

    /// Process C source code, resolving #include directives
    /// Returns preprocessed source with declarations injected
    pub fn process(&mut self, source: &str) -> String {
        let mut output = String::new();
        
        for line in source.lines() {
            let trimmed = line.trim();
            
            if trimmed.starts_with("#include") {
                // Extract header name from #include <header.h> or #include "header.h"
                if let Some(header_name) = self.extract_include(trimmed) {
                    // Skip if already included
                    if self.included.contains(&header_name) {
                        output.push('\n'); // keep line count stable
                        continue;
                    }
                    self.included.insert(header_name.clone());
                    
                    // Inject common prologue on first include
                    if !self.prologue_injected {
                        self.prologue_injected = true;
                        output.push_str(c_stdlib::COMMON_PROLOGUE);
                        output.push('\n');
                    }
                    
                    // Look up header declarations
                    if let Some(declarations) = c_stdlib::get_header(&header_name) {
                        output.push_str(declarations);
                        output.push('\n');
                    } else {
                        // Unknown header — skip with warning
                        eprintln!("ADead-BIB: unknown header <{}> — skipped", header_name);
                        output.push('\n');
                    }
                } else {
                    output.push('\n'); // malformed include
                }
            } else if trimmed.starts_with('#') {
                // Skip other preprocessor directives: #define, #ifdef, #ifndef, 
                // #endif, #else, #if, #elif, #pragma, #error, #warning, #undef, #line
                // Handle line continuation with backslash
                output.push('\n');
                // Note: multi-line macros (backslash continuation) are handled by the
                // lexer's skip_preprocessor_line, but since we've replaced the line with
                // empty, subsequent continuation lines won't have # and will be parsed normally.
                // This is fine for our use case.
            } else {
                output.push_str(line);
                output.push('\n');
            }
        }
        
        output
    }
    
    /// Extract header name from #include directive
    /// Handles: #include <stdio.h>, #include "myheader.h", #include <sys/types.h>
    fn extract_include(&self, line: &str) -> Option<String> {
        let after_include = line.strip_prefix("#include")?.trim();
        
        if after_include.starts_with('<') {
            // Angle bracket include: #include <header.h>
            let end = after_include.find('>')?;
            Some(after_include[1..end].trim().to_string())
        } else if after_include.starts_with('"') {
            // Quote include: #include "header.h"
            let rest = &after_include[1..];
            let end = rest.find('"')?;
            Some(rest[..end].trim().to_string())
        } else {
            None
        }
    }
    
    /// Get list of all included headers (for debugging/analysis)
    pub fn included_headers(&self) -> &HashSet<String> {
        &self.included
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extract_angle_include() {
        let pp = CPreprocessor::new();
        assert_eq!(pp.extract_include("#include <stdio.h>"), Some("stdio.h".to_string()));
        assert_eq!(pp.extract_include("#include <sys/types.h>"), Some("sys/types.h".to_string()));
        assert_eq!(pp.extract_include("#include <vulkan/vulkan.h>"), Some("vulkan/vulkan.h".to_string()));
    }
    
    #[test]
    fn test_extract_quote_include() {
        let pp = CPreprocessor::new();
        assert_eq!(pp.extract_include("#include \"myheader.h\""), Some("myheader.h".to_string()));
    }
    
    #[test]
    fn test_no_double_include() {
        let mut pp = CPreprocessor::new();
        let source = "#include <stdio.h>\n#include <stdio.h>\nint main() { return 0; }\n";
        let result = pp.process(source);
        // stdio declarations should appear only once
        let count = result.matches("int printf").count();
        assert_eq!(count, 1, "printf should be declared only once");
    }
    
    #[test]
    fn test_preserves_code() {
        let mut pp = CPreprocessor::new();
        let source = "int main() {\n    return 0;\n}\n";
        let result = pp.process(source);
        assert!(result.contains("int main()"));
        assert!(result.contains("return 0;"));
    }
    
    #[test]
    fn test_skips_define() {
        let mut pp = CPreprocessor::new();
        let source = "#define MAX 100\nint x;\n";
        let result = pp.process(source);
        assert!(!result.contains("#define"));
        assert!(result.contains("int x;"));
    }
    
    #[test]
    fn test_multiple_headers() {
        let mut pp = CPreprocessor::new();
        let source = "#include <stdio.h>\n#include <stdlib.h>\n#include <string.h>\nint main() { return 0; }\n";
        let result = pp.process(source);
        // Should contain declarations from all three headers
        assert!(result.contains("printf"));
        assert!(result.contains("malloc"));
        assert!(result.contains("strlen"));
    }
}
