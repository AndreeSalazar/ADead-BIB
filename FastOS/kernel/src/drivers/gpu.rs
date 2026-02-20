// ============================================================
// FastOS — GPU Driver (NVIDIA via PCI + SPIR-V)
// ============================================================
// Detects NVIDIA GPU via PCI enumeration and provides basic
// compute dispatch interface using SPIR-V shaders.
//
// Integration with ADead-BIB abi_translators for SPIR-V parsing.
// ============================================================

use crate::serial_print;

// PCI Configuration Space ports
const PCI_CONFIG_ADDR: u16 = 0xCF8;
const PCI_CONFIG_DATA: u16 = 0xCFC;

// PCI Vendor IDs
const VENDOR_NVIDIA: u16 = 0x10DE;
const VENDOR_AMD: u16 = 0x1002;
const VENDOR_INTEL: u16 = 0x8086;

// PCI Class codes
const CLASS_DISPLAY: u8 = 0x03;
const SUBCLASS_VGA: u8 = 0x00;
const SUBCLASS_3D: u8 = 0x02;

// GPU Memory regions (BAR)
const GPU_BAR0_MMIO: usize = 0;
const GPU_BAR1_VRAM: usize = 1;

/// GPU device information
#[derive(Debug, Clone)]
pub struct GpuDevice {
    pub vendor_id: u16,
    pub device_id: u16,
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub bar0: u64,      // MMIO registers
    pub bar1: u64,      // VRAM
    pub vram_size: u64, // Detected VRAM size
    pub name: &'static str,
}

/// GPU compute buffer
pub struct GpuBuffer {
    pub ptr: u64,
    pub size: usize,
    pub device_ptr: u64,
}

/// SPIR-V kernel for compute
pub struct SpirVKernel {
    pub name: [u8; 32],
    pub bytecode: &'static [u8],
    pub local_size: [u32; 3],
}

/// GPU Driver state
pub struct GpuDriver {
    pub device: Option<GpuDevice>,
    pub initialized: bool,
}

impl GpuDriver {
    pub const fn new() -> Self {
        Self {
            device: None,
            initialized: false,
        }
    }

    /// Initialize GPU driver - scan PCI for GPU
    pub fn init(&mut self) -> bool {
        serial_print("[GPU] Scanning PCI bus for GPU...\r\n");
        
        // Scan PCI buses (limited scan for speed)
        for bus in 0..8u8 {
            for device in 0..32u8 {
                for function in 0..8u8 {
                    if let Some(gpu) = self.probe_pci_device(bus, device, function) {
                        serial_print("[GPU] Found: ");
                        serial_print(gpu.name);
                        serial_print(" (");
                        // Print vendor:device
                        print_hex_u16(gpu.vendor_id);
                        serial_print(":");
                        print_hex_u16(gpu.device_id);
                        serial_print(")\r\n");
                        
                        // Print BAR info
                        serial_print("[GPU] BAR0=0x");
                        print_hex_u64(gpu.bar0);
                        serial_print(" BAR1=0x");
                        print_hex_u64(gpu.bar1);
                        serial_print("\r\n");
                        
                        self.device = Some(gpu);
                        self.initialized = true;
                        return true;
                    }
                }
            }
        }
        
        serial_print("[GPU] No compatible GPU found\r\n");
        false
    }

    /// Probe a PCI device to check if it's a GPU
    fn probe_pci_device(&self, bus: u8, device: u8, function: u8) -> Option<GpuDevice> {
        let vendor_id = pci_read_word(bus, device, function, 0x00);
        if vendor_id == 0xFFFF || vendor_id == 0x0000 {
            return None;
        }

        let device_id = pci_read_word(bus, device, function, 0x02);
        let class_code = pci_read_byte(bus, device, function, 0x0B);
        let subclass = pci_read_byte(bus, device, function, 0x0A);

        // Check if it's a display controller
        if class_code != CLASS_DISPLAY {
            return None;
        }
        if subclass != SUBCLASS_VGA && subclass != SUBCLASS_3D {
            return None;
        }

        // Read BARs
        let bar0 = pci_read_bar(bus, device, function, 0x10);
        let bar1 = pci_read_bar(bus, device, function, 0x14);

        // Identify GPU
        let name = match vendor_id {
            VENDOR_NVIDIA => identify_nvidia_gpu(device_id),
            VENDOR_AMD => "AMD GPU",
            VENDOR_INTEL => "Intel GPU",
            _ => "Unknown GPU",
        };

        // Estimate VRAM (simplified - real detection requires GPU-specific code)
        let vram_size = estimate_vram(vendor_id, device_id);

        Some(GpuDevice {
            vendor_id,
            device_id,
            bus,
            device,
            function,
            bar0,
            bar1,
            vram_size,
            name,
        })
    }

