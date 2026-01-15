// ============================================================================
// FastOS GPU Hardware Abstraction Layer (HAL)
// ============================================================================
// Interfaz común para todos los drivers GPU
//
// Author: Eddi Andreé Salazar Matos 🇵🇪
// ============================================================================

#![allow(dead_code)]

use core::result::Result;

/// Errores de GPU
#[derive(Debug, Clone, Copy)]
pub enum GpuError {
    NotFound,
    InitFailed,
    OutOfMemory,
    InvalidBuffer,
    InvalidShader,
    InvalidCommand,
    Unsupported,
}

/// ID de buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferId(pub u32);

/// ID de shader
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShaderId(pub u32);

/// ID de textura
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextureId(pub u32);

/// ID de pipeline
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PipelineId(pub u32);

/// Formato de textura
#[derive(Debug, Clone, Copy)]
pub enum TextureFormat {
    Rgba8,
    Bgra8,
    Rgb8,
    R8,
    Depth24,
    Depth32,
}

/// Tipo de shader
#[derive(Debug, Clone, Copy)]
pub enum ShaderStage {
    Vertex,
    Fragment,
    Compute,
}

/// Comando GPU
#[derive(Debug, Clone, Copy)]
pub enum GpuCommand {
    Clear { color: u32 },
    DrawRect { x: u32, y: u32, w: u32, h: u32, color: u32 },
    DrawTriangle { x0: u32, y0: u32, x1: u32, y1: u32, x2: u32, y2: u32, color: u32 },
    CopyBuffer { src: BufferId, dst: BufferId, size: u32 },
    BindPipeline { pipeline: PipelineId },
    BindBuffer { slot: u32, buffer: BufferId },
    Dispatch { x: u32, y: u32, z: u32 },
    Present,
}

/// Información del dispositivo GPU
#[derive(Debug, Clone)]
pub struct GpuDeviceInfo {
    pub vendor: GpuVendor,
    pub name: [u8; 64],
    pub vram_mb: u32,
    pub max_texture_size: u32,
    pub supports_compute: bool,
    pub supports_raytracing: bool,
}

/// Vendedor de GPU
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuVendor {
    Nvidia,
    Amd,
    Intel,
    Software,
    Unknown,
}

/// Trait principal para dispositivos GPU
pub trait GpuDevice {
    /// Inicializar el dispositivo
    fn init(&mut self) -> Result<(), GpuError>;
    
    /// Obtener información del dispositivo
    fn info(&self) -> &GpuDeviceInfo;
    
    /// Crear un buffer en VRAM
    fn create_buffer(&mut self, size: usize) -> Result<BufferId, GpuError>;
    
    /// Destruir un buffer
    fn destroy_buffer(&mut self, id: BufferId) -> Result<(), GpuError>;
    
    /// Escribir datos a un buffer
    fn write_buffer(&mut self, id: BufferId, offset: usize, data: &[u8]) -> Result<(), GpuError>;
    
    /// Leer datos de un buffer
    fn read_buffer(&mut self, id: BufferId, offset: usize, data: &mut [u8]) -> Result<(), GpuError>;
    
    /// Crear un shader
    fn create_shader(&mut self, stage: ShaderStage, code: &[u8]) -> Result<ShaderId, GpuError>;
    
    /// Destruir un shader
    fn destroy_shader(&mut self, id: ShaderId) -> Result<(), GpuError>;
    
    /// Crear una textura
    fn create_texture(&mut self, width: u32, height: u32, format: TextureFormat) -> Result<TextureId, GpuError>;
    
    /// Destruir una textura
    fn destroy_texture(&mut self, id: TextureId) -> Result<(), GpuError>;
    
    /// Crear un pipeline de renderizado
    fn create_pipeline(&mut self, vertex: ShaderId, fragment: ShaderId) -> Result<PipelineId, GpuError>;
    
    /// Destruir un pipeline
    fn destroy_pipeline(&mut self, id: PipelineId) -> Result<(), GpuError>;
    
    /// Enviar comandos a la GPU
    fn submit(&mut self, commands: &[GpuCommand]) -> Result<(), GpuError>;
    
    /// Presentar el framebuffer
    fn present(&mut self) -> Result<(), GpuError>;
    
    /// Obtener el framebuffer para escritura directa
    fn framebuffer(&mut self) -> Option<&mut [u8]>;
    
    /// Dimensiones del framebuffer
    fn dimensions(&self) -> (u32, u32);
}

/// Detectar GPU disponible
pub fn detect_gpu() -> GpuVendor {
    // En bare metal, detectamos via PCI
    // Por ahora, usamos software rendering
    GpuVendor::Software
}

/// Crear dispositivo GPU según el vendor
pub fn create_device(vendor: GpuVendor) -> Option<&'static mut dyn GpuDevice> {
    match vendor {
        GpuVendor::Software => {
            // Usar software renderer por defecto
            Some(unsafe { &mut super::software::SOFTWARE_GPU })
        }
        _ => None, // Otros vendors requieren drivers específicos
    }
}
