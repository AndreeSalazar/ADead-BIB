//! ADead-BIB Source Management
//!
//! Gestión de archivos fuente y mapeo de líneas.

use std::path::PathBuf;

/// Alias de ubicación para compatibilidad
pub type SourceLocation = Location;

/// Un archivo fuente
#[derive(Debug, Clone)]
pub struct SourceFile {
    pub path: PathBuf,
    pub content: String,
    pub id: usize,
}

/// Ubicación en el código fuente
#[derive(Debug, Clone, Copy)]
pub struct Location {
    pub file: usize,
    pub line: u32,
    pub column: u32,
}

impl SourceFile {
    pub fn new(path: impl Into<PathBuf>, content: impl Into<String>, id: usize) -> Self {
        Self {
            path: path.into(),
            content: content.into(),
            id,
        }
    }

    pub fn line(&self, line_num: usize) -> Option<&str> {
        self.content.lines().nth(line_num.saturating_sub(1))
    }
}

/// Administrador de archivos fuente
#[derive(Debug, Default)]
pub struct SourceMap {
    files: Vec<SourceFile>,
}

impl SourceMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_file(&mut self, path: impl Into<PathBuf>, content: impl Into<String>) -> usize {
        let id = self.files.len();
        self.files.push(SourceFile::new(path, content, id));
        id
    }

    pub fn get(&self, id: usize) -> Option<&SourceFile> {
        self.files.get(id)
    }
}
