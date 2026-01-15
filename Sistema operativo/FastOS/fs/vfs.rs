// ============================================================================
// FastOS - Virtual File System
// ============================================================================
// Capa de abstracción del sistema de archivos
//
// Author: Eddi Andreé Salazar Matos 🇵🇪
// ============================================================================

#![allow(dead_code)]

/// Tipos de nodo del VFS
#[derive(Clone, Copy, PartialEq)]
pub enum NodeType {
    File,
    Directory,
    Device,
}

/// Nodo del VFS
pub struct VfsNode {
    pub name: [u8; 64],
    pub node_type: NodeType,
    pub size: u64,
    pub inode: u64,
    pub permissions: u16,
}

impl VfsNode {
    /// Crear nodo vacío
    pub const fn empty() -> Self {
        VfsNode {
            name: [0; 64],
            node_type: NodeType::File,
            size: 0,
            inode: 0,
            permissions: 0o644,
        }
    }

    /// Crear archivo
    pub fn file(name: &str, size: u64) -> Self {
        let mut node = VfsNode::empty();
        node.set_name(name);
        node.node_type = NodeType::File;
        node.size = size;
        node
    }

    /// Crear directorio
    pub fn directory(name: &str) -> Self {
        let mut node = VfsNode::empty();
        node.set_name(name);
        node.node_type = NodeType::Directory;
        node.permissions = 0o755;
        node
    }

    /// Establecer nombre
    fn set_name(&mut self, name: &str) {
        let bytes = name.as_bytes();
        let len = bytes.len().min(63);
        self.name[..len].copy_from_slice(&bytes[..len]);
        self.name[len] = 0;
    }

    /// Obtener nombre como string
    pub fn name_str(&self) -> &str {
        let len = self.name.iter().position(|&c| c == 0).unwrap_or(64);
        core::str::from_utf8(&self.name[..len]).unwrap_or("")
    }

    /// Es directorio?
    pub fn is_dir(&self) -> bool {
        self.node_type == NodeType::Directory
    }

    /// Es archivo?
    pub fn is_file(&self) -> bool {
        self.node_type == NodeType::File
    }
}

/// Sistema de archivos virtual
pub struct Vfs {
    root: VfsNode,
    nodes: [Option<VfsNode>; 128],
    count: usize,
}

impl Vfs {
    /// Crear nuevo VFS
    pub const fn new() -> Self {
        const NONE: Option<VfsNode> = None;
        Vfs {
            root: VfsNode {
                name: [b'/', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                       0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                       0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                       0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                node_type: NodeType::Directory,
                size: 0,
                inode: 0,
                permissions: 0o755,
            },
            nodes: [NONE; 128],
            count: 0,
        }
    }

    /// Agregar nodo
    pub fn add_node(&mut self, node: VfsNode) -> Option<usize> {
        if self.count >= 128 {
            return None;
        }
        let index = self.count;
        self.nodes[index] = Some(node);
        self.count += 1;
        Some(index)
    }

    /// Buscar nodo por nombre
    pub fn find(&self, name: &str) -> Option<&VfsNode> {
        for node in self.nodes.iter().flatten() {
            if node.name_str() == name {
                return Some(node);
            }
        }
        None
    }

    /// Listar nodos
    pub fn list(&self) -> impl Iterator<Item = &VfsNode> {
        self.nodes.iter().flatten()
    }

    /// Número de nodos
    pub fn count(&self) -> usize {
        self.count
    }
}

/// Operaciones de archivo
pub trait FileOps {
    fn read(&self, buffer: &mut [u8], offset: u64) -> Result<usize, i32>;
    fn write(&mut self, buffer: &[u8], offset: u64) -> Result<usize, i32>;
    fn size(&self) -> u64;
}
