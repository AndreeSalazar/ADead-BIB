// ============================================================================
// memoria_continua.rs — Arena Allocator Determinista
// ============================================================================
//
// FILOSOFÍA: Todas las allocations son O(1). Free = reset del arena entero.
// Cero fragmentación. Cero syscalls en hot path.
//
// El arena pre-aloca un bloque contiguo de memoria al inicio.
// Cada allocation avanza el bump pointer linealmente.
// Cuando el arena se resetea, el bump pointer vuelve a 0.
//
// Esto es BRUTALMENTE más rápido que malloc/free porque:
//   1. No hay free-list traversal
//   2. No hay coalescing
//   3. No hay syscalls (mmap/VirtualAlloc) en hot path
//   4. Cache-friendly: todo es contiguo
//
// Autor: Eddi Andreé Salazar Matos
// ============================================================================

use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;

/// Configuración del Arena Allocator
#[derive(Debug, Clone)]
pub struct ArenaConfig {
    /// Tamaño inicial del arena en bytes (default: 4MB)
    pub initial_size: usize,
    /// Tamaño máximo antes de error (default: 256MB)
    pub max_size: usize,
    /// Alignment por defecto para allocations (default: 16 bytes, SSE-friendly)
    pub default_alignment: usize,
    /// Permitir crecimiento automático (default: false para determinismo)
    pub allow_growth: bool,
}

impl Default for ArenaConfig {
    fn default() -> Self {
        Self {
            initial_size: 4 * 1024 * 1024,   // 4MB
            max_size: 256 * 1024 * 1024,      // 256MB
            default_alignment: 16,             // SSE alignment
            allow_growth: false,               // Determinista: NO auto-grow
        }
    }
}

/// Estadísticas de uso del Arena
#[derive(Debug, Clone, Copy)]
pub struct ArenaStats {
    /// Total de bytes disponibles
    pub capacity: usize,
    /// Bytes actualmente usados
    pub used: usize,
    /// Bytes libres
    pub free: usize,
    /// Número de allocations realizadas desde el último reset
    pub allocation_count: u64,
    /// Peak de uso (watermark)
    pub peak_used: usize,
    /// Número de resets realizados
    pub reset_count: u64,
}

/// Arena Allocator — Bump allocator contiguo determinista
///
/// Invariantes:
///   - Toda allocation es O(1) (bump pointer + alignment)
///   - Free individual NO existe (solo reset global)
///   - El arena NUNCA crece automáticamente (a menos que allow_growth = true)
///   - Overflow = error explícito, NUNCA undefined behavior
#[derive(Debug)]
pub struct Arena {
    /// Pointer al inicio del bloque de memoria
    base: NonNull<u8>,
    /// Tamaño total del bloque
    capacity: usize,
    /// Offset actual (bump pointer)
    offset: usize,
    /// Peak de uso
    peak: usize,
    /// Contadores
    alloc_count: u64,
    reset_count: u64,
    /// Layout original para dealloc
    layout: Layout,
    /// Configuración
    config: ArenaConfig,
}

impl Arena {
    /// Crear un nuevo arena con la configuración dada
    pub fn new(config: ArenaConfig) -> Self {
        let layout = Layout::from_size_align(config.initial_size, config.default_alignment)
            .expect("Arena: invalid layout configuration");

        // SAFETY: Layout is valid, checked above
        let base = unsafe {
            let ptr = alloc(layout);
            NonNull::new(ptr).expect("Arena: allocation failed — out of memory")
        };

        // Zero-initialize para determinismo (evitar info leaks de memoria previa)
        unsafe {
            std::ptr::write_bytes(base.as_ptr(), 0, config.initial_size);
        }

        Self {
            base,
            capacity: config.initial_size,
            offset: 0,
            peak: 0,
            alloc_count: 0,
            reset_count: 0,
            layout,
            config,
        }
    }

    /// Allocar `size` bytes con alignment por defecto
    ///
    /// Retorna un slice mutable al bloque allocado, o error si no hay espacio.
    /// Costo: O(1) — un add + un align-up
    pub fn alloc(&mut self, size: usize) -> Result<&mut [u8], ArenaError> {
        self.alloc_aligned(size, self.config.default_alignment)
    }

