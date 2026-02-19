// ============================================================
// FastOS — Memory Manager
// ============================================================
// Physical frame allocator (bitmap) + kernel heap (bump).
// ============================================================

/// Physical frame size: 4KB
pub const FRAME_SIZE: usize = 4096;

/// Maximum physical memory supported: 4GB (1M frames)
const MAX_FRAMES: usize = 1024 * 1024;

/// Bitmap for frame allocation (1 bit per 4KB frame)
/// 1M frames / 8 = 128KB bitmap
static mut FRAME_BITMAP: [u8; MAX_FRAMES / 8] = [0xFF; MAX_FRAMES / 8]; // All marked used initially
static mut TOTAL_FRAMES: usize = 0;
static mut FREE_FRAMES: usize = 0;

/// Initialize memory manager from E820 memory map
pub fn init() {
    // Mark all frames as used initially
    // Stage2 will pass E820 map; for now we set up a basic region
    unsafe {
        // Mark first 16MB as available (conservative default)
        // Skip first 1MB (BIOS, VGA, bootloader)
        let start_frame = 0x100000 / FRAME_SIZE; // Frame 256 (1MB)
        let end_frame = 0x1000000 / FRAME_SIZE;  // Frame 4096 (16MB)

        for frame in start_frame..end_frame {
            free_frame(frame);
        }

        TOTAL_FRAMES = end_frame;
        FREE_FRAMES = end_frame - start_frame;
    }
}

/// Initialize from E820 memory map entries
pub fn init_from_e820(entries: &[crate::boot::E820Entry]) {
    unsafe {
        // Reset bitmap — all used
        for byte in FRAME_BITMAP.iter_mut() {
            *byte = 0xFF;
        }
        FREE_FRAMES = 0;

        for entry in entries {
            if entry.entry_type == crate::boot::memory_type::USABLE {
                let start = entry.base as usize;
                let end = start + entry.length as usize;

                // Skip first 1MB
                let start_frame = if start < 0x100000 { 0x100000 / FRAME_SIZE } else { start / FRAME_SIZE };
                let end_frame = if end > MAX_FRAMES * FRAME_SIZE { MAX_FRAMES } else { end / FRAME_SIZE };

                for frame in start_frame..end_frame {
                    free_frame(frame);
                }
            }
        }
    }
}

/// Allocate a physical frame, returns frame index or None
pub fn alloc_frame() -> Option<usize> {
    unsafe {
        for byte_idx in 0..(MAX_FRAMES / 8) {
            if FRAME_BITMAP[byte_idx] != 0xFF {
                // Found a byte with at least one free bit
                for bit in 0..8 {
                    if FRAME_BITMAP[byte_idx] & (1 << bit) == 0 {
                        // Mark as used
                        FRAME_BITMAP[byte_idx] |= 1 << bit;
                        FREE_FRAMES -= 1;
                        return Some(byte_idx * 8 + bit);
                    }
                }
            }
        }
        None
    }
}

/// Free a physical frame by index
pub fn free_frame(frame: usize) {
    if frame >= MAX_FRAMES { return; }
    unsafe {
        let byte_idx = frame / 8;
        let bit = frame % 8;
        FRAME_BITMAP[byte_idx] &= !(1 << bit);
        FREE_FRAMES += 1;
    }
}

/// Check if a frame is allocated
pub fn is_frame_used(frame: usize) -> bool {
    if frame >= MAX_FRAMES { return true; }
    unsafe {
        let byte_idx = frame / 8;
        let bit = frame % 8;
        FRAME_BITMAP[byte_idx] & (1 << bit) != 0
    }
}

/// Convert frame index to physical address
pub fn frame_to_addr(frame: usize) -> u64 {
    (frame * FRAME_SIZE) as u64
}

/// Convert physical address to frame index
pub fn addr_to_frame(addr: u64) -> usize {
    (addr as usize) / FRAME_SIZE
}

/// Get total memory in bytes
pub fn total_memory() -> usize {
    unsafe { TOTAL_FRAMES * FRAME_SIZE }
}

/// Get free memory in bytes
pub fn free_memory() -> usize {
    unsafe { FREE_FRAMES * FRAME_SIZE }
}

/// Get used memory in bytes
pub fn used_memory() -> usize {
    total_memory() - free_memory()
}

// ============================================================
// Kernel Heap (Bump Allocator)
// ============================================================

/// Kernel heap start (after kernel code, ~2MB mark)
const HEAP_START: usize = 0x200000;
/// Kernel heap size: 1MB
const HEAP_SIZE: usize = 0x100000;

static mut HEAP_NEXT: usize = HEAP_START;

/// Simple bump allocator for kernel heap
pub fn heap_alloc(size: usize, align: usize) -> *mut u8 {
    unsafe {
        // Align up
        let aligned = (HEAP_NEXT + align - 1) & !(align - 1);
        let end = aligned + size;

        if end > HEAP_START + HEAP_SIZE {
            return core::ptr::null_mut(); // Out of heap
        }

        HEAP_NEXT = end;
        aligned as *mut u8
    }
}

/// Get heap usage
pub fn heap_used() -> usize {
    unsafe { HEAP_NEXT - HEAP_START }
}

/// Get heap remaining
pub fn heap_free() -> usize {
    HEAP_SIZE - heap_used()
}

// ============================================================
// Virtual Memory Manager (4-level paging)
// ============================================================

use crate::arch::x86_64::paging::{PageTable, PageTableEntry, flags as PageFlags};

/// Get the current PML4 table address from CR3
pub fn current_pml4() -> u64 {
    crate::arch::x86_64::cpu::read_cr3()
}

