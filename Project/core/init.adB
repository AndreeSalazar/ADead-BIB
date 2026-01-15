// ============================================================================
// core/init.adB — Inicialización del Sistema
// ============================================================================
// Funciones de inicialización para CPU y GPU.
// ============================================================================

#![exports(init)]

pub fn init() {
    // Inicializar subsistema CPU
    cpu_init()
    
    // Inicializar subsistema GPU (si disponible)
    gpu_init()
    
    println("[CORE] System initialized")
}

fn cpu_init() {
    // Setup de registros base
    // En el futuro: cpu::setup_stack(), cpu::setup_heap()
}

fn gpu_init() {
    // Detectar GPU disponible
    // En el futuro: gpu::detect(), gpu::init()
}