    /// Allocar `size` bytes con alignment específico
    ///
    /// El alignment debe ser potencia de 2.
    /// Costo: O(1)
    pub fn alloc_aligned(&mut self, size: usize, alignment: usize) -> Result<&mut [u8], ArenaError> {
        if size == 0 {
            return Err(ArenaError::ZeroSizeAllocation);
        }

        if !alignment.is_power_of_two() {
            return Err(ArenaError::InvalidAlignment(alignment));
        }

        // Align up the current offset
        let aligned_offset = align_up(self.offset, alignment);
        let end = aligned_offset.checked_add(size)
            .ok_or(ArenaError::Overflow)?;

        if end > self.capacity {
            return Err(ArenaError::OutOfMemory {
                requested: size,
                available: self.capacity.saturating_sub(self.offset),
                capacity: self.capacity,
            });
        }

        // Bump the pointer
        let ptr = unsafe { self.base.as_ptr().add(aligned_offset) };
        self.offset = end;
        self.alloc_count += 1;

        // Update peak watermark
        if self.offset > self.peak {
            self.peak = self.offset;
        }

        // SAFETY: ptr is within the allocated block, end <= capacity
        Ok(unsafe { std::slice::from_raw_parts_mut(ptr, size) })
    }

    /// Allocar espacio para un valor de tipo T, alineado a align_of::<T>()
    ///
    /// Retorna un puntero mutable al espacio allocado.
    /// Costo: O(1)
    pub fn alloc_typed<T>(&mut self) -> Result<&mut T, ArenaError> {
        let size = std::mem::size_of::<T>();
        let alignment = std::mem::align_of::<T>();
        let slice = self.alloc_aligned(size, alignment)?;
        // SAFETY: slice is properly aligned and sized for T
        Ok(unsafe { &mut *(slice.as_mut_ptr() as *mut T) })
    }

    /// Allocar un array de N elementos de tipo T
    ///
    /// Retorna un slice mutable al array allocado.
    /// Costo: O(1)
    pub fn alloc_array<T>(&mut self, count: usize) -> Result<&mut [T], ArenaError> {
        let size = std::mem::size_of::<T>().checked_mul(count)
            .ok_or(ArenaError::Overflow)?;
        let alignment = std::mem::align_of::<T>();
        let slice = self.alloc_aligned(size, alignment)?;
        // SAFETY: slice is properly aligned and sized for [T; count]
        Ok(unsafe { std::slice::from_raw_parts_mut(slice.as_mut_ptr() as *mut T, count) })
    }

    /// Reset completo del arena — O(1)
    ///
    /// TODOS los punteros previos se invalidan.
    /// El bump pointer vuelve a 0. No se libera la memoria subyacente.
    pub fn reset(&mut self) {
        // Zero-initialize para determinismo
        unsafe {
            std::ptr::write_bytes(self.base.as_ptr(), 0, self.offset);
        }
        self.offset = 0;
        self.alloc_count = 0;
        self.reset_count += 1;
    }

    /// Reset sin zero-initialize (más rápido pero menos determinista)
    pub fn reset_fast(&mut self) {
        self.offset = 0;
        self.alloc_count = 0;
        self.reset_count += 1;
    }

    /// Obtener estadísticas actuales
    pub fn stats(&self) -> ArenaStats {
        ArenaStats {
            capacity: self.capacity,
            used: self.offset,
            free: self.capacity - self.offset,
            allocation_count: self.alloc_count,
            peak_used: self.peak,
            reset_count: self.reset_count,
        }
    }

    /// Crear un checkpoint (savepoint) para rollback parcial
    pub fn checkpoint(&self) -> ArenaCheckpoint {
        ArenaCheckpoint {
            offset: self.offset,
            alloc_count: self.alloc_count,
        }
    }

    /// Rollback al checkpoint — invalida todas las allocations posteriores
    pub fn rollback(&mut self, checkpoint: ArenaCheckpoint) {
        if checkpoint.offset <= self.offset {
            // Zero the rolled-back region for determinism
            unsafe {
                let start = self.base.as_ptr().add(checkpoint.offset);
                let len = self.offset - checkpoint.offset;
                std::ptr::write_bytes(start, 0, len);
            }
            self.offset = checkpoint.offset;
            self.alloc_count = checkpoint.alloc_count;
        }
    }

    /// Porcentaje de uso
    pub fn usage_percent(&self) -> f64 {
        (self.offset as f64 / self.capacity as f64) * 100.0
    }
}

impl Drop for Arena {
    fn drop(&mut self) {
        // SAFETY: base was allocated with self.layout
        unsafe {
            dealloc(self.base.as_ptr(), self.layout);
        }
    }
}

/// Checkpoint para rollback parcial del arena
#[derive(Debug, Clone, Copy)]
pub struct ArenaCheckpoint {
    offset: usize,
    alloc_count: u64,
}

