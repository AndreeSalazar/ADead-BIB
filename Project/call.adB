// ============================================================================
// call.adB — BINARIO DE COMPORTAMIENTO (OOP PURO)
// ============================================================================
// Este archivo contiene toda la lógica de alto nivel y OOP.
// NO conoce el entrypoint, NO emite bytes peligrosos.
//
// Rol:
//   - Definir objetos (structs)
//   - Definir métodos (impl)
//   - Definir lógica de negocio
//   - Exportar funciones públicas
// ============================================================================

#![exports(run)]

// ============================================================================
// Estructuras de Datos
// ============================================================================

struct Player {
    x: i32,
    y: i32,
    health: u8
}

impl Player {
    fn new() {
        return Player { x: 0, y: 0, health: 100 }
    }
    
    fn move(self, dx, dy) {
        self.x += dx
        self.y += dy
    }
    
    fn take_damage(self, amount) {
        if amount >= self.health {
            self.health = 0
        } else {
            self.health -= amount
        }
    }
    
    fn heal(self, amount) {
        self.health += amount
        if self.health > 100 {
            self.health = 100
        }
    }
    
    fn is_alive(self) {
        return self.health > 0
    }
}

struct Game {
    player: Player,
    running: bool,
    score: i32
}

impl Game {
    fn new() {
        return Game {
            player: Player::new(),
            running: false,
            score: 0
        }
    }
    
    fn start(self) {
        self.running = true
        println("=== Game Started ===")
    }
    
    fn update(self) {
        if self.player.is_alive() {
            self.player.move(1, 0)
            self.score += 10
        }
    }
    
    fn stop(self) {
        self.running = false
        println("=== Game Stopped ===")
        print("Final Score: ")
        println(self.score)
    }
}

// ============================================================================
// Función Pública Exportada
// ============================================================================
// Esta es la función que main.adB llama.
// Encapsula toda la lógica del programa.

pub fn run() {
    println("ADead-BIB Project - Demo")
    println("========================")
    
    // Crear instancia del juego
    let game = Game::new()
    
    // Iniciar
    game.start()
    
    // Loop principal (100 iteraciones)
    for i in 0..100 {
        game.update()
    }
    
    // Finalizar
    game.stop()
    
    println("========================")
    println("Demo completed!")
}
