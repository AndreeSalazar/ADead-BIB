// ============================================================
// FastOS — FastFS (Native Filesystem)
// ============================================================
// Custom filesystem for FastOS.
// Simple block-based filesystem with inodes.
// ============================================================
//
// Disk Layout:
//   Block 0:     Superblock
//   Block 1-N:   Inode bitmap
//   Block N+1-M: Data bitmap
//   Block M+1-K: Inode table
//   Block K+1-:  Data blocks
//
// Superblock:
//   magic: u32 (0x46535446 = "FSTF")
//   version: u32
//   block_size: u32 (4096)
//   total_blocks: u64
//   total_inodes: u64
//   free_blocks: u64
//   free_inodes: u64
//   root_inode: u64
//
// Inode:
//   size: u64
//   file_type: u8
//   permissions: u16
//   created: u64
//   modified: u64
//   direct_blocks: [u64; 12]
//   indirect_block: u64
//   double_indirect: u64
// ============================================================

use super::vfs::{Filesystem, FileType, DirEntry, FileStat, FsError};

/// FastFS magic number: "FSTF"
pub const FASTFS_MAGIC: u32 = 0x46535446;

/// Block size: 4KB
pub const BLOCK_SIZE: usize = 4096;

/// Maximum inodes in RAM filesystem
const MAX_INODES: usize = 256;

/// Maximum directory entries per directory
const MAX_DIR_ENTRIES: usize = 32;

/// Maximum file data size (48KB per file for RAM-backed FS)
const MAX_FILE_DATA: usize = 48 * 1024;

/// Superblock structure
#[repr(C)]
pub struct Superblock {
    pub magic: u32,
    pub version: u32,
    pub block_size: u32,
    pub _pad: u32,
    pub total_blocks: u64,
    pub total_inodes: u64,
    pub free_blocks: u64,
    pub free_inodes: u64,
    pub root_inode: u64,
}

/// Inode structure
#[repr(C)]
pub struct Inode {
    pub size: u64,
    pub file_type: u8,
    pub _pad: [u8; 3],
    pub permissions: u16,
    pub _pad2: u16,
    pub created: u64,
    pub modified: u64,
    pub direct_blocks: [u64; 12],
    pub indirect_block: u64,
    pub double_indirect: u64,
}

/// In-memory inode for RAM-backed filesystem
struct MemInode {
    used: bool,
    file_type: FileType,
    size: u64,
    created: u64,
    modified: u64,
    permissions: u16,
    // Directory entries (if directory)
    dir_entries: [MemDirEntry; MAX_DIR_ENTRIES],
    dir_count: usize,
    // File data (if regular file, stored inline for simplicity)
    data: [u8; 512],
    data_len: usize,
}

/// In-memory directory entry
#[derive(Clone, Copy)]
struct MemDirEntry {
    name: [u8; 64],
    name_len: usize,
    inode: u64,
}

impl MemDirEntry {
    const fn empty() -> Self {
        MemDirEntry { name: [0; 64], name_len: 0, inode: 0 }
    }
}

impl MemInode {
    const fn empty() -> Self {
        MemInode {
            used: false,
            file_type: FileType::Regular,
            size: 0,
            created: 0,
            modified: 0,
            permissions: 0o755,
            dir_entries: [MemDirEntry::empty(); MAX_DIR_ENTRIES],
            dir_count: 0,
            data: [0; 512],
            data_len: 0,
        }
    }
}

/// FastFS state — RAM-backed filesystem
pub struct FastFs {
    pub initialized: bool,
    inodes: [MemInode; MAX_INODES],
    next_inode: u64,
    // Scratch space for readdir results
    dir_result: [DirEntry; MAX_DIR_ENTRIES],
    dir_result_count: usize,
}

