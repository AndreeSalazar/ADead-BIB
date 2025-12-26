// ADead-BIB v0.9.0 - Test de Input
// input() lee un valor (por ahora retorna 42)

fn main() {
    println("=== Input ===")
    
    // input() retorna 42 por ahora (placeholder)
    let valor = input()
    print("Valor de input(): ")
    println(valor)
    
    // Usar el valor en operaciones
    let doble = valor * 2
    print("Doble: ")
    println(doble)
    
    println("Input funciona!")
}
