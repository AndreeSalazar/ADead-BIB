// ADead-BIB Heredar - GPU Context Template
// Contexto GPU inicializado listo para usar
// Nivel militar: zero-copy, determinista, optimizado por arquitectura
//
// Autor: Eddi Andreé Salazar Matos

/// Contexto GPU completo
#[derive(Debug)]
pub struct GpuContext {
    /// GPU detectada
    pub gpu_info: GpuInfo,
    /// Memoria asignada
    pub memory: MemoryContext,
    /// Shaders cargados
    pub shaders: Vec<ShaderModule>,
    /// Pipelines creados
    pub pipelines: Vec<ComputePipeline>,
    /// Estado
    pub state: ContextState,
}

/// Información de GPU
#[derive(Debug, Clone)]
pub struct GpuInfo {
    pub name: String,
    pub vendor: GpuVendor,
    pub vram_mb: u32,
    pub compute_units: u32,
    pub max_workgroup_size: u32,
    pub tflops_fp32: f32,
    pub tflops_fp16: f32,
    pub supports_fp16: bool,
    pub supports_fp64: bool,
    pub supports_int8: bool,
}

/// Vendor de GPU
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuVendor {
    Nvidia,
    Amd,
    Intel,
    Unknown,
}

/// Contexto de memoria
#[derive(Debug, Default)]
pub struct MemoryContext {
    pub device_allocated: u64,
    pub host_allocated: u64,
    pub staging_allocated: u64,
    pub buffers: Vec<BufferHandle>,
}

/// Handle de buffer
#[derive(Debug, Clone)]
pub struct BufferHandle {
    pub id: u32,
    pub size: u64,
    pub device_local: bool,
}

/// Módulo de shader
#[derive(Debug)]
pub struct ShaderModule {
    pub id: u32,
    pub spirv: Vec<u8>,
    pub entry_point: String,
}

/// Pipeline de compute
#[derive(Debug)]
pub struct ComputePipeline {
    pub id: u32,
    pub shader_id: u32,
    pub workgroup_size: (u32, u32, u32),
}

/// Estado del contexto
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextState {
    Uninitialized,
    Ready,
    Recording,
    Executing,
    Error,
}

impl GpuContext {
    /// Crea contexto para RTX 3060
    pub fn rtx3060() -> Self {
        Self {
            gpu_info: GpuInfo {
                name: "NVIDIA GeForce RTX 3060".to_string(),
                vendor: GpuVendor::Nvidia,
                vram_mb: 12288,
                compute_units: 28,
                max_workgroup_size: 1024,
                tflops_fp32: 12.74,
                tflops_fp16: 25.48,
                supports_fp16: true,
                supports_fp64: true,
                supports_int8: true,
            },
            memory: MemoryContext::default(),
            shaders: Vec::new(),
            pipelines: Vec::new(),
            state: ContextState::Uninitialized,
        }
    }
    
    /// Crea contexto para RTX 4090
    pub fn rtx4090() -> Self {
        Self {
            gpu_info: GpuInfo {
                name: "NVIDIA GeForce RTX 4090".to_string(),
                vendor: GpuVendor::Nvidia,
                vram_mb: 24576,
                compute_units: 128,
                max_workgroup_size: 1024,
                tflops_fp32: 82.58,
                tflops_fp16: 165.16,
                supports_fp16: true,
                supports_fp64: true,
                supports_int8: true,
            },
            memory: MemoryContext::default(),
            shaders: Vec::new(),
            pipelines: Vec::new(),
            state: ContextState::Uninitialized,
        }
    }
    
    /// Inicializa el contexto
    pub fn init(&mut self) -> Result<(), &'static str> {
        if self.state != ContextState::Uninitialized {
            return Err("Context already initialized");
        }
        
