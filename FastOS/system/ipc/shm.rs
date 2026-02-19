// ============================================================
// FastOS — Shared Memory IPC
// ============================================================
// Named shared memory regions between processes.
// ============================================================

/// Shared memory region descriptor
pub struct SharedMemory {
    pub name: [u8; 64],
    pub name_len: usize,
    pub base_addr: u64,
    pub size: usize,
    pub ref_count: u32,
}

impl SharedMemory {
    pub const fn empty() -> Self {
        SharedMemory {
            name: [0; 64],
            name_len: 0,
            base_addr: 0,
            size: 0,
            ref_count: 0,
        }
    }
}

// TODO: Implement shared memory allocation when memory manager supports it
