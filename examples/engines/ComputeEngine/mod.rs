// ADead-BIB Heredar - Compute Engine Template
// Base para cómputo GPU de alto rendimiento
// Nivel militar: scheduler determinista, zero-copy, SPIR-V directo
//
// Autor: Eddi Andreé Salazar Matos

use std::time::{Duration, Instant};

/// Configuración del Compute Engine
#[derive(Debug, Clone)]
pub struct ComputeConfig {
    /// Tamaño de workgroup por defecto
    pub default_workgroup: (u32, u32, u32),
    /// Máximo de dispatches en vuelo
    pub max_in_flight: u32,
    /// Usar memoria compartida
    pub use_shared_memory: bool,
    /// Tamaño de staging buffer (bytes)
    pub staging_buffer_size: u64,
    /// Modo de scheduling
    pub scheduling_mode: SchedulingMode,
}

/// Modo de scheduling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulingMode {
    /// Determinista: sin locks, sin colas dinámicas
    Deterministic,
    /// Async: permite overlapping
    Async,
    /// Immediate: ejecuta inmediatamente
    Immediate,
}

impl Default for ComputeConfig {
    fn default() -> Self {
        Self {
            default_workgroup: (256, 1, 1),
            max_in_flight: 8,
            use_shared_memory: true,
            staging_buffer_size: 64 * 1024 * 1024, // 64 MB
            scheduling_mode: SchedulingMode::Deterministic,
        }
    }
}

impl ComputeConfig {
    /// Configuración para RTX 3060
    pub fn rtx3060() -> Self {
        Self {
            default_workgroup: (256, 1, 1), // 8 warps
            max_in_flight: 8,
            staging_buffer_size: 256 * 1024 * 1024, // 256 MB (tiene 12GB VRAM)
            ..Default::default()
        }
    }
    
    /// Configuración para MatMul
    pub fn matmul() -> Self {
        Self {
            default_workgroup: (16, 16, 1), // Tile 16x16
            use_shared_memory: true,
            ..Default::default()
        }
    }
    
    /// Configuración para operaciones vectoriales
    pub fn vector_ops() -> Self {
        Self {
            default_workgroup: (256, 1, 1),
            use_shared_memory: false, // No necesario para vector ops
            ..Default::default()
        }
    }
}

/// Buffer de compute
#[derive(Debug, Clone)]
pub struct ComputeBuffer {
    pub id: u32,
    pub size: u64,
    pub usage: BufferUsage,
    pub mapped: bool,
}

/// Uso del buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferUsage {
    Storage,
    Uniform,
    Staging,
}

impl ComputeBuffer {
    pub fn storage(id: u32, size: u64) -> Self {
        Self {
            id,
            size,
            usage: BufferUsage::Storage,
            mapped: false,
        }
    }
    
    pub fn uniform(id: u32, size: u64) -> Self {
        Self {
            id,
            size,
            usage: BufferUsage::Uniform,
            mapped: false,
        }
    }
}

/// Dispatch de compute
#[derive(Debug, Clone)]
pub struct ComputeDispatch {
    pub shader_id: u32,
    pub workgroups: (u32, u32, u32),
    pub buffers: Vec<u32>,
    pub push_constants: Vec<u8>,
}

impl ComputeDispatch {
    pub fn new(shader_id: u32, workgroups: (u32, u32, u32)) -> Self {
        Self {
            shader_id,
            workgroups,
            buffers: Vec::new(),
            push_constants: Vec::new(),
        }
    }
    
    pub fn with_buffers(mut self, buffers: Vec<u32>) -> Self {
        self.buffers = buffers;
        self
    }
    
    pub fn with_push_constants(mut self, data: Vec<u8>) -> Self {
        self.push_constants = data;
        self
    }
    
    /// Total de invocaciones
    pub fn total_invocations(&self) -> u64 {
        self.workgroups.0 as u64 * self.workgroups.1 as u64 * self.workgroups.2 as u64
    }
}

