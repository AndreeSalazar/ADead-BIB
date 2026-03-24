// ============================================================
// CUDead-BIB v3.0 — GPU Runtime
// ============================================================
// Runtime que ejecuta kernels en RTX 3060 usando driver API
// Sin nvcc — Sin cuda_runtime.h — Directo al driver
// ============================================================

use super::cuda_driver::{CudaDriverApi, CUcontext, CUdevice, CUdeviceptr, CUfunction, CUmodule, attrib};
use std::collections::HashMap;
use std::ffi::c_void;

/// CUDead-BIB Runtime v3.0
pub struct CudeadRuntime {
    driver: CudaDriverApi,
    device: CUdevice,
    context: CUcontext,
    modules: HashMap<String, CUmodule>,
    allocations: HashMap<u64, usize>,
    device_name: String,
    total_memory: usize,
    sm_count: i32,
    compute_major: i32,
    compute_minor: i32,
    step_mode: bool,
}

impl CudeadRuntime {
    /// Initialize runtime with step mode
    pub fn new(step_mode: bool) -> Result<Self, String> {
        if step_mode {
            println!("\n--- Phase 00: GPU DETECT ---");
        }

        // Load driver
        if step_mode {
            println!("[DRIVER] Loading nvcuda.dll...");
        }
        let driver = CudaDriverApi::load()?;

        // Initialize CUDA
        if step_mode {
            println!("[DRIVER] Initializing CUDA...");
        }
        driver.init()?;

        // Get device count
        let count = driver.device_count()?;
        if count == 0 {
            return Err("No CUDA devices found".to_string());
        }
        if step_mode {
            println!("[DRIVER] Found {} CUDA device(s)", count);
        }

        // Get first device
        let device = driver.get_device(0)?;
        let device_name = driver.device_name(device)?;
        let total_memory = driver.total_memory(device)?;

        // Get compute capability
        let compute_major = driver.get_attribute(device, attrib::CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR)?;
        let compute_minor = driver.get_attribute(device, attrib::CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MINOR)?;
        let sm_count = driver.get_attribute(device, attrib::CU_DEVICE_ATTRIBUTE_MULTIPROCESSOR_COUNT)?;

        if step_mode {
            println!("[GPU]    {}", device_name);
            println!("[GPU]    VRAM: {} GB", total_memory / (1024 * 1024 * 1024));
            println!("[GPU]    SMs: {} — sm_{}{}", sm_count, compute_major, compute_minor);
        }

        // Create context
        if step_mode {
            println!("[DRIVER] Creating CUDA context...");
        }
        let context = driver.create_context(device)?;

        Ok(Self {
            driver,
            device,
            context,
            modules: HashMap::new(),
            allocations: HashMap::new(),
            device_name,
            total_memory,
            sm_count,
            compute_major,
            compute_minor,
            step_mode,
        })
    }

    /// Print GPU info
    pub fn print_info(&self) {
        println!("\n=== CUDead-BIB v3.0 GPU Info ===");
        println!("Device:  {}", self.device_name);
        println!("VRAM:    {} GB", self.total_memory / (1024 * 1024 * 1024));
        println!("SMs:     {}", self.sm_count);
        println!("Compute: sm_{}{}", self.compute_major, self.compute_minor);
        println!("================================\n");
    }

    /// Load PTX module
    pub fn load_ptx(&mut self, name: &str, ptx: &str) -> Result<(), String> {
        if self.step_mode {
            println!("\n--- Phase 06: LOAD MODULE ---");
            println!("[MODULE] Loading PTX: {}", name);
            println!("[MODULE] PTX size: {} bytes", ptx.len());
        }

        let module = self.driver.load_module(ptx)?;
        self.modules.insert(name.to_string(), module);

        if self.step_mode {
            println!("[MODULE] Module loaded successfully ✓");
        }

        Ok(())
    }

    /// Get kernel function
    pub fn get_kernel(&self, module_name: &str, kernel_name: &str) -> Result<CUfunction, String> {
        let module = self.modules.get(module_name)
            .ok_or_else(|| format!("Module '{}' not found", module_name))?;
        
        if self.step_mode {
            println!("[KERNEL] Getting function: {}", kernel_name);
        }

        self.driver.get_function(*module, kernel_name)
    }

    /// Allocate device memory
    pub fn malloc(&mut self, size: usize) -> Result<CUdeviceptr, String> {
        if self.step_mode {
            println!("[VRAM]   cudead_malloc({} bytes)", size);
        }

        let ptr = self.driver.mem_alloc(size)?;
        self.allocations.insert(ptr, size);

        if self.step_mode {
            println!("[VRAM]   Allocated at 0x{:X}", ptr);
        }

        Ok(ptr)
    }

    /// Free device memory
    pub fn free(&mut self, ptr: CUdeviceptr) -> Result<(), String> {
        if self.step_mode {
            println!("[VRAM]   cudead_free(0x{:X})", ptr);
        }

        self.driver.mem_free(ptr)?;
        self.allocations.remove(&ptr);

        Ok(())
    }

    /// Copy host to device
    pub fn push<T>(&self, host: &[T], device: CUdeviceptr) -> Result<(), String> {
        let size = std::mem::size_of_val(host);
        
        if self.step_mode {
            println!("[XFER]   cudead_push({} bytes) H2D", size);
        }

        self.driver.memcpy_htod(device, host.as_ptr() as *const c_void, size)
    }

    /// Copy device to host
    pub fn pull<T>(&self, device: CUdeviceptr, host: &mut [T]) -> Result<(), String> {
        let size = std::mem::size_of_val(host);
        
        if self.step_mode {
            println!("[XFER]   cudead_pull({} bytes) D2H", size);
        }

        self.driver.memcpy_dtoh(host.as_mut_ptr() as *mut c_void, device, size)
    }

