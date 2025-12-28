// ADead-BIB v1.4.0 - Test de input() real
// Lee numeros del teclado y hace operaciones

fn calcular(x) {
    return x * x
}

fn main() {
    println("+========================================+")
    println("|   ADead-BIB v1.4.0 - Test input()     |")
    println("+========================================+")
    println("")
    
    // Test 1: Leer un numero
    print("Ingresa un numero: ")
    let n = input()
    print("Leiste: ")
    println(n)
    
    // Test 2: Operaciones con el numero
    let doble = n * 2
    print("El doble es: ")
    println(doble)
    
    let cuadrado = calcular(n)
    print("El cuadrado es: ")
    println(cuadrado)
    
    // Test 3: Segundo input
    print("Ingresa otro numero: ")
    let m = input()
    
    let suma = n + m
    print("La suma es: ")
    println(suma)
    
    println("")
    println("+========================================+")
    println("|     OK - input() funciona!            |")
    println("+========================================+")
}
