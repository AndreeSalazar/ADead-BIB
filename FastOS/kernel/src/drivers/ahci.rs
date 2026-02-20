// ============================================================
// FastOS — AHCI Driver (Fase 9)
// ============================================================
// Advanced Host Controller Interface for SATA drives.
// Replaces ATA PIO with DMA-based SATA access.
// ============================================================

use crate::arch::x86_64::port;

/// AHCI Base Address (from PCI BAR5)
static mut AHCI_BASE: u64 = 0;
static mut AHCI_INITIALIZED: bool = false;

/// AHCI Generic Host Control registers (offsets from AHCI_BASE)
const HBA_CAP: u32 = 0x00;        // Host Capabilities
const HBA_GHC: u32 = 0x04;        // Global Host Control
const HBA_IS: u32 = 0x08;         // Interrupt Status
const HBA_PI: u32 = 0x0C;         // Ports Implemented
const HBA_VS: u32 = 0x10;         // Version

/// Port register offsets (each port = 0x80 bytes, starting at 0x100)
const PORT_BASE: u32 = 0x100;
const PORT_SIZE: u32 = 0x80;
const PORT_CLB: u32 = 0x00;       // Command List Base Address
const PORT_FB: u32 = 0x08;        // FIS Base Address
const PORT_IS: u32 = 0x10;        // Interrupt Status
const PORT_IE: u32 = 0x14;        // Interrupt Enable
const PORT_CMD: u32 = 0x18;       // Command and Status
const PORT_TFD: u32 = 0x20;       // Task File Data
const PORT_SIG: u32 = 0x24;       // Signature
const PORT_SSTS: u32 = 0x28;      // SATA Status
const PORT_SCTL: u32 = 0x2C;      // SATA Control
const PORT_SERR: u32 = 0x30;      // SATA Error

/// SATA device signatures
const SATA_SIG_ATA: u32 = 0x00000101;   // SATA drive
const SATA_SIG_ATAPI: u32 = 0xEB140101; // SATAPI drive

/// Maximum ports
const MAX_PORTS: usize = 32;

/// Port device type
#[derive(Clone, Copy, PartialEq)]
pub enum DeviceType {
    None,
    Sata,
    Satapi,
    Unknown,
}

/// Port info
#[derive(Clone, Copy)]
pub struct AhciPort {
    pub port_num: u8,
    pub device_type: DeviceType,
    pub present: bool,
}

static mut PORTS: [AhciPort; MAX_PORTS] = [AhciPort {
    port_num: 0, device_type: DeviceType::None, present: false,
}; MAX_PORTS];
static mut PORT_COUNT: usize = 0;

/// Scan PCI for AHCI controller (class 01h, subclass 06h)
pub fn init() {
    unsafe {
        // Scan PCI bus 0 for AHCI controller
        for dev in 0..32u8 {
            for func in 0..8u8 {
                let vendor = pci_read_u16(0, dev, func, 0x00);
                if vendor == 0xFFFF { continue; }

                let class_code = pci_read_u8(0, dev, func, 0x0B);
                let subclass = pci_read_u8(0, dev, func, 0x0A);

                // Class 01h = Mass Storage, Subclass 06h = SATA (AHCI)
                if class_code == 0x01 && subclass == 0x06 {
                    // Read BAR5 (AHCI ABAR)
                    let bar5 = pci_read_u32(0, dev, func, 0x24);
                    AHCI_BASE = (bar5 & 0xFFFFFFF0) as u64;

                    if AHCI_BASE != 0 {
                        probe_ports();
                        AHCI_INITIALIZED = true;
                    }
                    return;
                }
            }
        }
    }
}

/// Probe AHCI ports for connected devices
unsafe fn probe_ports() {
    let pi = read_hba(HBA_PI);
    PORT_COUNT = 0;

    for i in 0..32u8 {
        if pi & (1 << i) != 0 {
            let port_offset = PORT_BASE + (i as u32) * PORT_SIZE;
            let ssts = read_port_reg(port_offset, PORT_SSTS);
            let det = ssts & 0x0F;  // Device detection
            let ipm = (ssts >> 8) & 0x0F; // Interface power management

            if det == 3 && ipm == 1 {
                // Device present and active
                let sig = read_port_reg(port_offset, PORT_SIG);
                let dtype = match sig {
                    SATA_SIG_ATA => DeviceType::Sata,
                    SATA_SIG_ATAPI => DeviceType::Satapi,
                    _ => DeviceType::Unknown,
                };

                PORTS[PORT_COUNT] = AhciPort {
                    port_num: i,
                    device_type: dtype,
                    present: true,
                };
                PORT_COUNT += 1;
            }
        }
    }
}

/// Read from HBA register
unsafe fn read_hba(offset: u32) -> u32 {
    let ptr = (AHCI_BASE + offset as u64) as *const u32;
    core::ptr::read_volatile(ptr)
}

/// Write to HBA register
unsafe fn write_hba(offset: u32, value: u32) {
    let ptr = (AHCI_BASE + offset as u64) as *mut u32;
    core::ptr::write_volatile(ptr, value);
}

/// Read a port register
unsafe fn read_port_reg(port_base: u32, reg_offset: u32) -> u32 {
    let ptr = (AHCI_BASE + (port_base + reg_offset) as u64) as *const u32;
    core::ptr::read_volatile(ptr)
}

/// PCI configuration space access
fn pci_config_addr(bus: u8, dev: u8, func: u8, offset: u8) -> u32 {
    0x80000000
        | ((bus as u32) << 16)
        | ((dev as u32) << 11)
        | ((func as u32) << 8)
        | ((offset as u32) & 0xFC)
}

unsafe fn pci_read_u32(bus: u8, dev: u8, func: u8, offset: u8) -> u32 {
    port::outl(0xCF8, pci_config_addr(bus, dev, func, offset));
    port::inl(0xCFC)
}

unsafe fn pci_read_u16(bus: u8, dev: u8, func: u8, offset: u8) -> u16 {
    let val = pci_read_u32(bus, dev, func, offset & 0xFC);
    ((val >> ((offset & 2) * 8)) & 0xFFFF) as u16
}

unsafe fn pci_read_u8(bus: u8, dev: u8, func: u8, offset: u8) -> u8 {
    let val = pci_read_u32(bus, dev, func, offset & 0xFC);
    ((val >> ((offset & 3) * 8)) & 0xFF) as u8
}

/// Get number of detected AHCI ports
pub fn port_count() -> usize {
    unsafe { PORT_COUNT }
}

/// Check if AHCI is initialized
pub fn is_initialized() -> bool {
    unsafe { AHCI_INITIALIZED }
}

/// Get AHCI version string
pub fn version() -> (u8, u8) {
    unsafe {
        if !AHCI_INITIALIZED { return (0, 0); }
        let vs = read_hba(HBA_VS);
        let major = ((vs >> 16) & 0xFF) as u8;
        let minor = (vs & 0xFF) as u8;
        (major, minor)
    }
}
