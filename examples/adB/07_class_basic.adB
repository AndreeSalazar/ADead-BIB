// ADead-BIB v3.4 - Clases y Objetos (struct + impl)
// OOP ligero estilo Rust: struct define datos, impl define métodos

// ---- Definición con struct + impl ----
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

    fn distancia_sq(&self) -> int {
        return self.x * self.x + self.y * self.y
    }
}

// ---- Struct con un solo campo ----
struct Contador {
    valor: int
}

impl Contador {
    fn nuevo() -> Contador {
        Contador { valor: 0 }
    }

    fn incrementar(&mut self) {
        self.valor += 1
    }

    fn obtener(&self) -> int {
        return self.valor
    }
}

int main() {
    printf("=== Clases y Objetos (struct + impl) ===\n\n")

    // Crear Punto
    let p = Punto::nuevo(10, 20)
    printf("Creado: ")
    p.mostrar()

    // Mutar Punto
    p.mover(5, -3)
    printf("Despues de mover(5, -3): ")
    p.mostrar()

    // Método con retorno
    int d = p.distancia_sq()
    printf("distancia^2 = ")
    printf(d)
    printf("\n")

    // Contador
    printf("\n")
    let c = Contador::nuevo()
    c.incrementar()
    c.incrementar()
    c.incrementar()
    int val = c.obtener()
    printf("Contador despues de 3 incrementos: ")
    printf(val)
    printf("\n")

    return 0
}
