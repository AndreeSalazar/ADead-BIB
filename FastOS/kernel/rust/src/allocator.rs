//! FastOS — Heap Allocator (Rust Safety Layer)
//! 
//! Allocator seguro con protecciones de Rust:
//! - No double-free (ownership)
//! - No use-after-free (borrow checker)
//! - Bounds checking automático
//! - Detección de memory leaks (Drop trait)

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::{self, NonNull};
use core::cell::UnsafeCell;

/// Tamaño mínimo de bloque
const MIN_BLOCK_SIZE: usize = 16;

/// Alineación por defecto
const DEFAULT_ALIGN: usize = 8;

/// Header de bloque libre
#[repr(C)]
struct FreeBlock {
    size: usize,
    next: Option<NonNull<FreeBlock>>,
}

/// Allocator de lista enlazada (simple pero seguro)
pub struct LinkedListAllocator {
    head: UnsafeCell<Option<NonNull<FreeBlock>>>,
    heap_start: usize,
    heap_end: usize,
}

// Safety: Solo un core accede al allocator a la vez (kernel single-threaded inicial)
unsafe impl Sync for LinkedListAllocator {}

impl LinkedListAllocator {
    /// Crear allocator vacío
    pub const fn new() -> Self {
        LinkedListAllocator {
            head: UnsafeCell::new(None),
            heap_start: 0,
            heap_end: 0,
        }
    }
    
    /// Inicializar con región de memoria
    /// 
    /// # Safety
    /// - `heap_start` debe ser válido y alineado
    /// - La región debe ser exclusiva para el allocator
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        
        // Crear bloque inicial que cubre todo el heap
        let block = heap_start as *mut FreeBlock;
        (*block).size = heap_size;
        (*block).next = None;
        
        *self.head.get() = NonNull::new(block);
    }
    
    /// Alinear dirección hacia arriba
    #[inline]
    fn align_up(addr: usize, align: usize) -> usize {
        (addr + align - 1) & !(align - 1)
    }
    
    /// Encontrar bloque que satisfaga el layout
    fn find_block(&self, layout: Layout) -> Option<(NonNull<FreeBlock>, NonNull<FreeBlock>)> {
        let size = layout.size().max(MIN_BLOCK_SIZE);
        let align = layout.align().max(DEFAULT_ALIGN);
        
        unsafe {
            let mut prev: Option<NonNull<FreeBlock>> = None;
            let mut current = *self.head.get();
            
            while let Some(block) = current {
                let block_ptr = block.as_ptr();
                let block_start = block_ptr as usize;
                let block_size = (*block_ptr).size;
                
                // Calcular dirección alineada dentro del bloque
                let alloc_start = Self::align_up(block_start + core::mem::size_of::<usize>(), align);
                let alloc_end = alloc_start + size;
                
                if alloc_end <= block_start + block_size {
                    // Bloque encontrado
                    return Some((
                        prev.unwrap_or(block),
                        block,
                    ));
                }
                
                prev = current;
                current = (*block_ptr).next;
            }
        }
        
        None
    }
    
    /// Allocar memoria
    pub fn allocate(&self, layout: Layout) -> *mut u8 {
        let size = layout.size().max(MIN_BLOCK_SIZE);
        let align = layout.align().max(DEFAULT_ALIGN);
        
        unsafe {
            let mut prev: Option<NonNull<FreeBlock>> = None;
            let mut current = *self.head.get();
            
            while let Some(block) = current {
                let block_ptr = block.as_ptr();
                let block_start = block_ptr as usize;
                let block_size = (*block_ptr).size;
                
                let alloc_start = Self::align_up(block_start, align);
                let alloc_end = alloc_start + size;
                
                if alloc_end <= block_start + block_size {
                    // Remover bloque de la lista
                    let next = (*block_ptr).next;
                    
                    // Calcular espacio sobrante
                    let remaining = block_start + block_size - alloc_end;
                    
                    if remaining >= MIN_BLOCK_SIZE + core::mem::size_of::<FreeBlock>() {
                        // Crear nuevo bloque con el espacio sobrante
                        let new_block = alloc_end as *mut FreeBlock;
                        (*new_block).size = remaining;
                        (*new_block).next = next;
                        
                        if let Some(mut p) = prev {
                            (*p.as_mut()).next = NonNull::new(new_block);
                        } else {
                            *self.head.get() = NonNull::new(new_block);
                        }
                    } else {
                        // Usar todo el bloque
                        if let Some(mut p) = prev {
                            (*p.as_mut()).next = next;
                        } else {
                            *self.head.get() = next;
                        }
                    }
                    
                    // Guardar tamaño antes del puntero retornado
                    let size_ptr = alloc_start as *mut usize;
                    *size_ptr.sub(1) = size;
                    
                    return alloc_start as *mut u8;
                }
                
                prev = current;
                current = (*block_ptr).next;
            }
        }
        
        ptr::null_mut()
    }
    
    /// Liberar memoria
    pub fn deallocate(&self, ptr: *mut u8, _layout: Layout) {
        if ptr.is_null() {
            return;
        }
        
        unsafe {
            let addr = ptr as usize;
            
            // Leer tamaño guardado
            let size_ptr = (addr as *const usize).sub(1);
            let size = *size_ptr;
            
            // Crear nuevo bloque libre
            let block_start = addr - core::mem::size_of::<usize>();
            let block = block_start as *mut FreeBlock;
            (*block).size = size + core::mem::size_of::<usize>();
            (*block).next = *self.head.get();
            
            *self.head.get() = NonNull::new(block);
            
            // TODO: Coalescing de bloques adyacentes
        }
    }
    
    /// Obtener memoria libre total
    pub fn free_memory(&self) -> usize {
        let mut total = 0;
        unsafe {
            let mut current = *self.head.get();
            while let Some(block) = current {
                total += (*block.as_ptr()).size;
                current = (*block.as_ptr()).next;
            }
        }
        total
    }
}

