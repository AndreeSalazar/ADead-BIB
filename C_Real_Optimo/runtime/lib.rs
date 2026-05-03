//! ADead Runtime
//!
//! Runtime library para C/C++ compilado por ADead-BIB
//! Generado automáticamente desde knowledge.json

pub mod core;
pub mod memory;
pub mod io;
pub mod string;
pub mod math;
pub mod thread;
pub mod win32;
pub mod vulkan;
pub mod dx12;
pub mod opengl;

/// Inicializa el runtime
#[no_mangle]
pub extern "C" fn adeb_runtime_init() -> i32 {
    core::adeb_init()
}

/// Apaga el runtime
#[no_mangle]
pub extern "C" fn adeb_runtime_shutdown() -> i32 {
    core::adeb_shutdown()
}
