//! ADead-BIB Symbol Table
//!
//! Gestión de símbolos y scopes.

use std::collections::HashMap;

/// Un símbolo en la tabla
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub scope: usize,
}

/// Tipo de símbolo
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    Variable,
    Function,
    Type,
    Module,
}

/// Tabla de símbolos
#[derive(Debug, Default)]
pub struct SymbolTable {
    scopes: Vec<HashMap<String, Symbol>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()], // global scope
        }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn insert(&mut self, name: impl Into<String>, kind: SymbolKind) {
        let scope = self.scopes.len() - 1;
        let name = name.into();
        let symbol = Symbol {
            name: name.clone(),
            kind,
            scope,
        };
        self.scopes[scope].insert(name, symbol);
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(sym) = scope.get(name) {
                return Some(sym);
            }
        }
        None
    }
}
