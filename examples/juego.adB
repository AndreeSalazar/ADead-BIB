// ADead-BIB v0.5.0 - Juego
// Ejemplo de logica de juego

fn main() {
    println("=== Juego ===")
    
    // Jugador
    let px = 100
    let py = 100
    let hp = 100
    
    // Enemigo
    let ex = 200
    let ey = 150
    
    // Distancia
    let dx = ex - px
    let dy = ey - py
    let dist = dx * dx + dy * dy
    
    print("Distancia: ")
    println(dist)
}
