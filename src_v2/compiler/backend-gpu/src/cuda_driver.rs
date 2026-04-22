// ============================================================
// CUDead-BIB v3.0 — Direct CUDA Driver API
// ============================================================
// Llama directamente a los drivers NVIDIA instalados
// Sin nvcc — Sin cuda_runtime.h — Solo driver API
//
// Los drivers NVIDIA ya están instalados en el sistema
// Solo necesitamos cargar nvcuda.dll y llamar las funciones
// ============================================================

use std::ffi::{c_void, CString};
use std::ptr;

/// CUDA Driver API types
pub type CUdevice = i32;
pub type CUcontext = *mut c_void;
pub type CUmodule = *mut c_void;
pub type CUfunction = *mut c_void;
pub type CUdeviceptr = u64;
pub type CUstream = *mut c_void;
pub type CUresult = i32;

/// CUDA Driver error codes
pub const CUDA_SUCCESS: CUresult = 0;
pub const CUDA_ERROR_INVALID_VALUE: CUresult = 1;
pub const CUDA_ERROR_OUT_OF_MEMORY: CUresult = 2;
pub const CUDA_ERROR_NOT_INITIALIZED: CUresult = 3;
pub const CUDA_ERROR_NO_DEVICE: CUresult = 100;

/// CUDA Driver API function pointers
#[derive(Default)]
pub struct CudaDriverApi {
    // Initialization
    pub cuInit: Option<unsafe extern "system" fn(u32) -> CUresult>,
    
    // Device management
    pub cuDeviceGetCount: Option<unsafe extern "system" fn(*mut i32) -> CUresult>,
    pub cuDeviceGet: Option<unsafe extern "system" fn(*mut CUdevice, i32) -> CUresult>,
    pub cuDeviceGetName: Option<unsafe extern "system" fn(*mut u8, i32, CUdevice) -> CUresult>,
    pub cuDeviceTotalMem: Option<unsafe extern "system" fn(*mut usize, CUdevice) -> CUresult>,
    pub cuDeviceGetAttribute: Option<unsafe extern "system" fn(*mut i32, i32, CUdevice) -> CUresult>,
    
    // Context management
    pub cuCtxCreate: Option<unsafe extern "system" fn(*mut CUcontext, u32, CUdevice) -> CUresult>,
    pub cuCtxDestroy: Option<unsafe extern "system" fn(CUcontext) -> CUresult>,
    pub cuCtxSynchronize: Option<unsafe extern "system" fn() -> CUresult>,
    
    // Module management
    pub cuModuleLoadData: Option<unsafe extern "system" fn(*mut CUmodule, *const c_void) -> CUresult>,
    pub cuModuleGetFunction: Option<unsafe extern "system" fn(*mut CUfunction, CUmodule, *const i8) -> CUresult>,
    pub cuModuleUnload: Option<unsafe extern "system" fn(CUmodule) -> CUresult>,
    
    // Memory management
    pub cuMemAlloc: Option<unsafe extern "system" fn(*mut CUdeviceptr, usize) -> CUresult>,
    pub cuMemFree: Option<unsafe extern "system" fn(CUdeviceptr) -> CUresult>,
    pub cuMemcpyHtoD: Option<unsafe extern "system" fn(CUdeviceptr, *const c_void, usize) -> CUresult>,
    pub cuMemcpyDtoH: Option<unsafe extern "system" fn(*mut c_void, CUdeviceptr, usize) -> CUresult>,
    
    // Kernel launch
    pub cuLaunchKernel: Option<unsafe extern "system" fn(
        CUfunction,
        u32, u32, u32,  // grid
        u32, u32, u32,  // block
        u32,            // shared mem
        CUstream,       // stream
        *mut *mut c_void, // args
        *mut *mut c_void  // extra
    ) -> CUresult>,
    
    // Library handle
    lib_handle: Option<*mut c_void>,
}

