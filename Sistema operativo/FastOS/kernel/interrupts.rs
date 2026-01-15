// ============================================================================
// FastOS - Interrupt Handling
// ============================================================================
// Manejo de interrupciones del CPU
// - IDT (Interrupt Descriptor Table)
// - Excepciones
// - IRQs
//
// Author: Eddi Andreé Salazar Matos 🇵🇪
// ============================================================================

use lazy_static::lazy_static;
use spin::Mutex;

/// Estructura de la IDT Entry
#[derive(Clone, Copy)]
#[repr(C, packed)]
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
    /// Entrada vacía
    const fn empty() -> Self {
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

    /// Crear entrada con handler
    fn set_handler(&mut self, handler: u64) {
        self.offset_low = handler as u16;
        self.offset_mid = (handler >> 16) as u16;
        self.offset_high = (handler >> 32) as u32;
        self.selector = 0x08; // Code segment
        self.type_attr = 0x8E; // Present, Ring 0, Interrupt Gate
        self.ist = 0;
        self.zero = 0;
    }
}

/// IDT completa (256 entradas)
#[repr(C, align(16))]
pub struct Idt {
    entries: [IdtEntry; 256],
}

impl Idt {
    const fn new() -> Self {
        Idt {
            entries: [IdtEntry::empty(); 256],
        }
    }

    fn set_handler(&mut self, index: usize, handler: u64) {
        self.entries[index].set_handler(handler);
    }
}

/// IDT Pointer para lidt
#[repr(C, packed)]
struct IdtPointer {
    limit: u16,
    base: u64,
}

lazy_static! {
    static ref IDT: Mutex<Idt> = Mutex::new(Idt::new());
}

/// Inicializar IDT
pub fn init() {
    let mut idt = IDT.lock();
    
    // Excepciones del CPU
    idt.set_handler(0, divide_error as u64);
    idt.set_handler(3, breakpoint as u64);
    idt.set_handler(6, invalid_opcode as u64);
    idt.set_handler(8, double_fault as u64);
    idt.set_handler(13, general_protection as u64);
    idt.set_handler(14, page_fault as u64);
    
    // IRQs (PIC remapeado a 32-47)
    idt.set_handler(32, timer_interrupt as u64);
    idt.set_handler(33, keyboard_interrupt as u64);
    
    // Cargar IDT
    let ptr = IdtPointer {
        limit: (core::mem::size_of::<Idt>() - 1) as u16,
        base: &idt.entries as *const _ as u64,
    };
    
    unsafe {
        core::arch::asm!("lidt [{}]", in(reg) &ptr, options(readonly, nostack, preserves_flags));
    }
    
    // Inicializar PIC
    init_pic();
    
    // Habilitar interrupciones
    unsafe {
        core::arch::asm!("sti", options(nomem, nostack));
    }
}

/// Inicializar PIC 8259
fn init_pic() {
    unsafe {
        // ICW1
        outb(0x20, 0x11);
        outb(0xA0, 0x11);
        
        // ICW2 - Remapear IRQs
        outb(0x21, 0x20); // Master: IRQ 0-7 -> INT 32-39
        outb(0xA1, 0x28); // Slave: IRQ 8-15 -> INT 40-47
        
        // ICW3
        outb(0x21, 0x04);
        outb(0xA1, 0x02);
        
        // ICW4
        outb(0x21, 0x01);
        outb(0xA1, 0x01);
        
        // Máscaras - habilitar teclado (IRQ1) y timer (IRQ0)
        outb(0x21, 0xFC); // 11111100 - IRQ0 y IRQ1 habilitados
        outb(0xA1, 0xFF); // Deshabilitar slave
    }
}

/// Escribir a puerto I/O
#[inline]
unsafe fn outb(port: u16, value: u8) {
    core::arch::asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
}

/// Leer de puerto I/O
#[inline]
pub unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    core::arch::asm!("in al, dx", out("al") value, in("dx") port, options(nomem, nostack, preserves_flags));
    value
}

/// Enviar EOI al PIC
#[inline]
pub fn send_eoi(irq: u8) {
    unsafe {
        if irq >= 8 {
            outb(0xA0, 0x20);
        }
        outb(0x20, 0x20);
    }
}

// ============================================================================
// Exception Handlers
// ============================================================================

extern "x86-interrupt" fn divide_error(_frame: InterruptStackFrame) {
    crate::println!("EXCEPTION: Divide Error");
    loop {}
}

extern "x86-interrupt" fn breakpoint(_frame: InterruptStackFrame) {
    crate::println!("EXCEPTION: Breakpoint");
}

extern "x86-interrupt" fn invalid_opcode(_frame: InterruptStackFrame) {
    crate::println!("EXCEPTION: Invalid Opcode");
    loop {}
}

extern "x86-interrupt" fn double_fault(_frame: InterruptStackFrame, _error: u64) -> ! {
    crate::println!("EXCEPTION: Double Fault");
    loop {}
}

extern "x86-interrupt" fn general_protection(_frame: InterruptStackFrame, error: u64) {
    crate::println!("EXCEPTION: General Protection Fault (error: {})", error);
    loop {}
}

extern "x86-interrupt" fn page_fault(_frame: InterruptStackFrame, error: u64) {
    crate::println!("EXCEPTION: Page Fault (error: {})", error);
    loop {}
}

// ============================================================================
// IRQ Handlers
// ============================================================================

extern "x86-interrupt" fn timer_interrupt(_frame: InterruptStackFrame) {
    // Timer tick (silencioso)
    send_eoi(0);
}

extern "x86-interrupt" fn keyboard_interrupt(_frame: InterruptStackFrame) {
    let scancode = unsafe { inb(0x60) };
    crate::keyboard::handle_scancode(scancode);
    send_eoi(1);
}

/// Stack frame de interrupción
#[repr(C)]
pub struct InterruptStackFrame {
    pub instruction_pointer: u64,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segment: u64,
}
