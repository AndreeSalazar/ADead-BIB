//! FastOS — Buffer Manager (Rust Safety Layer)
//! 
//! Buffers seguros con protecciones de Rust:
//! - Bounds checking automático
//! - No buffer overflow
//! - No out-of-bounds read/write
//! - Lifetime tracking

use core::ptr;
use core::slice;

/// Buffer seguro con bounds checking
#[derive(Debug)]
pub struct SafeBuffer {
    ptr: *mut u8,
    len: usize,
    capacity: usize,
    owned: bool,
}

impl SafeBuffer {
    /// Crear buffer vacío
    pub const fn empty() -> Self {
        SafeBuffer {
            ptr: ptr::null_mut(),
            len: 0,
            capacity: 0,
            owned: false,
        }
    }
    
    /// Crear buffer desde puntero existente (no owned)
    /// 
    /// # Safety
    /// - `ptr` debe ser válido para `len` bytes
    /// - El buffer debe permanecer válido mientras SafeBuffer exista
    pub unsafe fn from_raw(ptr: *mut u8, len: usize) -> Self {
        SafeBuffer {
            ptr,
            len,
            capacity: len,
            owned: false,
        }
    }
    
    /// Obtener longitud
    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }
    
    /// Verificar si está vacío
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
    
    /// Obtener capacidad
    #[inline]
    pub const fn capacity(&self) -> usize {
        self.capacity
    }
    
    /// Obtener puntero raw (unsafe)
    #[inline]
    pub const fn as_ptr(&self) -> *const u8 {
        self.ptr
    }
    
    /// Obtener puntero mutable raw (unsafe)
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.ptr
    }
    
    /// Obtener slice (bounds checked)
    pub fn as_slice(&self) -> &[u8] {
        if self.ptr.is_null() || self.len == 0 {
            &[]
        } else {
            unsafe { slice::from_raw_parts(self.ptr, self.len) }
        }
    }
    
    /// Obtener slice mutable (bounds checked)
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        if self.ptr.is_null() || self.len == 0 {
            &mut []
        } else {
            unsafe { slice::from_raw_parts_mut(self.ptr, self.len) }
        }
    }
    
    /// Leer byte en índice (bounds checked)
    #[inline]
    pub fn get(&self, index: usize) -> Option<u8> {
        if index < self.len && !self.ptr.is_null() {
            Some(unsafe { *self.ptr.add(index) })
        } else {
            None
        }
    }
    
    /// Escribir byte en índice (bounds checked)
    #[inline]
    pub fn set(&mut self, index: usize, value: u8) -> bool {
        if index < self.len && !self.ptr.is_null() {
            unsafe { *self.ptr.add(index) = value; }
            true
        } else {
            false
        }
    }
    
    /// Leer slice en rango (bounds checked)
    pub fn get_slice(&self, start: usize, len: usize) -> Option<&[u8]> {
        if start.checked_add(len)? <= self.len && !self.ptr.is_null() {
            Some(unsafe { slice::from_raw_parts(self.ptr.add(start), len) })
        } else {
            None
        }
    }
    
    /// Copiar datos al buffer (bounds checked)
    pub fn copy_from(&mut self, src: &[u8], offset: usize) -> bool {
        if offset.checked_add(src.len()).map_or(false, |end| end <= self.len) 
           && !self.ptr.is_null() {
            unsafe {
                ptr::copy_nonoverlapping(src.as_ptr(), self.ptr.add(offset), src.len());
            }
            true
        } else {
            false
        }
    }
    
    /// Copiar datos desde el buffer (bounds checked)
    pub fn copy_to(&self, dst: &mut [u8], offset: usize) -> bool {
        if offset.checked_add(dst.len()).map_or(false, |end| end <= self.len)
           && !self.ptr.is_null() {
            unsafe {
                ptr::copy_nonoverlapping(self.ptr.add(offset), dst.as_mut_ptr(), dst.len());
            }
            true
        } else {
            false
        }
    }
    
    /// Llenar con valor
    pub fn fill(&mut self, value: u8) {
        if !self.ptr.is_null() && self.len > 0 {
            unsafe {
                ptr::write_bytes(self.ptr, value, self.len);
            }
        }
    }
    
    /// Limpiar (llenar con ceros)
    pub fn clear(&mut self) {
        self.fill(0);
    }
}

/// Ring Buffer seguro (para I/O)
pub struct RingBuffer {
    buffer: SafeBuffer,
    read_pos: usize,
    write_pos: usize,
    count: usize,
}

impl RingBuffer {
    /// Crear ring buffer desde memoria existente
    pub unsafe fn from_raw(ptr: *mut u8, capacity: usize) -> Self {
        RingBuffer {
            buffer: SafeBuffer::from_raw(ptr, capacity),
            read_pos: 0,
            write_pos: 0,
            count: 0,
        }
    }
    
    /// Capacidad total
    #[inline]
    pub fn capacity(&self) -> usize {
        self.buffer.capacity()
    }
    
    /// Bytes disponibles para leer
    #[inline]
    pub fn available(&self) -> usize {
        self.count
    }
    
    /// Espacio libre para escribir
    #[inline]
    pub fn free_space(&self) -> usize {
        self.buffer.capacity() - self.count
    }
    