        self.state = ContextState::Ready;
        Ok(())
    }
    
    /// Crea un buffer
    pub fn create_buffer(&mut self, size: u64, device_local: bool) -> u32 {
        let id = self.memory.buffers.len() as u32;
        
        self.memory.buffers.push(BufferHandle {
            id,
            size,
            device_local,
        });
        
        if device_local {
            self.memory.device_allocated += size;
        } else {
            self.memory.host_allocated += size;
        }
        
        id
    }
    
    /// Carga un shader SPIR-V
    pub fn load_shader(&mut self, spirv: Vec<u8>, entry_point: &str) -> u32 {
        let id = self.shaders.len() as u32;
        
        self.shaders.push(ShaderModule {
            id,
            spirv,
            entry_point: entry_point.to_string(),
        });
        
        id
    }
    
    /// Crea un pipeline de compute
    pub fn create_pipeline(&mut self, shader_id: u32, workgroup: (u32, u32, u32)) -> u32 {
        let id = self.pipelines.len() as u32;
        
        self.pipelines.push(ComputePipeline {
            id,
            shader_id,
            workgroup_size: workgroup,
        });
        
        id
    }
    
    /// Obtiene workgroup óptimo para esta GPU
    pub fn optimal_workgroup(&self) -> (u32, u32, u32) {
        match self.gpu_info.vendor {
            GpuVendor::Nvidia => (256, 1, 1),
            GpuVendor::Amd => (64, 1, 1),
            GpuVendor::Intel => (32, 1, 1),
            GpuVendor::Unknown => (128, 1, 1),
        }
    }
    
    /// Obtiene workgroup óptimo para MatMul
    pub fn optimal_matmul_workgroup(&self) -> (u32, u32, u32) {
        match self.gpu_info.vendor {
            GpuVendor::Nvidia => (16, 16, 1),
            GpuVendor::Amd => (16, 16, 1),
            _ => (8, 8, 1),
        }
    }
    
    /// Estima tiempo para MatMul
    pub fn estimate_matmul_ms(&self, m: u32, n: u32, k: u32) -> f64 {
        let flops = 2.0 * m as f64 * n as f64 * k as f64;
        let tflops = self.gpu_info.tflops_fp32 as f64 * 1e12;
        let efficiency = 0.5;
        (flops / (tflops * efficiency)) * 1000.0
    }
    
    /// Imprime info del contexto
    pub fn print_info(&self) {
        println!("╔══════════════════════════════════════════════════════════════╗");
        println!("║                    GPU CONTEXT                                ║");
        println!("╠══════════════════════════════════════════════════════════════╣");
        println!("║ GPU:        {:<48} ║", self.gpu_info.name);
        println!("║ VRAM:       {} MB                                          ║", self.gpu_info.vram_mb);
        println!("║ Compute:    {} SMs                                          ║", self.gpu_info.compute_units);
        println!("║ FP32:       {:.2} TFLOPS                                     ║", self.gpu_info.tflops_fp32);
        println!("║ FP16:       {:.2} TFLOPS                                     ║", self.gpu_info.tflops_fp16);
        println!("╠══════════════════════════════════════════════════════════════╣");
        println!("║ Device mem: {} MB                                          ║", self.memory.device_allocated / 1024 / 1024);
        println!("║ Host mem:   {} MB                                          ║", self.memory.host_allocated / 1024 / 1024);
        println!("║ Buffers:    {}                                              ║", self.memory.buffers.len());
        println!("║ Shaders:    {}                                              ║", self.shaders.len());
        println!("║ Pipelines:  {}                                              ║", self.pipelines.len());
        println!("║ State:      {:?}                                          ║", self.state);
        println!("╚══════════════════════════════════════════════════════════════╝");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rtx3060_context() {
        let mut ctx = GpuContext::rtx3060();
        ctx.init().unwrap();
        
        assert_eq!(ctx.gpu_info.vram_mb, 12288);
        assert_eq!(ctx.gpu_info.compute_units, 28);
        assert_eq!(ctx.state, ContextState::Ready);
    }
    
    #[test]
    fn test_buffer_creation() {
        let mut ctx = GpuContext::rtx3060();
        ctx.init().unwrap();
        
        let buf = ctx.create_buffer(1024 * 1024, true);
        assert_eq!(buf, 0);
        assert_eq!(ctx.memory.device_allocated, 1024 * 1024);
    }
}
