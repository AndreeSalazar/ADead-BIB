// ============================================================
// CUDead-BIB — 8 Core Primitives
// ============================================================
// Las ÚNICAS 8 abstracciones necesarias para GPU
// Todo lo demás es bloatware
//
// KERNEL DEFINITION:
// 1. __cudead_kernel__  → define función GPU
// 2. __cudead_device__  → función auxiliar GPU
//
// LAUNCH:
// 3. cudead_launch()    → lanza grid de hilos
// 4. cudead_sync()      → sincroniza CPU↔GPU
//
// MEMORY:
// 5. cudead_malloc()    → alloca VRAM
// 6. cudead_free()      → libera VRAM
// 7. cudead_push()      → CPU RAM → GPU VRAM
// 8. cudead_pull()      → GPU VRAM → CPU RAM
// ============================================================

use std::collections::HashMap;

/// Dimensiones de grid/block (x, y, z)
#[derive(Debug, Clone, Copy, Default)]
pub struct Dim3 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl Dim3 {
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }

    pub fn linear(n: u32) -> Self {
        Self { x: n, y: 1, z: 1 }
    }

    pub fn total(&self) -> u64 {
        self.x as u64 * self.y as u64 * self.z as u64
    }
}

impl From<u32> for Dim3 {
    fn from(n: u32) -> Self {
        Self::linear(n)
    }
}

impl From<(u32, u32)> for Dim3 {
    fn from((x, y): (u32, u32)) -> Self {
        Self { x, y, z: 1 }
    }
}

impl From<(u32, u32, u32)> for Dim3 {
    fn from((x, y, z): (u32, u32, u32)) -> Self {
        Self { x, y, z }
    }
}

/// Thread index within a block
#[derive(Debug, Clone, Copy, Default)]
pub struct ThreadIdx {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

/// Block index within a grid
#[derive(Debug, Clone, Copy, Default)]
pub struct BlockIdx {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

/// GPU memory pointer (VRAM address)
#[derive(Debug, Clone, Copy)]
pub struct GpuPtr {
    /// VRAM address
    pub addr: u64,
    /// Size in bytes
    pub size: usize,
    /// Element type size
    pub elem_size: usize,
}

impl GpuPtr {
    pub fn null() -> Self {
        Self {
            addr: 0,
            size: 0,
            elem_size: 0,
        }
    }

    pub fn is_null(&self) -> bool {
        self.addr == 0
    }

    pub fn element_count(&self) -> usize {
        if self.elem_size > 0 {
            self.size / self.elem_size
        } else {
            0
        }
    }
}

/// Kernel function type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KernelType {
    /// __cudead_kernel__ — entry point, can be launched from CPU
    Kernel,
    /// __cudead_device__ — helper function, only callable from GPU
    Device,
}

/// Kernel parameter
#[derive(Debug, Clone)]
pub struct KernelParam {
    pub name: String,
    pub param_type: ParamType,
    pub is_pointer: bool,
}

/// Parameter types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamType {
    Float,
    Double,
    Int,
    Long,
    Uint,
    Ulong,
    Void,
}

impl ParamType {
    pub fn size(&self) -> usize {
        match self {
            ParamType::Float => 4,
            ParamType::Double => 8,
            ParamType::Int => 4,
            ParamType::Long => 8,
            ParamType::Uint => 4,
            ParamType::Ulong => 8,
            ParamType::Void => 0,
        }
    }

    pub fn to_ptx(&self) -> &'static str {
        match self {
            ParamType::Float => ".f32",
            ParamType::Double => ".f64",
            ParamType::Int => ".s32",
            ParamType::Long => ".s64",
            ParamType::Uint => ".u32",
            ParamType::Ulong => ".u64",
            ParamType::Void => "",
        }
    }
}

/// Kernel definition
#[derive(Debug, Clone)]
pub struct KernelDef {
    pub name: String,
    pub kernel_type: KernelType,
    pub params: Vec<KernelParam>,
    pub body: String,
    pub shared_memory: usize,
    pub registers_used: u32,
}

impl KernelDef {
    pub fn new(name: &str, kernel_type: KernelType) -> Self {
        Self {
            name: name.to_string(),
            kernel_type,
            params: Vec::new(),
            body: String::new(),
            shared_memory: 0,
            registers_used: 0,
        }
    }

    pub fn add_param(&mut self, name: &str, param_type: ParamType, is_pointer: bool) {
        self.params.push(KernelParam {
            name: name.to_string(),
            param_type,
            is_pointer,
        });
    }
}

// ============================================================
// PRIMITIVE 1 & 2: __cudead_kernel__ and __cudead_device__
// ============================================================

/// Kernel registry — stores all defined kernels
#[derive(Debug, Default)]
pub struct KernelRegistry {
    kernels: HashMap<String, KernelDef>,
}

