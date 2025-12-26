// ADead-BIB v1.2.0 - Test de Structs
// struct, impl, métodos

struct Punto {
    x: i32,
    y: i32,
}

impl Punto {
    fn nuevo(x, y) {
        return x + y
    }
    
    fn suma(a, b) {
        return a + b
    }
}

fn main() {
    println("=== Structs ===")
    
    // Llamar método del impl
    let resultado = Punto::suma(10, 20)
    print("Punto::suma(10, 20) = ")
    println(resultado)
    
    let r2 = Punto::nuevo(5, 3)
    print("Punto::nuevo(5, 3) = ")
    println(r2)
    
    println("Structs funcionan!")
}
