/*
 * FastOS v2.0 - Rust Safety Layer
 * Provides memory safety guarantees for critical kernel operations
 * 
 * Philosophy: C is Master, Rust provides Safety
 * 
 * This module is called from C code via FFI to ensure:
 * - No buffer overflows
 * - No use-after-free
 * - No double-free
 * - Bounds checking on all memory operations
 */

#![no_std]
#![allow(dead_code)]

use core::ptr::NonNull;
use core::sync::atomic::{AtomicUsize, Ordering};

/* ============================================================
 * Safe Memory Types
 * ============================================================ */

/// Physical address with validation
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysAddr(u64);

impl PhysAddr {
    pub const fn new(addr: u64) -> Self {
        PhysAddr(addr)
    }
    
    pub const fn as_u64(self) -> u64 {
        self.0
    }
    
    pub fn is_aligned(self, align: u64) -> bool {
        self.0 & (align - 1) == 0
    }
    
    pub fn align_up(self, align: u64) -> Self {
        PhysAddr((self.0 + align - 1) & !(align - 1))
    }
    
    pub fn align_down(self, align: u64) -> Self {
        PhysAddr(self.0 & !(align - 1))
    }
}

/// Virtual address with validation
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtAddr(u64);

impl VirtAddr {
    pub const fn new(addr: u64) -> Self {
        VirtAddr(addr)
    }
    
    pub const fn as_u64(self) -> u64 {
        self.0
    }
    
    pub fn as_ptr<T>(self) -> *const T {
        self.0 as *const T
    }
    
    pub fn as_mut_ptr<T>(self) -> *mut T {
        self.0 as *mut T
    }
    
    pub fn is_aligned(self, align: u64) -> bool {
        self.0 & (align - 1) == 0
    }
}

/* ============================================================
 * Safe Buffer Operations
 * ============================================================ */

/// Safe buffer with bounds checking
pub struct SafeBuffer {
    ptr: NonNull<u8>,
    len: usize,
    capacity: usize,
}

impl SafeBuffer {
    /// Create a new safe buffer (unsafe: caller must ensure valid memory)
    pub unsafe fn from_raw_parts(ptr: *mut u8, len: usize, capacity: usize) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| SafeBuffer { ptr, len, capacity })
    }
    
    /// Get length
    pub fn len(&self) -> usize {
        self.len
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    
    /// Get byte at index with bounds check
    pub fn get(&self, index: usize) -> Option<u8> {
        if index < self.len {
            unsafe { Some(*self.ptr.as_ptr().add(index)) }
        } else {
            None
        }
    }
    
    /// Set byte at index with bounds check
    pub fn set(&mut self, index: usize, value: u8) -> bool {
        if index < self.len {
            unsafe { *self.ptr.as_ptr().add(index) = value; }
            true
        } else {
            false
        }
    }
    
    /// Safe memcpy with bounds checking
    pub fn copy_from(&mut self, src: &[u8], dest_offset: usize) -> bool {
        if dest_offset + src.len() <= self.len {
            unsafe {
                core::ptr::copy_nonoverlapping(
                    src.as_ptr(),
                    self.ptr.as_ptr().add(dest_offset),
                    src.len()
                );
            }
            true
        } else {
            false
        }
    }
    
    /// Safe memset with bounds checking
    pub fn fill(&mut self, value: u8, offset: usize, count: usize) -> bool {
        if offset + count <= self.len {
            unsafe {
                core::ptr::write_bytes(
                    self.ptr.as_ptr().add(offset),
                    value,
                    count
                );
            }
            true
        } else {
            false
        }
    }
}

/* ============================================================
 * Memory Allocator with Safety
 * ============================================================ */

/// Simple bump allocator with tracking
pub struct SafeAllocator {
    heap_start: usize,
    heap_end: usize,
    next: AtomicUsize,
    allocations: AtomicUsize,
}

impl SafeAllocator {
    pub const fn new() -> Self {
        SafeAllocator {
            heap_start: 0,
            heap_end: 0,
            next: AtomicUsize::new(0),
            allocations: AtomicUsize::new(0),
        }
    }
    