impl KernelRegistry {
    pub fn new() -> Self {
        Self {
            kernels: HashMap::new(),
        }
    }

    /// Register a new kernel (__cudead_kernel__)
    pub fn register_kernel(&mut self, def: KernelDef) {
        self.kernels.insert(def.name.clone(), def);
    }

    /// Get a kernel by name
    pub fn get(&self, name: &str) -> Option<&KernelDef> {
        self.kernels.get(name)
    }

    /// List all kernels
    pub fn list(&self) -> Vec<&str> {
        self.kernels.keys().map(|s| s.as_str()).collect()
    }

    /// Count kernels
    pub fn count(&self) -> usize {
        self.kernels.len()
    }
}

// ============================================================
// PRIMITIVE 3: cudead_launch()
// ============================================================

/// Launch configuration
#[derive(Debug, Clone)]
pub struct LaunchConfig {
    pub grid: Dim3,
    pub block: Dim3,
    pub shared_memory: usize,
    pub stream: u32,
}

impl LaunchConfig {
    pub fn new(grid: impl Into<Dim3>, block: impl Into<Dim3>) -> Self {
        Self {
            grid: grid.into(),
            block: block.into(),
            shared_memory: 0,
            stream: 0,
        }
    }

    pub fn with_shared_memory(mut self, bytes: usize) -> Self {
        self.shared_memory = bytes;
        self
    }

    pub fn total_threads(&self) -> u64 {
        self.grid.total() * self.block.total()
    }

    pub fn threads_per_block(&self) -> u64 {
        self.block.total()
    }

    /// Validate launch configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.block.total() > 1024 {
            return Err(format!(
                "Block size {} exceeds max 1024 threads",
                self.block.total()
            ));
        }
        if self.grid.x == 0 || self.grid.y == 0 || self.grid.z == 0 {
            return Err("Grid dimensions cannot be zero".to_string());
        }
        if self.block.x == 0 || self.block.y == 0 || self.block.z == 0 {
            return Err("Block dimensions cannot be zero".to_string());
        }
        Ok(())
    }
}

/// Kernel launch request
#[derive(Debug)]
pub struct LaunchRequest {
    pub kernel_name: String,
    pub config: LaunchConfig,
    pub args: Vec<GpuPtr>,
}

// ============================================================
// PRIMITIVE 4: cudead_sync()
// ============================================================

/// Synchronization type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncType {
    /// Synchronize all GPU operations
    DeviceSync,
    /// Synchronize a specific stream
    StreamSync(u32),
    /// Memory fence
    MemoryFence,
}

// ============================================================
// PRIMITIVE 5 & 6: cudead_malloc() and cudead_free()
// ============================================================

/// VRAM allocation request
#[derive(Debug)]
pub struct VramAlloc {
    pub size: usize,
    pub alignment: usize,
    pub zero_init: bool,
}

impl VramAlloc {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            alignment: 256, // Default 256-byte alignment for coalescing
            zero_init: false,
        }
    }

    pub fn with_alignment(mut self, alignment: usize) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn zeroed(mut self) -> Self {
        self.zero_init = true;
        self
    }
}

// ============================================================
// PRIMITIVE 7 & 8: cudead_push() and cudead_pull()
// ============================================================

/// Memory transfer direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferDirection {
    /// CPU RAM → GPU VRAM (cudead_push)
    HostToDevice,
    /// GPU VRAM → CPU RAM (cudead_pull)
    DeviceToHost,
    /// GPU VRAM → GPU VRAM
    DeviceToDevice,
}

/// Memory transfer request
#[derive(Debug)]
pub struct MemTransfer {
    pub direction: TransferDirection,
    pub src: u64,
    pub dst: u64,
    pub size: usize,
    pub async_transfer: bool,
    pub stream: u32,
}

impl MemTransfer {
    /// cudead_push: CPU → GPU
    pub fn push(host_ptr: u64, device_ptr: u64, size: usize) -> Self {
        Self {
            direction: TransferDirection::HostToDevice,
            src: host_ptr,
            dst: device_ptr,
            size,
            async_transfer: false,
            stream: 0,
        }
    }

    /// cudead_pull: GPU → CPU
    pub fn pull(device_ptr: u64, host_ptr: u64, size: usize) -> Self {
        Self {
            direction: TransferDirection::DeviceToHost,
            src: device_ptr,
            dst: host_ptr,
            size,
            async_transfer: false,
            stream: 0,
        }
    }
}

// ============================================================
// Runtime API (combines all 8 primitives)
// ============================================================

/// CUDead-BIB Runtime — manages GPU execution
pub struct CudeadRuntime {
    /// Registered kernels
    pub kernels: KernelRegistry,
    /// VRAM allocations
    allocations: HashMap<u64, GpuPtr>,
    /// Next allocation address (simulated)
    next_addr: u64,
    /// Total VRAM used
    vram_used: usize,
    /// Total VRAM available
    vram_total: usize,
}

