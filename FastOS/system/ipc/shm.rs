// ============================================================
// FastOS — Shared Memory IPC
// ============================================================
// Named shared memory regions between processes.
// Backed by physical frames from the kernel frame allocator.
// ============================================================

/// Maximum number of shared memory regions
const MAX_SHM_REGIONS: usize = 16;

/// Shared memory region start address (virtual, above kernel heap)
const SHM_BASE: u64 = 0x400000; // 4MB mark
/// Maximum size per region: 64KB
const SHM_MAX_SIZE: usize = 64 * 1024;

/// Shared memory region descriptor
pub struct SharedMemory {
    pub name: [u8; 64],
    pub name_len: usize,
    pub base_addr: u64,
    pub size: usize,
    pub ref_count: u32,
    pub used: bool,
}

impl SharedMemory {
    pub const fn empty() -> Self {
        SharedMemory {
            name: [0; 64],
            name_len: 0,
            base_addr: 0,
            size: 0,
            ref_count: 0,
            used: false,
        }
    }

    /// Set region name
    fn set_name(&mut self, name: &str) {
        let bytes = name.as_bytes();
        let len = if bytes.len() > 64 { 64 } else { bytes.len() };
        self.name[..len].copy_from_slice(&bytes[..len]);
        self.name_len = len;
    }

    /// Check if name matches
    fn name_matches(&self, name: &str) -> bool {
        if self.name_len != name.len() { return false; }
        let bytes = name.as_bytes();
        for i in 0..self.name_len {
            if self.name[i] != bytes[i] { return false; }
        }
        true
    }
}

/// Global shared memory table
static mut SHM_TABLE: [SharedMemory; MAX_SHM_REGIONS] = {
    const EMPTY: SharedMemory = SharedMemory::empty();
    [EMPTY; MAX_SHM_REGIONS]
};

static mut NEXT_SHM_ADDR: u64 = SHM_BASE;

/// Create or open a named shared memory region
/// Returns the base address of the region, or 0 on failure
pub fn shm_open(name: &str, size: usize) -> u64 {
    if size == 0 || size > SHM_MAX_SIZE { return 0; }

    unsafe {
        // Check if region already exists
        for i in 0..MAX_SHM_REGIONS {
            if SHM_TABLE[i].used && SHM_TABLE[i].name_matches(name) {
                SHM_TABLE[i].ref_count += 1;
                return SHM_TABLE[i].base_addr;
            }
        }

        // Find free slot
        for i in 0..MAX_SHM_REGIONS {
            if !SHM_TABLE[i].used {
                // Align size to page boundary
                let aligned_size = (size + 4095) & !4095;

                // Allocate virtual address range
                let addr = NEXT_SHM_ADDR;
                NEXT_SHM_ADDR += aligned_size as u64;

                SHM_TABLE[i].used = true;
                SHM_TABLE[i].set_name(name);
                SHM_TABLE[i].base_addr = addr;
                SHM_TABLE[i].size = aligned_size;
                SHM_TABLE[i].ref_count = 1;

                // Zero the memory region
                let ptr = addr as *mut u8;
                for j in 0..aligned_size {
                    core::ptr::write_volatile(ptr.add(j), 0);
                }

                return addr;
            }
        }

        0 // No free slots
    }
}

/// Close a shared memory region (decrement ref count)
/// Frees the region when ref_count reaches 0
pub fn shm_close(name: &str) -> bool {
    unsafe {
        for i in 0..MAX_SHM_REGIONS {
            if SHM_TABLE[i].used && SHM_TABLE[i].name_matches(name) {
                if SHM_TABLE[i].ref_count > 0 {
                    SHM_TABLE[i].ref_count -= 1;
                }
                if SHM_TABLE[i].ref_count == 0 {
                    SHM_TABLE[i] = SharedMemory::empty();
                }
                return true;
            }
        }
        false
    }
}

/// Get the base address of a named shared memory region
pub fn shm_get(name: &str) -> Option<u64> {
    unsafe {
        for i in 0..MAX_SHM_REGIONS {
            if SHM_TABLE[i].used && SHM_TABLE[i].name_matches(name) {
                return Some(SHM_TABLE[i].base_addr);
            }
        }
        None
    }
}

/// Get the size of a named shared memory region
pub fn shm_size(name: &str) -> Option<usize> {
    unsafe {
        for i in 0..MAX_SHM_REGIONS {
            if SHM_TABLE[i].used && SHM_TABLE[i].name_matches(name) {
                return Some(SHM_TABLE[i].size);
            }
        }
        None
    }
}

/// Count active shared memory regions
pub fn shm_count() -> usize {
    unsafe {
        SHM_TABLE.iter().filter(|s| s.used).count()
    }
}
