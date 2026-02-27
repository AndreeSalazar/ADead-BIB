// ============================================================
// ADead-BIB — MODO 1: Kernel Driver (64-bit Long Mode)
// ============================================================
// Driver de kernel con handlers de interrupción, PIC y CPUID.
//
// Compilar: adeadc raw MODE1_kernel_driver.adB -o driver.bin
//
// Atributos de función Modo 1:
//   @interrupt — auto push/pop + iretq
//   @exception — interrupt con error code en stack
//   @naked     — sin prologue/epilogue (control total)
// ============================================================

// ---- Constantes de hardware (PIC, Keyboard) ----
// PIC1_CMD  = 0x20   PIC1_DATA = 0x21
// PIC2_CMD  = 0xA0   PIC2_DATA = 0xA1
// KB_DATA   = 0x60   KB_STATUS = 0x64

// ---- Handler de teclado (IRQ1) ----
@interrupt
fn keyboard_handler() {
    // Leer scancode del puerto KB_DATA
    let scancode = port_in(0x60)

    // EOI al PIC1
    port_out(0x20, 0x20)
}

// ---- Handler de timer (IRQ0) ----
@interrupt
fn timer_handler() {
    // EOI al PIC1
    port_out(0x20, 0x20)
}

// ---- Handler de excepción (con error code) ----
@exception
fn page_fault_handler() {
    // Error code está en el stack automáticamente
    hlt
}

// ---- Inicializar PIC (8259A) ----
@naked
fn init_pic() {
    // ICW1: inicializar cascada
    port_out(0x20, 0x11)
    port_out(0xA0, 0x11)

    // ICW2: vector base (IRQ0→0x20, IRQ8→0x28)
    port_out(0x21, 0x20)
    port_out(0xA1, 0x28)

    // ICW3: cascada en IRQ2
    port_out(0x21, 0x04)
    port_out(0xA1, 0x02)

    // ICW4: modo 8086
    port_out(0x21, 0x01)
    port_out(0xA1, 0x01)

    // Máscara: habilitar IRQ0 (timer) e IRQ1 (keyboard)
    port_out(0x21, 0xFC)
    port_out(0xA1, 0xFF)
}

// ---- Leer identificador del CPU ----
@naked
fn get_cpu_vendor() {
    // CPUID con EAX=0: vendor string en EBX:EDX:ECX
    cpuid
}

// ---- Habilitar paginación (CR0 bit 31) ----
@naked
fn enable_paging(pml4_addr: long) {
    // Cargar dirección de PML4 en CR3
    raw { 0x0F, 0x22, 0xDF }  // mov cr3, rdi

    // Leer CR0, setear bit 31, escribir CR0
    raw { 0x0F, 0x20, 0xC0 }              // mov rax, cr0
    raw { 0x48, 0x0D, 0x00, 0x00, 0x00, 0x80 }  // or rax, 0x80000000
    raw { 0x0F, 0x22, 0xC0 }              // mov cr0, rax
}

// ---- Entry point del driver ----
fn driver_init() {
    // Mask interrupts durante el setup
    cli

    // Inicializar controlador de interrupciones
    init_pic()

    // Detectar CPU
    get_cpu_vendor()

    // Habilitar interrupts
    sti

    // Loop: el kernel maneja el resto via IRQs
    hlt
}
