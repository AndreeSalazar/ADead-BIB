// ============================================================
// FastOS — USB Host Controller Driver (Fase 9)
// ============================================================
// xHCI (USB 3.x) host controller driver.
// Scans PCI for xHCI, initializes, enumerates devices.
// ============================================================

use crate::arch::x86_64::port;

/// USB device descriptor
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct UsbDeviceDescriptor {
    pub length: u8,
    pub descriptor_type: u8,
    pub bcd_usb: u16,
    pub device_class: u8,
    pub device_subclass: u8,
    pub device_protocol: u8,
    pub max_packet_size0: u8,
    pub vendor_id: u16,
    pub product_id: u16,
    pub bcd_device: u16,
    pub manufacturer_idx: u8,
    pub product_idx: u8,
    pub serial_number_idx: u8,
    pub num_configurations: u8,
}

/// USB device info (simplified)
#[derive(Clone, Copy)]
pub struct UsbDevice {
    pub address: u8,
    pub port: u8,
    pub speed: UsbSpeed,
    pub vendor_id: u16,
    pub product_id: u16,
    pub device_class: u8,
    pub present: bool,
}

#[derive(Clone, Copy, PartialEq)]
pub enum UsbSpeed {
    Low,    // 1.5 Mbps
    Full,   // 12 Mbps
    High,   // 480 Mbps
    Super,  // 5 Gbps
    Unknown,
}

impl UsbDevice {
    pub const fn empty() -> Self {
        UsbDevice {
            address: 0, port: 0, speed: UsbSpeed::Unknown,
            vendor_id: 0, product_id: 0, device_class: 0, present: false,
        }
    }
}

/// Maximum USB devices
const MAX_USB_DEVICES: usize = 16;

static mut XHCI_BASE: u64 = 0;
static mut USB_INITIALIZED: bool = false;
static mut DEVICES: [UsbDevice; MAX_USB_DEVICES] = [UsbDevice::empty(); MAX_USB_DEVICES];
static mut DEVICE_COUNT: usize = 0;

/// xHCI Capability Registers
const XHCI_CAPLENGTH: u32 = 0x00;
const XHCI_HCSPARAMS1: u32 = 0x04;
const XHCI_HCSPARAMS2: u32 = 0x08;
const XHCI_HCCPARAMS1: u32 = 0x10;
const XHCI_DBOFF: u32 = 0x14;
const XHCI_RTSOFF: u32 = 0x18;

/// xHCI Operational Registers (offset = cap_length)
const XHCI_USBCMD: u32 = 0x00;
const XHCI_USBSTS: u32 = 0x04;
const XHCI_DNCTRL: u32 = 0x14;
const XHCI_CONFIG: u32 = 0x38;

/// Initialize USB by scanning PCI for xHCI controller
pub fn init() {
    unsafe {
        // Scan PCI for xHCI (class 0Ch, subclass 03h, prog-if 30h)
        for dev in 0..32u8 {
            for func in 0..8u8 {
                let vendor = pci_read_u16(0, dev, func, 0x00);
                if vendor == 0xFFFF { continue; }

                let class_code = pci_read_u8(0, dev, func, 0x0B);
                let subclass = pci_read_u8(0, dev, func, 0x0A);
                let prog_if = pci_read_u8(0, dev, func, 0x09);

                // Class 0Ch = Serial Bus, Subclass 03h = USB, Prog-IF 30h = xHCI
                if class_code == 0x0C && subclass == 0x03 && prog_if == 0x30 {
                    let bar0 = pci_read_u32(0, dev, func, 0x10);
                    XHCI_BASE = (bar0 & 0xFFFFFFF0) as u64;

                    if XHCI_BASE != 0 {
                        init_xhci();
                        USB_INITIALIZED = true;
                    }
                    return;
                }
            }
        }
    }
}

/// Initialize xHCI controller
unsafe fn init_xhci() {
    // Read capability length
    let cap_length = read_xhci(XHCI_CAPLENGTH) & 0xFF;
    let op_base = cap_length;

    // Read max ports and slots from HCSPARAMS1
    let hcsparams1 = read_xhci(XHCI_HCSPARAMS1);
    let _max_slots = hcsparams1 & 0xFF;
    let max_ports = (hcsparams1 >> 24) & 0xFF;

    // Stop controller
    let mut cmd = read_xhci(op_base + XHCI_USBCMD);
    cmd &= !1; // Clear Run/Stop bit
    write_xhci(op_base + XHCI_USBCMD, cmd);

    // Wait for halt
    for _ in 0..1000 {
        let sts = read_xhci(op_base + XHCI_USBSTS);
        if sts & 1 != 0 { break; } // HCHalted
    }

    // Reset controller
    cmd = read_xhci(op_base + XHCI_USBCMD);
    cmd |= 2; // Set HCRST
    write_xhci(op_base + XHCI_USBCMD, cmd);

    // Wait for reset complete
    for _ in 0..10000 {
        let c = read_xhci(op_base + XHCI_USBCMD);
        if c & 2 == 0 { break; }
    }

    // Enumerate connected ports (basic detection)
    DEVICE_COUNT = 0;
    let port_reg_base = op_base + 0x400; // Port register set starts at operational + 0x400

    for port in 0..max_ports {
        let portsc = read_xhci(port_reg_base + port * 0x10);
        let ccs = portsc & 1; // Current Connect Status

        if ccs != 0 {
            let speed_bits = (portsc >> 10) & 0xF;
            let speed = match speed_bits {
                1 => UsbSpeed::Full,
                2 => UsbSpeed::Low,
                3 => UsbSpeed::High,
                4 => UsbSpeed::Super,
                _ => UsbSpeed::Unknown,
            };

            if DEVICE_COUNT < MAX_USB_DEVICES {
                DEVICES[DEVICE_COUNT] = UsbDevice {
                    address: (DEVICE_COUNT + 1) as u8,
                    port: port as u8,
                    speed,
                    vendor_id: 0, // Would need full enumeration
                    product_id: 0,
                    device_class: 0,
                    present: true,
                };
                DEVICE_COUNT += 1;
            }
        }
    }
}

unsafe fn read_xhci(offset: u32) -> u32 {
    let ptr = (XHCI_BASE + offset as u64) as *const u32;
    core::ptr::read_volatile(ptr)
}

unsafe fn write_xhci(offset: u32, value: u32) {
    let ptr = (XHCI_BASE + offset as u64) as *mut u32;
    core::ptr::write_volatile(ptr, value);
}

fn pci_config_addr(bus: u8, dev: u8, func: u8, offset: u8) -> u32 {
    0x80000000 | ((bus as u32) << 16) | ((dev as u32) << 11)
        | ((func as u32) << 8) | ((offset as u32) & 0xFC)
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

pub fn is_initialized() -> bool { unsafe { USB_INITIALIZED } }
pub fn device_count() -> usize { unsafe { DEVICE_COUNT } }
