// ============================================================================
// FastOS Heap Allocator
// ============================================================================
// Allocador de memoria simple para el kernel
//
// Author: Eddi Andreé Salazar Matos 🇵🇪
// ============================================================================

#![allow(dead_code)]

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Tamaño del heap (1MB)
const HEAP_SIZE: usize = 1024 * 1024;

/// Heap estático
static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

/// Puntero al siguiente bloque libre
static HEAP_NEXT: AtomicUsize = AtomicUsize::new(0);

/// Allocador bump simple
pub struct BumpAllocator;

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();
        
        loop {
            let current = HEAP_NEXT.load(Ordering::SeqCst);
            let aligned = (current + align - 1) & !(align - 1);
            let next = aligned + size;
            
            if next > HEAP_SIZE {
                return null_mut();
            }
            
            if HEAP_NEXT.compare_exchange(current, next, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                return HEAP.as_mut_ptr().add(aligned);
            }
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Bump allocator no libera memoria individual
        // La memoria se libera toda junta con reset()
    }
}

/// Allocador global
#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator;

/// Resetear el heap (liberar toda la memoria)
pub fn reset() {
    HEAP_NEXT.store(0, Ordering::SeqCst);
}

/// Obtener memoria usada
pub fn used() -> usize {
    HEAP_NEXT.load(Ordering::SeqCst)
}

/// Obtener memoria libre
pub fn free() -> usize {
    HEAP_SIZE - used()
}

/// Obtener tamaño total
pub fn total() -> usize {
    HEAP_SIZE
}

/// Allocar memoria raw
pub fn alloc(size: usize) -> *mut u8 {
    let layout = Layout::from_size_align(size, 8).unwrap();
    unsafe { ALLOCATOR.alloc(layout) }
}

/// Allocar memoria zeroed
pub fn alloc_zeroed(size: usize) -> *mut u8 {
    let ptr = alloc(size);
    if !ptr.is_null() {
        unsafe {
            core::ptr::write_bytes(ptr, 0, size);
        }
    }
    ptr
}
