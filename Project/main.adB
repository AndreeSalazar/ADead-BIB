// ============================================================================
// main.adB — BINARIO BASE (ROOT)
// ============================================================================
// Este archivo define el punto de entrada real del programa.
// NO contiene lógica compleja, solo inicialización y control de flujo.
// 
// Rol:
//   - Entrypoint del programa
//   - Inicialización de CPU/GPU
//   - Llamada a la lógica principal (call.adB)
//   - Limpieza y shutdown
// ============================================================================

#![imports(call: run, run_gpu_demo, run_cpu_demo)]
#![imports(core: init, shutdown)]
#![imports(cpu: *)]
#![imports(gpu: *)]

// ============================================================================
// PUNTO DE ENTRADA PRINCIPAL
// ============================================================================

fn _start() {
    // ========================================
    // Fase 1: Inicialización del sistema
    // ========================================
    core::init()
    
    // ========================================
    // Fase 2: Ejecutar demos
    // ========================================
    
    // Demo principal: OOP Puro
    call::run()
    
    // Demo GPU: Compute con opcodes directos
    call::run_gpu_demo()
    
    // Demo CPU: Instrucciones x86-64 directas
    call::run_cpu_demo()
    
    // ========================================
    // Fase 3: Limpieza y salida
    // ========================================
    core::shutdown()
}

// ============================================================================
// FUNCIONES ALTERNATIVAS DE ENTRADA
// ============================================================================

// Entrada para modo solo OOP
fn main_oop_only() {
    core::init()
    call::run()
    core::shutdown()
}

// Entrada para modo GPU compute
fn main_gpu_only() {
    core::init()
    call::run_gpu_demo()
    core::shutdown()
}

// Entrada para modo CPU optimizado
fn main_cpu_only() {
    core::init()
    call::run_cpu_demo()
    core::shutdown()
}
