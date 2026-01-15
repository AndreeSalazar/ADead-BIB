// ============================================================================
// FastOS Software GPU Renderer
// ============================================================================
// Renderizador por software cuando no hay GPU hardware disponible
//
// Author: Eddi Andreé Salazar Matos 🇵🇪
// ============================================================================

#![allow(dead_code)]

use super::hal::*;

/// Buffer máximo de comandos
const MAX_COMMANDS: usize = 1024;
const MAX_BUFFERS: usize = 64;

/// GPU por software (global)
pub static mut SOFTWARE_GPU: SoftwareGpu = SoftwareGpu::new();

/// Estructura del renderizador por software
pub struct SoftwareGpu {
    info: GpuDeviceInfo,
    framebuffer: *mut u8,
    fb_size: usize,
    width: u32,
    height: u32,
    pitch: usize,
    bpp: usize,
    initialized: bool,
    next_buffer_id: u32,
    next_shader_id: u32,
    next_texture_id: u32,
    next_pipeline_id: u32,
}

impl SoftwareGpu {
    pub const fn new() -> Self {
        SoftwareGpu {
            info: GpuDeviceInfo {
                vendor: GpuVendor::Software,
                name: *b"FastOS Software Renderer                                        ",
                vram_mb: 0,
                max_texture_size: 4096,
                supports_compute: true,
                supports_raytracing: false,
            },
            framebuffer: core::ptr::null_mut(),
            fb_size: 0,
            width: 0,
            height: 0,
            pitch: 0,
            bpp: 4,
            initialized: false,
            next_buffer_id: 1,
            next_shader_id: 1,
            next_texture_id: 1,
            next_pipeline_id: 1,
        }
    }

    /// Configurar el framebuffer
    pub fn configure(&mut self, fb: *mut u8, width: u32, height: u32, pitch: usize, bpp: usize) {
        self.framebuffer = fb;
        self.width = width;
        self.height = height;
        self.pitch = pitch;
        self.bpp = bpp;
        self.fb_size = pitch * height as usize;
        self.initialized = true;
    }

    /// Dibujar un píxel
    #[inline]
    fn put_pixel(&mut self, x: u32, y: u32, color: u32) {
        if x >= self.width || y >= self.height || self.framebuffer.is_null() {
            return;
        }
        let offset = (y as usize * self.pitch) + (x as usize * self.bpp);
        if offset + 2 < self.fb_size {
            unsafe {
                let ptr = self.framebuffer.add(offset);
                *ptr = (color & 0xFF) as u8;
                *ptr.add(1) = ((color >> 8) & 0xFF) as u8;
                *ptr.add(2) = ((color >> 16) & 0xFF) as u8;
            }
        }
    }

    /// Dibujar rectángulo
    fn draw_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: u32) {
        for dy in 0..h {
            for dx in 0..w {
                self.put_pixel(x + dx, y + dy, color);
            }
        }
    }

    /// Limpiar pantalla
    fn clear(&mut self, color: u32) {
        self.draw_rect(0, 0, self.width, self.height, color);
    }

    /// Dibujar triángulo (rasterización básica)
    fn draw_triangle(&mut self, x0: u32, y0: u32, x1: u32, y1: u32, x2: u32, y2: u32, color: u32) {
        // Algoritmo de rasterización de triángulos por scanline
        let mut v0 = (x0 as i32, y0 as i32);
        let mut v1 = (x1 as i32, y1 as i32);
        let mut v2 = (x2 as i32, y2 as i32);

        // Ordenar vértices por Y
        if v0.1 > v1.1 { core::mem::swap(&mut v0, &mut v1); }
        if v0.1 > v2.1 { core::mem::swap(&mut v0, &mut v2); }
        if v1.1 > v2.1 { core::mem::swap(&mut v1, &mut v2); }

        let total_height = v2.1 - v0.1;
        if total_height == 0 { return; }

        for y in v0.1..=v2.1 {
            let second_half = y > v1.1 || v1.1 == v0.1;
            let segment_height = if second_half { v2.1 - v1.1 } else { v1.1 - v0.1 };
            if segment_height == 0 { continue; }

            let alpha = (y - v0.1) as f32 / total_height as f32;
            let beta = if second_half {
                (y - v1.1) as f32 / segment_height as f32
            } else {
                (y - v0.1) as f32 / segment_height as f32
            };

            let mut ax = v0.0 + ((v2.0 - v0.0) as f32 * alpha) as i32;
            let mut bx = if second_half {
                v1.0 + ((v2.0 - v1.0) as f32 * beta) as i32
            } else {
                v0.0 + ((v1.0 - v0.0) as f32 * beta) as i32
            };

            if ax > bx { core::mem::swap(&mut ax, &mut bx); }

            for x in ax..=bx {
                if x >= 0 && y >= 0 {
                    self.put_pixel(x as u32, y as u32, color);
                }
            }
        }
    }
}

