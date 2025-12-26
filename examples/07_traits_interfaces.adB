// ============================================
// ADead-BIB - Ejemplo 07: Traits e Interfaces
// Abstracción y contratos de comportamiento
// ============================================

// --- INTERFACE ESTILO PYTHON/JAVA ---
interface Dibujable:
    def dibujar(self)
    def obtener_color(self) -> str

interface Movible:
    def mover(self, x: i32, y: i32)
    def obtener_posicion(self) -> tuple

// --- TRAIT ESTILO RUST ---
trait Serializable {
    fn to_string(&self) -> String;
    fn from_string(s: &str) -> Self;
}

trait Clonable {
    fn clone(&self) -> Self;
}

trait Comparable {
    fn equals(&self, other: &Self) -> bool;
    fn compare(&self, other: &Self) -> i32;
}

// --- IMPLEMENTACIÓN DE TRAITS ---
struct Vector2D {
    x: f64,
    y: f64,
}

impl Serializable for Vector2D {
    fn to_string(&self) -> String {
        return "Vector2D"
    }
    
    fn from_string(s: &str) -> Self {
        return Vector2D { x: 0.0, y: 0.0 }
    }
}

impl Clonable for Vector2D {
    fn clone(&self) -> Self {
        return Vector2D { x: self.x, y: self.y }
    }
}

impl Comparable for Vector2D {
    fn equals(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y
    }
    
    fn compare(&self, other: &Self) -> i32 {
        let mag_self = self.x * self.x + self.y * self.y
        let mag_other = other.x * other.x + other.y * other.y
        if mag_self < mag_other {
            return -1
        }
        if mag_self > mag_other {
            return 1
        }
        return 0
    }
}

// --- CLASE CON MÚLTIPLES INTERFACES ---
class Sprite implements Dibujable, Movible:
    x = 0
    y = 0
    color = "rojo"
    imagen = ""
    
    def dibujar(self):
        print("Dibujando sprite en")
        print(self.x)
        print(self.y)
    
    def obtener_color(self) -> str:
        return self.color
    
    def mover(self, dx: i32, dy: i32):
        self.x = self.x + dx
        self.y = self.y + dy

fn main() {
    print("=== Traits e Interfaces ===")
    
    let v = Vector2D { x: 3.0, y: 4.0 }
    print("Vector creado")
    
    let s = v.to_string()
    print(s)
}
