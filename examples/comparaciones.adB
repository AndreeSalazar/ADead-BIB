// ADead-BIB v0.6.0 - Comparaciones
// ==, !=, <, >, <=, >=

fn main() {
    println("=== Comparaciones ===")
    
    let a = 10
    let b = 20
    let c = 10
    
    // == igual
    if a == c {
        println("a == c: verdadero")
    }
    
    // != diferente
    if a != b {
        println("a != b: verdadero")
    }
    
    // < menor que
    if a < b {
        println("a < b: verdadero")
    }
    
    // > mayor que
    if b > a {
        println("b > a: verdadero")
    }
    
    // <= menor o igual
    if a <= c {
        println("a <= c: verdadero")
    }
    
    // >= mayor o igual
    if b >= a {
        println("b >= a: verdadero")
    }
    
    println("Todas las comparaciones funcionan!")
}