    /// Check if GPU is available
    pub fn is_available(&self) -> bool {
        self.initialized && self.device.is_some()
    }

    /// Get GPU info string
    pub fn get_info(&self) -> &'static str {
        if let Some(ref dev) = self.device {
            dev.name
        } else {
            "No GPU"
        }
    }

    /// Allocate a GPU buffer (placeholder - requires real GPU memory management)
    pub fn alloc_buffer(&self, size: usize) -> Option<GpuBuffer> {
        if !self.is_available() {
            return None;
        }

        // In a real implementation, this would allocate from GPU VRAM
        // For now, we just track the allocation
        Some(GpuBuffer {
            ptr: 0,
            size,
            device_ptr: 0,
        })
    }

    /// Load a SPIR-V kernel (placeholder)
    pub fn load_spirv(&self, _bytecode: &[u8]) -> Option<SpirVKernel> {
        if !self.is_available() {
            return None;
        }

        // In a real implementation, this would:
        // 1. Parse SPIR-V using abi_translators
        // 2. Compile to GPU-native code
        // 3. Upload to GPU

        Some(SpirVKernel {
            name: [0; 32],
            bytecode: &[],
            local_size: [64, 1, 1],
        })
    }

    /// Dispatch a compute shader (placeholder)
    pub fn dispatch(&self, _kernel: &SpirVKernel, groups: [u32; 3]) -> bool {
        if !self.is_available() {
            return false;
        }

        serial_print("[GPU] Dispatch compute: ");
        // Would actually dispatch to GPU here
        let _ = groups;
        true
    }
}

// ============================================================
// Hex printing helpers
// ============================================================

fn print_hex_u16(val: u16) {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut buf = [0u8; 4];
    buf[0] = HEX[((val >> 12) & 0xF) as usize];
    buf[1] = HEX[((val >> 8) & 0xF) as usize];
    buf[2] = HEX[((val >> 4) & 0xF) as usize];
    buf[3] = HEX[(val & 0xF) as usize];
    for &c in &buf {
        print_char(c);
    }
}

fn print_hex_u64(val: u64) {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    for i in (0..16).rev() {
        let nibble = ((val >> (i * 4)) & 0xF) as usize;
        print_char(HEX[nibble]);
    }
}

fn print_char(c: u8) {
    unsafe {
        // Output to serial port COM1
        while (inb_simple(0x3FD) & 0x20) == 0 {}
        outb_simple(0x3F8, c);
    }
}

unsafe fn outb_simple(port: u16, value: u8) {
    core::arch::asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack));
}

unsafe fn inb_simple(port: u16) -> u8 {
    let value: u8;
    core::arch::asm!("in al, dx", in("dx") port, out("al") value, options(nomem, nostack));
    value
}

// ============================================================
// PCI Configuration Space Access
// ============================================================

fn pci_config_address(bus: u8, device: u8, function: u8, offset: u8) -> u32 {
    let bus = bus as u32;
    let device = device as u32;
    let function = function as u32;
    let offset = (offset & 0xFC) as u32;
    0x80000000 | (bus << 16) | (device << 11) | (function << 8) | offset
}

fn pci_read_dword(bus: u8, device: u8, function: u8, offset: u8) -> u32 {
    let addr = pci_config_address(bus, device, function, offset);
    unsafe {
        outl(PCI_CONFIG_ADDR, addr);
        inl(PCI_CONFIG_DATA)
    }
}

fn pci_read_word(bus: u8, device: u8, function: u8, offset: u8) -> u16 {
    let dword = pci_read_dword(bus, device, function, offset & 0xFC);
    let shift = ((offset & 2) * 8) as u32;
    ((dword >> shift) & 0xFFFF) as u16
}

fn pci_read_byte(bus: u8, device: u8, function: u8, offset: u8) -> u8 {
    let dword = pci_read_dword(bus, device, function, offset & 0xFC);
    let shift = ((offset & 3) * 8) as u32;
    ((dword >> shift) & 0xFF) as u8
}

fn pci_read_bar(bus: u8, device: u8, function: u8, bar_offset: u8) -> u64 {
    let low = pci_read_dword(bus, device, function, bar_offset);
    
    // Check if 64-bit BAR
    if (low & 0x06) == 0x04 {
        let high = pci_read_dword(bus, device, function, bar_offset + 4);
        let addr = ((high as u64) << 32) | ((low & 0xFFFFFFF0) as u64);
        addr
    } else {
        (low & 0xFFFFFFF0) as u64
    }
}

