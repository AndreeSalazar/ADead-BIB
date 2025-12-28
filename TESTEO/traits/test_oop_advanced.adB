// Test OOP Avanzado v1.6.0

impl Vector2D {
    fn create(x: i32, y: i32) -> i32 {
        return x + y
    }
    
    // Método estático - operación sin instancia
    fn add(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
        return x1 + x2 + y1 + y2
    }
    
    // Método de cálculo
    fn magnitude_squared(x: i32, y: i32) -> i32 {
        return x * x + y * y
    }
}

// ============================================
// PATRÓN 2: Trait + impl (Polimorfismo estático)
// ============================================

trait Drawable {
    fn draw(id: i32) -> i32;
}

impl Drawable for Rectangle {
    fn draw(id: i32) -> i32 {
        // Simular dibujar un rectángulo
        return id * 10
    }
}

impl Drawable for Circle {
    fn draw(id: i32) -> i32 {
        // Simular dibujar un círculo
        return id * 20
    }
}

// ============================================
// PATRÓN 3: Composición (preferido sobre herencia)
// ============================================

impl GameObject {
    fn create(x: i32, y: i32, w: i32, h: i32) -> i32 {
        // Crear objeto de juego con posición y tamaño
        return x + y + w + h
    }
    
    fn update(delta: i32) -> i32 {
        // Actualizar estado del objeto
        return delta * 2
    }
    
    fn render(id: i32) -> i32 {
        // Renderizar el objeto
        return id
    }
}

// ============================================
// MAIN - Demostración de uso
// ============================================

fn main() {
    println("+========================================+")
    println("|   ADead-BIB v1.6.0 - OOP Avanzado     |")
    println("+========================================+")
    println("")
    
    // Patrón 1: Struct + impl
    println("[1] Vector2D (Struct + impl)")
    print("    Vector2D::new(3, 4) = ")
    println(Vector2D::new(3, 4))
    
    print("    Vector2D::add(1,2, 3,4) = ")
    println(Vector2D::add(1, 2, 3, 4))
    
    print("    Vector2D::magnitude_squared(3, 4) = ")
    println(Vector2D::magnitude_squared(3, 4))
    
    println("")
    
    // Patrón 2: Traits (Polimorfismo)
    println("[2] Drawable Trait (Polimorfismo)")
    print("    Rectangle::Drawable::draw(5) = ")
    println(Rectangle::Drawable::draw(5))
    
    print("    Circle::Drawable::draw(5) = ")
    println(Circle::Drawable::draw(5))
    
    println("")
    
    // Patrón 3: Composición
    println("[3] GameObject (Composición)")
    print("    GameObject::create(10, 20, 100, 50) = ")
    println(GameObject::create(10, 20, 100, 50))
    
    print("    GameObject::update(16) = ")
    println(GameObject::update(16))
    
    println("")
    println("+========================================+")
    println("|     OK - OOP Avanzado funciona!       |")
    println("+========================================+")
}
