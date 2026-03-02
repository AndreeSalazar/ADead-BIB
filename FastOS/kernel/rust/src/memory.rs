//! FastOS — Memory Manager (Rust Safety Layer)
//! 
//! Virtual Memory Manager con protección de Rust:
//! - Bounds checking automático
//! - No null pointer dereference
//! - No use-after-free
//! - No double-free

use core::ptr::NonNull;

/// Tamaño de página (4KB)
pub const PAGE_SIZE: usize = 4096;

/// Flags de página
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PageFlags {
    Present     = 1 << 0,
    Writable    = 1 << 1,
    User        = 1 << 2,
    WriteThrough = 1 << 3,
    CacheDisable = 1 << 4,
    Accessed    = 1 << 5,
    Dirty       = 1 << 6,
    HugePage    = 1 << 7,
    Global      = 1 << 8,
    NoExecute   = 1 << 63,
}

/// Dirección física (wrapper seguro)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PhysAddr(u64);

impl PhysAddr {
    /// Crear nueva dirección física (validada)
    #[inline]
    pub const fn new(addr: u64) -> Option<Self> {
        // x86-64: solo 52 bits de dirección física
        if addr & 0xFFF0_0000_0000_0000 != 0 {
            None
        } else {
            Some(PhysAddr(addr))
        }
    }
    
    /// Crear sin validación (unsafe)
    #[inline]
    pub const unsafe fn new_unchecked(addr: u64) -> Self {
        PhysAddr(addr)
    }
    
    /// Obtener valor raw
    #[inline]
    pub const fn as_u64(self) -> u64 {
        self.0
    }
    
    /// Alinear hacia abajo a página
    #[inline]
    pub const fn align_down(self) -> Self {
        PhysAddr(self.0 & !(PAGE_SIZE as u64 - 1))
    }
    
    /// Alinear hacia arriba a página
    #[inline]
    pub const fn align_up(self) -> Self {
        PhysAddr((self.0 + PAGE_SIZE as u64 - 1) & !(PAGE_SIZE as u64 - 1))
    }
    
    /// Verificar si está alineado a página
    #[inline]
    pub const fn is_aligned(self) -> bool {
        self.0 % PAGE_SIZE as u64 == 0
    }
}

/// Dirección virtual (wrapper seguro)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct VirtAddr(u64);

impl VirtAddr {
    /// Crear nueva dirección virtual (validada - canonical form)
    #[inline]
    pub const fn new(addr: u64) -> Option<Self> {
        // x86-64 canonical form: bits 48-63 deben ser extensión de bit 47
        let sign_ext = ((addr << 16) as i64 >> 16) as u64;
        if addr == sign_ext {
            Some(VirtAddr(addr))
        } else {
            None
        }
    }
    
    /// Crear sin validación (unsafe)
    #[inline]
    pub const unsafe fn new_unchecked(addr: u64) -> Self {
        VirtAddr(addr)
    }
    
    /// Obtener valor raw
    #[inline]
    pub const fn as_u64(self) -> u64 {
        self.0
    }
    
    /// Convertir a puntero
    #[inline]
    pub const fn as_ptr<T>(self) -> *const T {
        self.0 as *const T
    }
    
    /// Convertir a puntero mutable
    #[inline]
    pub const fn as_mut_ptr<T>(self) -> *mut T {
        self.0 as *mut T
    }
    
    /// Alinear hacia abajo a página
    #[inline]
    pub const fn align_down(self) -> Self {
        VirtAddr(self.0 & !(PAGE_SIZE as u64 - 1))
    }
    
    /// Alinear hacia arriba a página
    #[inline]
    pub const fn align_up(self) -> Self {
        VirtAddr((self.0 + PAGE_SIZE as u64 - 1) & !(PAGE_SIZE as u64 - 1))
    }
    