impl GpuDevice for SoftwareGpu {
    fn init(&mut self) -> Result<(), GpuError> {
        if self.initialized {
            Ok(())
        } else {
            Err(GpuError::InitFailed)
        }
    }

    fn info(&self) -> &GpuDeviceInfo {
        &self.info
    }

    fn create_buffer(&mut self, _size: usize) -> Result<BufferId, GpuError> {
        let id = self.next_buffer_id;
        self.next_buffer_id += 1;
        Ok(BufferId(id))
    }

    fn destroy_buffer(&mut self, _id: BufferId) -> Result<(), GpuError> {
        Ok(())
    }

    fn write_buffer(&mut self, _id: BufferId, _offset: usize, _data: &[u8]) -> Result<(), GpuError> {
        Ok(())
    }

    fn read_buffer(&mut self, _id: BufferId, _offset: usize, _data: &mut [u8]) -> Result<(), GpuError> {
        Ok(())
    }

    fn create_shader(&mut self, _stage: ShaderStage, _code: &[u8]) -> Result<ShaderId, GpuError> {
        let id = self.next_shader_id;
        self.next_shader_id += 1;
        Ok(ShaderId(id))
    }

    fn destroy_shader(&mut self, _id: ShaderId) -> Result<(), GpuError> {
        Ok(())
    }

    fn create_texture(&mut self, _width: u32, _height: u32, _format: TextureFormat) -> Result<TextureId, GpuError> {
        let id = self.next_texture_id;
        self.next_texture_id += 1;
        Ok(TextureId(id))
    }

    fn destroy_texture(&mut self, _id: TextureId) -> Result<(), GpuError> {
        Ok(())
    }

    fn create_pipeline(&mut self, _vertex: ShaderId, _fragment: ShaderId) -> Result<PipelineId, GpuError> {
        let id = self.next_pipeline_id;
        self.next_pipeline_id += 1;
        Ok(PipelineId(id))
    }

    fn destroy_pipeline(&mut self, _id: PipelineId) -> Result<(), GpuError> {
        Ok(())
    }

    fn submit(&mut self, commands: &[GpuCommand]) -> Result<(), GpuError> {
        for cmd in commands {
            match *cmd {
                GpuCommand::Clear { color } => self.clear(color),
                GpuCommand::DrawRect { x, y, w, h, color } => self.draw_rect(x, y, w, h, color),
                GpuCommand::DrawTriangle { x0, y0, x1, y1, x2, y2, color } => {
                    self.draw_triangle(x0, y0, x1, y1, x2, y2, color);
                }
                GpuCommand::Present => { /* No-op for software */ }
                _ => { /* Otros comandos no soportados aún */ }
            }
        }
        Ok(())
    }

    fn present(&mut self) -> Result<(), GpuError> {
        Ok(())
    }

    fn framebuffer(&mut self) -> Option<&mut [u8]> {
        if self.framebuffer.is_null() || self.fb_size == 0 {
            None
        } else {
            Some(unsafe { core::slice::from_raw_parts_mut(self.framebuffer, self.fb_size) })
        }
    }

    fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
