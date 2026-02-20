// ============================================================
// ADead-BIB — MODO 1: Kernel Driver (64-bit Long Mode)
// ============================================================
// Driver de ejemplo para kernel en modo 64-bit.
// Demuestra acceso a hardware, interrupciones y I/O.
//
// Compilar: adeadc raw MODE1_kernel_driver.adB -o driver.bin
//
// Características:
//   - Funciones @naked (sin prologue/epilogue)
//   - Funciones @interrupt (para IDT)
//   - Acceso a puertos I/O
//   - Control de interrupciones
//   - Registros de control (CR0, CR3, CR4)
// ============================================================

// ---- Constantes de hardware ----
// PIC (Programmable Interrupt Controller)
// PIC1_CMD  = 0x20
// PIC1_DATA = 0x21
// PIC2_CMD  = 0xA0
// PIC2_DATA = 0xA1

// Keyboard controller
// KB_DATA = 0x60
// KB_STATUS = 0x64

// ---- Handler de teclado (IRQ1) ----
@interrupt
fn keyboard_handler() {
    // Leer scancode del puerto 0x60
    let scancode = port_in(0x60)
    
    // Enviar EOI (End Of Interrupt) al PIC
    port_out(0x20, 0x20)
}

// ---- Handler de timer (IRQ0) ----
@interrupt
fn timer_handler() {
    // Incrementar tick counter (en memoria)
    // mem_write(0x1000, mem_read(0x1000) + 1)
    
    // Enviar EOI al PIC
    port_out(0x20, 0x20)
}

// ---- Función para inicializar PIC ----
@naked
fn init_pic() {
    // ICW1: inicializar
    port_out(0x20, 0x11)
    port_out(0xA0, 0x11)
    
    // ICW2: vector offset (IRQ0 = 0x20, IRQ8 = 0x28)
    port_out(0x21, 0x20)
    port_out(0xA1, 0x28)
    
    // ICW3: cascading
    port_out(0x21, 0x04)
    port_out(0xA1, 0x02)
    
    // ICW4: 8086 mode
    port_out(0x21, 0x01)
    port_out(0xA1, 0x01)
    
    // Mask: habilitar solo IRQ0 (timer) e IRQ1 (keyboard)
    port_out(0x21, 0xFC)  // 11111100 = IRQ0 e IRQ1 habilitados
    port_out(0xA1, 0xFF)  // Deshabilitar todos en PIC2
}

// ---- Función para leer CPUID ----
@naked
fn get_cpu_vendor() {
    // CPUID con EAX=0 retorna vendor string en EBX:EDX:ECX
    cpuid
    // Resultado en RAX (EAX tiene max CPUID level)
}

// ---- Función para habilitar paginación ----
@naked
fn enable_paging(pml4_addr: long) {
    // Cargar PML4 en CR3
    // mov cr3, rdi (primer argumento en RDI en Linux ABI)
    raw { 0x0F, 0x22, 0xDF }  // mov cr3, rdi
    
    // Habilitar paginación en CR0 (bit 31)
    // mov rax, cr0
    raw { 0x0F, 0x20, 0xC0 }
    // or rax, 0x80000000
    raw { 0x48, 0x0D, 0x00, 0x00, 0x00, 0x80 }
    // mov cr0, rax
    raw { 0x0F, 0x22, 0xC0 }
}

// ---- Entry point del driver ----
fn driver_init() {
    // Deshabilitar interrupciones durante setup
    cli
    
    // Inicializar PIC
    init_pic()
    
    // Obtener info de CPU
    get_cpu_vendor()
    
    // Habilitar interrupciones
    sti
    
    // Loop infinito (el kernel maneja el resto)
    hlt
}