impl CudeadRuntime {
    pub fn new(vram_mb: usize) -> Self {
        Self {
            kernels: KernelRegistry::new(),
            allocations: HashMap::new(),
            next_addr: 0x1000_0000, // Start at 256MB
            vram_used: 0,
            vram_total: vram_mb * 1024 * 1024,
        }
    }

    /// PRIMITIVE 5: cudead_malloc
    pub fn malloc(&mut self, alloc: VramAlloc) -> Result<GpuPtr, String> {
        if self.vram_used + alloc.size > self.vram_total {
            return Err(format!(
                "Out of VRAM: need {} bytes, have {} free",
                alloc.size,
                self.vram_total - self.vram_used
            ));
        }

        // Align address
        let aligned_addr = (self.next_addr + alloc.alignment as u64 - 1)
            & !(alloc.alignment as u64 - 1);

        let ptr = GpuPtr {
            addr: aligned_addr,
            size: alloc.size,
            elem_size: 4, // Default to f32
        };

        self.allocations.insert(aligned_addr, ptr);
        self.next_addr = aligned_addr + alloc.size as u64;
        self.vram_used += alloc.size;

        Ok(ptr)
    }

    /// PRIMITIVE 6: cudead_free
    pub fn free(&mut self, ptr: GpuPtr) -> Result<(), String> {
        if let Some(alloc) = self.allocations.remove(&ptr.addr) {
            self.vram_used -= alloc.size;
            Ok(())
        } else {
            Err(format!("Invalid GPU pointer: 0x{:016X}", ptr.addr))
        }
    }

    /// PRIMITIVE 7: cudead_push (CPU → GPU)
    pub fn push(&self, transfer: MemTransfer) -> Result<(), String> {
        if transfer.direction != TransferDirection::HostToDevice {
            return Err("push() requires HostToDevice direction".to_string());
        }
        // In real implementation, this would DMA transfer
        Ok(())
    }

    /// PRIMITIVE 8: cudead_pull (GPU → CPU)
    pub fn pull(&self, transfer: MemTransfer) -> Result<(), String> {
        if transfer.direction != TransferDirection::DeviceToHost {
            return Err("pull() requires DeviceToHost direction".to_string());
        }
        // In real implementation, this would DMA transfer
        Ok(())
    }

    /// PRIMITIVE 3: cudead_launch
    pub fn launch(&self, request: LaunchRequest) -> Result<(), String> {
        request.config.validate()?;

        if self.kernels.get(&request.kernel_name).is_none() {
            return Err(format!("Kernel '{}' not found", request.kernel_name));
        }

        // In real implementation, this would dispatch to GPU
        Ok(())
    }

    /// PRIMITIVE 4: cudead_sync
    pub fn sync(&self, sync_type: SyncType) -> Result<(), String> {
        match sync_type {
            SyncType::DeviceSync => {
                // Wait for all GPU operations
            }
            SyncType::StreamSync(stream) => {
                // Wait for specific stream
                let _ = stream;
            }
            SyncType::MemoryFence => {
                // Memory barrier
            }
        }
        Ok(())
    }

    /// Get VRAM usage stats
    pub fn vram_stats(&self) -> (usize, usize) {
        (self.vram_used, self.vram_total)
    }
}

impl Default for CudeadRuntime {
    fn default() -> Self {
        Self::new(12 * 1024) // 12GB default (RTX 3060)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dim3() {
        let d = Dim3::new(16, 16, 1);
        assert_eq!(d.total(), 256);
    }

    #[test]
    fn test_launch_config() {
        let config = LaunchConfig::new((4, 4), (256, 1, 1));
        assert_eq!(config.total_threads(), 16 * 256);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_launch_config_invalid() {
        let config = LaunchConfig::new(1, (2048, 1, 1));
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_runtime_malloc() {
        let mut rt = CudeadRuntime::new(1024); // 1GB
        let ptr = rt.malloc(VramAlloc::new(1024 * 1024)).unwrap();
        assert!(!ptr.is_null());
        assert_eq!(ptr.size, 1024 * 1024);
    }

    #[test]
    fn test_runtime_free() {
        let mut rt = CudeadRuntime::new(1024);
        let ptr = rt.malloc(VramAlloc::new(1024)).unwrap();
        assert!(rt.free(ptr).is_ok());
    }

    #[test]
    fn test_kernel_registry() {
        let mut registry = KernelRegistry::new();
        let mut kernel = KernelDef::new("vectorAdd", KernelType::Kernel);
        kernel.add_param("A", ParamType::Float, true);
        kernel.add_param("B", ParamType::Float, true);
        kernel.add_param("C", ParamType::Float, true);
        kernel.add_param("n", ParamType::Int, false);
        registry.register_kernel(kernel);

        assert_eq!(registry.count(), 1);
        assert!(registry.get("vectorAdd").is_some());
    }
}
