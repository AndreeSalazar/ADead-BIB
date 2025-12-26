// ============================================
// ADead-BIB - Ejemplo 08: Game Engine
// Sistema completo de juego con OOP
// ============================================

// --- COMPONENTES BASE ---
struct Transform {
    x: f64,
    y: f64,
    rotation: f64,
    scale: f64,
}

struct Velocity {
    vx: f64,
    vy: f64,
}

// --- ENTIDAD BASE DEL JUEGO ---
class GameObject:
    id = 0
    name = ""
    active = true
    transform_x = 0
    transform_y = 0
    
    virtual def update(self, delta: f64):
        pass
    
    virtual def render(self):
        pass
    
    def set_position(self, x: i32, y: i32):
        self.transform_x = x
        self.transform_y = y

// --- JUGADOR ---
class Player extends GameObject:
    health = 100
    max_health = 100
    speed = 5
    score = 0
    
    override def update(self, delta: f64):
        // Lógica de movimiento
        print("Player update")
    
    override def render(self):
        print("[@] Player")
    
    def take_damage(self, amount: i32):
        self.health = self.health - amount
        if self.health < 0:
            self.health = 0
        print("Player HP:")
        print(self.health)
    
    def heal(self, amount: i32):
        self.health = self.health + amount
        if self.health > self.max_health:
            self.health = self.max_health

// --- ENEMIGO ---
class Enemy extends GameObject:
    health = 50
    damage = 10
    ai_state = "idle"
    
    override def update(self, delta: f64):
        // IA del enemigo
        if self.ai_state == "idle":
            print("Enemy idle")
        elif self.ai_state == "chase":
            print("Enemy chasing")
        elif self.ai_state == "attack":
            print("Enemy attacking")
    
    override def render(self):
        print("[X] Enemy")
    
    def attack(self, target: Player):
        target.take_damage(self.damage)

// --- SISTEMA DE COLISIONES ---
fn check_collision(a_x: i32, a_y: i32, b_x: i32, b_y: i32, radius: i32) -> bool {
    let dx = a_x - b_x
    let dy = a_y - b_y
    let dist = dx * dx + dy * dy
    return dist < radius * radius
}

// --- GAME LOOP ---
fn main() {
    print("=== ADead-BIB Game Engine ===")
    
    // Inicializar
    let player_x = 100
    let player_y = 100
    let player_hp = 100
    
    let enemy_x = 200
    let enemy_y = 150
    let enemy_hp = 50
    
    // Simular game loop
    let frame = 0
    while frame < 5 {
        print("--- Frame ---")
        print(frame)
        
        // Update
        print("Updating...")
        
        // Render
        print("Player @")
        print(player_x)
        print(player_y)
        
        print("Enemy X")
        print(enemy_x)
        print(enemy_y)
        
        frame = frame + 1
    }
    
    print("Game Over!")
}
