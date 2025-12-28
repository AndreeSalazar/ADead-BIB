// ADead-BIB v1.3.0 - Test de len() con arrays
// Obtener longitud de arrays

fn main() {
    println("=== Test len() con Arrays ===")
    
    // Crear array
    let arr = [1, 2, 3, 4, 5]
    
    // Obtener longitud
    let longitud = len(arr)
    print("len([1,2,3,4,5]) = ")
    println(longitud)
    
    // Array más pequeño
    let arr2 = [10, 20]
    let len2 = len(arr2)
    print("len([10,20]) = ")
    println(len2)
    
    println("Test len() completado!")
}
