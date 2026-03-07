// ============================================================
// Header Resolver — Automatic header resolution sin CMake
// ============================================================
// Busca header_main.h, resuelve includes automaticamente.
// Sin CMake. Sin Makefile. Sin flags raros.
// ============================================================

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::stdlib::header_main::HeaderMain;

/// Resuelve headers automaticamente sin CMake/Makefile
pub struct HeaderResolver {
    /// Directorios de busqueda para headers
    search_paths: Vec<PathBuf>,
    /// Cache de headers ya resueltos (path -> contenido)
    resolved_cache: HashMap<String, String>,
    /// Headers ya incluidos (para evitar doble inclusion)
    included: HashMap<String, bool>,
    /// ADead-BIB v7.0 — Central stdlib registry for header_main.h
    header_main: HeaderMain,
}

impl HeaderResolver {
    pub fn new() -> Self {
        Self {
            search_paths: Vec::new(),
            resolved_cache: HashMap::new(),
            included: HashMap::new(),
            header_main: HeaderMain::new(),
        }
    }

    /// Agrega directorio de busqueda
    pub fn add_search_path(&mut self, path: PathBuf) {
        if !self.search_paths.contains(&path) {
            self.search_paths.push(path);
        }
    }

    /// Configura paths default: directorio del archivo fuente + include/ + ~/.adead/include/
    pub fn setup_default_paths(&mut self, source_file: &Path) {
        // 1. Carpeta actual del proyecto
        if let Some(parent) = source_file.parent() {
            self.add_search_path(parent.to_path_buf());
            self.add_search_path(parent.join("include"));
            self.add_search_path(parent.join("src"));
        }
        // 2. ~/.adead/include/ (global headers)
        if let Some(home) = std::env::var_os("USERPROFILE")
            .or_else(|| std::env::var_os("HOME"))
        {
            let global_include = PathBuf::from(home).join(".adead").join("include");
            self.add_search_path(global_include);
        }
    }

    /// Resuelve un #include "header.h" — busca en search_paths
    pub fn resolve(&mut self, header_name: &str) -> Result<String, ResolverError> {
        // Skip si ya fue incluido (include guard automatico)
        if self.included.contains_key(header_name) {
            return Ok(String::new());
        }

        // header_main.h es especial — carga todo fastos
        if header_name == "header_main.h" {
            self.included.insert(header_name.to_string(), true);
            return Ok(self.generate_header_main());
        }

        // Buscar en cache
        if let Some(content) = self.resolved_cache.get(header_name) {
            return Ok(content.clone());
        }

        // Buscar en search paths
        for search_path in &self.search_paths {
            let full_path = search_path.join(header_name);
            if full_path.exists() {
                match std::fs::read_to_string(&full_path) {
                    Ok(content) => {
                        self.included.insert(header_name.to_string(), true);
                        self.resolved_cache
                            .insert(header_name.to_string(), content.clone());
                        return Ok(content);
                    }
                    Err(_) => continue,
                }
            }
        }

        Err(ResolverError::HeaderNotFound(header_name.to_string()))
    }

    /// Genera el contenido de header_main.h — todo FastOS en 1 linea
    /// ADead-BIB v7.0: delegates to c_stdlib::get_header("header_main.h")
    /// which returns HEADER_MAIN_COMPLETE with ALL C99 declarations.
    fn generate_header_main(&self) -> String {
        // Use the real header_main.h from c_stdlib which contains
        // ALL C99 standard library declarations
        if let Some(content) = crate::frontend::c::c_stdlib::get_header("header_main.h") {
            return content.to_string();
        }
        // Fallback (should never happen)
        String::from("// header_main.h — ADead-BIB v7.0\n")
    }

    /// ADead-BIB v7.0: Check if a symbol is a known stdlib symbol.
    /// Used for tree-shaking: mark only used symbols.
    pub fn is_stdlib_symbol(&self, name: &str) -> bool {
        self.header_main.is_known_symbol(name)
    }

    /// ADead-BIB v7.0: Mark a symbol as used for tree shaking.
    pub fn mark_symbol_used(&mut self, name: &str) {
        self.header_main.mark_used(name);
    }

    /// ADead-BIB v7.0: Resolve #include to internal stdlib origin.
    /// Returns the fastos module name if the header is known.
    pub fn resolve_to_stdlib(&self, header_name: &str) -> Option<String> {
        self.header_main
            .resolve_include(header_name)
            .map(|origin| format!("{:?}", origin))
    }

    /// Retorna true si un header ya fue incluido
    pub fn is_included(&self, header_name: &str) -> bool {
        self.included.contains_key(header_name)
    }

    /// Retorna cuantos headers fueron resueltos
    pub fn resolved_count(&self) -> usize {
        self.included.len()
    }
}

impl Default for HeaderResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum ResolverError {
    HeaderNotFound(String),
    ReadError(String),
}

impl std::fmt::Display for ResolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ResolverError::HeaderNotFound(h) => write!(f, "Header not found: '{}'", h),
            ResolverError::ReadError(e) => write!(f, "Read error: {}", e),
        }
    }
}

impl std::error::Error for ResolverError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolver_creation() {
        let resolver = HeaderResolver::new();
        assert_eq!(resolver.resolved_count(), 0);
    }

    #[test]
    fn test_header_main_resolution() {
        let mut resolver = HeaderResolver::new();
        let result = resolver.resolve("header_main.h");
        assert!(result.is_ok());
        assert!(result.unwrap().contains("header_main.h"));
    }

    #[test]
    fn test_double_include_guard() {
        let mut resolver = HeaderResolver::new();
        let _ = resolver.resolve("header_main.h");
        let second = resolver.resolve("header_main.h");
        assert!(second.is_ok());
        assert!(second.unwrap().is_empty()); // No duplicado
    }
}
