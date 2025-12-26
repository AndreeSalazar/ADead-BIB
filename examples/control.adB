// ADead-BIB v0.6.0 - Control de Flujo
// if/else, while, for, comparaciones

fn main() {
    println("=== Control de Flujo ===")
    
    // IF / ELSE
    let x = 10
    let y = 5
    
    if x > y {
        println("x es mayor que y")
    } else {
        println("x no es mayor que y")
    }
    
    // WHILE
    let i = 0
    println("Contando con while:")
    while i < 3 {
        print("i = ")
        println(i)
        i = i + 1
    }
    
    // FOR
    println("Contando con for:")
    for j in 0..3 {
        print("j = ")
        println(j)
    }
    
    println("Fin!")
}
