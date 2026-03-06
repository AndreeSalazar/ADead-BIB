// ============================================================
// Macro Expander — C++11-C++17 → C++98 Canon
// ============================================================
// Expande syntax sugar de C++11-C++17 a C++98 internamente.
// Lambda → struct, auto → tipo inferido, range-for → iterator.
// ============================================================

/// Expande C++11-C++17 features a C++98 equivalentes
pub struct MacroExpander {
    /// Contador para generar nombres unicos (lambdas, etc.)
    counter: u32,
}

impl MacroExpander {
    pub fn new() -> Self {
        Self { counter: 0 }
    }

    /// Genera un nombre unico para estructuras generadas
    fn unique_name(&mut self, prefix: &str) -> String {
        self.counter += 1;
        format!("__{}_{}__", prefix, self.counter)
    }

    /// Expande una lambda C++11 a un struct C++98 con operator()
    ///
    /// Input:  `[](int x) { return x + 1; }`
    /// Output: `struct __lambda_1__ { int operator()(int x) const { return x + 1; } };`
    pub fn expand_lambda(&mut self, params: &str, body: &str) -> String {
        let name = self.unique_name("lambda");
        format!(
            "struct {} {{ auto operator()({}) const {{ {} }} }};",
            name, params, body
        )
    }

    /// Expande range-for C++11 a iterator C++98
    ///
    /// Input:  `for (auto& item : lista) { ... }`
    /// Output: `for (auto it = lista.begin(); it != lista.end(); ++it) { auto& item = *it; ... }`
    pub fn expand_range_for(&self, var_name: &str, container: &str, body: &str) -> String {
        format!(
            "for (auto __it = {container}.begin(); __it != {container}.end(); ++__it) {{ auto& {var} = *__it; {body} }}",
            container = container,
            var = var_name,
            body = body
        )
    }

    /// Expande if constexpr C++17 — evalua en compilacion, solo incluye branch correcto
    pub fn expand_if_constexpr(
        &self,
        condition_is_true: bool,
        then_body: &str,
        else_body: Option<&str>,
    ) -> String {
        if condition_is_true {
            then_body.to_string()
        } else {
            else_body.unwrap_or("").to_string()
        }
    }

    /// Retorna cuantas expansiones se han realizado
    pub fn expansion_count(&self) -> u32 {
        self.counter
    }
}

impl Default for MacroExpander {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lambda_expansion() {
        let mut expander = MacroExpander::new();
        let result = expander.expand_lambda("int x", "return x + 1;");
        assert!(result.contains("__lambda_1__"));
        assert!(result.contains("operator()"));
    }

    #[test]
    fn test_range_for_expansion() {
        let expander = MacroExpander::new();
        let result = expander.expand_range_for("item", "lista", "process(item);");
        assert!(result.contains(".begin()"));
        assert!(result.contains(".end()"));
    }

    #[test]
    fn test_if_constexpr() {
        let expander = MacroExpander::new();
        let result = expander.expand_if_constexpr(true, "branch_true();", Some("branch_false();"));
        assert_eq!(result, "branch_true();");
    }
}
