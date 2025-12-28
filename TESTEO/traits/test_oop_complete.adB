// Test OOP Completo v1.6.0

impl Vector2D {
    fn create(x: i32, y: i32) -> i32 {
        return x + y
    }
    
    fn add(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
        return x1 + x2 + y1 + y2
    }
    
    fn magnitude_squared(x: i32, y: i32) -> i32 {
        return x * x + y * y
    }
}

trait Drawable {
    fn draw(id: i32) -> i32;
}

impl Drawable for Rectangle {
    fn draw(id: i32) -> i32 {
        return id * 10
    }
}

impl Drawable for Circle {
    fn draw(id: i32) -> i32 {
        return id * 20
    }
}

impl GameObject {
    fn create(x: i32, y: i32, w: i32, h: i32) -> i32 {
        return x + y + w + h
    }
    
    fn update(delta: i32) -> i32 {
        return delta * 2
    }
    
    fn render(id: i32) -> i32 {
        return id
    }
}

fn main() {
    println("+========================================+")
    println("|   ADead-BIB v1.6.0 - OOP Completo     |")
    println("+========================================+")
    println("")
    
    println("[1] Vector2D (impl)")
    print("    Vector2D::create(3, 4) = ")
    println(Vector2D::create(3, 4))
    
    print("    Vector2D::add(1,2, 3,4) = ")
    println(Vector2D::add(1, 2, 3, 4))
    
    print("    Vector2D::magnitude_squared(3, 4) = ")
    println(Vector2D::magnitude_squared(3, 4))
    
    println("")
    
    println("[2] Drawable Trait")
    print("    Rectangle::Drawable::draw(5) = ")
    println(Rectangle::Drawable::draw(5))
    
    print("    Circle::Drawable::draw(5) = ")
    println(Circle::Drawable::draw(5))
    
    println("")
    
    println("[3] GameObject")
    print("    GameObject::create(10, 20, 100, 50) = ")
    println(GameObject::create(10, 20, 100, 50))
    
    print("    GameObject::update(16) = ")
    println(GameObject::update(16))
    
    println("")
    println("+========================================+")
    println("|     OK - OOP Completo funciona!       |")
    println("+========================================+")
}
