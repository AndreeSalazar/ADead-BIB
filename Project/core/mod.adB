// ============================================================================
// core/mod.adB — Módulo Core: Intrínsecos del Sistema
// ============================================================================
// Funciones fundamentales del sistema que el compilador conoce.
// Estas funciones emiten bytes específicos sin runtime.
// ============================================================================

#![exports(
    init, shutdown,
    // I/O
    print, println, input,
    // Memoria
    alloc, free, copy, zero,
    // Sistema
    exit, panic,
    // Utilidades
    len, typeof, sizeof
)]

// ============================================================================
// INICIALIZACIÓN Y SHUTDOWN
// ============================================================================

pub fn init() {
    // Inicializar subsistemas
    cpu_init()
    gpu_init()
    heap_init()
}

pub fn shutdown() {
    // Limpiar subsistemas
    heap_cleanup()
    gpu_shutdown()
    cpu_shutdown()
}

fn cpu_init() {
    // Setup de stack y registros base
    // El compilador emite el prólogo estándar
}

fn cpu_shutdown() {
    // Cleanup de CPU
}

fn gpu_init() {
    // Detectar y preparar GPU si disponible
}

fn gpu_shutdown() {
    // Liberar recursos GPU
}

fn heap_init() {
    // Inicializar heap para allocaciones dinámicas
}

fn heap_cleanup() {
    // Liberar toda la memoria del heap
}

// ============================================================================
// I/O (Intrínsecos del Compilador)
// ============================================================================

// print — Imprimir sin newline
// El compilador emite syscall write directamente
pub fn print(msg: string) {
    // Intrínseco: el compilador genera los bytes para syscall write
    // Windows: WriteConsoleA via kernel32
    // Linux: syscall write (rax=1, rdi=1, rsi=msg, rdx=len)
}

// println — Imprimir con newline
pub fn println(msg: string) {
    print(msg)
    print("\n")
}

// input — Leer línea de entrada
// Retorna string leída del usuario
pub fn input() -> string {
    // Intrínseco: el compilador genera los bytes para syscall read
    // Windows: ReadConsoleA via kernel32
    // Linux: syscall read (rax=0, rdi=0, rsi=buffer, rdx=size)
}

// ============================================================================
// MEMORIA
// ============================================================================

// alloc — Reservar memoria en heap
// Retorna puntero a memoria reservada
pub fn alloc(size: u64) -> ptr<u8> {
    // Intrínseco: el compilador genera llamada a VirtualAlloc/mmap
}

// free — Liberar memoria
pub fn free(ptr: ptr<u8>) {
    // Intrínseco: el compilador genera llamada a VirtualFree/munmap
}

// copy — Copiar memoria
pub fn copy(dest: ptr<u8>, src: ptr<u8>, size: u64) {
    // Intrínseco: el compilador genera rep movsb o similar
}

// zero — Llenar memoria con ceros
pub fn zero(ptr: ptr<u8>, size: u64) {
    // Intrínseco: el compilador genera rep stosb
}

// ============================================================================
// SISTEMA
// ============================================================================

// exit — Terminar programa con código de salida
pub fn exit(code: i32) {
    // Intrínseco: el compilador genera syscall exit
    // Windows: ExitProcess via kernel32
    // Linux: syscall exit (rax=60, rdi=code)
}

// panic — Error fatal con mensaje
pub fn panic(msg: string) {
    println("[PANIC] ")
    println(msg)
    exit(1)
}

// ============================================================================
// UTILIDADES
// ============================================================================

// len — Longitud de string o array
pub fn len(s: string) -> u64 {
    // Intrínseco: el compilador conoce la longitud en tiempo de compilación
    // o la calcula del layout de memoria
}

// typeof — Obtener tipo de variable (para debug)
pub fn typeof(x: any) -> string {
    // Intrínseco: el compilador retorna el nombre del tipo
}

// sizeof — Tamaño en bytes de un tipo
pub fn sizeof(T: type) -> u64 {
    // Intrínseco: el compilador conoce el tamaño del tipo
}
