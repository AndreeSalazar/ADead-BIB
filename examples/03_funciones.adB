// ============================================
// ADead-BIB - Ejemplo 03: Funciones
// Definición y llamada de funciones
// ============================================

// --- FUNCIONES ESTILO RUST ---
fn saludar() {
    print("Hola desde fn!")
}

fn sumar(a: i32, b: i32) -> i32 {
    return a + b
}

fn factorial(n: i32) -> i32 {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

// --- FUNCIONES ESTILO PYTHON ---
def saludar_python():
    print("Hola desde def!")

def multiplicar(x, y):
    return x * y

def es_par(numero):
    if numero % 2 == 0:
        return true
    return false

// --- FUNCIÓN PRINCIPAL ---
fn main() {
    print("=== Funciones en ADead-BIB ===")
    
    // Llamar funciones
    saludar()
    saludar_python()
    
    // Funciones con retorno
    let resultado = sumar(10, 20)
    print("10 + 20 =")
    print(resultado)
    
    // Factorial
    let fact5 = factorial(5)
    print("5! =")
    print(fact5)
}