/// Errores del Arena — siempre explícitos, NUNCA panic
#[derive(Debug, Clone)]
pub enum ArenaError {
    /// Intento de allocar 0 bytes
    ZeroSizeAllocation,
    /// Alignment no es potencia de 2
    InvalidAlignment(usize),
    /// No hay espacio suficiente en el arena
    OutOfMemory {
        requested: usize,
        available: usize,
        capacity: usize,
    },
    /// Overflow aritmético en el cálculo de tamaño
    Overflow,
}

impl std::fmt::Display for ArenaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArenaError::ZeroSizeAllocation => write!(f, "Arena: zero-size allocation not allowed"),
            ArenaError::InvalidAlignment(a) => write!(f, "Arena: alignment {} is not a power of 2", a),
            ArenaError::OutOfMemory { requested, available, capacity } => {
                write!(f, "Arena: out of memory — requested {} bytes, {} available of {} total",
                    requested, available, capacity)
            }
            ArenaError::Overflow => write!(f, "Arena: arithmetic overflow in size calculation"),
        }
    }
}

impl std::error::Error for ArenaError {}

/// Align a value up to the next multiple of alignment
#[inline(always)]
const fn align_up(value: usize, alignment: usize) -> usize {
    (value + alignment - 1) & !(alignment - 1)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arena_basic_alloc() {
        let mut arena = Arena::new(ArenaConfig {
            initial_size: 1024,
            ..Default::default()
        });

        let slice = arena.alloc(64).unwrap();
        assert_eq!(slice.len(), 64);
        assert_eq!(arena.stats().used, 64);
        assert_eq!(arena.stats().allocation_count, 1);
    }

    #[test]
    fn test_arena_alignment() {
        let mut arena = Arena::new(ArenaConfig {
            initial_size: 1024,
            default_alignment: 16,
            ..Default::default()
        });

        // First alloc: 3 bytes → offset becomes 3, but aligned to 16 next time
        let s1 = arena.alloc_aligned(3, 1).unwrap();
        assert_eq!(s1.len(), 3);

        // Second alloc: aligned to 16
        let s2 = arena.alloc_aligned(32, 16).unwrap();
        assert_eq!(s2.len(), 32);
        // s2's address should be 16-aligned
        assert_eq!(s2.as_ptr() as usize % 16, 0);
    }

    #[test]
    fn test_arena_reset() {
        let mut arena = Arena::new(ArenaConfig {
            initial_size: 1024,
            ..Default::default()
        });

        arena.alloc(512).unwrap();
        assert_eq!(arena.stats().used, 512);

        arena.reset();
        assert_eq!(arena.stats().used, 0);
        assert_eq!(arena.stats().reset_count, 1);
        assert_eq!(arena.stats().peak_used, 512);
    }

    #[test]
    fn test_arena_out_of_memory() {
        let mut arena = Arena::new(ArenaConfig {
            initial_size: 64,
            ..Default::default()
        });

        let result = arena.alloc(128);
        assert!(result.is_err());
        match result.unwrap_err() {
            ArenaError::OutOfMemory { requested, .. } => assert_eq!(requested, 128),
            _ => panic!("Expected OutOfMemory error"),
        }
    }

    #[test]
    fn test_arena_checkpoint_rollback() {
        let mut arena = Arena::new(ArenaConfig {
            initial_size: 1024,
            ..Default::default()
        });

        arena.alloc(100).unwrap();
        let cp = arena.checkpoint();
        arena.alloc(200).unwrap();
        assert_eq!(arena.stats().used, 300);

        arena.rollback(cp);
        assert_eq!(arena.stats().used, 100);
    }

    #[test]
    fn test_arena_typed_alloc() {
        let mut arena = Arena::new(ArenaConfig {
            initial_size: 1024,
            ..Default::default()
        });

        let val: &mut u64 = arena.alloc_typed::<u64>().unwrap();
        *val = 0xDEADBEEF;
        assert_eq!(*val, 0xDEADBEEF);
    }

    #[test]
    fn test_arena_array_alloc() {
        let mut arena = Arena::new(ArenaConfig {
            initial_size: 4096,
            ..Default::default()
        });

        let arr: &mut [u32] = arena.alloc_array::<u32>(100).unwrap();
        assert_eq!(arr.len(), 100);
        for i in 0..100 {
            arr[i] = i as u32;
        }
        assert_eq!(arr[99], 99);
    }
}
