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

/// FastFS magic number: "FSTF"
pub const FASTFS_MAGIC: u32 = 0x46535446;

/// Block size: 4KB
pub const BLOCK_SIZE: usize = 4096;

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

/// FastFS state (will be initialized when disk driver is ready)
pub struct FastFs {
    pub initialized: bool,
}

impl FastFs {
    pub const fn new() -> Self {
        FastFs { initialized: false }
    }
}

// TODO: Implement Filesystem trait for FastFs when disk driver is ready