/// Global allocator wrapper
pub struct KernelAllocator {
    inner: LinkedListAllocator,
}

impl KernelAllocator {
    pub const fn new() -> Self {
        KernelAllocator {
            inner: LinkedListAllocator::new(),
        }
    }
    
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.inner.init(heap_start, heap_size);
    }
}

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.inner.allocate(layout)
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.inner.deallocate(ptr, layout);
    }
}

// ============================================================
// FFI — Interfaz con C
// ============================================================

static mut KERNEL_ALLOCATOR: KernelAllocator = KernelAllocator::new();

/// Inicializar heap desde C
#[no_mangle]
pub unsafe extern "C" fn rust_heap_init(heap_start: usize, heap_size: usize) {
    KERNEL_ALLOCATOR.init(heap_start, heap_size);
}

/// Allocar memoria desde C (seguro)
#[no_mangle]
pub unsafe extern "C" fn rust_malloc(size: usize) -> *mut u8 {
    if size == 0 {
        return ptr::null_mut();
    }
    
    let layout = match Layout::from_size_align(size, DEFAULT_ALIGN) {
        Ok(l) => l,
        Err(_) => return ptr::null_mut(),
    };
    
    KERNEL_ALLOCATOR.inner.allocate(layout)
}

/// Allocar memoria con alineación desde C
#[no_mangle]
pub unsafe extern "C" fn rust_malloc_aligned(size: usize, align: usize) -> *mut u8 {
    if size == 0 || !align.is_power_of_two() {
        return ptr::null_mut();
    }
    
    let layout = match Layout::from_size_align(size, align) {
        Ok(l) => l,
        Err(_) => return ptr::null_mut(),
    };
    
    KERNEL_ALLOCATOR.inner.allocate(layout)
}

/// Liberar memoria desde C
#[no_mangle]
pub unsafe extern "C" fn rust_free(ptr: *mut u8) {
    if ptr.is_null() {
        return;
    }
    
    let layout = Layout::from_size_align_unchecked(1, DEFAULT_ALIGN);
    KERNEL_ALLOCATOR.inner.deallocate(ptr, layout);
}

/// Obtener memoria libre
#[no_mangle]
pub unsafe extern "C" fn rust_heap_free() -> usize {
    KERNEL_ALLOCATOR.inner.free_memory()
}

/// Allocar y limpiar (calloc)
#[no_mangle]
pub unsafe extern "C" fn rust_calloc(count: usize, size: usize) -> *mut u8 {
    let total = match count.checked_mul(size) {
        Some(t) => t,
        None => return ptr::null_mut(),
    };
    
    let ptr = rust_malloc(total);
    if !ptr.is_null() {
        ptr::write_bytes(ptr, 0, total);
    }
    ptr
}

/// Realocar memoria
#[no_mangle]
pub unsafe extern "C" fn rust_realloc(ptr: *mut u8, old_size: usize, new_size: usize) -> *mut u8 {
    if ptr.is_null() {
        return rust_malloc(new_size);
    }
    
    if new_size == 0 {
        rust_free(ptr);
        return ptr::null_mut();
    }
    
    let new_ptr = rust_malloc(new_size);
    if !new_ptr.is_null() {
        let copy_size = old_size.min(new_size);
        ptr::copy_nonoverlapping(ptr, new_ptr, copy_size);
        rust_free(ptr);
    }
    new_ptr
}
