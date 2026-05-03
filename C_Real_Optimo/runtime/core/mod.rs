//! ADead Runtime - Core Module
//!
//! Funciones base del runtime: inicialización, shutdown, utilidades

use std::ffi::c_void;

/// Inicializa el runtime ADead
#[no_mangle]
pub extern "C" fn adeb_init() -> i32 {
    0 // OK
}

/// Apaga el runtime ADead
#[no_mangle]
pub extern "C" fn adeb_shutdown() -> i32 {
    0 // OK
}

/// Obtiene la versión del runtime
#[no_mangle]
pub extern "C" fn adeb_version() -> *const u8 {
    b"1.0.0\0".as_ptr()
}

/// Handler de panic del runtime
#[no_mangle]
pub extern "C" fn adeb_panic_handler(msg: *const u8) {
    unsafe {
        if !msg.is_null() {
            // TODO: escribir a stderr
        }
    }
    std::process::abort();
}

/// Alloca memoria temporal (stack-like)
#[no_mangle]
pub extern "C" fn adeb_alloca(size: usize) -> *mut c_void {
    let layout = std::alloc::Layout::from_size_align(size, 8).unwrap();
    unsafe { std::alloc::alloc(layout) as *mut c_void }
}

/// Libera memoria temporal
#[no_mangle]
pub extern "C" fn adeb_free(ptr: *mut c_void, size: usize) {
    if ptr.is_null() {
        return;
    }
    let layout = std::alloc::Layout::from_size_align(size, 8).unwrap();
    unsafe { std::alloc::dealloc(ptr as *mut u8, layout) }
}

/// Copia memoria
#[no_mangle]
pub extern "C" fn adeb_memcpy(dst: *mut c_void, src: *const c_void, size: usize) -> *mut c_void {
    unsafe {
        std::ptr::copy::<u8>(src as *const u8, dst as *mut u8, size);
    }
    dst
}

/// Llena memoria con un byte
#[no_mangle]
pub extern "C" fn adeb_memset(ptr: *mut c_void, value: i32, size: usize) -> *mut c_void {
    unsafe {
        std::ptr::write_bytes(ptr.cast::<u8>(), value as u8, size);
    }
    ptr
}

/// Compara memoria
#[no_mangle]
pub extern "C" fn adeb_memcmp(s1: *const c_void, s2: *const c_void, size: usize) -> i32 {
    unsafe {
        let s1_slice = std::slice::from_raw_parts(s1.cast::<u8>(), size);
        let s2_slice = std::slice::from_raw_parts(s2.cast::<u8>(), size);
        s1_slice.cmp(s2_slice) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_shutdown() {
        assert_eq!(adeb_init(), 0);
        assert_eq!(adeb_shutdown(), 0);
    }

    #[test]
    fn test_memset() {
        let mut buf = [0u8; 16];
        adeb_memset(buf.as_mut_ptr() as _, 0xAA, 16);
        assert!(buf.iter().all(|&x| x == 0xAA));
    }

    #[test]
    fn test_memcpy() {
        let src = [1u8, 2, 3, 4];
        let mut dst = [0u8; 4];
        adeb_memcpy(dst.as_mut_ptr() as _, src.as_ptr() as _, 4);
        assert_eq!(dst, src);
    }

    #[test]
    fn test_memcmp() {
        let a = [1u8, 2, 3];
        let b = [1u8, 2, 3];
        let c = [1u8, 2, 4];
        assert_eq!(adeb_memcmp(a.as_ptr() as _, b.as_ptr() as _, 3), 0);
        assert_ne!(adeb_memcmp(a.as_ptr() as _, c.as_ptr() as _, 3), 0);
    }
}