impl CudaDriverApi {
    /// Load CUDA driver from nvcuda.dll
    pub fn load() -> Result<Self, String> {
        #[cfg(windows)]
        {
            use std::os::windows::ffi::OsStrExt;
            
            let dll_name = "nvcuda.dll";
            let wide: Vec<u16> = std::ffi::OsStr::new(dll_name)
                .encode_wide()
                .chain(std::iter::once(0))
                .collect();
            
            let handle = unsafe {
                LoadLibraryW(wide.as_ptr())
            };
            
            if handle.is_null() {
                return Err("Failed to load nvcuda.dll - NVIDIA driver not installed?".to_string());
            }
            
            let mut api = CudaDriverApi::default();
            api.lib_handle = Some(handle);
            
            // Load all function pointers
            unsafe {
                api.cuInit = Self::get_proc(handle, "cuInit");
                api.cuDeviceGetCount = Self::get_proc(handle, "cuDeviceGetCount");
                api.cuDeviceGet = Self::get_proc(handle, "cuDeviceGet");
                api.cuDeviceGetName = Self::get_proc(handle, "cuDeviceGetName");
                api.cuDeviceTotalMem = Self::get_proc(handle, "cuDeviceTotalMem_v2");
                api.cuDeviceGetAttribute = Self::get_proc(handle, "cuDeviceGetAttribute");
                api.cuCtxCreate = Self::get_proc(handle, "cuCtxCreate_v2");
                api.cuCtxDestroy = Self::get_proc(handle, "cuCtxDestroy_v2");
                api.cuCtxSynchronize = Self::get_proc(handle, "cuCtxSynchronize");
                api.cuModuleLoadData = Self::get_proc(handle, "cuModuleLoadData");
                api.cuModuleGetFunction = Self::get_proc(handle, "cuModuleGetFunction");
                api.cuModuleUnload = Self::get_proc(handle, "cuModuleUnload");
                api.cuMemAlloc = Self::get_proc(handle, "cuMemAlloc_v2");
                api.cuMemFree = Self::get_proc(handle, "cuMemFree_v2");
                api.cuMemcpyHtoD = Self::get_proc(handle, "cuMemcpyHtoD_v2");
                api.cuMemcpyDtoH = Self::get_proc(handle, "cuMemcpyDtoH_v2");
                api.cuLaunchKernel = Self::get_proc(handle, "cuLaunchKernel");
            }
            
            // Verify essential functions loaded
            if api.cuInit.is_none() {
                return Err("Failed to load cuInit from nvcuda.dll".to_string());
            }
            
            Ok(api)
        }
        
        #[cfg(not(windows))]
        {
            Err("CUDA driver loading only implemented for Windows".to_string())
        }
    }
    
    #[cfg(windows)]
    unsafe fn get_proc<T>(handle: *mut c_void, name: &str) -> Option<T> {
        let cname = CString::new(name).ok()?;
        let ptr = GetProcAddress(handle, cname.as_ptr());
        if ptr.is_null() {
            None
        } else {
            Some(std::mem::transmute_copy(&ptr))
        }
    }
    
    /// Initialize CUDA
    pub fn init(&self) -> Result<(), String> {
        let func = self.cuInit.ok_or("cuInit not loaded")?;
        let result = unsafe { func(0) };
        if result != CUDA_SUCCESS {
            return Err(format!("cuInit failed with error {}", result));
        }
        Ok(())
    }
    
    /// Get device count
    pub fn device_count(&self) -> Result<i32, String> {
        let func = self.cuDeviceGetCount.ok_or("cuDeviceGetCount not loaded")?;
        let mut count = 0;
        let result = unsafe { func(&mut count) };
        if result != CUDA_SUCCESS {
            return Err(format!("cuDeviceGetCount failed with error {}", result));
        }
        Ok(count)
    }
    
    /// Get device handle
    pub fn get_device(&self, ordinal: i32) -> Result<CUdevice, String> {
        let func = self.cuDeviceGet.ok_or("cuDeviceGet not loaded")?;
        let mut device = 0;
        let result = unsafe { func(&mut device, ordinal) };
        if result != CUDA_SUCCESS {
            return Err(format!("cuDeviceGet failed with error {}", result));
        }
        Ok(device)
    }
    
