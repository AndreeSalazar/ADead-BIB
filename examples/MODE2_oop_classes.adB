// ============================================================
// ADead-BIB — MODO 2: OOP Demo (Funciones C-style)
// ============================================================
// Demostración de funciones y cálculos.
//
// Compilar: adeadc MODE2_oop_classes.adB -o oop.exe
//
// Sintaxis: C-style sin punto y coma
// ============================================================

// ---- Funciones para Punto ----
int punto_distancia(int x, int y) {
    return x * x + y * y
}

void punto_mostrar(int x, int y) {
    printf("Punto(")
    printf(x)
    printf(", ")
    printf(y)
    printf(")\n")
}

// ---- Funciones para Rectangulo ----
int rect_area(int ancho, int alto) {
    return ancho * alto
}

int rect_perimetro(int ancho, int alto) {
    return 2 * ancho + 2 * alto
}

void rect_mostrar(int ancho, int alto) {
    printf("Rectangulo ")
    printf(ancho)
    printf("x")
    printf(alto)
    printf("\n")
}

// ---- Funciones para Circulo ----
int circ_area(int radio) {
    return 3 * radio * radio
}

int circ_perimetro(int radio) {
    return 6 * radio
}

void circ_mostrar(int radio) {
    printf("Circulo r=")
    printf(radio)
    printf("\n")
}

// ---- Funciones matemáticas ----
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
    
    // Demo Punto
    printf("Punto(3, 4):\n")
    punto_mostrar(3, 4)
    int dist = punto_distancia(3, 4)
    printf("  distancia^2 = ")
    printf(dist)
    printf("\n")
    
    // Demo Rectangulo
    printf("\nRectangulo(10, 5):\n")
    rect_mostrar(10, 5)
    int area_r = rect_area(10, 5)
    printf("  area = ")
    printf(area_r)
    printf("\n")
    int peri_r = rect_perimetro(10, 5)
    printf("  perimetro = ")
    printf(peri_r)
    printf("\n")
    
    // Demo Circulo
    printf("\nCirculo(7):\n")
    circ_mostrar(7)
    int area_c = circ_area(7)
    printf("  area = ")
    printf(area_c)
    printf("\n")
    int peri_c = circ_perimetro(7)
    printf("  perimetro = ")
    printf(peri_c)
    printf("\n")
    
    // Demo funciones matematicas
    printf("\nFunciones matematicas:\n")
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