impl FastFs {
    pub const fn new() -> Self {
        const EMPTY_INODE: MemInode = MemInode::empty();
        const EMPTY_DIR: DirEntry = DirEntry {
            name: [0; 256],
            name_len: 0,
            file_type: FileType::Regular,
            size: 0,
            inode: 0,
        };
        FastFs {
            initialized: false,
            inodes: [EMPTY_INODE; MAX_INODES],
            next_inode: 1,
            dir_result: [EMPTY_DIR; MAX_DIR_ENTRIES],
            dir_result_count: 0,
        }
    }

    /// Format and initialize the filesystem with a root directory
    pub fn format(&mut self) {
        // Inode 0 = root directory
        self.inodes[0].used = true;
        self.inodes[0].file_type = FileType::Directory;
        self.inodes[0].size = 0;
        self.inodes[0].permissions = 0o755;
        self.inodes[0].dir_count = 0;
        self.next_inode = 1;
        self.initialized = true;
    }

    /// Allocate a new inode, returns inode number
    fn alloc_inode(&mut self) -> Option<u64> {
        for i in 0..MAX_INODES {
            if !self.inodes[i].used {
                self.inodes[i].used = true;
                if (i as u64) >= self.next_inode {
                    self.next_inode = i as u64 + 1;
                }
                return Some(i as u64);
            }
        }
        None
    }

    /// Free an inode
    fn free_inode(&mut self, inode: u64) {
        let idx = inode as usize;
        if idx < MAX_INODES {
            self.inodes[idx] = MemInode::empty();
        }
    }

    /// Check if name matches a directory entry
    fn name_matches(entry: &MemDirEntry, name: &str) -> bool {
        if entry.name_len != name.len() { return false; }
        let bytes = name.as_bytes();
        for i in 0..entry.name_len {
            if entry.name[i] != bytes[i] { return false; }
        }
        true
    }
}

impl Filesystem for FastFs {
    fn name(&self) -> &str {
        "FastFS"
    }

    fn read(&self, inode: u64, offset: u64, buf: &mut [u8]) -> Result<usize, FsError> {
        let idx = inode as usize;
        if idx >= MAX_INODES || !self.inodes[idx].used {
            return Err(FsError::NotFound);
        }
        let node = &self.inodes[idx];
        if node.file_type == FileType::Directory {
            return Err(FsError::IsADirectory);
        }

        let off = offset as usize;
        if off >= node.data_len { return Ok(0); }

        let available = node.data_len - off;
        let to_read = if buf.len() < available { buf.len() } else { available };
        buf[..to_read].copy_from_slice(&node.data[off..off + to_read]);
        Ok(to_read)
    }

    fn write(&mut self, inode: u64, offset: u64, data: &[u8]) -> Result<usize, FsError> {
        let idx = inode as usize;
        if idx >= MAX_INODES || !self.inodes[idx].used {
            return Err(FsError::NotFound);
        }
        if self.inodes[idx].file_type == FileType::Directory {
            return Err(FsError::IsADirectory);
        }

        let off = offset as usize;
        let end = off + data.len();
        if end > 512 {
            return Err(FsError::DiskFull);
        }

        self.inodes[idx].data[off..end].copy_from_slice(data);
        if end > self.inodes[idx].data_len {
            self.inodes[idx].data_len = end;
            self.inodes[idx].size = end as u64;
        }
        Ok(data.len())
    }

    fn readdir(&self, inode: u64) -> Result<&[DirEntry], FsError> {
        let idx = inode as usize;
        if idx >= MAX_INODES || !self.inodes[idx].used {
            return Err(FsError::NotFound);
        }
        if self.inodes[idx].file_type != FileType::Directory {
            return Err(FsError::NotADirectory);
        }

        // We can't easily return a dynamic slice from const data in no_std
        // without allocation, so we return an empty slice and callers use lookup
        Ok(&[])
    }

