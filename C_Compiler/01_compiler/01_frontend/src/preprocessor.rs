// ============================================================
// ADead-BIB C Preprocessor
// ============================================================
// Resolves #include directives by injecting built-in headers
// Handles: #include <header.h>, #include "header.h"
// Skips: #define, #ifdef, #ifndef, #endif, #else, #if, #pragma
//
// No GCC. No Clang. ADead-BIB owns the headers. 💀🦈
// ============================================================

use super::c_stdlib;
use std::collections::{HashMap, HashSet};

/// A #define macro: either object-like or function-like
#[derive(Debug, Clone)]
enum Macro {
    /// #define NAME value
    Object(String),
    /// #define NAME(a,b) body
    Function { params: Vec<String>, body: String, variadic: bool },
}

pub struct CPreprocessor {
    /// Track included headers to prevent double inclusion
    included: HashSet<String>,
    /// Whether the common prologue has been injected
    prologue_injected: bool,
    /// Defined macros
    macros: HashMap<String, Macro>,
}

impl CPreprocessor {
    pub fn new() -> Self {
        let mut macros = HashMap::new();
        // Standard predefined macros
        macros.insert("__STDC__".to_string(), Macro::Object("1".to_string()));
        macros.insert("__STDC_VERSION__".to_string(), Macro::Object("201112L".to_string()));
        macros.insert("__ADEAD_BIB__".to_string(), Macro::Object("1".to_string()));
        macros.insert("__x86_64__".to_string(), Macro::Object("1".to_string()));
        macros.insert("__LP64__".to_string(), Macro::Object("1".to_string()));

        Self {
            included: HashSet::new(),
            prologue_injected: false,
            macros,
        }
    }

