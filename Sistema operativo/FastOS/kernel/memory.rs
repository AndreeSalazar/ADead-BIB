// ============================================================================
// FastOS - Memory Management
// ============================================================================
// Gestión de memoria del kernel
// - Allocator básico
// - Paging (futuro)
//
// Author: Eddi Andreé Salazar Matos 🇵🇪
// ============================================================================

use spin::Mutex;

/// Tamaño del heap del kernel (64KB inicial)
const HEAP_SIZE: usize = 64 * 1024;

/// Heap estático del kernel
static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

/// Allocator simple (bump allocator)
pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
    allocations: usize,
}

impl BumpAllocator {
    /// Crear nuevo allocator
    pub const fn new() -> Self {
        BumpAllocator {
            heap_start: 0,
            heap_end: 0,
            next: 0,
            allocations: 0,
        }
    }

    /// Inicializar con rango de memoria
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        self.next = heap_start;
    }

    /// Allocar memoria
    pub fn alloc(&mut self, size: usize, align: usize) -> Option<*mut u8> {
        let alloc_start = align_up(self.next, align);
        let alloc_end = alloc_start.checked_add(size)?;

        if alloc_end > self.heap_end {
            None // Sin memoria
        } else {
            self.next = alloc_end;
            self.allocations += 1;
            Some(alloc_start as *mut u8)
        }
    }

    /// Liberar memoria (no-op en bump allocator)
    pub fn dealloc(&mut self) {
        self.allocations -= 1;

        if self.allocations == 0 {
            self.next = self.heap_start;
        }
    }

    /// Memoria usada
    pub fn used(&self) -> usize {
        self.next - self.heap_start
    }

    /// Memoria disponible
    pub fn free(&self) -> usize {
        self.heap_end - self.next
    }
}

/// Allocator global
pub static ALLOCATOR: Mutex<BumpAllocator> = Mutex::new(BumpAllocator::new());

/// Inicializar memoria
pub fn init() {
    unsafe {
        let heap_start = HEAP.as_ptr() as usize;
        ALLOCATOR.lock().init(heap_start, HEAP_SIZE);
    }
}

/// Alinear hacia arriba
fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

/// Obtener estadísticas de memoria
pub fn stats() -> (usize, usize) {
    let alloc = ALLOCATOR.lock();
    (alloc.used(), alloc.free())
}
