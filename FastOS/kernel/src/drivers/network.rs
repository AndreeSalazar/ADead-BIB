// ============================================================
// FastOS — Network Driver (Fase 9)
// ============================================================
// Intel E1000/E1000E Ethernet driver (most common in QEMU).
// PCI scan → MMIO init → basic packet TX/RX.
// ============================================================

use crate::arch::x86_64::port;

/// E1000 register offsets
const E1000_CTRL: u32 = 0x0000;
const E1000_STATUS: u32 = 0x0008;
const E1000_EERD: u32 = 0x0014;
const E1000_ICR: u32 = 0x00C0;
const E1000_IMS: u32 = 0x00D0;
const E1000_IMC: u32 = 0x00D8;
const E1000_RCTL: u32 = 0x0100;
const E1000_TCTL: u32 = 0x0400;
const E1000_RDBAL: u32 = 0x2800;
const E1000_RDBAH: u32 = 0x2804;
const E1000_RDLEN: u32 = 0x2808;
const E1000_RDH: u32 = 0x2810;
const E1000_RDT: u32 = 0x2818;
const E1000_TDBAL: u32 = 0x3800;
const E1000_TDBAH: u32 = 0x3804;
const E1000_TDLEN: u32 = 0x3808;
const E1000_TDH: u32 = 0x3810;
const E1000_TDT: u32 = 0x3818;
const E1000_RAL: u32 = 0x5400;
const E1000_RAH: u32 = 0x5404;

/// CTRL register bits
const CTRL_RST: u32 = 1 << 26;
const CTRL_SLU: u32 = 1 << 6;

/// RCTL register bits
const RCTL_EN: u32 = 1 << 1;
const RCTL_BAM: u32 = 1 << 15;
const RCTL_BSIZE_2048: u32 = 0;
const RCTL_SECRC: u32 = 1 << 26;

/// TCTL register bits
const TCTL_EN: u32 = 1 << 1;
const TCTL_PSP: u32 = 1 << 3;

/// MAC address
static mut MAC_ADDR: [u8; 6] = [0; 6];
static mut NIC_BASE: u64 = 0;
static mut NIC_INITIALIZED: bool = false;

/// Known E1000 vendor/device IDs
const INTEL_VENDOR: u16 = 0x8086;
const E1000_DEVICE_IDS: [u16; 4] = [0x100E, 0x100F, 0x10D3, 0x153A];

/// Initialize network by scanning PCI for Intel E1000
pub fn init() {
    unsafe {
        for dev in 0..32u8 {
            for func in 0..8u8 {
                let vendor = pci_read_u16(0, dev, func, 0x00);
                if vendor != INTEL_VENDOR { continue; }

                let device_id = pci_read_u16(0, dev, func, 0x02);
                let is_e1000 = E1000_DEVICE_IDS.iter().any(|&id| id == device_id);

                if is_e1000 {
                    let bar0 = pci_read_u32(0, dev, func, 0x10);
                    NIC_BASE = (bar0 & 0xFFFFFFF0) as u64;

                    if NIC_BASE != 0 {
                        // Enable PCI bus mastering
                        let cmd = pci_read_u16(0, dev, func, 0x04);
                        pci_write_u16(0, dev, func, 0x04, cmd | 0x07);

                        init_e1000();
                        NIC_INITIALIZED = true;
                    }
                    return;
                }
            }
        }
    }
}

/// Initialize E1000 NIC
unsafe fn init_e1000() {
    // Reset
    let ctrl = read_nic(E1000_CTRL);
    write_nic(E1000_CTRL, ctrl | CTRL_RST);

    // Wait for reset
    for _ in 0..100000 {
        if read_nic(E1000_CTRL) & CTRL_RST == 0 { break; }
    }

    // Disable interrupts
    write_nic(E1000_IMC, 0xFFFFFFFF);

    // Set link up
    let ctrl = read_nic(E1000_CTRL);
    write_nic(E1000_CTRL, ctrl | CTRL_SLU);

    // Read MAC address from EEPROM or RAL/RAH
    let ral = read_nic(E1000_RAL);
    let rah = read_nic(E1000_RAH);
    MAC_ADDR[0] = (ral & 0xFF) as u8;
    MAC_ADDR[1] = ((ral >> 8) & 0xFF) as u8;
    MAC_ADDR[2] = ((ral >> 16) & 0xFF) as u8;
    MAC_ADDR[3] = ((ral >> 24) & 0xFF) as u8;
    MAC_ADDR[4] = (rah & 0xFF) as u8;
    MAC_ADDR[5] = ((rah >> 8) & 0xFF) as u8;

    // Enable receive
    write_nic(E1000_RCTL, RCTL_EN | RCTL_BAM | RCTL_BSIZE_2048 | RCTL_SECRC);

    // Enable transmit
    write_nic(E1000_TCTL, TCTL_EN | TCTL_PSP);
}

unsafe fn read_nic(offset: u32) -> u32 {
    let ptr = (NIC_BASE + offset as u64) as *const u32;
    core::ptr::read_volatile(ptr)
}

unsafe fn write_nic(offset: u32, value: u32) {
    let ptr = (NIC_BASE + offset as u64) as *mut u32;
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

unsafe fn pci_write_u16(bus: u8, dev: u8, func: u8, offset: u8, value: u16) {
    let mut val = pci_read_u32(bus, dev, func, offset & 0xFC);
    let shift = (offset & 2) * 8;
    val &= !(0xFFFF << shift);
    val |= (value as u32) << shift;
    port::outl(0xCF8, pci_config_addr(bus, dev, func, offset));
    port::outl(0xCFC, val);
}

/// Get MAC address
pub fn mac_address() -> [u8; 6] { unsafe { MAC_ADDR } }
pub fn is_initialized() -> bool { unsafe { NIC_INITIALIZED } }

/// Check link status
pub fn link_up() -> bool {
    unsafe {
        if !NIC_INITIALIZED { return false; }
        read_nic(E1000_STATUS) & 2 != 0
    }
}