    /// Process C source code, resolving #include directives
    /// Returns preprocessed source with declarations injected
    pub fn process(&mut self, source: &str) -> String {
        // Phase 0: splice backslash-newline continuations (C99 §5.1.1.2/1)
        let source = source.replace("\\\n", "");

        let mut output = String::new();
        let mut skip_mode = false;
        let mut skip_depth: i32 = 0;
        let mut skip_else_ok = false;
        let mut in_block_comment = false;

        for (i, line) in source.lines().enumerate() {
            let source_line = i + 1;
            let trimmed = line.trim();

            // Handle conditional compilation skip mode
            if skip_mode {
                if trimmed.starts_with("#ifdef")
                    || trimmed.starts_with("#ifndef")
                    || trimmed.starts_with("#if ")
                {
                    skip_depth += 1;
                } else if trimmed == "#endif"
                    || trimmed.starts_with("#endif ")
                    || trimmed.starts_with("#endif/")
                {
                    skip_depth -= 1;
                    if skip_depth <= 0 {
                        skip_mode = false;
                        skip_depth = 0;
                    }
                } else if (trimmed == "#else"
                    || trimmed.starts_with("#else ")
                    || trimmed.starts_with("#else/"))
                    && skip_depth == 1
                    && skip_else_ok
                {
                    skip_mode = false;
                    skip_depth = 0;
                    // But we need to mark that the next #else/#elif should skip
                    skip_else_ok = false;
                } else if trimmed.starts_with("#elif ") && skip_depth == 1 && skip_else_ok {
                    let cond = trimmed[6..].trim();
                    let active = if cond == "0" {
                        false
                    } else if cond == "1" {
                        true
                    } else if cond.starts_with("defined(") {
                        let name = cond.trim_start_matches("defined(").trim_end_matches(')');
                        self.macros.contains_key(name)
                    } else {
                        self.macros.contains_key(cond)
                    };
                    if active {
                        skip_mode = false;
                        skip_depth = 0;
                        skip_else_ok = false;
                    }
                }
                output.push('\n');
                continue;
            }

            // Handle #endif when not in skip mode (from #else branch)
            if trimmed == "#endif"
                || trimmed.starts_with("#endif ")
                || trimmed.starts_with("#endif/")
            {
                output.push('\n');
                continue;
            }
            // Handle #else when not skipping (we were in the true branch, now skip)
            if trimmed == "#else" || trimmed.starts_with("#else ") || trimmed.starts_with("#else/")
            {
                skip_mode = true;
                skip_depth = 1;
                skip_else_ok = false;
                output.push('\n');
                continue;
            }
            // Handle #elif when not skipping (we were in the true branch, skip rest)
            if trimmed.starts_with("#elif ") {
                skip_mode = true;
                skip_depth = 1;
                skip_else_ok = false;
                output.push('\n');
                continue;
            }

            if trimmed.starts_with("#include") {
                // Extract header name from #include <header.h> or #include "header.h"
                if let Some(header_name) = self.extract_include(trimmed) {
                    // Skip if already included
                    if self.included.contains(&header_name) {
                        output.push('\n'); // keep line count stable
                        continue;
                    }
                    self.included.insert(header_name.clone());

                    // Wait, track if we injected lines
                    let mut lines_injected = false;

                    // Inject common prologue on first include
                    if !self.prologue_injected {
                        self.prologue_injected = true;
                        output.push_str(&format!("# 1 \"<common_prologue>\"\n"));
                        output.push_str(c_stdlib::COMMON_PROLOGUE);
                        output.push('\n');
                        lines_injected = true;
                    }

                    // Look up header declarations
                    if let Some(declarations) = c_stdlib::get_header(&header_name) {
                        output.push_str(&format!("# 1 \"{}\"\n", header_name));
                        output.push_str(declarations);
                        output.push('\n');
                        lines_injected = true;
                    } else {
                        // Unknown header — skip with warning
                        eprintln!("ADead-BIB: unknown header <{}> — skipped", header_name);
                        output.push('\n');
                    }

                    if lines_injected {
                        // Resync to main file line
                        output.push_str(&format!("# {} \"main\"\n", source_line + 1));
                    }
                } else {
                    output.push('\n'); // malformed include
                }
            } else if trimmed.starts_with("#define ") || trimmed.starts_with("#define\t") {
                self.parse_define(trimmed);
                output.push('\n');
            } else if trimmed.starts_with("#undef ") {
                let name = trimmed[7..].trim().to_string();
                self.macros.remove(&name);
                output.push('\n');
            } else if trimmed.starts_with("#ifdef ") {
                let name = trimmed[7..].trim();
                if !self.macros.contains_key(name) {
                    // Skip until #else or #endif
                    skip_mode = true;
                    skip_depth = 1;
                    skip_else_ok = true;
                }
                output.push('\n');
            } else if trimmed.starts_with("#ifndef ") {
                let name = trimmed[8..].trim();
                if self.macros.contains_key(name) {
                    skip_mode = true;
                    skip_depth = 1;
                    skip_else_ok = true;
                }
                output.push('\n');
            } else if trimmed.starts_with("#if ") {
                // Simple: #if 0 → skip, #if 1 → keep, #if DEFINED → check
                let cond = trimmed[4..].trim();
                let active = if cond == "0" {
                    false
                } else if cond == "1" {
                    true
                } else if cond.starts_with("defined(") {
                    let name = cond.trim_start_matches("defined(").trim_end_matches(')');
                    self.macros.contains_key(name)
                } else {
                    self.macros.contains_key(cond)
                };
                if !active {
                    skip_mode = true;
                    skip_depth = 1;
                    skip_else_ok = true;
                }
                output.push('\n');
            } else if trimmed.starts_with('#') {
                // Skip other preprocessor directives: #pragma, #error, #warning, #line, etc.
                output.push('\n');
            } else {
                // Expand macros in regular code lines
                let expanded = self.expand_macros_preserving_literals(line, &mut in_block_comment);
                output.push_str(&expanded);
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

    /// Parse a #define directive and store the macro
    fn parse_define(&mut self, line: &str) {
        let rest = line.strip_prefix("#define").unwrap().trim();
        if rest.is_empty() {
            return;
        }

        // Check for function-like macro: NAME(params) body
        if let Some(paren_pos) = rest.find('(') {
            let name = rest[..paren_pos].trim();
            // Only function-like if '(' immediately follows name (no space)
            if !name.is_empty() && !name.contains(' ') {
                let after_name = &rest[paren_pos..];
                if let Some(close) = after_name.find(')') {
                    let params_str = &after_name[1..close];
                    let mut params: Vec<String> = params_str
                        .split(',')
                        .map(|p| p.trim().to_string())
                        .filter(|p| !p.is_empty())
                        .collect();
                    let variadic = params.last().map(|p| p == "...").unwrap_or(false);
                    if variadic {
                        params.pop();
                    }
                    let body = after_name[close + 1..].trim().to_string();
                    self.macros
                        .insert(name.to_string(), Macro::Function { params, body, variadic });
                    return;
                }
            }
        }

        // Object-like macro: NAME value
        let mut parts = rest.splitn(2, |c: char| c == ' ' || c == '\t');
        let name = parts.next().unwrap_or("").trim();
        if name.is_empty() {
            return;
        }
        // Check for trailing // comment
        let value = parts.next().unwrap_or("").trim();
        let value = if let Some(comment_pos) = value.find("//") {
            value[..comment_pos].trim()
        } else {
            value
        };
        self.macros
            .insert(name.to_string(), Macro::Object(value.to_string()));
    }

    /// Expand macros in a line of code
    fn expand_macros(&self, line: &str) -> String {
        if self.macros.is_empty() {
            return line.to_string();
        }

        let mut result = line.to_string();
        // Multiple passes to handle nested macros (limit to prevent infinite loops)
        for _ in 0..8 {
            let prev = result.clone();
            for (name, mac) in &self.macros {
                match mac {
                    Macro::Object(value) => {
                        // Replace whole-word occurrences only
                        result = self.replace_whole_word(&result, name, value);
                    }
                    Macro::Function { params, body, variadic } => {
                        result = self.expand_function_macro(&result, name, params, body, *variadic);
                    }
                }
            }
            if result == prev {
                break;
            }
        }
        result
    }

    fn expand_macros_preserving_literals(
        &self,
        line: &str,
        in_block_comment: &mut bool,
    ) -> String {
        if self.macros.is_empty() {
            return line.to_string();
        }

        let chars: Vec<char> = line.chars().collect();
        let mut result = String::with_capacity(line.len());
        let mut code_chunk = String::new();
        let mut i = 0;

        while i < chars.len() {
            if *in_block_comment {
                if chars[i] == '*' && i + 1 < chars.len() && chars[i + 1] == '/' {
                    result.push(chars[i]);
                    result.push(chars[i + 1]);
                    *in_block_comment = false;
                    i += 2;
                } else {
                    result.push(chars[i]);
                    i += 1;
                }
                continue;
            }

            if chars[i] == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
                if !code_chunk.is_empty() {
                    result.push_str(&self.expand_macros(&code_chunk));
                    code_chunk.clear();
                }
                result.extend(chars[i..].iter());
                break;
            }

            if chars[i] == '/' && i + 1 < chars.len() && chars[i + 1] == '*' {
                if !code_chunk.is_empty() {
                    result.push_str(&self.expand_macros(&code_chunk));
                    code_chunk.clear();
                }
                result.push(chars[i]);
                result.push(chars[i + 1]);
                *in_block_comment = true;
                i += 2;
                continue;
            }

            if chars[i] == '"' || chars[i] == '\'' {
                if !code_chunk.is_empty() {
                    result.push_str(&self.expand_macros(&code_chunk));
                    code_chunk.clear();
                }

                let quote = chars[i];
                result.push(quote);
                i += 1;
                while i < chars.len() {
                    let ch = chars[i];
                    result.push(ch);
                    i += 1;
                    if ch == '\\' && i < chars.len() {
                        result.push(chars[i]);
                        i += 1;
                        continue;
                    }
                    if ch == quote {
                        break;
                    }
                }
                continue;
            }

            code_chunk.push(chars[i]);
            i += 1;
        }

        if !code_chunk.is_empty() {
            result.push_str(&self.expand_macros(&code_chunk));
        }

        result
    }

    /// Replace whole-word occurrences of `name` with `value`
    fn replace_whole_word(&self, text: &str, name: &str, value: &str) -> String {
        let mut result = String::with_capacity(text.len());
        let chars: Vec<char> = text.chars().collect();
        let name_chars: Vec<char> = name.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            if i + name_chars.len() <= chars.len()
                && &chars[i..i + name_chars.len()] == name_chars.as_slice()
            {
                // Check word boundary before
                let before_ok = i == 0 || !chars[i - 1].is_alphanumeric() && chars[i - 1] != '_';
                // Check word boundary after
                let after_idx = i + name_chars.len();
                let after_ok = after_idx >= chars.len()
                    || !chars[after_idx].is_alphanumeric() && chars[after_idx] != '_';
                if before_ok && after_ok {
                    result.push_str(value);
                    i += name_chars.len();
                    continue;
                }
            }
            result.push(chars[i]);
            i += 1;
        }
        result
    }

    /// Expand function-like macro invocations: NAME(arg1, arg2)
    fn expand_function_macro(
        &self,
        text: &str,
        name: &str,
        params: &[String],
        body: &str,
        variadic: bool,
    ) -> String {
        let mut result = String::new();
        let chars: Vec<char> = text.chars().collect();
        let name_chars: Vec<char> = name.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            // Look for macro name followed by '('
            if i + name_chars.len() < chars.len()
                && &chars[i..i + name_chars.len()] == name_chars.as_slice()
            {
                let before_ok = i == 0 || !chars[i - 1].is_alphanumeric() && chars[i - 1] != '_';
                let mut after_idx = i + name_chars.len();
                while after_idx < chars.len() && chars[after_idx].is_ascii_whitespace() {
                    after_idx += 1;
                }
                if before_ok && after_idx < chars.len() && chars[after_idx] == '(' {
                    // Extract arguments
                    if let Some((args, end_pos)) = self.extract_macro_args(&chars, after_idx) {
                        // Substitute parameters in body
                        let mut expanded = body.to_string();

                        // Handle # stringification: #param → "arg"
                        for (pi, param) in params.iter().enumerate() {
                            if pi < args.len() {
                                let pattern = format!("#{}", param);
                                let replacement = format!("\"{}\"", args[pi].replace('\\', "\\\\").replace('"', "\\\""));
                                expanded = expanded.replace(&pattern, &replacement);
                            }
                        }

                        for (pi, param) in params.iter().enumerate() {
                            if pi < args.len() {
                                expanded = self.replace_whole_word(&expanded, param, &args[pi]);
                            }
                        }

                        // Handle __VA_ARGS__ for variadic macros
                        if variadic {
                            let va_args = if args.len() > params.len() {
                                args[params.len()..].join(", ")
                            } else {
                                String::new()
                            };
                            expanded = expanded.replace("__VA_ARGS__", &va_args);
                        }

                        // Handle ## token pasting
                        expanded = expanded.replace(" ## ", "");
                        expanded = expanded.replace("## ", "");
                        expanded = expanded.replace(" ##", "");

                        // Wrap in parentheses for safety
                        result.push('(');
                        result.push_str(&expanded);
                        result.push(')');
                        i = end_pos;
                        continue;
                    }
                }
            }
            result.push(chars[i]);
            i += 1;
        }
        result
    }

