// ============================================================================
// ADead-BIB Game Logic - Flappy Bird
// ============================================================================
// Lógica crítica del juego en ADead-BIB para CPU/GPU
// Este archivo define la física y colisiones branchless
//
// Author: Eddi Andreé Salazar Matos 🇵🇪
// ============================================================================

// Constantes del juego
const GRAVITY = 800
const FLAP_FORCE = 400
const PIPE_SPEED = 150
const GAP_SIZE = 180
const MIN_PIPE_DISTANCE = 300

// ============================================================================
// FÍSICA BRANCHLESS (CPU Optimizado)
// ============================================================================

// Aplicar gravedad sin branches
fn apply_gravity(velocity: i32, delta_ms: i32) {
    let gravity_effect = GRAVITY * delta_ms / 1000
    let new_vel = velocity + gravity_effect
    println(new_vel)
}

// Actualizar posición
fn update_position(y: i32, velocity: i32, delta_ms: i32) {
    let new_y = y + velocity * delta_ms / 1000
    println(new_y)
}

// ============================================================================
// COLISIONES BRANCHLESS (GPU Optimizado)
// ============================================================================

// Verificar colisión AABB sin branches
fn check_collision_aabb(ax: i32, ay: i32, aw: i32, ah: i32, bx: i32, by: i32, bw: i32, bh: i32) {
    // Calcular bordes
    let a_right = ax + aw
    let a_bottom = ay + ah
    let b_right = bx + bw
    let b_bottom = by + bh
    
    // Verificar overlap horizontal
    let overlap_x = a_right - bx
    let overlap_x2 = b_right - ax
    
    // Verificar overlap vertical
    let overlap_y = a_bottom - by
    let overlap_y2 = b_bottom - ay
    
    print("  Overlap X: ")
    println(overlap_x)
    print("  Overlap Y: ")
    println(overlap_y)
}

// ============================================================================
// SPAWN DE PIPES (Pseudo-random)
// ============================================================================

// Generar posición Y del gap usando frame como seed
fn generate_gap_y(frame: i32, screen_height: i32) {
    let seed = frame * 17 + 31
    let max_gap = screen_height - GAP_SIZE - 100
    let gap_y = 80 + seed % max_gap
    
    print("  Gap Y generado: ")
    println(gap_y)
}

// ============================================================================
// MAIN - Tests de la lógica del juego
// ============================================================================

fn main() {
    println("========================================")
    println("   ADead-BIB Game Logic Module")
    println("   Branchless Physics & Collision")
    println("========================================")
    println("")
    
    // Test de física
    println("Test 1: Fisica branchless")
    print("  Gravedad aplicada (vel=0, dt=16ms): ")
    apply_gravity(0, 16)
    println("")
    
    // Test de posición
    println("Test 2: Actualizar posicion")
    print("  Nueva posicion (y=300, vel=100, dt=16ms): ")
    update_position(300, 100, 16)
    println("")
    
    // Test de colisión
    println("Test 3: Colision AABB")
    println("  Bird: (100,100) 34x24")
    println("  Pipe: (120,110) 80x200")
    check_collision_aabb(100, 100, 34, 24, 120, 110, 80, 200)
    println("")
    
    // Test de generación de gap
    println("Test 4: Generar gap Y")
    generate_gap_y(100, 600)
    println("")
    
    // Constantes del juego
    println("========================================")
    println("   Constantes del Juego")
    println("========================================")
    print("  GRAVITY: ")
    println(GRAVITY)
    print("  FLAP_FORCE: ")
    println(FLAP_FORCE)
    print("  PIPE_SPEED: ")
    println(PIPE_SPEED)
    print("  GAP_SIZE: ")
    println(GAP_SIZE)
    println("")
    
    println("========================================")
    println("   Modulo listo para integracion")
    println("   con Rust + Vulkan")
    println("========================================")
}
