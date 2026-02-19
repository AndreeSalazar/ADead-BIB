// ADead-BIB v3.1 - Clases y Objetos (OOP Básico)
// Definición de clases estilo C++ simplificado

// Definición de clase usando struct + impl
struct Punto {
    x: int
    y: int
}

impl Punto {
    fn nuevo(x: int, y: int) -> Punto {
        Punto { x: x, y: y }
    }
    
    fn mostrar(&self) {
        printf("Punto(")
        printf(self.x)
        printf(", ")
        printf(self.y)
        printf(")\n")
    }
    
    fn mover(&mut self, dx: int, dy: int) {
        self.x = self.x + dx
        self.y = self.y + dy
    }
}

int main() {
    printf("=== Clases y Objetos ===\n")
    
    // Crear objeto
    let p = Punto::nuevo(10, 20)
    printf("Punto creado: ")
    p.mostrar()
    
    // Modificar objeto
    p.mover(5, 5)
    printf("Despues de mover(5,5): ")
    p.mostrar()
    
    return 0
}