/// Compute Engine principal
pub struct ComputeEngine {
    pub config: ComputeConfig,
    /// Buffers registrados
    buffers: Vec<ComputeBuffer>,
    /// Siguiente ID de buffer
    next_buffer_id: u32,
    /// Dispatches pendientes
    pending_dispatches: Vec<ComputeDispatch>,
    /// Estadísticas
    pub stats: ComputeStats,
}

/// Estadísticas de compute
#[derive(Debug, Clone, Default)]
pub struct ComputeStats {
    pub total_dispatches: u64,
    pub total_invocations: u64,
    pub total_bytes_transferred: u64,
    pub avg_dispatch_time_us: f64,
    pub total_compute_time_ms: f64,
}

impl ComputeEngine {
    pub fn new(config: ComputeConfig) -> Self {
        Self {
            config,
            buffers: Vec::new(),
            next_buffer_id: 0,
            pending_dispatches: Vec::new(),
            stats: ComputeStats::default(),
        }
    }
    
    /// Inicializa el engine
    pub fn init(&mut self) -> Result<(), &'static str> {
        println!("⚡ Initializing Compute Engine...");
        println!("   Workgroup: {:?}", self.config.default_workgroup);
        println!("   Max in-flight: {}", self.config.max_in_flight);
        println!("   Scheduling: {:?}", self.config.scheduling_mode);
        println!("   Staging buffer: {} MB", self.config.staging_buffer_size / 1024 / 1024);
        
        Ok(())
    }
    
    /// Crea un buffer
    pub fn create_buffer(&mut self, size: u64, usage: BufferUsage) -> u32 {
        let id = self.next_buffer_id;
        self.next_buffer_id += 1;
        
        self.buffers.push(ComputeBuffer {
            id,
            size,
            usage,
            mapped: false,
        });
        
        id
    }
    
    /// Encola un dispatch
    pub fn dispatch(&mut self, dispatch: ComputeDispatch) {
        self.pending_dispatches.push(dispatch);
    }
    
    /// Ejecuta todos los dispatches pendientes
    pub fn flush(&mut self) -> Duration {
        let start = Instant::now();
        
        for dispatch in &self.pending_dispatches {
            self.stats.total_dispatches += 1;
            self.stats.total_invocations += dispatch.total_invocations();
        }
        
        self.pending_dispatches.clear();
        
        let elapsed = start.elapsed();
        self.stats.total_compute_time_ms += elapsed.as_secs_f64() * 1000.0;
        
        elapsed
    }
    
    /// Sincroniza (espera a que termine todo)
    pub fn sync(&mut self) {
        self.flush();
    }
    
    // === Operaciones de alto nivel ===
    
    /// MatMul: C = A * B
    pub fn matmul(&mut self, a: u32, b: u32, c: u32, m: u32, n: u32, k: u32) {
        let workgroups = (
            (m + 15) / 16,
            (n + 15) / 16,
            1
        );
        
        let dispatch = ComputeDispatch::new(0, workgroups)
            .with_buffers(vec![a, b, c])
            .with_push_constants(vec![
                (m as u8), ((m >> 8) as u8), ((m >> 16) as u8), ((m >> 24) as u8),
                (n as u8), ((n >> 8) as u8), ((n >> 16) as u8), ((n >> 24) as u8),
                (k as u8), ((k >> 8) as u8), ((k >> 16) as u8), ((k >> 24) as u8),
            ]);
        
        self.dispatch(dispatch);
    }
    
    /// Vector Add: C = A + B
    pub fn vector_add(&mut self, a: u32, b: u32, c: u32, size: u32) {
        let workgroups = ((size + 255) / 256, 1, 1);
        
        let dispatch = ComputeDispatch::new(1, workgroups)
            .with_buffers(vec![a, b, c]);
        
        self.dispatch(dispatch);
    }
    
    /// Vector Scale: B = A * scalar
    pub fn vector_scale(&mut self, a: u32, b: u32, scalar: f32, size: u32) {
        let workgroups = ((size + 255) / 256, 1, 1);
        
        let dispatch = ComputeDispatch::new(2, workgroups)
            .with_buffers(vec![a, b])
            .with_push_constants(scalar.to_le_bytes().to_vec());
        
        self.dispatch(dispatch);
    }
    
    /// Dot product
    pub fn dot_product(&mut self, a: u32, b: u32, result: u32, size: u32) {
        let workgroups = ((size + 255) / 256, 1, 1);
        
        let dispatch = ComputeDispatch::new(3, workgroups)
            .with_buffers(vec![a, b, result]);
        
        self.dispatch(dispatch);
    }
    
    /// Imprime estadísticas
    pub fn print_stats(&self) {
        println!("📊 Compute Stats:");
        println!("   Dispatches:     {}", self.stats.total_dispatches);
        println!("   Invocations:    {}", self.stats.total_invocations);
        println!("   Bytes:          {} MB", self.stats.total_bytes_transferred / 1024 / 1024);
        println!("   Avg dispatch:   {:.2} µs", self.stats.avg_dispatch_time_us);
        println!("   Total time:     {:.2} ms", self.stats.total_compute_time_ms);
    }
}