/// Map a virtual address to a physical address in the current page tables
/// Creates intermediate tables as needed using frame allocator
pub fn map_page(virt: u64, phys: u64, flags: u64) -> bool {
    let pml4_addr = current_pml4();
    map_page_in(pml4_addr, virt, phys, flags)
}

/// Map a virtual address to a physical address in a specific PML4
pub fn map_page_in(pml4_addr: u64, virt: u64, phys: u64, flags: u64) -> bool {
    let pml4_idx = ((virt >> 39) & 0x1FF) as usize;
    let pdpt_idx = ((virt >> 30) & 0x1FF) as usize;
    let pd_idx   = ((virt >> 21) & 0x1FF) as usize;
    let pt_idx   = ((virt >> 12) & 0x1FF) as usize;

    unsafe {
        let pml4 = &mut *(pml4_addr as *mut PageTable);

        // Ensure PDPT exists
        if !pml4.entries[pml4_idx].is_present() {
            let frame = match alloc_frame() {
                Some(f) => f,
                None => return false,
            };
            let addr = frame_to_addr(frame);
            zero_page(addr);
            pml4.entries[pml4_idx].set(addr, PageFlags::PRESENT | PageFlags::WRITABLE);
        }

        let pdpt_addr = pml4.entries[pml4_idx].address();
        let pdpt = &mut *(pdpt_addr as *mut PageTable);

        // Ensure PD exists
        if !pdpt.entries[pdpt_idx].is_present() {
            let frame = match alloc_frame() {
                Some(f) => f,
                None => return false,
            };
            let addr = frame_to_addr(frame);
            zero_page(addr);
            pdpt.entries[pdpt_idx].set(addr, PageFlags::PRESENT | PageFlags::WRITABLE);
        }

        let pd_addr = pdpt.entries[pdpt_idx].address();
        let pd = &mut *(pd_addr as *mut PageTable);

        // Ensure PT exists
        if !pd.entries[pd_idx].is_present() {
            let frame = match alloc_frame() {
                Some(f) => f,
                None => return false,
            };
            let addr = frame_to_addr(frame);
            zero_page(addr);
            pd.entries[pd_idx].set(addr, PageFlags::PRESENT | PageFlags::WRITABLE);
        }

        let pt_addr = pd.entries[pd_idx].address();
        let pt = &mut *(pt_addr as *mut PageTable);

        // Map the page
        pt.entries[pt_idx].set(phys, flags);

        // Flush TLB for this address
        crate::arch::x86_64::paging::flush_tlb(virt);
    }

    true
}

/// Unmap a virtual address from the current page tables
pub fn unmap_page(virt: u64) {
    let pml4_addr = current_pml4();
    let pml4_idx = ((virt >> 39) & 0x1FF) as usize;
    let pdpt_idx = ((virt >> 30) & 0x1FF) as usize;
    let pd_idx   = ((virt >> 21) & 0x1FF) as usize;
    let pt_idx   = ((virt >> 12) & 0x1FF) as usize;

    unsafe {
        let pml4 = &mut *(pml4_addr as *mut PageTable);
        if !pml4.entries[pml4_idx].is_present() { return; }

        let pdpt = &mut *(pml4.entries[pml4_idx].address() as *mut PageTable);
        if !pdpt.entries[pdpt_idx].is_present() { return; }

        let pd = &mut *(pdpt.entries[pdpt_idx].address() as *mut PageTable);
        if !pd.entries[pd_idx].is_present() { return; }

        let pt = &mut *(pd.entries[pd_idx].address() as *mut PageTable);

        // Clear the entry
        pt.entries[pt_idx].set(0, 0);

        crate::arch::x86_64::paging::flush_tlb(virt);
    }
}

/// Translate a virtual address to physical using current page tables
pub fn virt_to_phys(virt: u64) -> Option<u64> {
    let pml4_addr = current_pml4();
    let pml4_idx = ((virt >> 39) & 0x1FF) as usize;
    let pdpt_idx = ((virt >> 30) & 0x1FF) as usize;
    let pd_idx   = ((virt >> 21) & 0x1FF) as usize;
    let pt_idx   = ((virt >> 12) & 0x1FF) as usize;
    let offset   = virt & 0xFFF;

    unsafe {
        let pml4 = &*(pml4_addr as *const PageTable);
        if !pml4.entries[pml4_idx].is_present() { return None; }

        let pdpt = &*(pml4.entries[pml4_idx].address() as *const PageTable);
        if !pdpt.entries[pdpt_idx].is_present() { return None; }

        let pd = &*(pdpt.entries[pdpt_idx].address() as *const PageTable);
        if !pd.entries[pd_idx].is_present() { return None; }

        let pt = &*(pd.entries[pd_idx].address() as *const PageTable);
        if !pt.entries[pt_idx].is_present() { return None; }

        Some(pt.entries[pt_idx].address() + offset)
    }
}

/// Zero a 4KB page
fn zero_page(addr: u64) {
    unsafe {
        let ptr = addr as *mut u8;
        for i in 0..4096 {
            core::ptr::write_volatile(ptr.add(i), 0);
        }
    }
}

/// Create a new empty PML4 for a user process (identity-maps kernel space)
pub fn create_user_page_table() -> Option<u64> {
    let frame = alloc_frame()?;
    let addr = frame_to_addr(frame);
    zero_page(addr);

    // Copy kernel mappings (upper half) from current PML4
    unsafe {
        let current = &*(current_pml4() as *const PageTable);
        let new_pml4 = &mut *(addr as *mut PageTable);

        // Copy entries 256-511 (kernel space, upper half)
        for i in 256..512 {
            new_pml4.entries[i] = current.entries[i];
        }
    }

    Some(addr)
}