// ============================================================
// Port I/O
// ============================================================

unsafe fn outl(port: u16, value: u32) {
    core::arch::asm!(
        "out dx, eax",
        in("dx") port,
        in("eax") value,
        options(nomem, nostack)
    );
}

unsafe fn inl(port: u16) -> u32 {
    let value: u32;
    core::arch::asm!(
        "in eax, dx",
        in("dx") port,
        out("eax") value,
        options(nomem, nostack)
    );
    value
}

// ============================================================
// GPU Identification
// ============================================================

fn identify_nvidia_gpu(device_id: u16) -> &'static str {
    // Common NVIDIA device IDs (RTX 30 series, etc.)
    match device_id {
        // RTX 40 series
        0x2684 => "NVIDIA RTX 4090",
        0x2704 => "NVIDIA RTX 4080",
        0x2782 => "NVIDIA RTX 4070 Ti",
        0x2786 => "NVIDIA RTX 4070",
        
        // RTX 30 series
        0x2204 => "NVIDIA RTX 3090",
        0x2206 => "NVIDIA RTX 3080",
        0x2208 => "NVIDIA RTX 3080 Ti",
        0x2484 => "NVIDIA RTX 3070",
        0x2488 => "NVIDIA RTX 3070 Ti",
        0x2503 => "NVIDIA RTX 3060",
        0x2504 => "NVIDIA RTX 3060 Ti",
        0x2520 => "NVIDIA RTX 3060 Mobile",
        0x2531 => "NVIDIA RTX 3060 Laptop",
        
        // RTX 20 series
        0x1E04 => "NVIDIA RTX 2080 Ti",
        0x1E07 => "NVIDIA RTX 2080 Super",
        0x1E82 => "NVIDIA RTX 2080",
        0x1F02 => "NVIDIA RTX 2070",
        0x1F07 => "NVIDIA RTX 2070 Super",
        0x1F08 => "NVIDIA RTX 2060 Super",
        0x1F82 => "NVIDIA RTX 2060",
        
        // GTX 16 series
        0x2182 => "NVIDIA GTX 1660 Ti",
        0x2184 => "NVIDIA GTX 1660",
        0x21C4 => "NVIDIA GTX 1660 Super",
        0x1F91 => "NVIDIA GTX 1650 Super",
        0x1F82 => "NVIDIA GTX 1650",
        
        // GTX 10 series
        0x1B80 => "NVIDIA GTX 1080",
        0x1B81 => "NVIDIA GTX 1070",
        0x1B82 => "NVIDIA GTX 1070 Ti",
        0x1C02 => "NVIDIA GTX 1060 6GB",
        0x1C03 => "NVIDIA GTX 1060 3GB",
        0x1C81 => "NVIDIA GTX 1050",
        0x1C82 => "NVIDIA GTX 1050 Ti",
        
        // Quadro / Professional
        0x1DB6 => "NVIDIA Quadro RTX 5000",
        0x1E30 => "NVIDIA Quadro RTX 6000",
        
        _ => "NVIDIA GPU (Unknown Model)",
    }
}

fn estimate_vram(vendor_id: u16, device_id: u16) -> u64 {
    if vendor_id == VENDOR_NVIDIA {
        match device_id {
            0x2684 => 24 * 1024 * 1024 * 1024, // RTX 4090: 24GB
            0x2704 => 16 * 1024 * 1024 * 1024, // RTX 4080: 16GB
            0x2204 => 24 * 1024 * 1024 * 1024, // RTX 3090: 24GB
            0x2206 => 10 * 1024 * 1024 * 1024, // RTX 3080: 10GB
            0x2503 => 12 * 1024 * 1024 * 1024, // RTX 3060: 12GB
            0x2504 => 8 * 1024 * 1024 * 1024,  // RTX 3060 Ti: 8GB
            _ => 8 * 1024 * 1024 * 1024,       // Default 8GB
        }
    } else {
        4 * 1024 * 1024 * 1024 // Default 4GB
    }
}

// ============================================================
// Global GPU Driver Instance
// ============================================================

static mut GPU_DRIVER: GpuDriver = GpuDriver::new();

pub fn init() -> bool {
    unsafe { GPU_DRIVER.init() }
}

pub fn get_driver() -> &'static GpuDriver {
    unsafe { &GPU_DRIVER }
}

pub fn get_driver_mut() -> &'static mut GpuDriver {
    unsafe { &mut GPU_DRIVER }
}
