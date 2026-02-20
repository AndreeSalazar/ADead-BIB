// ============================================================
// FastOS — Audio Driver (Fase 9)
// ============================================================
// Intel HD Audio (HDA) controller driver.
// PCI scan → codec discovery → basic PCM playback.
// ============================================================

use crate::arch::x86_64::port;

/// HDA register offsets
const HDA_GCAP: u32 = 0x00;       // Global Capabilities
const HDA_VMIN: u32 = 0x02;       // Minor Version
const HDA_VMAJ: u32 = 0x03;       // Major Version
const HDA_GCTL: u32 = 0x08;       // Global Control
const HDA_WAKEEN: u32 = 0x0C;     // Wake Enable
const HDA_STATESTS: u32 = 0x0E;   // State Change Status
const HDA_INTCTL: u32 = 0x20;     // Interrupt Control
const HDA_INTSTS: u32 = 0x24;     // Interrupt Status
const HDA_CORBLBASE: u32 = 0x40;  // CORB Lower Base Address
const HDA_CORBUBASE: u32 = 0x44;  // CORB Upper Base Address
const HDA_CORBWP: u32 = 0x48;     // CORB Write Pointer
const HDA_CORBRP: u32 = 0x4A;     // CORB Read Pointer
const HDA_CORBCTL: u32 = 0x4C;    // CORB Control
const HDA_RIRLBASE: u32 = 0x50;   // RIRB Lower Base Address
const HDA_RIRBUBASE: u32 = 0x54;  // RIRB Upper Base Address
const HDA_RIRBWP: u32 = 0x58;     // RIRB Write Pointer
const HDA_RIRBCTL: u32 = 0x5C;    // RIRB Control

/// GCTL bits
const GCTL_CRST: u32 = 1;         // Controller Reset

/// Audio codec info
#[derive(Clone, Copy)]
pub struct AudioCodec {
    pub address: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub present: bool,
}

impl AudioCodec {
    pub const fn empty() -> Self {
        AudioCodec { address: 0, vendor_id: 0, device_id: 0, present: false }
    }
}

const MAX_CODECS: usize = 4;

static mut HDA_BASE: u64 = 0;
static mut HDA_INITIALIZED: bool = false;
static mut CODECS: [AudioCodec; MAX_CODECS] = [AudioCodec::empty(); MAX_CODECS];
static mut CODEC_COUNT: usize = 0;

/// Audio sample format
#[derive(Clone, Copy)]
pub struct AudioFormat {
    pub sample_rate: u32,
    pub bits_per_sample: u8,
    pub channels: u8,
}

pub const DEFAULT_FORMAT: AudioFormat = AudioFormat {
    sample_rate: 44100,
    bits_per_sample: 16,
    channels: 2,
};

/// Initialize audio by scanning PCI for HDA controller
pub fn init() {
    unsafe {
        for dev in 0..32u8 {
            for func in 0..8u8 {
                let vendor = pci_read_u16(0, dev, func, 0x00);
                if vendor == 0xFFFF { continue; }

                let class_code = pci_read_u8(0, dev, func, 0x0B);
                let subclass = pci_read_u8(0, dev, func, 0x0A);

                // Class 04h = Multimedia, Subclass 03h = HD Audio
                if class_code == 0x04 && subclass == 0x03 {
                    let bar0 = pci_read_u32(0, dev, func, 0x10);
                    HDA_BASE = (bar0 & 0xFFFFFFF0) as u64;

                    if HDA_BASE != 0 {
                        // Enable PCI bus mastering + memory space
                        let cmd = pci_read_u16(0, dev, func, 0x04);
                        pci_write_u16(0, dev, func, 0x04, cmd | 0x06);

                        init_hda();
                        HDA_INITIALIZED = true;
                    }
                    return;
                }
            }
        }
    }
}

/// Initialize HDA controller
unsafe fn init_hda() {
    // Reset controller
    write_hda(HDA_GCTL, 0); // Clear CRST
    for _ in 0..10000 {
        if read_hda(HDA_GCTL) & GCTL_CRST == 0 { break; }
    }

    // Take out of reset
    write_hda(HDA_GCTL, GCTL_CRST);
    for _ in 0..10000 {
        if read_hda(HDA_GCTL) & GCTL_CRST != 0 { break; }
    }

    // Wait for codecs to enumerate
    for _ in 0..100000 {
        let statests = read_hda_u16(HDA_STATESTS);
        if statests != 0 { break; }
    }

    // Detect codecs from STATESTS
    let statests = read_hda_u16(HDA_STATESTS);
    CODEC_COUNT = 0;

    for addr in 0..4u8 {
        if statests & (1 << addr) != 0 {
            CODECS[CODEC_COUNT] = AudioCodec {
                address: addr,
                vendor_id: 0, // Would need CORB/RIRB to query
                device_id: 0,
                present: true,
            };
            CODEC_COUNT += 1;
        }
    }
}

unsafe fn read_hda(offset: u32) -> u32 {
    let ptr = (HDA_BASE + offset as u64) as *const u32;
    core::ptr::read_volatile(ptr)
}

unsafe fn read_hda_u16(offset: u32) -> u16 {
    let ptr = (HDA_BASE + offset as u64) as *const u16;
    core::ptr::read_volatile(ptr)
}

unsafe fn write_hda(offset: u32, value: u32) {
    let ptr = (HDA_BASE + offset as u64) as *mut u32;
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

unsafe fn pci_write_u16(bus: u8, dev: u8, func: u8, offset: u8, value: u16) {
    let mut val = pci_read_u32(bus, dev, func, offset & 0xFC);
    let shift = (offset & 2) * 8;
    val &= !(0xFFFF << shift);
    val |= (value as u32) << shift;
    port::outl(0xCF8, pci_config_addr(bus, dev, func, offset));
    port::outl(0xCFC, val);
}

pub fn is_initialized() -> bool { unsafe { HDA_INITIALIZED } }
pub fn codec_count() -> usize { unsafe { CODEC_COUNT } }

/// Get HDA version
pub fn version() -> (u8, u8) {
    unsafe {
        if !HDA_INITIALIZED { return (0, 0); }
        let major = read_hda_u16(HDA_VMAJ as u32) as u8;
        let minor = read_hda_u16(HDA_VMIN as u32) as u8;
        (major, minor)
    }
}
