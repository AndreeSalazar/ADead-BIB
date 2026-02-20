// ============================================================
// ADead-BIB — MODO 2: OOP con Clases (Sintaxis Python/C++)
// ============================================================
// Demostración de programación orientada a objetos.
//
// Compilar: adeadc MODE2_oop_classes.adB -o oop.exe
//
// Características:
//   - Clases con campos y métodos
//   - Constructor (__init__)
//   - Herencia (class Child(Parent))
//   - Polimorfismo (virtual methods)
//   - Interfaces/Traits
// ============================================================

// ---- Clase básica ----
class Point:
    x: int
    y: int
    
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y
    
    def distance_from_origin(self) -> int:
        return self.x * self.x + self.y * self.y
    
    def move(self, dx: int, dy: int):
        self.x = self.x + dx
        self.y = self.y + dy
    
    def print_coords(self):
        printf("(")
        printf(self.x)
        printf(", ")
        printf(self.y)
        printf(")")

// ---- Herencia ----
class Point3D(Point):
    z: int
    
    def __init__(self, x: int, y: int, z: int):
        self.x = x
        self.y = y
        self.z = z
    
    def distance_from_origin(self) -> int:
        return self.x * self.x + self.y * self.y + self.z * self.z

// ---- Interface/Trait ----
interface Drawable:
    def draw(self)
    def get_area(self) -> int

// ---- Clase que implementa interface ----
class Rectangle:
    width: int
    height: int
    
    def __init__(self, w: int, h: int):
        self.width = w
        self.height = h
    
    def draw(self):
        printf("Rectangle ")
        printf(self.width)
        printf("x")
        printf(self.height)
        printf("\n")
    
    def get_area(self) -> int:
        return self.width * self.height

class Circle:
    radius: int
    
    def __init__(self, r: int):
        self.radius = r
    
    def draw(self):
        printf("Circle r=")
        printf(self.radius)
        printf("\n")
    
    def get_area(self) -> int:
        return 3 * self.radius * self.radius

// ---- Main ----
int main() {
    printf("=== ADead-BIB OOP Demo ===\n\n")
    
    // Crear objetos
    printf("Point p1 = Point(3, 4)\n")
    // let p1 = Point(3, 4)
    // p1.print_coords()
    // printf("\n")
    
    printf("Rectangle rect = Rectangle(10, 5)\n")
    // let rect = Rectangle(10, 5)
    // rect.draw()
    // printf("Area: ")
    // printf(rect.get_area())
    // printf("\n")
    
    printf("Circle circ = Circle(7)\n")
    // let circ = Circle(7)
    // circ.draw()
    // printf("Area: ")
    // printf(circ.get_area())
    // printf("\n")
    
    return 0
}
