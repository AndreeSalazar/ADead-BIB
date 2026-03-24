// ============================================================
// CUDead-BIB v3.0 — RTX 3060 Test
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

// Windows FFI
#[cfg(windows)]
#[link(name = "kernel32")]
extern "system" {
    fn LoadLibraryW(lpLibFileName: *const u16) -> *mut c_void;
    fn GetProcAddress(hModule: *mut c_void, lpProcName: *const i8) -> *mut c_void;
}

fn main() {
    println!("========================================");
    println!("CUDead-BIB v3.0 — RTX 3060 Direct Test");
    println!("========================================\n");

    // Step 1: Load nvcuda.dll
    println!("--- Phase 00: LOAD DRIVER ---");
    
    #[cfg(windows)]
    let lib = unsafe {
        let name: Vec<u16> = "nvcuda.dll\0".encode_utf16().collect();
        LoadLibraryW(name.as_ptr())
    };

    #[cfg(not(windows))]
    let lib: *mut c_void = ptr::null_mut();

    if lib.is_null() {
        println!("[ERROR] Failed to load nvcuda.dll");
        println!("        Make sure NVIDIA drivers are installed");
        return;
    }
    println!("[OK]     nvcuda.dll loaded");

    // Step 2: Get function pointers
    println!("\n--- Phase 01: GET FUNCTIONS ---");
    
    #[cfg(windows)]
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
        cu_device_get_attr.unwrap()(&mut major, 75, device); // CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR
        cu_device_get_attr.unwrap()(&mut minor, 76, device); // CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MINOR
        println!("[GPU]    Compute: sm_{}{}", major, minor);

        let mut sm_count = 0;
        cu_device_get_attr.unwrap()(&mut sm_count, 16, device); // CU_DEVICE_ATTRIBUTE_MULTIPROCESSOR_COUNT
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

        // Step 7: Load PTX module
        println!("\n--- Phase 05: LOAD PTX ---");
        let ptx = r#"
.version 7.0
.target sm_86
.address_size 64

.visible .entry vectorAdd(
    .param .u64 A,
    .param .u64 B,
    .param .u64 C,
    .param .u32 n
) {
    .reg .pred %p<2>;
    .reg .f32 %f<4>;
    .reg .b32 %r<6>;
    .reg .b64 %rd<10>;

    ld.param.u64 %rd1, [A];
    ld.param.u64 %rd2, [B];
    ld.param.u64 %rd3, [C];
    ld.param.u32 %r1, [n];

    mov.u32 %r2, %ctaid.x;
    mov.u32 %r3, %ntid.x;
    mov.u32 %r4, %tid.x;
    mad.lo.s32 %r5, %r2, %r3, %r4;

    setp.ge.s32 %p1, %r5, %r1;
    @%p1 bra $L_exit;

    cvt.s64.s32 %rd4, %r5;
    shl.b64 %rd5, %rd4, 2;

    add.s64 %rd6, %rd1, %rd5;
    add.s64 %rd7, %rd2, %rd5;
    add.s64 %rd8, %rd3, %rd5;

    ld.global.f32 %f1, [%rd6];
    ld.global.f32 %f2, [%rd7];
    add.f32 %f3, %f1, %f2;
    st.global.f32 [%rd8], %f3;

$L_exit:
    ret;
}
"#;
        let ptx_cstr = CString::new(ptx).unwrap();
        let mut module: CUmodule = ptr::null_mut();
        let result = cu_module_load.unwrap()(&mut module, ptx_cstr.as_ptr() as *const c_void);
        if result != CUDA_SUCCESS {
            println!("[ERROR] cuModuleLoadData failed: {}", result);
            println!("        PTX compilation error");
            cu_ctx_destroy.unwrap()(ctx);
            return;
        }
        println!("[OK]     PTX module loaded");

        // Step 8: Get kernel function
        let kernel_name = CString::new("vectorAdd").unwrap();
        let mut kernel: CUfunction = ptr::null_mut();
        let result = cu_module_get_func.unwrap()(&mut kernel, module, kernel_name.as_ptr());
        if result != CUDA_SUCCESS {
            println!("[ERROR] cuModuleGetFunction failed: {}", result);
            cu_ctx_destroy.unwrap()(ctx);
            return;
        }
        println!("[OK]     Kernel 'vectorAdd' found");

        // Step 9: Allocate memory
        println!("\n--- Phase 06: VRAM ALLOC ---");
        let n: u32 = 1024;
        let size = (n as usize) * std::mem::size_of::<f32>();
        println!("[VRAM]   N = {} elements", n);
        println!("[VRAM]   Size = {} bytes per array", size);

        let mut d_a: CUdeviceptr = 0;
        let mut d_b: CUdeviceptr = 0;
        let mut d_c: CUdeviceptr = 0;

        cu_mem_alloc.unwrap()(&mut d_a, size);
        cu_mem_alloc.unwrap()(&mut d_b, size);
        cu_mem_alloc.unwrap()(&mut d_c, size);
        println!("[VRAM]   d_A = 0x{:X}", d_a);
        println!("[VRAM]   d_B = 0x{:X}", d_b);
        println!("[VRAM]   d_C = 0x{:X}", d_c);

        // Step 10: Initialize host data
        println!("\n--- Phase 07: INIT DATA ---");
        let h_a: Vec<f32> = (0..n).map(|i| i as f32).collect();
        let h_b: Vec<f32> = (0..n).map(|i| (i * 2) as f32).collect();
        let mut h_c: Vec<f32> = vec![0.0; n as usize];
        println!("[HOST]   h_A[0..3] = {:?}", &h_a[0..3]);
        println!("[HOST]   h_B[0..3] = {:?}", &h_b[0..3]);

        // Step 11: Copy to device
        println!("\n--- Phase 08: H2D TRANSFER ---");
        cu_memcpy_htod.unwrap()(d_a, h_a.as_ptr() as *const c_void, size);
        cu_memcpy_htod.unwrap()(d_b, h_b.as_ptr() as *const c_void, size);
        println!("[XFER]   {} bytes copied to GPU", size * 2);

        // Step 12: Launch kernel
        println!("\n--- Phase 09: LAUNCH KERNEL ---");
        let block_size = 256u32;
        let grid_size = (n + block_size - 1) / block_size;
        println!("[LAUNCH] Grid: {}", grid_size);
        println!("[LAUNCH] Block: {}", block_size);
        println!("[LAUNCH] Total threads: {}", grid_size * block_size);

        let mut args: [*mut c_void; 4] = [
            &d_a as *const _ as *mut c_void,
            &d_b as *const _ as *mut c_void,
            &d_c as *const _ as *mut c_void,
            &n as *const _ as *mut c_void,
        ];

        let result = cu_launch.unwrap()(
            kernel,
            grid_size, 1, 1,
            block_size, 1, 1,
            0,
            ptr::null_mut(),
            args.as_mut_ptr(),
            ptr::null_mut(),
        );
        if result != CUDA_SUCCESS {
            println!("[ERROR] cuLaunchKernel failed: {}", result);
            cu_mem_free.unwrap()(d_a);
            cu_mem_free.unwrap()(d_b);
            cu_mem_free.unwrap()(d_c);
            cu_ctx_destroy.unwrap()(ctx);
            return;
        }
        println!("[OK]     Kernel launched");

        // Step 13: Synchronize
        println!("\n--- Phase 10: SYNC ---");
        let result = cu_ctx_sync.unwrap()();
        if result != CUDA_SUCCESS {
            println!("[ERROR] cuCtxSynchronize failed: {}", result);
        } else {
            println!("[OK]     GPU synchronized");
        }

        // Step 14: Copy back
        println!("\n--- Phase 11: D2H TRANSFER ---");
        cu_memcpy_dtoh.unwrap()(h_c.as_mut_ptr() as *mut c_void, d_c, size);
        println!("[XFER]   {} bytes copied from GPU", size);

        // Step 15: Verify
        println!("\n--- Phase 12: VERIFY ---");
        let mut errors = 0;
        for i in 0..n as usize {
            let expected = h_a[i] + h_b[i];
            if (h_c[i] - expected).abs() > 0.001 {
                if errors < 5 {
                    println!("[ERROR] h_c[{}] = {}, expected {}", i, h_c[i], expected);
                }
                errors += 1;
            }
        }

        if errors == 0 {
            println!("[OK]     All {} elements correct ✓", n);
            println!("[RESULT] h_c[0] = {} (expected {})", h_c[0], h_a[0] + h_b[0]);
            println!("[RESULT] h_c[{}] = {} (expected {})", n-1, h_c[(n-1) as usize], h_a[(n-1) as usize] + h_b[(n-1) as usize]);
        } else {
            println!("[FAIL]   {} errors", errors);
        }

        // Step 16: Cleanup
        println!("\n--- Phase 13: CLEANUP ---");
        cu_mem_free.unwrap()(d_a);
        cu_mem_free.unwrap()(d_b);
        cu_mem_free.unwrap()(d_c);
        cu_ctx_destroy.unwrap()(ctx);
        println!("[OK]     Resources freed");

        println!("\n========================================");
        println!("CUDead-BIB v3.0 — Test Complete ✓");
        println!("==========================================");
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