    /// Verificar si está vacío
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    
    /// Verificar si está lleno
    #[inline]
    pub fn is_full(&self) -> bool {
        self.count == self.buffer.capacity()
    }
    
    /// Escribir un byte
    pub fn write_byte(&mut self, byte: u8) -> bool {
        if self.is_full() {
            return false;
        }
        
        if self.buffer.set(self.write_pos, byte) {
            self.write_pos = (self.write_pos + 1) % self.buffer.capacity();
            self.count += 1;
            true
        } else {
            false
        }
    }
    
    /// Leer un byte
    pub fn read_byte(&mut self) -> Option<u8> {
        if self.is_empty() {
            return None;
        }
        
        let byte = self.buffer.get(self.read_pos)?;
        self.read_pos = (self.read_pos + 1) % self.buffer.capacity();
        self.count -= 1;
        Some(byte)
    }
    
    /// Escribir múltiples bytes
    pub fn write(&mut self, data: &[u8]) -> usize {
        let mut written = 0;
        for &byte in data {
            if !self.write_byte(byte) {
                break;
            }
            written += 1;
        }
        written
    }
    
    /// Leer múltiples bytes
    pub fn read(&mut self, buf: &mut [u8]) -> usize {
        let mut read = 0;
        for slot in buf.iter_mut() {
            match self.read_byte() {
                Some(byte) => {
                    *slot = byte;
                    read += 1;
                }
                None => break,
            }
        }
        read
    }
    
    /// Limpiar buffer
    pub fn reset(&mut self) {
        self.read_pos = 0;
        self.write_pos = 0;
        self.count = 0;
    }
}

// ============================================================
// FFI — Interfaz con C
// ============================================================

/// Crear SafeBuffer desde C
#[no_mangle]
pub unsafe extern "C" fn rust_buffer_create(ptr: *mut u8, len: usize) -> *mut SafeBuffer {
    let buffer = Box::new(SafeBuffer::from_raw(ptr, len));
    Box::into_raw(buffer)
}

/// Destruir SafeBuffer
#[no_mangle]
pub unsafe extern "C" fn rust_buffer_destroy(buffer: *mut SafeBuffer) {
    if !buffer.is_null() {
        drop(Box::from_raw(buffer));
    }
}

/// Leer byte de buffer (bounds checked)
#[no_mangle]
pub unsafe extern "C" fn rust_buffer_get(buffer: *const SafeBuffer, index: usize) -> i32 {
    if buffer.is_null() {
        return -1;
    }
    match (*buffer).get(index) {
        Some(b) => b as i32,
        None => -1,
    }
}

/// Escribir byte en buffer (bounds checked)
#[no_mangle]
pub unsafe extern "C" fn rust_buffer_set(buffer: *mut SafeBuffer, index: usize, value: u8) -> bool {
    if buffer.is_null() {
        return false;
    }
    (*buffer).set(index, value)
}

/// Copiar datos al buffer (bounds checked)
#[no_mangle]
pub unsafe extern "C" fn rust_buffer_copy_from(
    buffer: *mut SafeBuffer, 
    src: *const u8, 
    src_len: usize,
    offset: usize
) -> bool {
    if buffer.is_null() || src.is_null() {
        return false;
    }
    let src_slice = slice::from_raw_parts(src, src_len);
    (*buffer).copy_from(src_slice, offset)
}

/// Copiar datos desde buffer (bounds checked)
#[no_mangle]
pub unsafe extern "C" fn rust_buffer_copy_to(
    buffer: *const SafeBuffer,
    dst: *mut u8,
    dst_len: usize,
    offset: usize
) -> bool {
    if buffer.is_null() || dst.is_null() {
        return false;
    }
    let dst_slice = slice::from_raw_parts_mut(dst, dst_len);
    (*buffer).copy_to(dst_slice, offset)
}

/// Llenar buffer con valor
#[no_mangle]
pub unsafe extern "C" fn rust_buffer_fill(buffer: *mut SafeBuffer, value: u8) {
    if !buffer.is_null() {
        (*buffer).fill(value);
    }
}

/// Memoria segura: copiar con bounds check
#[no_mangle]
pub unsafe extern "C" fn rust_memcpy_safe(
    dst: *mut u8,
    dst_size: usize,
    src: *const u8,
    count: usize
) -> bool {
    if dst.is_null() || src.is_null() {
        return false;
    }
    if count > dst_size {
        return false;  // Buffer overflow prevented!
    }
    ptr::copy_nonoverlapping(src, dst, count);
    true
}

/// Memoria segura: mover con bounds check
#[no_mangle]
pub unsafe extern "C" fn rust_memmove_safe(
    dst: *mut u8,
    dst_size: usize,
    src: *const u8,
    count: usize
) -> bool {
    if dst.is_null() || src.is_null() {
        return false;
    }
    if count > dst_size {
        return false;
    }
    ptr::copy(src, dst, count);
    true
}

/// Memoria segura: set con bounds check
#[no_mangle]
pub unsafe extern "C" fn rust_memset_safe(
    dst: *mut u8,
    dst_size: usize,
    value: u8,
    count: usize
) -> bool {
    if dst.is_null() {
        return false;
    }
    if count > dst_size {
        return false;
    }
    ptr::write_bytes(dst, value, count);
    true
}
