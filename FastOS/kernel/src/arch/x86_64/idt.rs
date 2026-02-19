// ============================================================
// FastOS — Interrupt Descriptor Table (IDT)
// ============================================================
// 256-entry IDT for CPU exceptions (0-31) and hardware IRQs (32-47).
// PIC remapping: Master IRQ 0-7 → INT 32-39, Slave IRQ 8-15 → INT 40-47.
// ============================================================

use super::cpu::{IdtDescriptor, lidt};
use super::port;

/// IDT gate entry (16 bytes in 64-bit mode)
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct IdtGate {
    offset_low: u16,
    selector: u16,
    ist: u8,           // bits 0-2: IST index, bits 3-7: reserved
    type_attr: u8,     // type + DPL + present
    offset_mid: u16,
    offset_high: u32,
    _reserved: u32,
}

impl IdtGate {
    const fn empty() -> Self {
        IdtGate {
            offset_low: 0, selector: 0, ist: 0, type_attr: 0,
            offset_mid: 0, offset_high: 0, _reserved: 0,
        }
    }

    fn set_handler(&mut self, handler: u64, selector: u16, ist: u8, gate_type: u8) {
        self.offset_low = handler as u16;
        self.offset_mid = (handler >> 16) as u16;
        self.offset_high = (handler >> 32) as u32;
        self.selector = selector;
        self.ist = ist;
        self.type_attr = gate_type | 0x80; // Present bit
        self._reserved = 0;
    }
}

const IDT_SIZE: usize = 256;
static mut IDT: [IdtGate; IDT_SIZE] = [IdtGate::empty(); IDT_SIZE];
static mut IDTR: IdtDescriptor = IdtDescriptor { limit: 0, base: 0 };

// PIC ports
const PIC1_CMD: u16  = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_CMD: u16  = 0xA0;
const PIC2_DATA: u16 = 0xA1;
const PIC_EOI: u8    = 0x20;

/// Remap the PIC: Master → INT 32-39, Slave → INT 40-47
fn remap_pic() {
    // Save masks
    let mask1 = port::inb(PIC1_DATA);
    let mask2 = port::inb(PIC2_DATA);

    // ICW1: Initialize + ICW4 needed
    port::outb(PIC1_CMD, 0x11); port::io_wait();
    port::outb(PIC2_CMD, 0x11); port::io_wait();

    // ICW2: Vector offset
    port::outb(PIC1_DATA, 0x20); port::io_wait(); // Master → INT 32
    port::outb(PIC2_DATA, 0x28); port::io_wait(); // Slave  → INT 40

    // ICW3: Master/Slave wiring
    port::outb(PIC1_DATA, 0x04); port::io_wait(); // Slave on IRQ2
    port::outb(PIC2_DATA, 0x02); port::io_wait(); // Slave cascade identity

    // ICW4: 8086 mode
    port::outb(PIC1_DATA, 0x01); port::io_wait();
    port::outb(PIC2_DATA, 0x01); port::io_wait();

    // Restore masks
    port::outb(PIC1_DATA, mask1);
    port::outb(PIC2_DATA, mask2);
}

/// Send End-Of-Interrupt to PIC
pub fn send_eoi(irq: u8) {
    if irq >= 8 {
        port::outb(PIC2_CMD, PIC_EOI);
    }
    port::outb(PIC1_CMD, PIC_EOI);
}

/// Enable a specific IRQ line
pub fn enable_irq(irq: u8) {
    if irq < 8 {
        let mask = port::inb(PIC1_DATA);
        port::outb(PIC1_DATA, mask & !(1 << irq));
    } else {
        let mask = port::inb(PIC2_DATA);
        port::outb(PIC2_DATA, mask & !(1 << (irq - 8)));
    }
}

/// Disable a specific IRQ line
pub fn disable_irq(irq: u8) {
    if irq < 8 {
        let mask = port::inb(PIC1_DATA);
        port::outb(PIC1_DATA, mask | (1 << irq));
    } else {
        let mask = port::inb(PIC2_DATA);
        port::outb(PIC2_DATA, mask | (1 << (irq - 8)));
    }
}

// Default exception handler (stub)
extern "x86-interrupt" fn default_exception_handler(_frame: InterruptStackFrame) {
    // TODO: proper exception handling with error info
    loop { super::cpu::hlt(); }
}

// Page fault handler
extern "x86-interrupt" fn page_fault_handler(_frame: InterruptStackFrame, _error_code: u64) {
    // TODO: read CR2 for faulting address, handle or kill process
    loop { super::cpu::hlt(); }
}

// Double fault handler (uses IST1)
extern "x86-interrupt" fn double_fault_handler(_frame: InterruptStackFrame, _error_code: u64) -> ! {
    // Unrecoverable — halt
    loop { super::cpu::hlt(); }
}

// Timer IRQ handler (IRQ 0 → INT 32)
extern "x86-interrupt" fn timer_handler(_frame: InterruptStackFrame) {
    // TODO: increment tick counter, trigger scheduler
    send_eoi(0);
}

// Keyboard IRQ handler (IRQ 1 → INT 33)
extern "x86-interrupt" fn keyboard_handler(_frame: InterruptStackFrame) {
    let _scancode = port::inb(0x60);
    // TODO: push scancode to keyboard buffer
    send_eoi(1);
}

// Mouse IRQ handler (IRQ 12 → INT 44)
extern "x86-interrupt" fn mouse_handler(_frame: InterruptStackFrame) {
    let _data = port::inb(0x60);
    // TODO: push mouse packet byte to mouse driver
    send_eoi(12);
}

/// Interrupt Stack Frame (pushed by CPU on interrupt)
#[repr(C)]
pub struct InterruptStackFrame {
    pub instruction_pointer: u64,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segment: u64,
}

/// Initialize IDT and PIC
pub fn init() {
    remap_pic();

    unsafe {
        // CPU Exceptions (0-31) — set default handlers
        for i in 0..32 {
            IDT[i].set_handler(
                default_exception_handler as u64,
                super::gdt::KERNEL_CODE_SELECTOR,
                0,
                0x8E, // 64-bit interrupt gate, DPL=0
            );
        }

        // Page Fault (#14) — special handler with error code
        IDT[14].set_handler(
            page_fault_handler as u64,
            super::gdt::KERNEL_CODE_SELECTOR,
            0,
            0x8E,
        );

        // Double Fault (#8) — uses IST1
        IDT[8].set_handler(
            double_fault_handler as u64,
            super::gdt::KERNEL_CODE_SELECTOR,
            1,  // IST1
            0x8E,
        );

        // Hardware IRQs
        // IRQ 0 → INT 32: Timer
        IDT[32].set_handler(
            timer_handler as u64,
            super::gdt::KERNEL_CODE_SELECTOR,
            0,
            0x8E,
        );

        // IRQ 1 → INT 33: Keyboard
        IDT[33].set_handler(
            keyboard_handler as u64,
            super::gdt::KERNEL_CODE_SELECTOR,
            0,
            0x8E,
        );

        // IRQ 12 → INT 44: Mouse
        IDT[44].set_handler(
            mouse_handler as u64,
            super::gdt::KERNEL_CODE_SELECTOR,
            0,
            0x8E,
        );

        // Load IDT
        IDTR = IdtDescriptor {
            limit: (core::mem::size_of::<[IdtGate; IDT_SIZE]>() - 1) as u16,
            base: &IDT as *const _ as u64,
        };

        lidt(&IDTR);
    }

    // Enable timer + keyboard + mouse IRQs
    enable_irq(0);  // Timer
    enable_irq(1);  // Keyboard
    enable_irq(12); // Mouse
}
