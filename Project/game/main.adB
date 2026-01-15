// ============================================================================
// ADead-BIB - Juego Simple Funcional
// ============================================================================
// Este juego COMPILA DE VERDAD con el backend actual
// Ejecutar: adeadc run main.adB
// ============================================================================

fn main() {
    println("========================================")
    println("     ADead-BIB - Aventura Simple")
    println("     Binary Is Binary")
    println("========================================")
    println("")
    
    // Estado del jugador
    let player_x = 0
    let player_y = 0
    let player_hp = 100
    let player_gold = 0
    let player_level = 1
    
    // Estado del enemigo
    let enemy_x = 10
    let enemy_y = 5
    let enemy_hp = 50
    
    // Mostrar estado inicial
    println("=== Estado Inicial ===")
    print("Jugador HP: ")
    println(player_hp)
    print("Jugador Nivel: ")
    println(player_level)
    print("Oro: ")
    println(player_gold)
    println("")
    
    // Simular movimiento
    println("=== Moviendo al jugador ===")
    let player_x = player_x + 5
    let player_y = player_y + 3
    print("Nueva posicion: (")
    print(player_x)
    print(", ")
    print(player_y)
    println(")")
    println("")
    
    // Calcular distancia al enemigo
    println("=== Calculando distancia al enemigo ===")
    let dx = enemy_x - player_x
    let dy = enemy_y - player_y
    let dist_squared = dx * dx + dy * dy
    print("Distancia^2: ")
    println(dist_squared)
    println("")
    
    // Combate simple
    println("=== Combate! ===")
    let damage = 25
    let enemy_hp = enemy_hp - damage
    print("Atacas al enemigo por ")
    print(damage)
    println(" de dano!")
    print("Enemigo HP restante: ")
    println(enemy_hp)
    println("")
    
    // Segundo ataque
    let enemy_hp = enemy_hp - damage
    print("Atacas de nuevo por ")
    print(damage)
    println(" de dano!")
    print("Enemigo HP restante: ")
    println(enemy_hp)
    println("")
    
    // Victoria
    println("=== Victoria! ===")
    let gold_reward = 50
    let player_gold = player_gold + gold_reward
    print("Ganaste ")
    print(gold_reward)
    println(" de oro!")
    print("Oro total: ")
    println(player_gold)
    println("")
    
    // Subir de nivel
    let exp = 100
    let player_level = player_level + 1
    print("Subiste al nivel ")
    println(player_level)
    println("")
    
    // Estado final
    println("=== Estado Final ===")
    print("Jugador HP: ")
    println(player_hp)
    print("Jugador Nivel: ")
    println(player_level)
    print("Oro: ")
    println(player_gold)
    println("")
    
    println("========================================")
    println("     Gracias por jugar!")
    println("     ADead-BIB v2.0")
    println("========================================")
}