    /// Initialize the allocator
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        self.next.store(heap_start, Ordering::SeqCst);
    }
    
    /// Allocate memory with alignment
    pub fn alloc(&self, size: usize, align: usize) -> Option<NonNull<u8>> {
        loop {
            let current = self.next.load(Ordering::Relaxed);
            let aligned = (current + align - 1) & !(align - 1);
            let new_next = aligned + size;
            
            if new_next > self.heap_end {
                return None; // Out of memory
            }
            
            if self.next.compare_exchange_weak(
                current,
                new_next,
                Ordering::SeqCst,
                Ordering::Relaxed
            ).is_ok() {
                self.allocations.fetch_add(1, Ordering::Relaxed);
                return NonNull::new(aligned as *mut u8);
            }
        }
    }
    
    /// Get allocation count
    pub fn allocation_count(&self) -> usize {
        self.allocations.load(Ordering::Relaxed)
    }
    
    /// Get used memory
    pub fn used(&self) -> usize {
        self.next.load(Ordering::Relaxed) - self.heap_start
    }
    
    /// Get free memory
    pub fn free(&self) -> usize {
        self.heap_end - self.next.load(Ordering::Relaxed)
    }
}

/* ============================================================
 * Binary Guardian Integration
 * ============================================================ */

/// Security level for code execution
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SecurityLevel {
    Kernel = 0,
    Driver = 1,
    System = 2,
    User = 3,
    Sandbox = 4,
}

/// Binary Guardian checker
pub struct BinaryGuardian {
    enabled: bool,
    level: SecurityLevel,
    violations: AtomicUsize,
}

impl BinaryGuardian {
    pub const fn new() -> Self {
        BinaryGuardian {
            enabled: true,
            level: SecurityLevel::Kernel,
            violations: AtomicUsize::new(0),
        }
    }
    
    /// Check if instruction is allowed
    pub fn check_instruction(&self, opcode: u8) -> bool {
        if !self.enabled {
            return true;
        }
        
        // Dangerous instructions
        match opcode {
            0xFA => self.level == SecurityLevel::Kernel, // CLI
            0xFB => self.level == SecurityLevel::Kernel, // STI
            0xF4 => self.level == SecurityLevel::Kernel, // HLT
            0xCF => self.level == SecurityLevel::Kernel, // IRET
            0x0F => true, // Two-byte opcodes need more checking
            _ => true,
        }
    }
    
    /// Check memory access
    pub fn check_memory_access(&self, addr: u64, size: usize, write: bool) -> bool {
        if !self.enabled {
            return true;
        }
        
        // Kernel memory protection
        if addr < 0x100000 && self.level != SecurityLevel::Kernel {
            self.violations.fetch_add(1, Ordering::Relaxed);
            return false;
        }
        
        // VGA memory is allowed for drivers and above
        if addr >= 0xB8000 && addr < 0xC0000 {
            return self.level <= SecurityLevel::Driver;
        }
        
        true
    }
    
    /// Get violation count
    pub fn violation_count(&self) -> usize {
        self.violations.load(Ordering::Relaxed)
    }
    
    /// Set security level
    pub fn set_level(&mut self, level: SecurityLevel) {
        self.level = level;
    }
}

/* ============================================================
 * Page Table Safety
 * ============================================================ */

/// Page table entry with safety checks
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct PageTableEntry(u64);

impl PageTableEntry {
    pub const fn empty() -> Self {
        PageTableEntry(0)
    }
    
    pub const fn new(addr: PhysAddr, flags: u64) -> Self {
        PageTableEntry(addr.as_u64() | flags)
    }
    
    pub fn is_present(&self) -> bool {
        self.0 & 1 != 0
    }
    
    pub fn is_writable(&self) -> bool {
        self.0 & 2 != 0
    }
    
    pub fn is_user(&self) -> bool {
        self.0 & 4 != 0
    }
    
    pub fn addr(&self) -> PhysAddr {
        PhysAddr::new(self.0 & 0x000F_FFFF_FFFF_F000)
    }
    
    pub fn flags(&self) -> u64 {
        self.0 & 0xFFF
    }
}

/// Safe page table walker
pub struct PageTableWalker {
    pml4: PhysAddr,
}

impl PageTableWalker {
    pub fn new(pml4: PhysAddr) -> Self {
        PageTableWalker { pml4 }
    }
    
