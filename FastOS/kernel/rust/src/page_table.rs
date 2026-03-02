//! FastOS — Page Table Manager (Rust Safety Layer)
//! 
//! Manejo seguro de page tables x86-64:
//! - 4-level paging (PML4 → PDPT → PD → PT)
//! - Bounds checking en índices
//! - Validación de flags
//! - No corrupción de memoria

use crate::memory::{PhysAddr, VirtAddr, PAGE_SIZE};
use core::ptr::NonNull;

/// Número de entradas por tabla
const ENTRIES_PER_TABLE: usize = 512;

/// Flags de entrada de page table
#[repr(u64)]
#[derive(Clone, Copy, Debug)]
pub enum PageTableFlags {
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

/// Entrada de page table (segura)
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PageTableEntry(u64);

impl PageTableEntry {
    /// Crear entrada vacía
    #[inline]
    pub const fn empty() -> Self {
        PageTableEntry(0)
    }
    
    /// Crear entrada con dirección y flags
    pub fn new(addr: PhysAddr, flags: u64) -> Self {
        // Validar que la dirección esté alineada a 4KB
        debug_assert!(addr.is_aligned(), "Address must be page-aligned");
        PageTableEntry((addr.as_u64() & 0x000F_FFFF_FFFF_F000) | flags)
    }
    
    /// Verificar si está presente
    #[inline]
    pub const fn is_present(&self) -> bool {
        self.0 & (PageTableFlags::Present as u64) != 0
    }
    
    /// Verificar si es writable
    #[inline]
    pub const fn is_writable(&self) -> bool {
        self.0 & (PageTableFlags::Writable as u64) != 0
    }
    
    /// Verificar si es accesible desde user mode
    #[inline]
    pub const fn is_user(&self) -> bool {
        self.0 & (PageTableFlags::User as u64) != 0
    }
    
    /// Verificar si es huge page (2MB o 1GB)
    #[inline]
    pub const fn is_huge(&self) -> bool {
        self.0 & (PageTableFlags::HugePage as u64) != 0
    }
    
    /// Obtener dirección física
    #[inline]
    pub fn addr(&self) -> PhysAddr {
        unsafe { PhysAddr::new_unchecked(self.0 & 0x000F_FFFF_FFFF_F000) }
    }
    
    /// Obtener flags
    #[inline]
    pub const fn flags(&self) -> u64 {
        self.0 & 0xFFF0_0000_0000_0FFF
    }
    
    /// Establecer flags
    #[inline]
    pub fn set_flags(&mut self, flags: u64) {
        self.0 = (self.0 & 0x000F_FFFF_FFFF_F000) | flags;
    }
    
    /// Obtener valor raw
    #[inline]
    pub const fn as_u64(&self) -> u64 {
        self.0
    }
}

/// Page Table (512 entradas)
#[repr(C, align(4096))]
pub struct PageTable {
    entries: [PageTableEntry; ENTRIES_PER_TABLE],
}

impl PageTable {
    /// Crear tabla vacía
    pub const fn new() -> Self {
        PageTable {
            entries: [PageTableEntry::empty(); ENTRIES_PER_TABLE],
        }
    }
    
    /// Obtener entrada por índice (bounds checked)
    #[inline]
    pub fn get(&self, index: usize) -> Option<&PageTableEntry> {
        self.entries.get(index)
    }
    
    /// Obtener entrada mutable por índice (bounds checked)
    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut PageTableEntry> {
        self.entries.get_mut(index)
    }
    
    /// Acceso directo (unsafe, sin bounds check)
    #[inline]
    pub unsafe fn get_unchecked(&self, index: usize) -> &PageTableEntry {
        self.entries.get_unchecked(index)
    }
    
    /// Acceso mutable directo (unsafe)
    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut PageTableEntry {
        self.entries.get_unchecked_mut(index)
    }
    
    /// Limpiar toda la tabla
    pub fn clear(&mut self) {
        for entry in self.entries.iter_mut() {
            *entry = PageTableEntry::empty();
        }
    }
    
    /// Contar entradas presentes
    pub fn count_present(&self) -> usize {
        self.entries.iter().filter(|e| e.is_present()).count()
    }
}

/// Page Table Walker (seguro)
pub struct PageTableWalker {
    pml4: NonNull<PageTable>,
}

impl PageTableWalker {
    /// Crear walker desde dirección de PML4
    /// 
    /// # Safety
    /// - `pml4_addr` debe apuntar a una PageTable válida
    pub unsafe fn new(pml4_addr: PhysAddr) -> Self {
        PageTableWalker {
            pml4: NonNull::new_unchecked(pml4_addr.as_u64() as *mut PageTable),
        }
    }
    
    /// Traducir dirección virtual a física
    pub fn translate(&self, virt: VirtAddr) -> Option<PhysAddr> {
        let indices = virt.page_table_indices();
        
        unsafe {
            // PML4
            let pml4 = self.pml4.as_ref();
            let pml4e = pml4.get(indices[0])?;
            if !pml4e.is_present() {
                return None;
            }
            
            // PDPT
            let pdpt = &*(pml4e.addr().as_u64() as *const PageTable);
            let pdpte = pdpt.get(indices[1])?;
            if !pdpte.is_present() {
                return None;
            }
            
            // Check for 1GB huge page
            if pdpte.is_huge() {
                let offset = virt.as_u64() & 0x3FFF_FFFF;  // 30-bit offset
                return PhysAddr::new(pdpte.addr().as_u64() + offset);
            }
            
            // PD
            let pd = &*(pdpte.addr().as_u64() as *const PageTable);
            let pde = pd.get(indices[2])?;
            if !pde.is_present() {
                return None;
            }
            
            // Check for 2MB huge page
            if pde.is_huge() {
                let offset = virt.as_u64() & 0x1F_FFFF;  // 21-bit offset
                return PhysAddr::new(pde.addr().as_u64() + offset);
            }
            
            // PT
            let pt = &*(pde.addr().as_u64() as *const PageTable);
            let pte = pt.get(indices[3])?;
            if !pte.is_present() {
                return None;
            }
            
            // Final address
            let offset = virt.page_offset() as u64;
            PhysAddr::new(pte.addr().as_u64() + offset)
        }
    }
    
