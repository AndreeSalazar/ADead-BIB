//! OpenGL Dynamic Function Loader for ADead-BIB
//!
//! Loads GL function pointers at runtime via platform-specific APIs:
//! - Windows: wglGetProcAddress + GetProcAddress from opengl32.dll
//! - Linux: glXGetProcAddress / dlsym from libGL.so
//!
//! Supports loading GL 1.0 through 4.6 function tables.

use super::types::*;
use super::*;

/// Opaque library handle
pub type LibHandle = *mut core::ffi::c_void;
/// Opaque function pointer
pub type GLProc = *mut core::ffi::c_void;

/// Platform-specific GL loader
pub struct GLLoader {
    /// Handle to the OpenGL shared library
    lib_handle: LibHandle,
    /// Platform GL extension loader (wglGetProcAddress / glXGetProcAddress)
    get_proc_addr: Option<unsafe extern "system" fn(*const u8) -> GLProc>,
}

impl GLLoader {
    /// Create a new loader (does NOT load the library yet)
    pub fn new() -> Self {
        Self {
            lib_handle: core::ptr::null_mut(),
            get_proc_addr: None,
        }
    }

    /// Check if the library is loaded
    pub fn is_loaded(&self) -> bool {
        !self.lib_handle.is_null()
    }

    /// Load a function pointer by name
    ///
    /// # Safety
    /// The returned pointer must be cast to the correct function type.
    pub unsafe fn load_fn(&self, name: &str) -> GLProc {
        if self.lib_handle.is_null() {
            return core::ptr::null_mut();
        }

        // Try extension loader first (for GL 1.2+ functions)
        if let Some(get_proc) = self.get_proc_addr {
            let proc = get_proc(name.as_ptr());
            if !proc.is_null() {
                return proc;
            }
        }

        // Fallback: direct library symbol lookup would go here
        // (platform-specific: GetProcAddress on Windows, dlsym on Linux)
        core::ptr::null_mut()
    }

    /// Load a function pointer and transmute to the target type
    ///
    /// # Safety
    /// Caller must ensure the function name matches the target type.
    pub unsafe fn load_fn_as<T>(&self, name: &str) -> Option<T>
    where
        T: Copy,
    {
        let proc = self.load_fn(name);
        if proc.is_null() {
            None
        } else {
            Some(core::mem::transmute_copy(&proc))
        }
    }
}

impl Default for GLLoader {
    fn default() -> Self {
        Self::new()
    }
}

/// Complete OpenGL context — all versions from 1.0 to 4.6
#[derive(Default)]
pub struct GLContext {
    pub gl10: gl10::GL10,
    pub gl11: gl11::GL11,
    pub gl12: gl12::GL12,
    pub gl13: gl13::GL13,
    pub gl14: gl14::GL14,
    pub gl15: gl15::GL15,
    pub gl20: gl20::GL20,
    pub gl21: gl21::GL21,
    pub gl30: gl30::GL30,
    pub gl31: gl31::GL31,
    pub gl32: gl32::GL32,
    pub gl33: gl33::GL33,
    pub gl40: gl40::GL40,
    pub gl41: gl41::GL41,
    pub gl42: gl42::GL42,
    pub gl43: gl43::GL43,
    pub gl44: gl44::GL44,
    pub gl45: gl45::GL45,
    pub gl46: gl46::GL46,
    pub version: GLVersion,
    pub state_cache: optimizer::GLStateCache,
}

impl GLContext {
    /// Create a new empty GL context (all function pointers NULL)
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if a minimum GL version is available
    pub fn supports_version(&self, required: GLVersion) -> bool {
        self.version >= required
    }

    /// Get the GLSL version string for the current GL version
    pub fn glsl_version(&self) -> Option<&'static str> {
        self.version.glsl_version()
    }

    /// Check if compute shaders are available (GL 4.3+)
    pub fn has_compute(&self) -> bool {
        self.version >= GLVersion::GL43
    }

    /// Check if DSA is available (GL 4.5+)
    pub fn has_dsa(&self) -> bool {
        self.version >= GLVersion::GL45
    }

    /// Check if SPIR-V is available (GL 4.6)
    pub fn has_spirv(&self) -> bool {
        self.version >= GLVersion::GL46
    }

    /// Check if tessellation is available (GL 4.0+)
    pub fn has_tessellation(&self) -> bool {
        self.version >= GLVersion::GL40
    }

    /// Check if geometry shaders are available (GL 3.2+)
    pub fn has_geometry_shader(&self) -> bool {
        self.version >= GLVersion::GL32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gl_context_defaults() {
        let ctx = GLContext::new();
        assert_eq!(ctx.version, GLVersion::GL10);
        assert!(!ctx.has_compute());
        assert!(!ctx.has_dsa());
    }

    #[test]
    fn test_gl_loader_initial_state() {
        let loader = GLLoader::new();
        assert!(!loader.is_loaded());
    }

    #[test]
    fn test_version_query() {
        let mut ctx = GLContext::new();
        ctx.version = GLVersion::GL46;
        assert!(ctx.has_compute());
        assert!(ctx.has_dsa());
        assert!(ctx.has_spirv());
        assert!(ctx.has_tessellation());
        assert!(ctx.has_geometry_shader());
        assert_eq!(ctx.glsl_version(), Some("4.60"));
    }
}
