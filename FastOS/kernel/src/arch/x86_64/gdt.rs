// ============================================================
// FastOS — Global Descriptor Table (GDT)
// ============================================================
// Sets up GDT with null, kernel code64, kernel data64,
// user code64, user data64, and TSS segments.
// ============================================================

use super::cpu::{GdtDescriptor, lgdt};

/// GDT entry (8 bytes)
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

impl GdtEntry {
    const fn null() -> Self {
        GdtEntry {
            limit_low: 0, base_low: 0, base_mid: 0,
            access: 0, granularity: 0, base_high: 0,
        }
    }

    const fn new(access: u8, granularity: u8) -> Self {
        GdtEntry {
            limit_low: 0xFFFF,
            base_low: 0,
            base_mid: 0,
            access,
            granularity,
            base_high: 0,
        }
    }
}

/// TSS entry (16 bytes in 64-bit mode)
#[repr(C, packed)]
pub struct TssEntry {
    length: u16,
    base_low: u16,
    base_mid: u8,
    flags1: u8,
    flags2: u8,
    base_high: u8,
    base_upper: u32,
    _reserved: u32,
}

/// Task State Segment (104 bytes minimum)
#[repr(C, packed)]
pub struct Tss {
    _reserved0: u32,
    pub rsp0: u64,       // Kernel stack for Ring 0
    pub rsp1: u64,
    pub rsp2: u64,
    _reserved1: u64,
    pub ist1: u64,       // Interrupt Stack Table entry 1 (double fault)
    pub ist2: u64,
    pub ist3: u64,
    pub ist4: u64,
    pub ist5: u64,
    pub ist6: u64,
    pub ist7: u64,
    _reserved2: u64,
    _reserved3: u16,
    pub iomap_base: u16,
}

impl Tss {
    pub const fn new() -> Self {
        Tss {
            _reserved0: 0,
            rsp0: 0, rsp1: 0, rsp2: 0,
            _reserved1: 0,
            ist1: 0, ist2: 0, ist3: 0, ist4: 0,
            ist5: 0, ist6: 0, ist7: 0,
            _reserved2: 0,
            _reserved3: 0,
            iomap_base: core::mem::size_of::<Tss>() as u16,
        }
    }
}

// GDT with 5 entries + TSS (takes 2 slots)
// Total: 7 GDT slots = 56 bytes
const GDT_ENTRIES: usize = 7;

static mut GDT: [GdtEntry; GDT_ENTRIES] = [GdtEntry::null(); GDT_ENTRIES];
static mut TSS: Tss = Tss::new();
static mut GDTR: GdtDescriptor = GdtDescriptor { limit: 0, base: 0 };

// Segment selectors (index * 8)
pub const KERNEL_CODE_SELECTOR: u16 = 1 * 8;       // 0x08
pub const KERNEL_DATA_SELECTOR: u16 = 2 * 8;       // 0x10
pub const USER_CODE_SELECTOR: u16   = 3 * 8 | 3;   // 0x1B (RPL=3)
pub const USER_DATA_SELECTOR: u16   = 4 * 8 | 3;   // 0x23 (RPL=3)
pub const TSS_SELECTOR: u16         = 5 * 8;        // 0x28

/// Initialize GDT with kernel/user segments and TSS
pub fn init() {
    unsafe {
        // Entry 0: Null
        GDT[0] = GdtEntry::null();

        // Entry 1: Kernel Code 64-bit (execute/read, present, DPL=0, long mode)
        GDT[1] = GdtEntry::new(0x9A, 0x20);

        // Entry 2: Kernel Data 64-bit (read/write, present, DPL=0)
        GDT[2] = GdtEntry::new(0x92, 0x00);

        // Entry 3: User Code 64-bit (execute/read, present, DPL=3, long mode)
        GDT[3] = GdtEntry::new(0xFA, 0x20);

        // Entry 4: User Data 64-bit (read/write, present, DPL=3)
        GDT[4] = GdtEntry::new(0xF2, 0x00);

        // Entry 5-6: TSS (16 bytes, spans 2 GDT entries)
        let tss_addr = &TSS as *const Tss as u64;
        let tss_size = (core::mem::size_of::<Tss>() - 1) as u16;

        let tss_ptr = &mut GDT[5] as *mut GdtEntry as *mut TssEntry;
        (*tss_ptr) = TssEntry {
            length: tss_size,
            base_low: tss_addr as u16,
            base_mid: (tss_addr >> 16) as u8,
            flags1: 0x89,  // Present, 64-bit TSS (available)
            flags2: 0x00,
            base_high: (tss_addr >> 24) as u8,
            base_upper: (tss_addr >> 32) as u32,
            _reserved: 0,
        };

        // Load GDT
        GDTR = GdtDescriptor {
            limit: (core::mem::size_of::<[GdtEntry; GDT_ENTRIES]>() - 1) as u16,
            base: &GDT as *const _ as u64,
        };

        lgdt(&GDTR);

        // Reload segment registers
        core::arch::asm!(
            "mov ax, {data_sel:x}",
            "mov ds, ax",
            "mov es, ax",
            "mov fs, ax",
            "mov gs, ax",
            "mov ss, ax",
            data_sel = in(reg) KERNEL_DATA_SELECTOR as u64,
            options(nostack)
        );

        // Load TSS
        super::cpu::ltr(TSS_SELECTOR);
    }
}

/// Set the kernel stack pointer in TSS (used for Ring 3 → Ring 0 transitions)
pub fn set_kernel_stack(stack_top: u64) {
    unsafe {
        TSS.rsp0 = stack_top;
    }
}