    /// Mapear página virtual a física
    /// 
    /// # Safety
    /// - Requiere que las tablas intermedias existan o se puedan crear
    pub unsafe fn map(&mut self, virt: VirtAddr, phys: PhysAddr, flags: u64,
                      alloc_table: fn() -> Option<PhysAddr>) -> Result<(), &'static str> {
        let indices = virt.page_table_indices();
        
        // PML4
        let pml4 = self.pml4.as_mut();
        let pml4e = pml4.get_mut(indices[0]).ok_or("Invalid PML4 index")?;
        
        if !pml4e.is_present() {
            let table_addr = alloc_table().ok_or("Failed to allocate PDPT")?;
            *pml4e = PageTableEntry::new(table_addr, 
                PageTableFlags::Present as u64 | 
                PageTableFlags::Writable as u64 |
                PageTableFlags::User as u64);
            
            // Clear new table
            let new_table = &mut *(table_addr.as_u64() as *mut PageTable);
            new_table.clear();
        }
        
        // PDPT
        let pdpt = &mut *(pml4e.addr().as_u64() as *mut PageTable);
        let pdpte = pdpt.get_mut(indices[1]).ok_or("Invalid PDPT index")?;
        
        if !pdpte.is_present() {
            let table_addr = alloc_table().ok_or("Failed to allocate PD")?;
            *pdpte = PageTableEntry::new(table_addr,
                PageTableFlags::Present as u64 |
                PageTableFlags::Writable as u64 |
                PageTableFlags::User as u64);
            
            let new_table = &mut *(table_addr.as_u64() as *mut PageTable);
            new_table.clear();
        }
        
        // PD
        let pd = &mut *(pdpte.addr().as_u64() as *mut PageTable);
        let pde = pd.get_mut(indices[2]).ok_or("Invalid PD index")?;
        
        if !pde.is_present() {
            let table_addr = alloc_table().ok_or("Failed to allocate PT")?;
            *pde = PageTableEntry::new(table_addr,
                PageTableFlags::Present as u64 |
                PageTableFlags::Writable as u64 |
                PageTableFlags::User as u64);
            
            let new_table = &mut *(table_addr.as_u64() as *mut PageTable);
            new_table.clear();
        }
        
        // PT
        let pt = &mut *(pde.addr().as_u64() as *mut PageTable);
        let pte = pt.get_mut(indices[3]).ok_or("Invalid PT index")?;
        
        *pte = PageTableEntry::new(phys, flags | PageTableFlags::Present as u64);
        
        Ok(())
    }
    
    /// Desmapear página
    pub unsafe fn unmap(&mut self, virt: VirtAddr) -> Option<PhysAddr> {
        let indices = virt.page_table_indices();
        
        let pml4 = self.pml4.as_mut();
        let pml4e = pml4.get(indices[0])?;
        if !pml4e.is_present() { return None; }
        
        let pdpt = &*(pml4e.addr().as_u64() as *const PageTable);
        let pdpte = pdpt.get(indices[1])?;
        if !pdpte.is_present() { return None; }
        
        let pd = &*(pdpte.addr().as_u64() as *const PageTable);
        let pde = pd.get(indices[2])?;
        if !pde.is_present() { return None; }
        
        let pt = &mut *(pde.addr().as_u64() as *mut PageTable);
        let pte = pt.get_mut(indices[3])?;
        
        if !pte.is_present() { return None; }
        
        let phys = pte.addr();
        *pte = PageTableEntry::empty();
        
        // Invalidar TLB
        core::arch::asm!("invlpg [{}]", in(reg) virt.as_u64(), options(nostack, preserves_flags));
        
        Some(phys)
    }
}

// ============================================================
// FFI — Interfaz con C
// ============================================================

/// Crear entrada de page table desde C
#[no_mangle]
pub extern "C" fn rust_pte_new(addr: u64, flags: u64) -> u64 {
    if let Some(phys) = PhysAddr::new(addr) {
        PageTableEntry::new(phys, flags).as_u64()
    } else {
        0
    }
}

/// Verificar si entrada está presente
#[no_mangle]
pub extern "C" fn rust_pte_is_present(entry: u64) -> bool {
    PageTableEntry(entry).is_present()
}

/// Obtener dirección de entrada
#[no_mangle]
pub extern "C" fn rust_pte_addr(entry: u64) -> u64 {
    PageTableEntry(entry).addr().as_u64()
}

/// Traducir dirección virtual a física
#[no_mangle]
pub unsafe extern "C" fn rust_translate(pml4: u64, virt: u64) -> u64 {
    if let (Some(pml4_addr), Some(virt_addr)) = (PhysAddr::new(pml4), VirtAddr::new(virt)) {
        let walker = PageTableWalker::new(pml4_addr);
        match walker.translate(virt_addr) {
            Some(phys) => phys.as_u64(),
            None => 0,
        }
    } else {
        0
    }
}