    /// Obtener índices de page table
    #[inline]
    pub const fn page_table_indices(self) -> [usize; 4] {
        [
            ((self.0 >> 39) & 0x1FF) as usize,  // PML4
            ((self.0 >> 30) & 0x1FF) as usize,  // PDPT
            ((self.0 >> 21) & 0x1FF) as usize,  // PD
            ((self.0 >> 12) & 0x1FF) as usize,  // PT
        ]
    }
    
    /// Obtener offset dentro de página
    #[inline]
    pub const fn page_offset(self) -> usize {
        (self.0 & 0xFFF) as usize
    }
}

/// Región de memoria (segura)
#[derive(Debug)]
pub struct MemoryRegion {
    start: PhysAddr,
    size: usize,
    flags: u64,
}

impl MemoryRegion {
    /// Crear nueva región
    pub fn new(start: PhysAddr, size: usize, flags: u64) -> Option<Self> {
        if size == 0 {
            return None;
        }
        Some(MemoryRegion { start, size, flags })
    }
    
    /// Obtener inicio
    #[inline]
    pub fn start(&self) -> PhysAddr {
        self.start
    }
    
    /// Obtener tamaño
    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }
    
    /// Obtener fin (exclusivo)
    #[inline]
    pub fn end(&self) -> PhysAddr {
        unsafe { PhysAddr::new_unchecked(self.start.as_u64() + self.size as u64) }
    }
    
    /// Verificar si contiene dirección
    #[inline]
    pub fn contains(&self, addr: PhysAddr) -> bool {
        addr >= self.start && addr.as_u64() < self.end().as_u64()
    }
    
    /// Verificar si se superpone con otra región
    pub fn overlaps(&self, other: &MemoryRegion) -> bool {
        self.start < other.end() && other.start < self.end()
    }
}

/// Frame físico (una página de memoria física)
#[derive(Debug)]
pub struct PhysFrame {
    addr: PhysAddr,
}

impl PhysFrame {
    /// Crear frame desde dirección alineada
    pub fn from_addr(addr: PhysAddr) -> Option<Self> {
        if addr.is_aligned() {
            Some(PhysFrame { addr })
        } else {
            None
        }
    }
    
    /// Obtener dirección del frame
    #[inline]
    pub fn addr(&self) -> PhysAddr {
        self.addr
    }
    
    /// Obtener número de frame
    #[inline]
    pub fn number(&self) -> u64 {
        self.addr.as_u64() / PAGE_SIZE as u64
    }
}

// ============================================================
// FFI — Interfaz con C
// ============================================================

/// Crear PhysAddr desde C
#[no_mangle]
pub extern "C" fn rust_phys_addr_new(addr: u64) -> u64 {
    match PhysAddr::new(addr) {
        Some(pa) => pa.as_u64(),
        None => 0,  // Inválido
    }
}

/// Crear VirtAddr desde C
#[no_mangle]
pub extern "C" fn rust_virt_addr_new(addr: u64) -> u64 {
    match VirtAddr::new(addr) {
        Some(va) => va.as_u64(),
        None => 0,  // Inválido
    }
}

/// Alinear dirección hacia abajo
#[no_mangle]
pub extern "C" fn rust_align_down(addr: u64) -> u64 {
    addr & !(PAGE_SIZE as u64 - 1)
}

/// Alinear dirección hacia arriba
#[no_mangle]
pub extern "C" fn rust_align_up(addr: u64) -> u64 {
    (addr + PAGE_SIZE as u64 - 1) & !(PAGE_SIZE as u64 - 1)
}

/// Obtener índices de page table
#[no_mangle]
pub extern "C" fn rust_get_page_indices(addr: u64, out: *mut usize) {
    if let Some(va) = VirtAddr::new(addr) {
        let indices = va.page_table_indices();
        unsafe {
            *out.add(0) = indices[0];
            *out.add(1) = indices[1];
            *out.add(2) = indices[2];
            *out.add(3) = indices[3];
        }
    }
}
