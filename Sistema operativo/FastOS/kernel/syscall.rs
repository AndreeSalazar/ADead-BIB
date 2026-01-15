// ============================================================================
// FastOS System Calls (Syscalls)
// ============================================================================
// API para que programas ADead-BIB accedan al framebuffer y sistema
// Los programas llaman a estas funciones a través de la SyscallTable
// ============================================================================

use crate::gpu::GpuDriver;

/// Tabla de syscalls que se pasa a programas ADead-BIB
#[repr(C)]
pub struct SyscallTable {
    // GPU/Framebuffer
    pub gpu_clear: extern "C" fn(color: u32),
    pub gpu_put_pixel: extern "C" fn(x: u32, y: u32, color: u32),
    pub gpu_draw_rect: extern "C" fn(x: u32, y: u32, w: u32, h: u32, color: u32),
    pub gpu_get_width: extern "C" fn() -> u32,
    pub gpu_get_height: extern "C" fn() -> u32,
    
    // Sistema
    pub sys_exit: extern "C" fn(code: i32) -> !,
    pub sys_print: extern "C" fn(msg: *const u8, len: u32),
    
    // Memoria
    pub mem_alloc: extern "C" fn(size: u32) -> *mut u8,
    pub mem_free: extern "C" fn(ptr: *mut u8),
}

// Implementaciones de syscalls

extern "C" fn syscall_gpu_clear(color: u32) {
    if let Some(gpu) = GpuDriver::get() {
        gpu.clear(color);
    }
}

extern "C" fn syscall_gpu_put_pixel(x: u32, y: u32, color: u32) {
    if let Some(gpu) = GpuDriver::get() {
        gpu.put_pixel(x as usize, y as usize, color);
    }
}

extern "C" fn syscall_gpu_draw_rect(x: u32, y: u32, w: u32, h: u32, color: u32) {
    if let Some(gpu) = GpuDriver::get() {
        gpu.draw_rect(x as usize, y as usize, w as usize, h as usize, color);
    }
}

extern "C" fn syscall_gpu_get_width() -> u32 {
    if let Some(gpu) = GpuDriver::get() {
        gpu.width as u32
    } else {
        0
    }
}

extern "C" fn syscall_gpu_get_height() -> u32 {
    if let Some(gpu) = GpuDriver::get() {
        gpu.height as u32
    } else {
        0
    }
}

extern "C" fn syscall_sys_exit(_code: i32) -> ! {
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}

extern "C" fn syscall_sys_print(_msg: *const u8, _len: u32) {
    // TODO: Implementar print a pantalla
}

extern "C" fn syscall_mem_alloc(_size: u32) -> *mut u8 {
    // TODO: Implementar allocator
    core::ptr::null_mut()
}

extern "C" fn syscall_mem_free(_ptr: *mut u8) {
    // TODO: Implementar free
}

/// Tabla de syscalls global
pub static SYSCALL_TABLE: SyscallTable = SyscallTable {
    gpu_clear: syscall_gpu_clear,
    gpu_put_pixel: syscall_gpu_put_pixel,
    gpu_draw_rect: syscall_gpu_draw_rect,
    gpu_get_width: syscall_gpu_get_width,
    gpu_get_height: syscall_gpu_get_height,
    sys_exit: syscall_sys_exit,
    sys_print: syscall_sys_print,
    mem_alloc: syscall_mem_alloc,
    mem_free: syscall_mem_free,
};

/// Obtener puntero a la tabla de syscalls
pub fn get_syscall_table() -> *const SyscallTable {
    &SYSCALL_TABLE as *const SyscallTable
}

// ============================================================================
// Ejemplo de cómo un programa ADead-BIB usaría los syscalls:
// ============================================================================
//
// ```adB
// // programa.adB
// #![target(fastos)]
//
// fn main(syscalls: *SyscallTable) -> i32 {
//     // Limpiar pantalla
//     syscalls.gpu_clear(0x000000);
//     
//     // Dibujar rectángulo rojo
//     syscalls.gpu_draw_rect(100, 100, 200, 150, 0xFF0000);
//     
//     // Obtener dimensiones
//     let w = syscalls.gpu_get_width();
//     let h = syscalls.gpu_get_height();
//     
//     // Dibujar pixel
//     syscalls.gpu_put_pixel(w/2, h/2, 0x00FF00);
//     
//     return 0;
// }
// ```
// ============================================================================
