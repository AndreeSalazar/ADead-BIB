// ============================================================================
// GPU Renderer - ADead-BIB Engine
// ============================================================================
// Rendering directo a GPU usando wgpu (Vulkan/DX12/Metal)
// Sin softbuffer - TODO va directo al GPU
//
// Author: Eddi Andreé Salazar Matos 🇵🇪
// ============================================================================

use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::window::Window;

/// Vertex para renderizar rectángulos
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x2,  // position
        1 => Float32x4,  // color
    ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

/// Renderer GPU directo
pub struct GpuRenderer {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    vertices: Vec<Vertex>,
    pub width: u32,
    pub height: u32,
}

impl GpuRenderer {
    pub fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        
        // Crear instancia wgpu
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        
        // Crear surface
        let surface = instance.create_surface(window.clone()).unwrap();
        
        // Obtener adapter (GPU)
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })).expect("No se encontró GPU compatible");
        
        println!("🎮 GPU: {}", adapter.get_info().name);
        println!("🎮 Backend: {:?}", adapter.get_info().backend);
        
        // Crear device y queue
        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: Some("ADead-BIB GPU Device"),
            },
            None,
        )).expect("No se pudo crear device GPU");
        
        // Configurar surface
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);
        
        // Crear shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("ADead-BIB Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/rect.wgsl").into()),
        });
        
        // Crear pipeline
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
        
        // Crear vertex buffer inicial
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: 1024 * 1024,  // 1MB para vertices
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        Self {
            surface,
            device,
            queue,
            config,
            render_pipeline,
            vertex_buffer,
            vertices: Vec::with_capacity(10000),
            width: size.width,
            height: size.height,
        }
    }
    
    /// Redimensionar
    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        if new_width > 0 && new_height > 0 {
            self.width = new_width;
            self.height = new_height;
            self.config.width = new_width;
            self.config.height = new_height;
            self.surface.configure(&self.device, &self.config);
        }
    }
    
    /// Limpiar pantalla (preparar frame)
    pub fn clear(&mut self) {
        self.vertices.clear();
    }
    
    /// Dibujar rectángulo (coordenadas de pantalla)
    pub fn draw_rect(&mut self, x: i32, y: i32, w: u32, h: u32, color: [f32; 4]) {
        // Convertir coordenadas de pantalla a NDC (-1 a 1)
        let x1 = (x as f32 / self.width as f32) * 2.0 - 1.0;
        let y1 = 1.0 - (y as f32 / self.height as f32) * 2.0;
        let x2 = ((x as f32 + w as f32) / self.width as f32) * 2.0 - 1.0;
        let y2 = 1.0 - ((y as f32 + h as f32) / self.height as f32) * 2.0;
        
        // Dos triángulos para el rectángulo
        self.vertices.extend_from_slice(&[
            Vertex { position: [x1, y1], color },
            Vertex { position: [x2, y1], color },
            Vertex { position: [x1, y2], color },
            Vertex { position: [x2, y1], color },
            Vertex { position: [x2, y2], color },
            Vertex { position: [x1, y2], color },
        ]);
    }
    
    /// Presentar frame
    pub fn present(&mut self, clear_color: [f32; 4]) {
        // Obtener frame
        let output = match self.surface.get_current_texture() {
            Ok(output) => output,
            Err(_) => {
                self.surface.configure(&self.device, &self.config);
                return;
            }
        };
        
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        // Actualizar vertex buffer
        if !self.vertices.is_empty() {
            self.queue.write_buffer(
                &self.vertex_buffer,
                0,
                bytemuck::cast_slice(&self.vertices),
            );
        }
        
        // Crear command encoder
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: clear_color[0] as f64,
                            g: clear_color[1] as f64,
                            b: clear_color[2] as f64,
                            a: clear_color[3] as f64,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..self.vertices.len() as u32, 0..1);
        }
        
        // Enviar comandos a GPU
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}

/// Color helper
#[derive(Clone, Copy)]
pub struct GpuColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl GpuColor {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: 1.0,
        }
    }
    
    pub fn to_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
    
    pub const SKY_BLUE: GpuColor = GpuColor::rgb(135, 206, 235);
    pub const GREEN: GpuColor = GpuColor::rgb(0, 128, 0);
    pub const DARK_GREEN: GpuColor = GpuColor::rgb(0, 85, 0);
    pub const BROWN: GpuColor = GpuColor::rgb(139, 69, 19);
    pub const GRASS: GpuColor = GpuColor::rgb(34, 139, 34);
    pub const YELLOW: GpuColor = GpuColor::rgb(255, 215, 0);
    pub const ORANGE: GpuColor = GpuColor::rgb(255, 136, 0);
    pub const WHITE: GpuColor = GpuColor::rgb(255, 255, 255);
    pub const BLACK: GpuColor = GpuColor::rgb(0, 0, 0);
    pub const RED: GpuColor = GpuColor::rgb(204, 0, 0);
}
