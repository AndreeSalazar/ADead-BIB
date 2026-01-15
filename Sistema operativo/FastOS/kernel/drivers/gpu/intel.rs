// ============================================================================
// FastOS Intel GPU Driver
// ============================================================================
// Driver para GPUs Intel (Integrated Graphics, Arc)
//
// Author: Eddi Andreé Salazar Matos 🇵🇪
// ============================================================================

#![allow(dead_code)]

use super::hal::*;

/// ID de vendor Intel
pub const INTEL_VENDOR_ID: u16 = 0x8086;

/// Intel UHD Graphics
pub const UHD_620: u16 = 0x5917;
pub const UHD_630: u16 = 0x3E92;
pub const UHD_770: u16 = 0x4680;

/// Intel Arc
pub const ARC_A380: u16 = 0x56A5;
pub const ARC_A580: u16 = 0x56A1;
pub const ARC_A750: u16 = 0x56A0;
pub const ARC_A770: u16 = 0x56A0;

/// Driver Intel
pub struct IntelGpu {
    info: GpuDeviceInfo,
    bar0: u64,
    initialized: bool,
}

impl IntelGpu {
    pub const fn new() -> Self {
        IntelGpu {
            info: GpuDeviceInfo {
                vendor: GpuVendor::Intel,
                name: *b"Intel GPU (Not Initialized)                                     ",
                vram_mb: 0,
                max_texture_size: 8192,
                supports_compute: true,
                supports_raytracing: false,
            },
            bar0: 0,
            initialized: false,
        }
    }

    pub fn detect() -> Option<(u16, u64)> {
        for bus in 0..256u16 {
            for device in 0..32u8 {
                for function in 0..8u8 {
                    if let Some((vendor, dev_id, bar0)) = pci_read_device(bus as u8, device, function) {
                        if vendor == INTEL_VENDOR_ID && is_gpu_device(dev_id) {
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
            UHD_620 =>   b"Intel UHD Graphics 620                                          ",
            UHD_630 =>   b"Intel UHD Graphics 630                                          ",
            UHD_770 =>   b"Intel UHD Graphics 770                                          ",
            ARC_A380 =>  b"Intel Arc A380                                                  ",
            ARC_A580 =>  b"Intel Arc A580                                                  ",
            ARC_A750 =>  b"Intel Arc A750                                                  ",
            ARC_A770 =>  b"Intel Arc A770                                                  ",
            _ =>         b"Intel GPU (Unknown Model)                                       ",
        }
    }
}

fn is_gpu_device(device_id: u16) -> bool {
    matches!(device_id, UHD_620 | UHD_630 | UHD_770 | ARC_A380 | ARC_A580 | ARC_A750 | ARC_A770)
}

impl GpuDevice for IntelGpu {
    fn init(&mut self) -> Result<(), GpuError> {
        if let Some((device_id, bar0)) = Self::detect() {
            self.bar0 = bar0;
            self.info.name = *Self::get_device_name(device_id);
            self.info.vram_mb = 2048; // Shared memory
            if matches!(device_id, ARC_A380 | ARC_A580 | ARC_A750 | ARC_A770) {
                self.info.supports_raytracing = true;
            }
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
