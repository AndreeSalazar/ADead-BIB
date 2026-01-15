// ============================================================================
// FastOS Virtual File System (VFS)
// ============================================================================
// Sistema de archivos virtual con RAM disk
//
// Author: Eddi Andreé Salazar Matos 🇵🇪
// ============================================================================

#![allow(dead_code)]

/// Tamaño máximo de nombre de archivo
const MAX_NAME_LEN: usize = 64;

/// Máximo de archivos en el sistema
const MAX_FILES: usize = 128;

/// Tamaño máximo de archivo (64KB)
const MAX_FILE_SIZE: usize = 64 * 1024;

/// Tipo de entrada
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    File,
    Directory,
}

/// Entrada del sistema de archivos
#[derive(Clone, Copy)]
pub struct FileEntry {
    pub name: [u8; MAX_NAME_LEN],
    pub file_type: FileType,
    pub size: usize,
    pub data_offset: usize,
    pub parent: usize,
    pub used: bool,
}

impl FileEntry {
    pub const fn empty() -> Self {
        FileEntry {
            name: [0; MAX_NAME_LEN],
            file_type: FileType::File,
            size: 0,
            data_offset: 0,
            parent: 0,
            used: false,
        }
    }

    pub fn name_str(&self) -> &str {
        let len = self.name.iter().position(|&c| c == 0).unwrap_or(MAX_NAME_LEN);
        core::str::from_utf8(&self.name[..len]).unwrap_or("")
    }

    pub fn set_name(&mut self, name: &str) {
        let bytes = name.as_bytes();
        let len = bytes.len().min(MAX_NAME_LEN - 1);
        self.name[..len].copy_from_slice(&bytes[..len]);
        self.name[len] = 0;
    }
}

/// RAM Disk storage
static mut RAM_DISK: [u8; MAX_FILES * MAX_FILE_SIZE] = [0; MAX_FILES * MAX_FILE_SIZE];

/// Tabla de archivos
static mut FILE_TABLE: [FileEntry; MAX_FILES] = [FileEntry::empty(); MAX_FILES];

/// Directorio actual
static mut CURRENT_DIR: usize = 0;

/// Inicializar VFS
pub fn init() {
    unsafe {
        // Crear directorio raíz
        FILE_TABLE[0].used = true;
        FILE_TABLE[0].file_type = FileType::Directory;
        FILE_TABLE[0].set_name("/");
        FILE_TABLE[0].parent = 0;

        // Crear algunos archivos de ejemplo
        create_file("/readme.txt", b"Bienvenido a FastOS!\n\nComandos:\n  help - Ayuda\n  ls - Listar\n  cat <file> - Ver archivo\n  clear - Limpiar\n");
        create_file("/version.txt", b"FastOS v0.2.0\nAutor: Eddi Andree Salazar Matos\nPais: Peru\n");
        create_dir("/bin");
        create_dir("/home");
    }
}

/// Crear archivo
pub fn create_file(path: &str, content: &[u8]) -> Result<usize, &'static str> {
    unsafe {
        // Buscar slot libre
        let slot = find_free_slot()?;
        
        // Configurar entrada
        FILE_TABLE[slot].used = true;
        FILE_TABLE[slot].file_type = FileType::File;
        FILE_TABLE[slot].set_name(get_filename(path));
        FILE_TABLE[slot].parent = find_parent_dir(path);
        FILE_TABLE[slot].size = content.len().min(MAX_FILE_SIZE);
        FILE_TABLE[slot].data_offset = slot * MAX_FILE_SIZE;

        // Copiar contenido
        let offset = FILE_TABLE[slot].data_offset;
        RAM_DISK[offset..offset + FILE_TABLE[slot].size].copy_from_slice(&content[..FILE_TABLE[slot].size]);

        Ok(slot)
    }
}

/// Crear directorio
pub fn create_dir(path: &str) -> Result<usize, &'static str> {
    unsafe {
        let slot = find_free_slot()?;
        
        FILE_TABLE[slot].used = true;
        FILE_TABLE[slot].file_type = FileType::Directory;
        FILE_TABLE[slot].set_name(get_filename(path));
        FILE_TABLE[slot].parent = find_parent_dir(path);
        FILE_TABLE[slot].size = 0;

        Ok(slot)
    }
}

/// Leer archivo
pub fn read_file(path: &str) -> Option<&'static [u8]> {
    let entry = find_entry(path)?;
    unsafe {
        if FILE_TABLE[entry].file_type != FileType::File {
            return None;
        }
        let offset = FILE_TABLE[entry].data_offset;
        let size = FILE_TABLE[entry].size;
        Some(&RAM_DISK[offset..offset + size])
    }
}

/// Listar directorio
pub fn list_dir(path: &str) -> impl Iterator<Item = &'static FileEntry> {
    let dir_idx = find_entry(path).unwrap_or(0);
    unsafe {
        FILE_TABLE.iter().filter(move |e| e.used && e.parent == dir_idx && e.name[0] != b'/')
    }
}

/// Listar directorio actual
pub fn list_current() -> impl Iterator<Item = &'static FileEntry> {
    unsafe {
        let current = CURRENT_DIR;
        FILE_TABLE.iter().filter(move |e| e.used && e.parent == current && e.name[0] != b'/')
    }
}

/// Cambiar directorio
pub fn change_dir(path: &str) -> Result<(), &'static str> {
    if path == "/" {
        unsafe { CURRENT_DIR = 0; }
        return Ok(());
    }
    
    if path == ".." {
        unsafe {
            CURRENT_DIR = FILE_TABLE[CURRENT_DIR].parent;
        }
        return Ok(());
    }

    let entry = find_entry(path).ok_or("Directory not found")?;
    unsafe {
        if FILE_TABLE[entry].file_type != FileType::Directory {
            return Err("Not a directory");
        }
        CURRENT_DIR = entry;
    }
    Ok(())
}

/// Obtener directorio actual
pub fn current_path() -> &'static str {
    unsafe { FILE_TABLE[CURRENT_DIR].name_str() }
}

// Funciones auxiliares

fn find_free_slot() -> Result<usize, &'static str> {
    unsafe {
        for i in 1..MAX_FILES {
            if !FILE_TABLE[i].used {
                return Ok(i);
            }
        }
        Err("No free slots")
    }
}

fn find_entry(path: &str) -> Option<usize> {
    let name = get_filename(path);
    unsafe {
        for i in 0..MAX_FILES {
            if FILE_TABLE[i].used && FILE_TABLE[i].name_str() == name {
                return Some(i);
            }
        }
        None
    }
}

fn find_parent_dir(path: &str) -> usize {
    if path.starts_with('/') {
        0 // Root
    } else {
        unsafe { CURRENT_DIR }
    }
}

fn get_filename(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or(path)
}

/// Verificar si existe
pub fn exists(path: &str) -> bool {
    find_entry(path).is_some()
}

/// Obtener tamaño de archivo
pub fn file_size(path: &str) -> Option<usize> {
    let entry = find_entry(path)?;
    unsafe { Some(FILE_TABLE[entry].size) }
}
