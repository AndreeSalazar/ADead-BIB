// ============================================================================
// FastOS IDT - Interrupt Descriptor Table
// ============================================================================
// Manejo de interrupciones del sistema
//
// Author: Eddi Andreé Salazar Matos 🇵🇪
// ============================================================================

#![allow(dead_code)]

use core::arch::asm;

/// Entrada de la IDT (64-bit)
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct IdtEntry {
    offset_low: u16,
    selector: u16,
    ist: u8,
    type_attr: u8,
    offset_mid: u16,
    offset_high: u32,
    zero: u32,
}

impl IdtEntry {
    pub const fn empty() -> Self {
        IdtEntry {
            offset_low: 0,
            selector: 0,
            ist: 0,
            type_attr: 0,
            offset_mid: 0,
            offset_high: 0,
            zero: 0,
        }
    }

    pub fn set_handler(&mut self, handler: u64) {
        self.offset_low = (handler & 0xFFFF) as u16;
        self.offset_mid = ((handler >> 16) & 0xFFFF) as u16;
        self.offset_high = ((handler >> 32) & 0xFFFFFFFF) as u32;
        self.selector = 0x08; // Code segment
        self.ist = 0;
        self.type_attr = 0x8E; // Present, Ring 0, Interrupt Gate
        self.zero = 0;
    }
}

/// Puntero a la IDT
#[repr(C, packed)]
pub struct IdtPtr {
    limit: u16,
    base: u64,
}

/// IDT global (256 entradas)
static mut IDT: [IdtEntry; 256] = [IdtEntry::empty(); 256];
static mut IDT_PTR: IdtPtr = IdtPtr { limit: 0, base: 0 };

/// Inicializar la IDT
pub fn init() {
    unsafe {
        // Configurar handlers de excepciones
        IDT[0].set_handler(exception_0 as u64);   // Division by zero
        IDT[1].set_handler(exception_1 as u64);   // Debug
        IDT[2].set_handler(exception_2 as u64);   // NMI
        IDT[3].set_handler(exception_3 as u64);   // Breakpoint
        IDT[4].set_handler(exception_4 as u64);   // Overflow
        IDT[5].set_handler(exception_5 as u64);   // Bound Range
        IDT[6].set_handler(exception_6 as u64);   // Invalid Opcode
        IDT[7].set_handler(exception_7 as u64);   // Device Not Available
        IDT[8].set_handler(exception_8 as u64);   // Double Fault
        IDT[13].set_handler(exception_13 as u64); // General Protection
        IDT[14].set_handler(exception_14 as u64); // Page Fault

        // IRQs (remapeados a 32-47)
        IDT[32].set_handler(irq_0 as u64);  // Timer
        IDT[33].set_handler(irq_1 as u64);  // Keyboard
        IDT[44].set_handler(irq_12 as u64); // Mouse

        // Configurar puntero IDT
        IDT_PTR.limit = (core::mem::size_of::<[IdtEntry; 256]>() - 1) as u16;
        IDT_PTR.base = IDT.as_ptr() as u64;

        // Cargar IDT
        asm!("lidt [{}]", in(reg) &IDT_PTR, options(nostack));
    }

    // Remapear PIC
    remap_pic();

    // Habilitar interrupciones
    unsafe { asm!("sti"); }
}

/// Remapear el PIC (8259)
fn remap_pic() {
    unsafe {
        // ICW1
        outb(0x20, 0x11);
        outb(0xA0, 0x11);
        
        // ICW2 - Offset
        outb(0x21, 0x20); // Master: IRQ 0-7 -> INT 32-39
        outb(0xA1, 0x28); // Slave: IRQ 8-15 -> INT 40-47
        
        // ICW3
        outb(0x21, 0x04);
        outb(0xA1, 0x02);
        
        // ICW4
        outb(0x21, 0x01);
        outb(0xA1, 0x01);
        
        // Máscaras - Habilitar Timer, Keyboard, Mouse
        outb(0x21, 0xF8); // 11111000 - IRQ 0,1,2 habilitados
        outb(0xA1, 0xEF); // 11101111 - IRQ 12 habilitado
    }
}

/// Enviar EOI al PIC
pub fn send_eoi(irq: u8) {
    unsafe {
        if irq >= 8 {
            outb(0xA0, 0x20);
        }
        outb(0x20, 0x20);
    }
}

// Handlers de excepciones
extern "x86-interrupt" fn exception_0(_frame: InterruptFrame) {
    panic!("Division by zero!");
}

extern "x86-interrupt" fn exception_1(_frame: InterruptFrame) {}
extern "x86-interrupt" fn exception_2(_frame: InterruptFrame) {}
extern "x86-interrupt" fn exception_3(_frame: InterruptFrame) {}
extern "x86-interrupt" fn exception_4(_frame: InterruptFrame) {}
extern "x86-interrupt" fn exception_5(_frame: InterruptFrame) {}
extern "x86-interrupt" fn exception_6(_frame: InterruptFrame) {
    panic!("Invalid opcode!");
}
extern "x86-interrupt" fn exception_7(_frame: InterruptFrame) {}
extern "x86-interrupt" fn exception_8(_frame: InterruptFrame, _code: u64) {
    panic!("Double fault!");
}
extern "x86-interrupt" fn exception_13(_frame: InterruptFrame, _code: u64) {
    panic!("General protection fault!");
}
extern "x86-interrupt" fn exception_14(_frame: InterruptFrame, _code: u64) {
    panic!("Page fault!");
}

// Handlers de IRQs
extern "x86-interrupt" fn irq_0(_frame: InterruptFrame) {
    crate::timer::tick();
    send_eoi(0);
}

extern "x86-interrupt" fn irq_1(_frame: InterruptFrame) {
    let scancode = unsafe { inb(0x60) };
    crate::keyboard::handle_scancode(scancode);
    send_eoi(1);
}

extern "x86-interrupt" fn irq_12(_frame: InterruptFrame) {
    let data = unsafe { inb(0x60) };
    crate::mouse::handle_byte(data);
    send_eoi(12);
}

/// Frame de interrupción
#[repr(C)]
pub struct InterruptFrame {
    pub ip: u64,
    pub cs: u64,
    pub flags: u64,
    pub sp: u64,
    pub ss: u64,
}

// I/O
#[inline]
unsafe fn outb(port: u16, value: u8) {
    asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack));
}

#[inline]
unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    asm!("in al, dx", out("al") value, in("dx") port, options(nomem, nostack));
    value
}
