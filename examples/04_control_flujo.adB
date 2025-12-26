// ============================================
// ADead-BIB - Ejemplo 04: Control de Flujo
// if/elif/else, while, for, match
// ============================================

// --- CONDICIONALES ---
fn condicionales() {
    let edad = 18
    
    // if simple
    if edad >= 18 {
        print("Mayor de edad")
    }
    
    // if-else
    if edad < 18 {
        print("Menor")
    } else {
        print("Mayor")
    }
    
    // if-elif-else (Python style)
    let nota = 85
    if nota >= 90:
        print("Excelente")
    elif nota >= 80:
        print("Muy bien")
    elif nota >= 70:
        print("Bien")
    else:
        print("Necesita mejorar")
}

// --- BUCLES WHILE ---
fn bucle_while() {
    print("Contando del 1 al 5:")
    let i = 1
    while i <= 5 {
        print(i)
        i = i + 1
    }
}

// --- BUCLES FOR ---
def bucle_for():
    print("For loop:")
    for i in range(5):
        print(i)
    
    print("For con inicio y fin:")
    for j in range(1, 10):
        print(j)

// --- BREAK Y CONTINUE ---
fn control_bucles() {
    let i = 0
    while i < 10 {
        i = i + 1
        if i == 3 {
            continue  // Saltar iteración
        }
        if i == 7 {
            break  // Salir del bucle
        }
        print(i)
    }
}

// --- FUNCIÓN PRINCIPAL ---
fn main() {
    print("=== Control de Flujo ===")
    condicionales()
    bucle_while()
    control_bucles()
}
