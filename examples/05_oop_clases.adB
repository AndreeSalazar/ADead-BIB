// ============================================
// ADead-BIB - Ejemplo 05: OOP - Clases
// Programación Orientada a Objetos
// ============================================

// --- STRUCT ESTILO RUST ---
struct Punto {
    x: i32,
    y: i32,
}

struct Rectangulo {
    ancho: i32,
    alto: i32,
}

// Implementación de métodos (Rust-style)
impl Punto {
    fn new(x: i32, y: i32) -> Punto {
        return Punto { x: x, y: y }
    }
    
    fn mover(&mut self, dx: i32, dy: i32) {
        self.x = self.x + dx
        self.y = self.y + dy
    }
    
    fn distancia(&self) -> i32 {
        return self.x + self.y
    }
}

impl Rectangulo {
    fn area(&self) -> i32 {
        return self.ancho * self.alto
    }
    
    fn perimetro(&self) -> i32 {
        return 2 * (self.ancho + self.alto)
    }
}

// --- CLASS ESTILO PYTHON/C++ ---
class Persona:
    nombre = ""
    edad = 0
    
    def __init__(self, nombre, edad):
        self.nombre = nombre
        self.edad = edad
    
    def saludar(self):
        print("Hola, soy")
        print(self.nombre)
    
    def cumpleanios(self):
        self.edad = self.edad + 1
        print("Feliz cumpleaños!")

class Estudiante extends Persona:
    carrera = ""
    
    def __init__(self, nombre, edad, carrera):
        super().__init__(nombre, edad)
        self.carrera = carrera
    
    override def saludar(self):
        print("Hola, soy estudiante de")
        print(self.carrera)

// --- FUNCIÓN PRINCIPAL ---
fn main() {
    print("=== OOP en ADead-BIB ===")
    
    // Usando structs
    let p = Punto { x: 10, y: 20 }
    print("Punto creado")
    
    let rect = Rectangulo { ancho: 5, alto: 3 }
    print("Rectangulo creado")
    
    print("Area del rectangulo:")
    let area = rect.area()
    print(area)
}