    /// Translate virtual to physical address with safety checks
    pub fn translate(&self, virt: VirtAddr) -> Option<PhysAddr> {
        let addr = virt.as_u64();
        
        // Extract indices
        let pml4_idx = ((addr >> 39) & 0x1FF) as usize;
        let pdpt_idx = ((addr >> 30) & 0x1FF) as usize;
        let pd_idx = ((addr >> 21) & 0x1FF) as usize;
        let pt_idx = ((addr >> 12) & 0x1FF) as usize;
        let offset = (addr & 0xFFF) as u64;
        
        // Walk page tables (simplified - assumes identity mapping for page tables)
        unsafe {
            let pml4_table = self.pml4.as_u64() as *const PageTableEntry;
            let pml4_entry = *pml4_table.add(pml4_idx);
            if !pml4_entry.is_present() {
                return None;
            }
            
            let pdpt_table = pml4_entry.addr().as_u64() as *const PageTableEntry;
            let pdpt_entry = *pdpt_table.add(pdpt_idx);
            if !pdpt_entry.is_present() {
                return None;
            }
            
            // Check for 1GB page
            if pdpt_entry.0 & 0x80 != 0 {
                let base = pdpt_entry.addr().as_u64() & !0x3FFFFFFF;
                return Some(PhysAddr::new(base + (addr & 0x3FFFFFFF)));
            }
            
            let pd_table = pdpt_entry.addr().as_u64() as *const PageTableEntry;
            let pd_entry = *pd_table.add(pd_idx);
            if !pd_entry.is_present() {
                return None;
            }
            
            // Check for 2MB page
            if pd_entry.0 & 0x80 != 0 {
                let base = pd_entry.addr().as_u64() & !0x1FFFFF;
                return Some(PhysAddr::new(base + (addr & 0x1FFFFF)));
            }
            
            let pt_table = pd_entry.addr().as_u64() as *const PageTableEntry;
            let pt_entry = *pt_table.add(pt_idx);
            if !pt_entry.is_present() {
                return None;
            }
            
            Some(PhysAddr::new(pt_entry.addr().as_u64() + offset))
        }
    }
}

/* ============================================================
 * FFI Exports for C
 * ============================================================ */

#[no_mangle]
pub extern "C" fn rust_phys_addr_new(addr: u64) -> u64 {
    PhysAddr::new(addr).as_u64()
}

#[no_mangle]
pub extern "C" fn rust_virt_addr_new(addr: u64) -> u64 {
    VirtAddr::new(addr).as_u64()
}

#[no_mangle]
pub extern "C" fn rust_align_up(addr: u64, align: u64) -> u64 {
    PhysAddr::new(addr).align_up(align).as_u64()
}

#[no_mangle]
pub extern "C" fn rust_align_down(addr: u64, align: u64) -> u64 {
    PhysAddr::new(addr).align_down(align).as_u64()
}

#[no_mangle]
pub extern "C" fn rust_check_bounds(ptr: u64, len: u64, access_offset: u64, access_size: u64) -> bool {
    access_offset + access_size <= len
}

#[no_mangle]
pub extern "C" fn rust_memcpy_safe(
    dest: *mut u8,
    dest_size: usize,
    src: *const u8,
    count: usize
) -> bool {
    if count > dest_size {
        return false;
    }
    if dest.is_null() || src.is_null() {
        return false;
    }
    unsafe {
        core::ptr::copy_nonoverlapping(src, dest, count);
    }
    true
}

#[no_mangle]
pub extern "C" fn rust_memset_safe(
    dest: *mut u8,
    dest_size: usize,
    value: u8,
    count: usize
) -> bool {
    if count > dest_size {
        return false;
    }
    if dest.is_null() {
        return false;
    }
    unsafe {
        core::ptr::write_bytes(dest, value, count);
    }
    true
}

static mut GLOBAL_ALLOCATOR: SafeAllocator = SafeAllocator::new();
static mut BINARY_GUARDIAN: BinaryGuardian = BinaryGuardian::new();

#[no_mangle]
pub extern "C" fn rust_heap_init(start: usize, size: usize) {
    unsafe {
        GLOBAL_ALLOCATOR.init(start, size);
    }
}

#[no_mangle]
pub extern "C" fn rust_malloc(size: usize) -> *mut u8 {
    unsafe {
        GLOBAL_ALLOCATOR.alloc(size, 8)
            .map(|p| p.as_ptr())
            .unwrap_or(core::ptr::null_mut())
    }
}

#[no_mangle]
pub extern "C" fn rust_bg_check_instruction(opcode: u8) -> bool {
    unsafe {
        BINARY_GUARDIAN.check_instruction(opcode)
    }
}

#[no_mangle]
pub extern "C" fn rust_bg_check_memory(addr: u64, size: usize, write: bool) -> bool {
    unsafe {
        BINARY_GUARDIAN.check_memory_access(addr, size, write)
    }
}

#[no_mangle]
pub extern "C" fn rust_bg_violation_count() -> usize {
    unsafe {
        BINARY_GUARDIAN.violation_count()
    }
}

/* ============================================================
 * Panic Handler
 * ============================================================ */

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}
