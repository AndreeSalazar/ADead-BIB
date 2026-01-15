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

#![imports(call: run)]

fn _start() {
    // ========================================
    // Fase 1: Inicialización del sistema
    // ========================================
    core::init()
    
    // ========================================
    // Fase 2: Ejecutar lógica principal
    // ========================================
    // La lógica OOP vive en call.adB
    // main.adB solo orquesta, no implementa
    call::run()
    
    // ========================================
    // Fase 3: Limpieza y salida
    // ========================================
    core::shutdown()
}