    /// Launch kernel
    pub fn launch(
        &self,
        kernel: CUfunction,
        grid: (u32, u32, u32),
        block: (u32, u32, u32),
        shared_mem: u32,
        args: &mut [*mut c_void],
    ) -> Result<(), String> {
        if self.step_mode {
            println!("\n--- Phase 09: EXECUTE ---");
            println!("[LAUNCH] Grid:  ({}, {}, {})", grid.0, grid.1, grid.2);
            println!("[LAUNCH] Block: ({}, {}, {})", block.0, block.1, block.2);
            println!("[LAUNCH] Shared: {} bytes", shared_mem);
            println!("[LAUNCH] Total threads: {}", 
                grid.0 * grid.1 * grid.2 * block.0 * block.1 * block.2);
        }

        self.driver.launch_kernel(kernel, grid, block, shared_mem, args)
    }

    /// Synchronize
    pub fn sync(&self) -> Result<(), String> {
        if self.step_mode {
            println!("[SYNC]   cudead_sync()...");
        }

        self.driver.synchronize()?;

        if self.step_mode {
            println!("[SYNC]   Done ✓");
        }

        Ok(())
    }

    /// Get device name
    pub fn device_name(&self) -> &str {
        &self.device_name
    }

    /// Get total memory
    pub fn total_memory(&self) -> usize {
        self.total_memory
    }

    /// Get SM count
    pub fn sm_count(&self) -> i32 {
        self.sm_count
    }

    /// Get compute capability
    pub fn compute_capability(&self) -> (i32, i32) {
        (self.compute_major, self.compute_minor)
    }
}

impl Drop for CudeadRuntime {
    fn drop(&mut self) {
        // Free all allocations
        for (ptr, _) in self.allocations.drain() {
            let _ = self.driver.mem_free(ptr);
        }

        // Destroy context
        let _ = self.driver.destroy_context(self.context);
    }
}

/// Vector addition example using runtime
pub fn example_vecadd(step_mode: bool) -> Result<(), String> {
    println!("\n========================================");
    println!("CUDead-BIB v3.0 — Vector Addition");
    println!("========================================\n");

    // Initialize runtime
    let mut runtime = CudeadRuntime::new(step_mode)?;
    runtime.print_info();

    // PTX for vectorAdd kernel
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

    // Load module
    if step_mode {
        println!("\n--- Phase 01: READ SOURCE ---");
        println!("[SRC]    vectorAdd kernel (PTX inline)");
    }

    runtime.load_ptx("vecadd", ptx)?;

    // Get kernel
    let kernel = runtime.get_kernel("vecadd", "vectorAdd")?;

    // Setup data
    let n: u32 = 1024;
    let size = (n as usize) * std::mem::size_of::<f32>();

    if step_mode {
        println!("\n--- Phase 07: VRAM LAYOUT ---");
        println!("[VRAM]   N = {} elements", n);
        println!("[VRAM]   Size = {} bytes per array", size);
    }

    // Host arrays
    let mut h_a: Vec<f32> = (0..n).map(|i| i as f32).collect();
    let mut h_b: Vec<f32> = (0..n).map(|i| (i * 2) as f32).collect();
    let mut h_c: Vec<f32> = vec![0.0; n as usize];

    // Allocate device memory
    if step_mode {
        println!("\n--- Phase 08: MEMORY ALLOC ---");
    }
    let d_a = runtime.malloc(size)?;
    let d_b = runtime.malloc(size)?;
    let d_c = runtime.malloc(size)?;

    // Copy to device
    runtime.push(&h_a, d_a)?;
    runtime.push(&h_b, d_b)?;

    // Launch config
    let block_size = 256u32;
    let grid_size = (n + block_size - 1) / block_size;

    // Prepare arguments
    let mut args: Vec<*mut c_void> = vec![
        &d_a as *const _ as *mut c_void,
        &d_b as *const _ as *mut c_void,
        &d_c as *const _ as *mut c_void,
        &n as *const _ as *mut c_void,
    ];

    // Launch kernel
    runtime.launch(
        kernel,
        (grid_size, 1, 1),
        (block_size, 1, 1),
        0,
        &mut args,
    )?;

    // Synchronize
    runtime.sync()?;

    // Copy back
    runtime.pull(d_c, &mut h_c)?;

    // Verify
    println!("\n--- VERIFICATION ---");
    let mut errors = 0;
    for i in 0..n as usize {
        let expected = h_a[i] + h_b[i];
        if (h_c[i] - expected).abs() > 0.001 {
            if errors < 5 {
                println!("ERROR: h_c[{}] = {}, expected {}", i, h_c[i], expected);
            }
            errors += 1;
        }
    }

    if errors == 0 {
        println!("OK: All {} elements correct ✓", n);
        println!("h_c[0] = {} (expected {})", h_c[0], h_a[0] + h_b[0]);
        println!("h_c[{}] = {} (expected {})", n-1, h_c[(n-1) as usize], h_a[(n-1) as usize] + h_b[(n-1) as usize]);
    } else {
        println!("FAIL: {} errors", errors);
    }

    // Free
    runtime.free(d_a)?;
    runtime.free(d_b)?;
    runtime.free(d_c)?;

    println!("\n========================================");
    println!("CUDead-BIB v3.0 — Complete ✓");
    println!("========================================\n");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_init() {
        match CudeadRuntime::new(true) {
            Ok(runtime) => {
                runtime.print_info();
                println!("Runtime initialized successfully");
            }
            Err(e) => {
                println!("Runtime not available: {}", e);
            }
        }
    }
}
