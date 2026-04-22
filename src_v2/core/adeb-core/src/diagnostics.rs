//! ADead-BIB Diagnostics
//!
//! Sistema de diagnósticos y reporte de errores.

use std::fmt;

/// Nivel de severidad del diagnóstico (alias para compatibilidad)
pub type DiagnosticLevel = Severity;

/// Manager de diagnósticos (alias para compatibilidad)
pub type DiagnosticManager = DiagnosticEmitter;

/// Nivel de severidad del diagnóstico
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Note,
    Help,
}

/// Un diagnóstico individual
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub message: String,
    pub location: Option<Location>,
}

/// Ubicación en el código fuente
#[derive(Debug, Clone, Copy)]
pub struct Location {
    pub file: usize,
    pub line: u32,
    pub column: u32,
}

impl Diagnostic {
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            message: message.into(),
            location: None,
        }
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            message: message.into(),
            location: None,
        }
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = match self.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Note => "note",
            Severity::Help => "help",
        };
        write!(f, "{}: {}", prefix, self.message)
    }
}

/// Emisor de diagnósticos
#[derive(Debug, Default)]
pub struct DiagnosticEmitter {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticEmitter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn emit(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| matches!(d.severity, Severity::Error))
    }

    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }
}
