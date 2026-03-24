// ============================================================
// CUDead-BIB v3.0 — RTX 3060 DEFINITIVE TEST
// ============================================================
// Solución DEFINITIVA para cargar kernels en RTX 3060
// Sin CUDA Toolkit — Solo nvcuda.dll
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
type CUjit_option = i32;

const CUDA_SUCCESS: CUresult = 0;

// JIT options for cuModuleLoadDataEx
const CU_JIT_ERROR_LOG_BUFFER: CUjit_option = 5;
const CU_JIT_ERROR_LOG_BUFFER_SIZE_BYTES: CUjit_option = 6;
const CU_JIT_INFO_LOG_BUFFER: CUjit_option = 3;
const CU_JIT_INFO_LOG_BUFFER_SIZE_BYTES: CUjit_option = 4;
const CU_JIT_TARGET: CUjit_option = 9;
const CU_JIT_TARGET_FROM_CUCONTEXT: i32 = 0;

fn main() {
    println!("========================================");
    println!("CUDead-BIB v3.0 — RTX 3060 DEFINITIVE");
    println!("========================================\n");

    #[cfg(windows)]
    {
        run_definitive_test();
    }

    #[cfg(not(windows))]
    {
        println!("[ERROR] Windows only");
    }
}

#[cfg(windows)]
fn run_definitive_test() {
    use std::os::windows::ffi::OsStrExt;

    // Load nvcuda.dll
    println!("--- Phase 00: LOAD DRIVER ---");
    let dll_name = "nvcuda.dll";
    let wide: Vec<u16> = std::ffi::OsStr::new(dll_name)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    
    let lib = unsafe { LoadLibraryW(wide.as_ptr()) };
    if lib.is_null() {
        println!("[ERROR] nvcuda.dll not found");
        return;
    }
    println!("[OK]     nvcuda.dll loaded");

    unsafe {
        // Get all function pointers
        let cu_init = get_proc::<unsafe extern "system" fn(u32) -> CUresult>(lib, "cuInit").unwrap();
        let cu_device_get_count = get_proc::<unsafe extern "system" fn(*mut i32) -> CUresult>(lib, "cuDeviceGetCount").unwrap();
        let cu_device_get = get_proc::<unsafe extern "system" fn(*mut CUdevice, i32) -> CUresult>(lib, "cuDeviceGet").unwrap();
        let cu_device_get_name = get_proc::<unsafe extern "system" fn(*mut u8, i32, CUdevice) -> CUresult>(lib, "cuDeviceGetName").unwrap();
        let cu_ctx_create = get_proc::<unsafe extern "system" fn(*mut CUcontext, u32, CUdevice) -> CUresult>(lib, "cuCtxCreate_v2").unwrap();
        let cu_ctx_destroy = get_proc::<unsafe extern "system" fn(CUcontext) -> CUresult>(lib, "cuCtxDestroy_v2").unwrap();
        let cu_ctx_sync = get_proc::<unsafe extern "system" fn() -> CUresult>(lib, "cuCtxSynchronize").unwrap();
        let cu_mem_alloc = get_proc::<unsafe extern "system" fn(*mut CUdeviceptr, usize) -> CUresult>(lib, "cuMemAlloc_v2").unwrap();
        let cu_mem_free = get_proc::<unsafe extern "system" fn(CUdeviceptr) -> CUresult>(lib, "cuMemFree_v2").unwrap();
        let cu_memcpy_htod = get_proc::<unsafe extern "system" fn(CUdeviceptr, *const c_void, usize) -> CUresult>(lib, "cuMemcpyHtoD_v2").unwrap();
        let cu_memcpy_dtoh = get_proc::<unsafe extern "system" fn(*mut c_void, CUdeviceptr, usize) -> CUresult>(lib, "cuMemcpyDtoH_v2").unwrap();
        
        // cuModuleLoadDataEx - CRITICAL: this gives us JIT error messages
        let cu_module_load_data_ex = get_proc::<unsafe extern "system" fn(
            *mut CUmodule, 
            *const c_void,
            u32,  // numOptions
            *mut CUjit_option,  // options
            *mut *mut c_void,   // optionValues
        ) -> CUresult>(lib, "cuModuleLoadDataEx").unwrap();
        
        let cu_module_get_func = get_proc::<unsafe extern "system" fn(*mut CUfunction, CUmodule, *const i8) -> CUresult>(lib, "cuModuleGetFunction").unwrap();
        let cu_launch_kernel = get_proc::<unsafe extern "system" fn(
            CUfunction, u32, u32, u32, u32, u32, u32, u32, CUstream, *mut *mut c_void, *mut *mut c_void
        ) -> CUresult>(lib, "cuLaunchKernel").unwrap();

        // Initialize CUDA
        println!("\n--- Phase 01: INIT ---");
        let result = cu_init(0);
        if result != CUDA_SUCCESS {
            println!("[ERROR] cuInit failed: {}", result);
            return;
        }
        println!("[OK]     CUDA initialized");

        // Get device
        let mut count = 0;
        cu_device_get_count(&mut count);
        if count == 0 {
            println!("[ERROR] No CUDA devices");
            return;
        }

        let mut device: CUdevice = 0;
        cu_device_get(&mut device, 0);

        let mut name = [0u8; 256];
        cu_device_get_name(name.as_mut_ptr(), 256, device);
        let name_str = std::ffi::CStr::from_ptr(name.as_ptr() as *const i8).to_string_lossy();
        println!("[GPU]    {}", name_str);

        // Create context
        println!("\n--- Phase 02: CONTEXT ---");
        let mut ctx: CUcontext = ptr::null_mut();
        let result = cu_ctx_create(&mut ctx, 0, device);
        if result != CUDA_SUCCESS {
            println!("[ERROR] cuCtxCreate failed: {}", result);
            return;
        }
        println!("[OK]     Context created");

        // ============================================================
        // THE CRITICAL PART: PTX that NVIDIA driver WILL accept
        // ============================================================
        // Key insights from reverse engineering:
        // 1. PTX MUST start with .version as FIRST non-comment line
        // 2. .target MUST match or be lower than device capability
        // 3. .address_size MUST be 64 for 64-bit pointers
        // 4. Entry point MUST use .visible .entry
        // 5. Parameters MUST be in .param space
        // 6. NO leading whitespace before directives
        // 7. Register declarations MUST come before use
        // ============================================================

        println!("\n--- Phase 03: LOAD PTX ---");
        
        // ============================================================
        // WORKING PTX FOR VECTORADD
        // ============================================================
        // Key discoveries:
        // 1. .target sm_52 works on sm_86 (forward compatible)
        // 2. No leading newline
        // 3. Simple function names (no C++ mangling)
        // 4. Must declare registers before use
        // ============================================================
        
        let ptx_vecadd = b"\
.version 6.5
.target sm_52
.address_size 64

.visible .entry vectorAdd(
    .param .u64 param_A,
    .param .u64 param_B,
    .param .u64 param_C,
    .param .u32 param_n
)
{
    .reg .pred %p<2>;
    .reg .f32 %f<4>;
    .reg .b32 %r<8>;
    .reg .b64 %rd<12>;

    ld.param.u64 %rd1, [param_A];
    ld.param.u64 %rd2, [param_B];
    ld.param.u64 %rd3, [param_C];
    ld.param.u32 %r1, [param_n];

    mov.u32 %r2, %ctaid.x;
    mov.u32 %r3, %ntid.x;
    mov.u32 %r4, %tid.x;
    mad.lo.s32 %r5, %r2, %r3, %r4;

    setp.ge.s32 %p1, %r5, %r1;
    @%p1 bra LBL_EXIT;

    cvt.s64.s32 %rd4, %r5;
    shl.b64 %rd5, %rd4, 2;

    add.s64 %rd6, %rd1, %rd5;
    add.s64 %rd7, %rd2, %rd5;
    add.s64 %rd8, %rd3, %rd5;

    ld.global.f32 %f1, [%rd6];
    ld.global.f32 %f2, [%rd7];
    add.f32 %f3, %f1, %f2;
    st.global.f32 [%rd8], %f3;

LBL_EXIT:
    ret;
}
\0";
        
        let ptx_minimal = ptx_vecadd;

        // Prepare JIT options to get error messages
        let mut error_log = vec![0u8; 4096];
        let mut info_log = vec![0u8; 4096];
        let mut error_log_size: usize = error_log.len();
        let mut info_log_size: usize = info_log.len();

        let mut options: [CUjit_option; 4] = [
            CU_JIT_ERROR_LOG_BUFFER,
            CU_JIT_ERROR_LOG_BUFFER_SIZE_BYTES,
            CU_JIT_INFO_LOG_BUFFER,
            CU_JIT_INFO_LOG_BUFFER_SIZE_BYTES,
        ];

        let mut option_values: [*mut c_void; 4] = [
            error_log.as_mut_ptr() as *mut c_void,
            &mut error_log_size as *mut usize as *mut c_void,
            info_log.as_mut_ptr() as *mut c_void,
            &mut info_log_size as *mut usize as *mut c_void,
        ];

        println!("[PTX]    Attempting minimal kernel...");
        println!("[PTX]    Size: {} bytes", ptx_minimal.len());
        
        let mut module: CUmodule = ptr::null_mut();
        let result = cu_module_load_data_ex(
            &mut module,
            ptx_minimal.as_ptr() as *const c_void,
            4,
            options.as_mut_ptr(),
            option_values.as_mut_ptr(),
        );

        if result != CUDA_SUCCESS {
            println!("[ERROR] cuModuleLoadDataEx failed: {}", result);
            
            // Print JIT error log
            let error_str = std::ffi::CStr::from_ptr(error_log.as_ptr() as *const i8);
            println!("[JIT ERROR LOG]:");
            println!("{}", error_str.to_string_lossy());
            
            let info_str = std::ffi::CStr::from_ptr(info_log.as_ptr() as *const i8);
            if !info_str.to_string_lossy().is_empty() {
                println!("[JIT INFO LOG]:");
                println!("{}", info_str.to_string_lossy());
            }

            // Try alternative PTX formats
            println!("\n[RETRY] Trying alternative PTX format...");
            
            // Alternative: Use C-style function name
            let ptx_alt = b"\
.version 6.5
.target sm_52
.address_size 64

.visible .entry kernel()
{
    ret;
}
\0";
            
            let result = cu_module_load_data_ex(
                &mut module,
                ptx_alt.as_ptr() as *const c_void,
                4,
                options.as_mut_ptr(),
                option_values.as_mut_ptr(),
            );

            if result != CUDA_SUCCESS {
                println!("[ERROR] Alternative also failed: {}", result);
                let error_str = std::ffi::CStr::from_ptr(error_log.as_ptr() as *const i8);
                println!("[JIT ERROR]: {}", error_str.to_string_lossy());

                // Try with explicit register allocation
                println!("\n[RETRY] Trying with explicit registers...");
                
                let ptx_regs = b"\
.version 6.5
.target sm_52
.address_size 64

.visible .entry kernel()
{
    .reg .b32 %r<1>;
    ret;
}
\0";
                
                let result = cu_module_load_data_ex(
                    &mut module,
                    ptx_regs.as_ptr() as *const c_void,
                    4,
                    options.as_mut_ptr(),
                    option_values.as_mut_ptr(),
                );

                if result != CUDA_SUCCESS {
                    println!("[ERROR] Registers version failed: {}", result);
                    let error_str = std::ffi::CStr::from_ptr(error_log.as_ptr() as *const i8);
                    println!("[JIT ERROR]: {}", error_str.to_string_lossy());
                    
                    // Last resort: try cuModuleLoadData without options
                    println!("\n[RETRY] Trying cuModuleLoadData (no options)...");
                    
                    let cu_module_load_data = get_proc::<unsafe extern "system" fn(
                        *mut CUmodule, 
                        *const c_void,
                    ) -> CUresult>(lib, "cuModuleLoadData").unwrap();

                    let result = cu_module_load_data(&mut module, ptx_regs.as_ptr() as *const c_void);
                    
                    if result != CUDA_SUCCESS {
                        println!("[ERROR] cuModuleLoadData failed: {}", result);
                        println!("\n[ANALYSIS] PTX JIT is rejecting all formats.");
                        println!("[ANALYSIS] Error 218 = CUDA_ERROR_INVALID_PTX");
                        println!("[ANALYSIS] This usually means:");
                        println!("  1. PTX syntax error");
                        println!("  2. Unsupported PTX version for this driver");
                        println!("  3. Missing required directives");
                        println!("\n[SOLUTION] Generating CUBIN directly...");
                        
                        // ============================================
                        // CUBIN GENERATION - THE NUCLEAR OPTION
                        // ============================================
                        generate_and_load_cubin(
                            lib,
                            ctx,
                            cu_mem_alloc,
                            cu_mem_free,
                            cu_memcpy_htod,
                            cu_memcpy_dtoh,
                            cu_ctx_sync,
                            cu_ctx_destroy,
                        );
                        return;
                    }
                }
            }
        }

        println!("[OK]     PTX module loaded!");
        
        // Get kernel function
        let kernel_name = CString::new("vectorAdd").unwrap();
        let mut kernel: CUfunction = ptr::null_mut();
        let result = cu_module_get_func(&mut kernel, module, kernel_name.as_ptr());
        if result != CUDA_SUCCESS {
            println!("[ERROR] cuModuleGetFunction failed: {}", result);
            cu_ctx_destroy(ctx);
            return;
        }
        println!("[OK]     Kernel 'vectorAdd' found");

        // ============================================================
        // FULL VECTORADD EXECUTION
        // ============================================================
        println!("\n--- Phase 04: ALLOCATE VRAM ---");
        
        let n: u32 = 1024 * 1024; // 1M elements
        let size = (n as usize) * std::mem::size_of::<f32>();
        println!("[VRAM]   N = {} elements", n);
        println!("[VRAM]   Size = {} MB per array", size / (1024 * 1024));

        let mut d_a: CUdeviceptr = 0;
        let mut d_b: CUdeviceptr = 0;
        let mut d_c: CUdeviceptr = 0;

        cu_mem_alloc(&mut d_a, size);
        cu_mem_alloc(&mut d_b, size);
        cu_mem_alloc(&mut d_c, size);
        println!("[VRAM]   d_A = 0x{:X}", d_a);
        println!("[VRAM]   d_B = 0x{:X}", d_b);
        println!("[VRAM]   d_C = 0x{:X}", d_c);

        // Initialize host data
        println!("\n--- Phase 05: INIT DATA ---");
        let h_a: Vec<f32> = (0..n).map(|i| i as f32).collect();
        let h_b: Vec<f32> = (0..n).map(|i| (i * 2) as f32).collect();
        let mut h_c: Vec<f32> = vec![0.0; n as usize];
        println!("[HOST]   h_A[0..3] = {:?}", &h_a[0..3]);
        println!("[HOST]   h_B[0..3] = {:?}", &h_b[0..3]);

        // Copy to device
        println!("\n--- Phase 06: H2D TRANSFER ---");
        let start = std::time::Instant::now();
        cu_memcpy_htod(d_a, h_a.as_ptr() as *const c_void, size);
        cu_memcpy_htod(d_b, h_b.as_ptr() as *const c_void, size);
        let h2d_time = start.elapsed();
        println!("[XFER]   {} MB copied to GPU", (size * 2) / (1024 * 1024));
        println!("[PERF]   H2D time: {:.2} ms", h2d_time.as_secs_f64() * 1000.0);

        // Launch kernel
        println!("\n--- Phase 07: LAUNCH KERNEL ---");
        let block_size = 256u32;
        let grid_size = (n + block_size - 1) / block_size;
        println!("[LAUNCH] Grid: {}", grid_size);
        println!("[LAUNCH] Block: {}", block_size);
        println!("[LAUNCH] Total threads: {}", grid_size * block_size);

        // Prepare kernel arguments
        // CRITICAL: cuLaunchKernel expects pointers to the actual values
        let mut args: [*mut c_void; 4] = [
            &d_a as *const _ as *mut c_void,
            &d_b as *const _ as *mut c_void,
            &d_c as *const _ as *mut c_void,
            &n as *const _ as *mut c_void,
        ];

        let start = std::time::Instant::now();
        let result = cu_launch_kernel(
            kernel,
            grid_size, 1, 1,  // grid
            block_size, 1, 1, // block
            0,                // shared mem
            ptr::null_mut(),  // stream
            args.as_mut_ptr(),
            ptr::null_mut(),  // extra
        );
        if result != CUDA_SUCCESS {
            println!("[ERROR] cuLaunchKernel failed: {}", result);
            cu_mem_free(d_a);
            cu_mem_free(d_b);
            cu_mem_free(d_c);
            cu_ctx_destroy(ctx);
            return;
        }
        println!("[OK]     Kernel launched");

        // Sync
        let result = cu_ctx_sync();
        let kernel_time = start.elapsed();
        if result != CUDA_SUCCESS {
            println!("[ERROR] cuCtxSynchronize failed: {}", result);
        } else {
            println!("[OK]     GPU synchronized");
            println!("[PERF]   Kernel time: {:.2} ms", kernel_time.as_secs_f64() * 1000.0);
        }

        // Copy back
        println!("\n--- Phase 08: D2H TRANSFER ---");
        let start = std::time::Instant::now();
        cu_memcpy_dtoh(h_c.as_mut_ptr() as *mut c_void, d_c, size);
        let d2h_time = start.elapsed();
        println!("[XFER]   {} MB copied from GPU", size / (1024 * 1024));
        println!("[PERF]   D2H time: {:.2} ms", d2h_time.as_secs_f64() * 1000.0);

        // Verify
        println!("\n--- Phase 09: VERIFY ---");
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

        // Cleanup
        println!("\n--- Phase 10: CLEANUP ---");
        cu_mem_free(d_a);
        cu_mem_free(d_b);
        cu_mem_free(d_c);
        cu_ctx_destroy(ctx);
        println!("[OK]     Resources freed");

        println!("\n========================================");
        println!("CUDead-BIB v3.0 — VECTORADD COMPLETE!");
        println!("========================================");
        println!("");
        println!("GPU:     NVIDIA GeForce RTX 3060");
        println!("Kernel:  vectorAdd (PTX JIT compiled)");
        println!("Data:    {} elements ({} MB)", n, (size * 3) / (1024 * 1024));
        println!("Result:  {} errors", errors);
        println!("");
        println!("NO CUDA TOOLKIT. NO NVCC. JUST DRIVER.");
        println!("========================================");
    }
}

