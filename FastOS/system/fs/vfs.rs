// ============================================================
// FastOS — Virtual Filesystem (VFS)
// ============================================================
// Abstraction layer over concrete filesystems.
// All file operations go through VFS.
// ============================================================

/// File types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileType {
    Regular,
    Directory,
    Symlink,
    Device,
}

/// Directory entry
pub struct DirEntry {
    pub name: [u8; 256],
    pub name_len: usize,
    pub file_type: FileType,
    pub size: u64,
    pub inode: u64,
}

/// File descriptor
pub struct FileDescriptor {
    pub inode: u64,
    pub offset: u64,
    pub flags: u32,
}

/// VFS operations trait — each filesystem implements this
pub trait Filesystem {
    fn name(&self) -> &str;
    fn read(&self, inode: u64, offset: u64, buf: &mut [u8]) -> Result<usize, FsError>;
    fn write(&mut self, inode: u64, offset: u64, data: &[u8]) -> Result<usize, FsError>;
    fn readdir(&self, inode: u64) -> Result<&[DirEntry], FsError>;
    fn lookup(&self, parent: u64, name: &str) -> Result<u64, FsError>;
    fn create(&mut self, parent: u64, name: &str, file_type: FileType) -> Result<u64, FsError>;
    fn delete(&mut self, parent: u64, name: &str) -> Result<(), FsError>;
    fn stat(&self, inode: u64) -> Result<FileStat, FsError>;
}

/// File statistics
pub struct FileStat {
    pub inode: u64,
    pub size: u64,
    pub file_type: FileType,
    pub created: u64,
    pub modified: u64,
}

/// Filesystem errors
#[derive(Debug)]
pub enum FsError {
    NotFound,
    PermissionDenied,
    AlreadyExists,
    NotADirectory,
    IsADirectory,
    DiskFull,
    IoError,
}

/// Standard filesystem paths
pub const PATH_BOOT: &str = "/boot";
pub const PATH_SYSTEM: &str = "/system";
pub const PATH_APPS: &str = "/apps";
pub const PATH_USER: &str = "/user";
pub const PATH_DESKTOP: &str = "/user/desktop";
