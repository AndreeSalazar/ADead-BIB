// ============================================================
// FastOS — x86_64 Paging (4-Level Page Tables)
// ============================================================
// PML4 → PDPT → PD → PT → Physical Frame
// Supports 4KB, 2MB, and 1GB pages.
// ============================================================

use super::cpu;

/// Page table entry flags
pub mod flags {
    pub const PRESENT: u64       = 1 << 0;
    pub const WRITABLE: u64      = 1 << 1;
    pub const USER: u64          = 1 << 2;
    pub const WRITE_THROUGH: u64 = 1 << 3;
    pub const NO_CACHE: u64      = 1 << 4;
    pub const ACCESSED: u64      = 1 << 5;
    pub const DIRTY: u64         = 1 << 6;
    pub const HUGE_PAGE: u64     = 1 << 7;  // 2MB in PD, 1GB in PDPT
    pub const GLOBAL: u64        = 1 << 8;
    pub const NO_EXECUTE: u64    = 1 << 63;
}

/// A single page table entry (8 bytes)
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PageTableEntry(u64);

impl PageTableEntry {
    pub const fn empty() -> Self {
        PageTableEntry(0)
    }

    pub fn is_present(&self) -> bool {
        self.0 & flags::PRESENT != 0
    }

    pub fn set(&mut self, addr: u64, flags: u64) {
        self.0 = (addr & 0x000F_FFFF_FFFF_F000) | flags;
    }

    pub fn address(&self) -> u64 {
        self.0 & 0x000F_FFFF_FFFF_F000
    }

    pub fn flags(&self) -> u64 {
        self.0 & 0xFFF0_0000_0000_0FFF
    }
}

/// A page table (512 entries, 4KB aligned)
#[repr(C, align(4096))]
pub struct PageTable {
    pub entries: [PageTableEntry; 512],
}

impl PageTable {
    pub const fn new() -> Self {
        PageTable {
            entries: [PageTableEntry::empty(); 512],
        }
    }
}

/// Extract page table indices from a virtual address
pub fn pml4_index(vaddr: u64) -> usize {
    ((vaddr >> 39) & 0x1FF) as usize
}

pub fn pdpt_index(vaddr: u64) -> usize {
    ((vaddr >> 30) & 0x1FF) as usize
}

pub fn pd_index(vaddr: u64) -> usize {
    ((vaddr >> 21) & 0x1FF) as usize
}

pub fn pt_index(vaddr: u64) -> usize {
    ((vaddr >> 12) & 0x1FF) as usize
}

pub fn page_offset(vaddr: u64) -> usize {
    (vaddr & 0xFFF) as usize
}

/// Flush TLB for a specific virtual address
pub fn flush_tlb(vaddr: u64) {
    cpu::invlpg(vaddr);
}

/// Flush entire TLB by reloading CR3
pub fn flush_tlb_all() {
    let cr3 = cpu::read_cr3();
    cpu::write_cr3(cr3);
}

/// Get the current PML4 physical address
pub fn current_pml4() -> u64 {
    cpu::read_cr3() & 0x000F_FFFF_FFFF_F000
}
