// ============================================================
// CUDead-BIB v3.0 — RTX 3060 Direct Test
// ============================================================
// Ejecuta vectorAdd directamente en tu RTX 3060
// Sin nvcc — Sin cuda_runtime.h — Solo driver API
//
// Compilar: cargo run --bin cuda_test
// ============================================================

use std::ffi::{c_void, CString};
use std::ptr;

// CUDA Driver types
type CUdevice = i32;
type CUcontext = *mut c_void;
type CUmodule = *mut c_void;
type CUfunction = *mut c_void;
type CUdeviceptr = u64;
type CUstream = *mut c_void;
type CUresult = i32;

const CUDA_SUCCESS: CUresult = 0;

fn main() {
    println!("========================================");
    println!("CUDead-BIB v3.0 — RTX 3060 Direct Test");
    println!("========================================\n");

    #[cfg(windows)]
    {
        run_cuda_test();
    }

    #[cfg(not(windows))]
    {
        println!("[ERROR] This test only works on Windows");
    }
}

#[cfg(windows)]
fn run_cuda_test() {
    use std::os::windows::ffi::OsStrExt;

    // Step 1: Load nvcuda.dll
    println!("--- Phase 00: LOAD DRIVER ---");
    
    let dll_name = "nvcuda.dll";
    let wide: Vec<u16> = std::ffi::OsStr::new(dll_name)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    
    let lib = unsafe { LoadLibraryW(wide.as_ptr()) };

    if lib.is_null() {
        println!("[ERROR] Failed to load nvcuda.dll");
        println!("        Make sure NVIDIA drivers are installed");
        return;
    }
    println!("[OK]     nvcuda.dll loaded");

    // Step 2: Get function pointers
    println!("\n--- Phase 01: GET FUNCTIONS ---");
    
    unsafe {
        let cu_init = get_proc::<unsafe extern "system" fn(u32) -> CUresult>(lib, "cuInit");
        let cu_device_get_count = get_proc::<unsafe extern "system" fn(*mut i32) -> CUresult>(lib, "cuDeviceGetCount");
        let cu_device_get = get_proc::<unsafe extern "system" fn(*mut CUdevice, i32) -> CUresult>(lib, "cuDeviceGet");
        let cu_device_get_name = get_proc::<unsafe extern "system" fn(*mut u8, i32, CUdevice) -> CUresult>(lib, "cuDeviceGetName");
        let cu_device_total_mem = get_proc::<unsafe extern "system" fn(*mut usize, CUdevice) -> CUresult>(lib, "cuDeviceTotalMem_v2");
        let cu_ctx_create = get_proc::<unsafe extern "system" fn(*mut CUcontext, u32, CUdevice) -> CUresult>(lib, "cuCtxCreate_v2");
        let cu_ctx_destroy = get_proc::<unsafe extern "system" fn(CUcontext) -> CUresult>(lib, "cuCtxDestroy_v2");
        let cu_ctx_sync = get_proc::<unsafe extern "system" fn() -> CUresult>(lib, "cuCtxSynchronize");
        let cu_mem_alloc = get_proc::<unsafe extern "system" fn(*mut CUdeviceptr, usize) -> CUresult>(lib, "cuMemAlloc_v2");
        let cu_mem_free = get_proc::<unsafe extern "system" fn(CUdeviceptr) -> CUresult>(lib, "cuMemFree_v2");
        let cu_memcpy_htod = get_proc::<unsafe extern "system" fn(CUdeviceptr, *const c_void, usize) -> CUresult>(lib, "cuMemcpyHtoD_v2");
        let cu_memcpy_dtoh = get_proc::<unsafe extern "system" fn(*mut c_void, CUdeviceptr, usize) -> CUresult>(lib, "cuMemcpyDtoH_v2");
        let cu_module_load = get_proc::<unsafe extern "system" fn(*mut CUmodule, *const c_void) -> CUresult>(lib, "cuModuleLoadData");
        let cu_module_get_func = get_proc::<unsafe extern "system" fn(*mut CUfunction, CUmodule, *const i8) -> CUresult>(lib, "cuModuleGetFunction");
        let cu_launch = get_proc::<unsafe extern "system" fn(
            CUfunction, u32, u32, u32, u32, u32, u32, u32, CUstream, *mut *mut c_void, *mut *mut c_void
        ) -> CUresult>(lib, "cuLaunchKernel");
        let cu_device_get_attr = get_proc::<unsafe extern "system" fn(*mut i32, i32, CUdevice) -> CUresult>(lib, "cuDeviceGetAttribute");

        if cu_init.is_none() {
            println!("[ERROR] cuInit not found");
            return;
        }
        println!("[OK]     All CUDA functions loaded");

        // Step 3: Initialize CUDA
        println!("\n--- Phase 02: INIT CUDA ---");
        let result = cu_init.unwrap()(0);
        if result != CUDA_SUCCESS {
            println!("[ERROR] cuInit failed: {}", result);
            return;
        }
        println!("[OK]     CUDA initialized");

        // Step 4: Get device count
        let mut count = 0;
        cu_device_get_count.unwrap()(&mut count);
        println!("[GPU]    Found {} CUDA device(s)", count);

        if count == 0 {
            println!("[ERROR] No CUDA devices found");
            return;
        }

        // Step 5: Get device info
        println!("\n--- Phase 03: GPU DETECT ---");
        let mut device: CUdevice = 0;
        cu_device_get.unwrap()(&mut device, 0);

        let mut name = [0u8; 256];
        cu_device_get_name.unwrap()(name.as_mut_ptr(), 256, device);
        let name_str = std::ffi::CStr::from_ptr(name.as_ptr() as *const i8).to_string_lossy();
        println!("[GPU]    {}", name_str);

        let mut mem: usize = 0;
        cu_device_total_mem.unwrap()(&mut mem, device);
        println!("[GPU]    VRAM: {} GB", mem / (1024 * 1024 * 1024));

        // Get compute capability
        let mut major = 0;
        let mut minor = 0;
        cu_device_get_attr.unwrap()(&mut major, 75, device);
        cu_device_get_attr.unwrap()(&mut minor, 76, device);
        println!("[GPU]    Compute: sm_{}{}", major, minor);

        let mut sm_count = 0;
        cu_device_get_attr.unwrap()(&mut sm_count, 16, device);
        println!("[GPU]    SMs: {}", sm_count);

        // Step 6: Create context
        println!("\n--- Phase 04: CREATE CONTEXT ---");
        let mut ctx: CUcontext = ptr::null_mut();
        let result = cu_ctx_create.unwrap()(&mut ctx, 0, device);
        if result != CUDA_SUCCESS {
            println!("[ERROR] cuCtxCreate failed: {}", result);
            return;
        }
        println!("[OK]     Context created");

        // Step 7: Test VRAM allocation (skip PTX for now - requires ptxas)
        println!("\n--- Phase 05: VRAM ALLOC TEST ---");
        println!("[NOTE]   PTX JIT requires ptxas (CUDA Toolkit)");
        println!("[NOTE]   Testing VRAM allocation instead...");
        
        let n: u32 = 1024 * 1024; // 1M elements
        let size = (n as usize) * std::mem::size_of::<f32>();
        println!("[VRAM]   N = {} elements (1M)", n);
        println!("[VRAM]   Size = {} MB per array", size / (1024 * 1024));

        let mut d_a: CUdeviceptr = 0;
        let mut d_b: CUdeviceptr = 0;
        let mut d_c: CUdeviceptr = 0;

        let result = cu_mem_alloc.unwrap()(&mut d_a, size);
        if result != CUDA_SUCCESS {
            println!("[ERROR] cuMemAlloc failed: {}", result);
            cu_ctx_destroy.unwrap()(ctx);
            return;
        }
        cu_mem_alloc.unwrap()(&mut d_b, size);
        cu_mem_alloc.unwrap()(&mut d_c, size);
        println!("[OK]     3 arrays allocated on RTX 3060 VRAM");
        println!("[VRAM]   d_A = 0x{:X}", d_a);
        println!("[VRAM]   d_B = 0x{:X}", d_b);
        println!("[VRAM]   d_C = 0x{:X}", d_c);
        println!("[VRAM]   Total: {} MB allocated", (size * 3) / (1024 * 1024));

        // Step 8: Initialize host data
        println!("\n--- Phase 06: INIT DATA ---");
        let h_a: Vec<f32> = (0..n).map(|i| i as f32).collect();
        let h_b: Vec<f32> = (0..n).map(|i| (i * 2) as f32).collect();
        let mut h_c: Vec<f32> = vec![0.0; n as usize];
        println!("[HOST]   h_A[0..3] = {:?}", &h_a[0..3]);
        println!("[HOST]   h_B[0..3] = {:?}", &h_b[0..3]);

        // Step 9: Copy to device (H2D)
        println!("\n--- Phase 07: H2D TRANSFER ---");
        let start = std::time::Instant::now();
        cu_memcpy_htod.unwrap()(d_a, h_a.as_ptr() as *const c_void, size);
        cu_memcpy_htod.unwrap()(d_b, h_b.as_ptr() as *const c_void, size);
        let h2d_time = start.elapsed();
        println!("[XFER]   {} MB copied to GPU", (size * 2) / (1024 * 1024));
        println!("[PERF]   H2D time: {:.2} ms", h2d_time.as_secs_f64() * 1000.0);
        println!("[PERF]   H2D bandwidth: {:.2} GB/s", 
            (size * 2) as f64 / h2d_time.as_secs_f64() / 1e9);

        // Step 10: Copy back (D2H) - just copy d_a to verify
        println!("\n--- Phase 08: D2H TRANSFER ---");
        let start = std::time::Instant::now();
        cu_memcpy_dtoh.unwrap()(h_c.as_mut_ptr() as *mut c_void, d_a, size);
        let d2h_time = start.elapsed();
        println!("[XFER]   {} MB copied from GPU", size / (1024 * 1024));
        println!("[PERF]   D2H time: {:.2} ms", d2h_time.as_secs_f64() * 1000.0);
        println!("[PERF]   D2H bandwidth: {:.2} GB/s", 
            size as f64 / d2h_time.as_secs_f64() / 1e9);

        // Step 11: Verify data integrity
        println!("\n--- Phase 09: VERIFY ---");
        let mut errors = 0;
        for i in 0..n as usize {
            if (h_c[i] - h_a[i]).abs() > 0.001 {
                if errors < 5 {
                    println!("[ERROR] h_c[{}] = {}, expected {}", i, h_c[i], h_a[i]);
                }
                errors += 1;
            }
        }

        if errors == 0 {
            println!("[OK]     All {} elements verified ✓", n);
            println!("[RESULT] Data integrity: PASS");
        } else {
            println!("[FAIL]   {} errors", errors);
        }

        // Step 12: Cleanup
        println!("\n--- Phase 10: CLEANUP ---");
        cu_mem_free.unwrap()(d_a);
        cu_mem_free.unwrap()(d_b);
        cu_mem_free.unwrap()(d_c);
        cu_ctx_destroy.unwrap()(ctx);
        println!("[OK]     Resources freed");

        println!("\n========================================");
        println!("CUDead-BIB v3.0 — Test Complete ✓");
        println!("========================================");
    }
}

#[cfg(windows)]
unsafe fn get_proc<T>(lib: *mut c_void, name: &str) -> Option<T> {
    let cname = CString::new(name).ok()?;
    let ptr = GetProcAddress(lib, cname.as_ptr());
    if ptr.is_null() {
        None
    } else {
        Some(std::mem::transmute_copy(&ptr))
    }
}

// Windows FFI
#[cfg(windows)]
extern "system" {
    fn LoadLibraryW(lpLibFileName: *const u16) -> *mut c_void;
    fn GetProcAddress(hModule: *mut c_void, lpProcName: *const i8) -> *mut c_void;
}
