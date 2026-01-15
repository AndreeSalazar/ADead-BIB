// ============================================================================
// FastOS AMD GPU Driver
// ============================================================================
// Driver para GPUs AMD (Radeon RX)
//
// Author: Eddi Andreé Salazar Matos 🇵🇪
// ============================================================================

#![allow(dead_code)]

use super::hal::*;

/// ID de vendor AMD
pub const AMD_VENDOR_ID: u16 = 0x1002;

/// Dispositivos RX 6000
pub const RX_6600: u16 = 0x73FF;
pub const RX_6700_XT: u16 = 0x73DF;
pub const RX_6800: u16 = 0x73BF;
pub const RX_6900_XT: u16 = 0x73AF;

/// Dispositivos RX 7000
pub const RX_7600: u16 = 0x7480;
pub const RX_7700_XT: u16 = 0x7470;
pub const RX_7800_XT: u16 = 0x7460;
pub const RX_7900_XTX: u16 = 0x744C;

/// Driver AMD
pub struct AmdGpu {
    info: GpuDeviceInfo,
    bar0: u64,
    vram_size: u64,
    initialized: bool,
}

impl AmdGpu {
    pub const fn new() -> Self {
        AmdGpu {
            info: GpuDeviceInfo {
                vendor: GpuVendor::Amd,
                name: *b"AMD GPU (Not Initialized)                                       ",
                vram_mb: 0,
                max_texture_size: 16384,
                supports_compute: true,
                supports_raytracing: true,
            },
            bar0: 0,
            vram_size: 0,
            initialized: false,
        }
    }

    /// Detectar GPU AMD via PCI
    pub fn detect() -> Option<(u16, u64)> {
        for bus in 0..256u16 {
            for device in 0..32u8 {
                for function in 0..8u8 {
                    if let Some((vendor, dev_id, bar0)) = pci_read_device(bus as u8, device, function) {
                        if vendor == AMD_VENDOR_ID {
                            return Some((dev_id, bar0));
                        }
                    }
                }
            }
        }
        None
    }

    fn get_device_name(device_id: u16) -> &'static [u8; 64] {
        match device_id {
            RX_6600 =>     b"AMD Radeon RX 6600                                              ",
            RX_6700_XT =>  b"AMD Radeon RX 6700 XT                                           ",
            RX_6800 =>     b"AMD Radeon RX 6800                                              ",
            RX_6900_XT =>  b"AMD Radeon RX 6900 XT                                           ",
            RX_7600 =>     b"AMD Radeon RX 7600                                              ",
            RX_7700_XT =>  b"AMD Radeon RX 7700 XT                                           ",
            RX_7800_XT =>  b"AMD Radeon RX 7800 XT                                           ",
            RX_7900_XTX => b"AMD Radeon RX 7900 XTX                                          ",
            _ =>           b"AMD GPU (Unknown Model)                                         ",
        }
    }
}

impl GpuDevice for AmdGpu {
    fn init(&mut self) -> Result<(), GpuError> {
        if let Some((device_id, bar0)) = Self::detect() {
            self.bar0 = bar0;
            self.info.name = *Self::get_device_name(device_id);
            self.vram_size = match device_id {
                RX_6600 => 8 * 1024 * 1024 * 1024,
                RX_6700_XT => 12 * 1024 * 1024 * 1024,
                RX_6800 => 16 * 1024 * 1024 * 1024,
                RX_6900_XT => 16 * 1024 * 1024 * 1024,
                RX_7600 => 8 * 1024 * 1024 * 1024,
                RX_7700_XT => 12 * 1024 * 1024 * 1024,
                RX_7800_XT => 16 * 1024 * 1024 * 1024,
                RX_7900_XTX => 24 * 1024 * 1024 * 1024,
                _ => 8 * 1024 * 1024 * 1024,
            };
            self.info.vram_mb = (self.vram_size / (1024 * 1024)) as u32;
            self.initialized = true;
            Ok(())
        } else {
            Err(GpuError::NotFound)
        }
    }

    fn info(&self) -> &GpuDeviceInfo { &self.info }
    fn create_buffer(&mut self, _size: usize) -> Result<BufferId, GpuError> { Err(GpuError::Unsupported) }
    fn destroy_buffer(&mut self, _id: BufferId) -> Result<(), GpuError> { Err(GpuError::Unsupported) }
    fn write_buffer(&mut self, _id: BufferId, _offset: usize, _data: &[u8]) -> Result<(), GpuError> { Err(GpuError::Unsupported) }
    fn read_buffer(&mut self, _id: BufferId, _offset: usize, _data: &mut [u8]) -> Result<(), GpuError> { Err(GpuError::Unsupported) }
    fn create_shader(&mut self, _stage: ShaderStage, _code: &[u8]) -> Result<ShaderId, GpuError> { Err(GpuError::Unsupported) }
    fn destroy_shader(&mut self, _id: ShaderId) -> Result<(), GpuError> { Err(GpuError::Unsupported) }
    fn create_texture(&mut self, _width: u32, _height: u32, _format: TextureFormat) -> Result<TextureId, GpuError> { Err(GpuError::Unsupported) }
    fn destroy_texture(&mut self, _id: TextureId) -> Result<(), GpuError> { Err(GpuError::Unsupported) }
    fn create_pipeline(&mut self, _vertex: ShaderId, _fragment: ShaderId) -> Result<PipelineId, GpuError> { Err(GpuError::Unsupported) }
    fn destroy_pipeline(&mut self, _id: PipelineId) -> Result<(), GpuError> { Err(GpuError::Unsupported) }
    fn submit(&mut self, _commands: &[GpuCommand]) -> Result<(), GpuError> { Err(GpuError::Unsupported) }
    fn present(&mut self) -> Result<(), GpuError> { Err(GpuError::Unsupported) }
    fn framebuffer(&mut self) -> Option<&mut [u8]> { None }
    fn dimensions(&self) -> (u32, u32) { (0, 0) }
}

fn pci_read_device(bus: u8, device: u8, function: u8) -> Option<(u16, u16, u64)> {
    use core::arch::asm;
    let address: u32 = (1 << 31) | ((bus as u32) << 16) | ((device as u32) << 11) | ((function as u32) << 8);
    unsafe {
        asm!("out dx, eax", in("dx") 0xCF8u16, in("eax") address, options(nomem, nostack));
        let vendor_device: u32;
        asm!("in eax, dx", out("eax") vendor_device, in("dx") 0xCFCu16, options(nomem, nostack));
        let vendor = (vendor_device & 0xFFFF) as u16;
        let device_id = ((vendor_device >> 16) & 0xFFFF) as u16;
        if vendor == 0xFFFF || vendor == 0 { return None; }
        let addr_bar0 = address | 0x10;
        asm!("out dx, eax", in("dx") 0xCF8u16, in("eax") addr_bar0, options(nomem, nostack));
        let bar0: u32;
        asm!("in eax, dx", out("eax") bar0, in("dx") 0xCFCu16, options(nomem, nostack));
        Some((vendor, device_id, (bar0 & 0xFFFFFFF0) as u64))
    }
}