    fn lookup(&self, parent: u64, name: &str) -> Result<u64, FsError> {
        let idx = parent as usize;
        if idx >= MAX_INODES || !self.inodes[idx].used {
            return Err(FsError::NotFound);
        }
        if self.inodes[idx].file_type != FileType::Directory {
            return Err(FsError::NotADirectory);
        }

        let node = &self.inodes[idx];
        for i in 0..node.dir_count {
            if Self::name_matches(&node.dir_entries[i], name) {
                return Ok(node.dir_entries[i].inode);
            }
        }
        Err(FsError::NotFound)
    }

    fn create(&mut self, parent: u64, name: &str, file_type: FileType) -> Result<u64, FsError> {
        let pidx = parent as usize;
        if pidx >= MAX_INODES || !self.inodes[pidx].used {
            return Err(FsError::NotFound);
        }
        if self.inodes[pidx].file_type != FileType::Directory {
            return Err(FsError::NotADirectory);
        }

        // Check if name already exists
        if self.lookup(parent, name).is_ok() {
            return Err(FsError::AlreadyExists);
        }

        // Check directory capacity
        if self.inodes[pidx].dir_count >= MAX_DIR_ENTRIES {
            return Err(FsError::DiskFull);
        }

        // Allocate new inode
        let new_inode = self.alloc_inode().ok_or(FsError::DiskFull)?;
        let nidx = new_inode as usize;
        self.inodes[nidx].file_type = file_type;
        self.inodes[nidx].size = 0;
        self.inodes[nidx].permissions = 0o644;
        self.inodes[nidx].data_len = 0;
        self.inodes[nidx].dir_count = 0;

        // Add directory entry to parent
        let slot = self.inodes[pidx].dir_count;
        let bytes = name.as_bytes();
        let len = if bytes.len() > 64 { 64 } else { bytes.len() };
        self.inodes[pidx].dir_entries[slot].name[..len].copy_from_slice(&bytes[..len]);
        self.inodes[pidx].dir_entries[slot].name_len = len;
        self.inodes[pidx].dir_entries[slot].inode = new_inode;
        self.inodes[pidx].dir_count += 1;

        Ok(new_inode)
    }

    fn delete(&mut self, parent: u64, name: &str) -> Result<(), FsError> {
        let pidx = parent as usize;
        if pidx >= MAX_INODES || !self.inodes[pidx].used {
            return Err(FsError::NotFound);
        }

        // Find the entry
        let mut found_idx = None;
        for i in 0..self.inodes[pidx].dir_count {
            if Self::name_matches(&self.inodes[pidx].dir_entries[i], name) {
                found_idx = Some(i);
                break;
            }
        }

        let entry_idx = found_idx.ok_or(FsError::NotFound)?;
        let target_inode = self.inodes[pidx].dir_entries[entry_idx].inode;

        // Free the inode
        self.free_inode(target_inode);

        // Remove directory entry by shifting
        let count = self.inodes[pidx].dir_count;
        for i in entry_idx..count - 1 {
            self.inodes[pidx].dir_entries[i] = self.inodes[pidx].dir_entries[i + 1];
        }
        self.inodes[pidx].dir_entries[count - 1] = MemDirEntry::empty();
        self.inodes[pidx].dir_count -= 1;

        Ok(())
    }

    fn stat(&self, inode: u64) -> Result<FileStat, FsError> {
        let idx = inode as usize;
        if idx >= MAX_INODES || !self.inodes[idx].used {
            return Err(FsError::NotFound);
        }
        let node = &self.inodes[idx];
        Ok(FileStat {
            inode,
            size: node.size,
            file_type: node.file_type,
            created: node.created,
            modified: node.modified,
        })
    }
}

/// Global FastFS instance
static mut FASTFS: FastFs = FastFs::new();

/// Initialize the global FastFS instance
pub fn init() {
    unsafe {
        FASTFS.format();
    }
}

/// Get reference to global FastFS
pub fn get() -> &'static FastFs {
    unsafe { &FASTFS }
}

/// Get mutable reference to global FastFS
pub fn get_mut() -> &'static mut FastFs {
    unsafe { &mut FASTFS }
}