    /// Extract comma-separated arguments from a function-like macro call
    /// Returns (args, position after closing paren)
    fn extract_macro_args(
        &self,
        chars: &[char],
        open_paren: usize,
    ) -> Option<(Vec<String>, usize)> {
        let mut depth = 0;
        let mut args = Vec::new();
        let mut current = String::new();
        let mut i = open_paren;

        while i < chars.len() {
            let c = chars[i];
            if c == '(' {
                depth += 1;
                if depth > 1 {
                    current.push(c);
                }
            } else if c == ')' {
                depth -= 1;
                if depth == 0 {
                    args.push(current.trim().to_string());
                    return Some((args, i + 1));
                }
                current.push(c);
            } else if c == ',' && depth == 1 {
                args.push(current.trim().to_string());
                current = String::new();
            } else {
                current.push(c);
            }
            i += 1;
        }
        None
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
        assert_eq!(
            pp.extract_include("#include <stdio.h>"),
            Some("stdio.h".to_string())
        );
        assert_eq!(
            pp.extract_include("#include <sys/types.h>"),
            Some("sys/types.h".to_string())
        );
        assert_eq!(
            pp.extract_include("#include <vulkan/vulkan.h>"),
            Some("vulkan/vulkan.h".to_string())
        );
    }

    #[test]
    fn test_extract_quote_include() {
        let pp = CPreprocessor::new();
        assert_eq!(
            pp.extract_include("#include \"myheader.h\""),
            Some("myheader.h".to_string())
        );
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

    #[test]
    fn test_macro_expansion_skips_strings_and_comments() {
        let mut pp = CPreprocessor::new();
        let source = r#"
            #define VALUE 42
            const char *msg = "VALUE";
            int x = VALUE; // VALUE
            /* VALUE */
        "#;
        let result = pp.process(source);

        assert!(result.contains(r#""VALUE""#));
        assert!(result.contains("int x = 42; // VALUE"));
        assert!(result.contains("/* VALUE */"));
    }

    #[test]
    fn test_function_macro_allows_space_before_paren() {
        let mut pp = CPreprocessor::new();
        let source = r#"
            #define ADD(a, b) ((a) + (b))
            int x = ADD (1, 2);
        "#;
        let result = pp.process(source);

        assert!(result.contains("int x = (((1) + (2)));"));
    }

    #[test]
    fn test_elif_handling() {
        let mut pp = CPreprocessor::new();
        let source = r#"
#define MODE 2
#if MODE == 1
int x = 1;
#elif MODE == 2
int x = 2;
#else
int x = 3;
#endif
"#;
        // Note: our #if doesn't evaluate == expressions yet,
        // but #elif structure should not crash
        let result = pp.process(source);
        assert!(!result.contains("#elif"));
    }

    #[test]
    fn test_line_continuation() {
        let mut pp = CPreprocessor::new();
        let source = "#define LONG_MACRO \\\n    value\nint x = LONG_MACRO;\n";
        let result = pp.process(source);
        assert!(result.contains("int x = value;"));
    }

    #[test]
    fn test_predefined_macros() {
        let mut pp = CPreprocessor::new();
        let source = "int stdc = __STDC__;\nint adead = __ADEAD_BIB__;\n";
        let result = pp.process(source);
        assert!(result.contains("int stdc = 1;"));
        assert!(result.contains("int adead = 1;"));
    }

    #[test]
    fn test_variadic_macro() {
        let mut pp = CPreprocessor::new();
        let source = "#define LOG(fmt, ...) my_log(fmt, __VA_ARGS__)\nLOG(msg, 1, 2, 3);\n";
        let result = pp.process(source);
        assert!(result.contains("my_log"));
        assert!(result.contains("1, 2, 3"));
    }
}
