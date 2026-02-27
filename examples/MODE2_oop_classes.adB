// ============================================================
// ADead-BIB — MODO 2: OOP con struct + impl (Rust-style)
// ============================================================
// Demostración de OOP ligero: structs con métodos.
//
// Compilar: adeadc MODE2_oop_classes.adB -o oop.exe
//
// Sintaxis: struct + impl + &self (sin punto y coma)
// ============================================================

// ---- Struct Punto ----
struct Punto {
    x: int
    y: int
}

impl Punto {
    fn nuevo(x: int, y: int) -> Punto {
        Punto { x: x, y: y }
    }

    fn distancia_sq(&self) -> int {
        return self.x * self.x + self.y * self.y
    }

    fn mover(&mut self, dx: int, dy: int) {
        self.x = self.x + dx
        self.y = self.y + dy
    }

    fn mostrar(&self) {
        printf("Punto(")
        printf(self.x)
        printf(", ")
        printf(self.y)
        printf(")\n")
    }
}

// ---- Struct Rectangulo ----
struct Rectangulo {
    ancho: int
    alto: int
}

impl Rectangulo {
    fn nuevo(ancho: int, alto: int) -> Rectangulo {
        Rectangulo { ancho: ancho, alto: alto }
    }

    fn area(&self) -> int {
        return self.ancho * self.alto
    }

    fn perimetro(&self) -> int {
        return 2 * self.ancho + 2 * self.alto
    }

    fn es_cuadrado(&self) -> bool {
        return self.ancho == self.alto
    }

    fn mostrar(&self) {
        printf("Rectangulo ")
        printf(self.ancho)
        printf("x")
        printf(self.alto)
        printf("\n")
    }
}

// ---- Struct Circulo ----
struct Circulo {
    radio: int
}

impl Circulo {
    fn nuevo(radio: int) -> Circulo {
        Circulo { radio: radio }
    }

    fn area(&self) -> int {
        return 3 * self.radio * self.radio
    }

    fn perimetro(&self) -> int {
        return 6 * self.radio
    }

    fn mostrar(&self) {
        printf("Circulo r=")
        printf(self.radio)
        printf("\n")
    }
}

// ---- Funciones libres ----
int max(int a, int b) {
    if a > b {
        return a
    }
    return b
}

int min(int a, int b) {
    if a < b {
        return a
    }
    return b
}

int abs_val(int x) {
    if x < 0 {
        return 0 - x
    }
    return x
}

// ---- Main ----
int main() {
    printf("=== ADead-BIB v3.4 OOP Demo ===\n\n")

    // Demo Punto con struct + impl
    let p = Punto::nuevo(3, 4)
    printf("Creado: ")
    p.mostrar()
    int dist = p.distancia_sq()
    printf("  distancia^2 = ")
    printf(dist)
    printf("\n")

    p.mover(2, 1)
    printf("Despues de mover(2,1): ")
    p.mostrar()

    // Demo Rectangulo
    printf("\n")
    let r = Rectangulo::nuevo(10, 5)
    r.mostrar()
    int area_r = r.area()
    printf("  area = ")
    printf(area_r)
    printf("\n")
    int peri_r = r.perimetro()
    printf("  perimetro = ")
    printf(peri_r)
    printf("\n")

    // Demo Circulo
    printf("\n")
    let c = Circulo::nuevo(7)
    c.mostrar()
    int area_c = c.area()
    printf("  area = ")
    printf(area_c)
    printf("\n")
    int peri_c = c.perimetro()
    printf("  perimetro = ")
    printf(peri_c)
    printf("\n")

    // Funciones libres
    printf("\nFunciones libres:\n")
    int m = max(15, 8)
    printf("max(15, 8) = ")
    printf(m)
    printf("\n")

    int n = min(15, 8)
    printf("min(15, 8) = ")
    printf(n)
    printf("\n")

    int a = abs_val(0 - 42)
    printf("abs(-42) = ")
    printf(a)
    printf("\n")

    printf("\n=== Compilado sin NASM, sin LLVM ===\n")
    return 0
}