    /// Get device name
    pub fn device_name(&self, device: CUdevice) -> Result<String, String> {
        let func = self.cuDeviceGetName.ok_or("cuDeviceGetName not loaded")?;
        let mut name = [0u8; 256];
        let result = unsafe { func(name.as_mut_ptr(), 256, device) };
        if result != CUDA_SUCCESS {
            return Err(format!("cuDeviceGetName failed with error {}", result));
        }
        let name = unsafe { std::ffi::CStr::from_ptr(name.as_ptr() as *const i8) };
        Ok(name.to_string_lossy().to_string())
    }
    
    /// Get total memory
    pub fn total_memory(&self, device: CUdevice) -> Result<usize, String> {
        let func = self.cuDeviceTotalMem.ok_or("cuDeviceTotalMem not loaded")?;
        let mut bytes = 0usize;
        let result = unsafe { func(&mut bytes, device) };
        if result != CUDA_SUCCESS {
            return Err(format!("cuDeviceTotalMem failed with error {}", result));
        }
        Ok(bytes)
    }
    
    /// Get device attribute
    pub fn get_attribute(&self, device: CUdevice, attrib: i32) -> Result<i32, String> {
        let func = self.cuDeviceGetAttribute.ok_or("cuDeviceGetAttribute not loaded")?;
        let mut value = 0;
        let result = unsafe { func(&mut value, attrib, device) };
        if result != CUDA_SUCCESS {
            return Err(format!("cuDeviceGetAttribute failed with error {}", result));
        }
        Ok(value)
    }
    
    /// Create context
    pub fn create_context(&self, device: CUdevice) -> Result<CUcontext, String> {
        let func = self.cuCtxCreate.ok_or("cuCtxCreate not loaded")?;
        let mut ctx: CUcontext = ptr::null_mut();
        let result = unsafe { func(&mut ctx, 0, device) };
        if result != CUDA_SUCCESS {
            return Err(format!("cuCtxCreate failed with error {}", result));
        }
        Ok(ctx)
    }
    
    /// Destroy context
    pub fn destroy_context(&self, ctx: CUcontext) -> Result<(), String> {
        let func = self.cuCtxDestroy.ok_or("cuCtxDestroy not loaded")?;
        let result = unsafe { func(ctx) };
        if result != CUDA_SUCCESS {
            return Err(format!("cuCtxDestroy failed with error {}", result));
        }
        Ok(())
    }
    
    /// Synchronize context
    pub fn synchronize(&self) -> Result<(), String> {
        let func = self.cuCtxSynchronize.ok_or("cuCtxSynchronize not loaded")?;
        let result = unsafe { func() };
        if result != CUDA_SUCCESS {
            return Err(format!("cuCtxSynchronize failed with error {}", result));
        }
        Ok(())
    }
    
    /// Allocate device memory
    pub fn mem_alloc(&self, size: usize) -> Result<CUdeviceptr, String> {
        let func = self.cuMemAlloc.ok_or("cuMemAlloc not loaded")?;
        let mut ptr: CUdeviceptr = 0;
        let result = unsafe { func(&mut ptr, size) };
        if result != CUDA_SUCCESS {
            return Err(format!("cuMemAlloc failed with error {}", result));
        }
        Ok(ptr)
    }
    
    /// Free device memory
    pub fn mem_free(&self, ptr: CUdeviceptr) -> Result<(), String> {
        let func = self.cuMemFree.ok_or("cuMemFree not loaded")?;
        let result = unsafe { func(ptr) };
        if result != CUDA_SUCCESS {
            return Err(format!("cuMemFree failed with error {}", result));
        }
        Ok(())
    }
    
    /// Copy host to device
    pub fn memcpy_htod(&self, dst: CUdeviceptr, src: *const c_void, size: usize) -> Result<(), String> {
        let func = self.cuMemcpyHtoD.ok_or("cuMemcpyHtoD not loaded")?;
        let result = unsafe { func(dst, src, size) };
        if result != CUDA_SUCCESS {
            return Err(format!("cuMemcpyHtoD failed with error {}", result));
        }
        Ok(())
    }
    