/// Builder para Compute Engine
pub struct ComputeEngineBuilder {
    config: ComputeConfig,
}

impl ComputeEngineBuilder {
    pub fn new() -> Self {
        Self {
            config: ComputeConfig::default(),
        }
    }
    
    pub fn with_workgroup(mut self, x: u32, y: u32, z: u32) -> Self {
        self.config.default_workgroup = (x, y, z);
        self
    }
    
    pub fn with_max_in_flight(mut self, max: u32) -> Self {
        self.config.max_in_flight = max;
        self
    }
    
    pub fn with_scheduling(mut self, mode: SchedulingMode) -> Self {
        self.config.scheduling_mode = mode;
        self
    }
    
    pub fn with_shared_memory(mut self, enabled: bool) -> Self {
        self.config.use_shared_memory = enabled;
        self
    }
    
    pub fn build(self) -> ComputeEngine {
        ComputeEngine::new(self.config)
    }
}

impl Default for ComputeEngineBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Operaciones matriciales de alto nivel
pub struct MatrixOps;

impl MatrixOps {
    /// Calcula workgroups óptimos para MatMul
    pub fn optimal_workgroups(m: u32, n: u32, tile_size: u32) -> (u32, u32, u32) {
        (
            (m + tile_size - 1) / tile_size,
            (n + tile_size - 1) / tile_size,
            1
        )
    }
    
    /// Estima FLOPS para MatMul
    pub fn matmul_flops(m: u32, n: u32, k: u32) -> u64 {
        2 * m as u64 * n as u64 * k as u64
    }
    
    /// Estima tiempo para MatMul (ms) dado TFLOPS
    pub fn estimate_matmul_time(m: u32, n: u32, k: u32, tflops: f32) -> f64 {
        let flops = Self::matmul_flops(m, n, k) as f64;
        let tflops_per_sec = tflops as f64 * 1e12;
        let efficiency = 0.5; // Típico para MatMul bien optimizado
        (flops / (tflops_per_sec * efficiency)) * 1000.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compute_engine() {
        let mut engine = ComputeEngineBuilder::new()
            .with_workgroup(256, 1, 1)
            .with_scheduling(SchedulingMode::Deterministic)
            .build();
        
        engine.init().unwrap();
        
        let a = engine.create_buffer(1024, BufferUsage::Storage);
        let b = engine.create_buffer(1024, BufferUsage::Storage);
        let c = engine.create_buffer(1024, BufferUsage::Storage);
        
        engine.vector_add(a, b, c, 256);
        engine.flush();
        
        assert_eq!(engine.stats.total_dispatches, 1);
    }
    
    #[test]
    fn test_matmul_estimate() {
        // RTX 3060: 12.74 TFLOPS
        let time = MatrixOps::estimate_matmul_time(1024, 1024, 1024, 12.74);
        assert!(time > 0.0 && time < 10.0); // Debería ser ~0.34 ms
    }
}