#[cfg(windows)]
unsafe fn generate_and_load_cubin(
    lib: *mut c_void,
    ctx: CUcontext,
    cu_mem_alloc: unsafe extern "system" fn(*mut CUdeviceptr, usize) -> CUresult,
    cu_mem_free: unsafe extern "system" fn(CUdeviceptr) -> CUresult,
    cu_memcpy_htod: unsafe extern "system" fn(CUdeviceptr, *const c_void, usize) -> CUresult,
    cu_memcpy_dtoh: unsafe extern "system" fn(*mut c_void, CUdeviceptr, usize) -> CUresult,
    cu_ctx_sync: unsafe extern "system" fn() -> CUresult,
    cu_ctx_destroy: unsafe extern "system" fn(CUcontext) -> CUresult,
) {
    println!("\n========================================");
    println!("CUBIN GENERATION - Direct Machine Code");
    println!("========================================\n");

    // ============================================================
    // CUBIN FORMAT (ELF-based)
    // ============================================================
    // CUBIN is an ELF64 file with NVIDIA-specific sections:
    // - .nv.info: Kernel metadata
    // - .nv.info.<kernel>: Per-kernel info
    // - .nv.constant0.<kernel>: Constant memory
    // - .text.<kernel>: SASS machine code
    //
    // For sm_86 (Ampere), we need SASS instructions.
    // SASS is undocumented but can be reverse-engineered.
    // ============================================================

    println!("[CUBIN] sm_86 (Ampere) SASS format required");
    println!("[CUBIN] ELF64 + NVIDIA sections");
    println!("");
    println!("[INFO]  CUBIN generation requires SASS knowledge.");
    println!("[INFO]  SASS opcodes for Ampere are partially documented.");
    println!("[INFO]  Alternative: Use nvdisasm on existing .cubin files");
    println!("[INFO]  to reverse-engineer the instruction encoding.");
    println!("");
    
    // For now, demonstrate that memory operations work
    println!("[DEMO]  Demonstrating VRAM operations instead...\n");

    let n: u32 = 1024;
    let size = (n as usize) * std::mem::size_of::<f32>();

    let mut d_a: CUdeviceptr = 0;
    let mut d_b: CUdeviceptr = 0;

    cu_mem_alloc(&mut d_a, size);
    cu_mem_alloc(&mut d_b, size);
    println!("[VRAM]  Allocated {} bytes x 2", size);

    let h_a: Vec<f32> = (0..n).map(|i| i as f32).collect();
    let mut h_b: Vec<f32> = vec![0.0; n as usize];

    cu_memcpy_htod(d_a, h_a.as_ptr() as *const c_void, size);
    println!("[H2D]   Copied {} floats to GPU", n);

    cu_memcpy_dtoh(h_b.as_mut_ptr() as *mut c_void, d_a, size);
    println!("[D2H]   Copied {} floats from GPU", n);

    // Verify
    let mut ok = true;
    for i in 0..n as usize {
        if (h_b[i] - h_a[i]).abs() > 0.001 {
            ok = false;
            break;
        }
    }

    if ok {
        println!("[OK]    Data integrity verified ✓");
    } else {
        println!("[FAIL]  Data mismatch");
    }

    cu_mem_free(d_a);
    cu_mem_free(d_b);
    cu_ctx_destroy(ctx);

    println!("\n========================================");
    println!("VRAM Operations: WORKING");
    println!("Kernel Execution: Requires SASS/CUBIN");
    println!("========================================");
    println!("");
    println!("NEXT STEPS for full kernel execution:");
    println!("1. Obtain a working .cubin file (from nvcc -cubin)");
    println!("2. Reverse-engineer the SASS encoding");
    println!("3. Generate CUBIN directly in ADead-BIB");
    println!("");
    println!("Or: Use PTX with correct format (see JIT errors above)");
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

#[cfg(windows)]
extern "system" {
    fn LoadLibraryW(lpLibFileName: *const u16) -> *mut c_void;
    fn GetProcAddress(hModule: *mut c_void, lpProcName: *const i8) -> *mut c_void;
}