    /// Copy device to host
    pub fn memcpy_dtoh(&self, dst: *mut c_void, src: CUdeviceptr, size: usize) -> Result<(), String> {
        let func = self.cuMemcpyDtoH.ok_or("cuMemcpyDtoH not loaded")?;
        let result = unsafe { func(dst, src, size) };
        if result != CUDA_SUCCESS {
            return Err(format!("cuMemcpyDtoH failed with error {}", result));
        }
        Ok(())
    }
    
    /// Load PTX module
    pub fn load_module(&self, ptx: &str) -> Result<CUmodule, String> {
        let func = self.cuModuleLoadData.ok_or("cuModuleLoadData not loaded")?;
        let ptx_cstr = CString::new(ptx).map_err(|e| e.to_string())?;
        let mut module: CUmodule = ptr::null_mut();
        let result = unsafe { func(&mut module, ptx_cstr.as_ptr() as *const c_void) };
        if result != CUDA_SUCCESS {
            return Err(format!("cuModuleLoadData failed with error {}", result));
        }
        Ok(module)
    }
    
    /// Get kernel function from module
    pub fn get_function(&self, module: CUmodule, name: &str) -> Result<CUfunction, String> {
        let func = self.cuModuleGetFunction.ok_or("cuModuleGetFunction not loaded")?;
        let name_cstr = CString::new(name).map_err(|e| e.to_string())?;
        let mut function: CUfunction = ptr::null_mut();
        let result = unsafe { func(&mut function, module, name_cstr.as_ptr()) };
        if result != CUDA_SUCCESS {
            return Err(format!("cuModuleGetFunction failed with error {}", result));
        }
        Ok(function)
    }
    
    /// Launch kernel
    pub fn launch_kernel(
        &self,
        function: CUfunction,
        grid: (u32, u32, u32),
        block: (u32, u32, u32),
        shared_mem: u32,
        args: &mut [*mut c_void],
    ) -> Result<(), String> {
        let func = self.cuLaunchKernel.ok_or("cuLaunchKernel not loaded")?;
        let result = unsafe {
            func(
                function,
                grid.0, grid.1, grid.2,
                block.0, block.1, block.2,
                shared_mem,
                ptr::null_mut(), // default stream
                args.as_mut_ptr(),
                ptr::null_mut(),
            )
        };
        if result != CUDA_SUCCESS {
            return Err(format!("cuLaunchKernel failed with error {}", result));
        }
        Ok(())
    }
}

// Windows FFI
#[cfg(windows)]
extern "system" {
    fn LoadLibraryW(lpLibFileName: *const u16) -> *mut c_void;
    fn GetProcAddress(hModule: *mut c_void, lpProcName: *const i8) -> *mut c_void;
}

/// Device attributes
pub mod attrib {
    pub const CU_DEVICE_ATTRIBUTE_MAX_THREADS_PER_BLOCK: i32 = 1;
    pub const CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_X: i32 = 2;
    pub const CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_Y: i32 = 3;
    pub const CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_Z: i32 = 4;
    pub const CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_X: i32 = 5;
    pub const CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_Y: i32 = 6;
    pub const CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_Z: i32 = 7;
    pub const CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_BLOCK: i32 = 8;
    pub const CU_DEVICE_ATTRIBUTE_WARP_SIZE: i32 = 10;
    pub const CU_DEVICE_ATTRIBUTE_MULTIPROCESSOR_COUNT: i32 = 16;
    pub const CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR: i32 = 75;
    pub const CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MINOR: i32 = 76;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_driver() {
        // This test will only pass if NVIDIA driver is installed
        match CudaDriverApi::load() {
            Ok(api) => {
                println!("CUDA driver loaded successfully");
                if let Ok(()) = api.init() {
                    println!("CUDA initialized");
                    if let Ok(count) = api.device_count() {
                        println!("Device count: {}", count);
                    }
                }
            }
            Err(e) => {
                println!("CUDA driver not available: {}", e);
            }
        }
    }
}
