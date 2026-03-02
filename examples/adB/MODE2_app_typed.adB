// ============================================================
// ADead-BIB — MODO 2: Aplicación con Tipado C + OOP Ligero
// ============================================================
// Genera ejecutables PE/ELF con tipos C explícitos.
// Ideal para: aplicaciones, herramientas CLI, scripts compilados.
//
// Compilar Windows: adeadc MODE2_app_typed.adB -o app.exe
// Compilar Linux:   adeadc MODE2_app_typed.adB -o app --linux
//
// Sintaxis: C-style sin punto y coma, llaves para bloques
// ============================================================

// ---- Struct para acumular estadísticas ----
struct Stats {
    count: int
    total: int
}

impl Stats {
    fn nuevo() -> Stats {
        Stats { count: 0, total: 0 }
    }

    fn agregar(&mut self, valor: int) {
        self.count = self.count + 1
        self.total = self.total + valor
    }

    fn promedio(&self) -> int {
        if self.count == 0 {
            return 0
        }
        return self.total / self.count
    }

    fn mostrar(&self) {
        printf("Stats { count=")
        printf(self.count)
        printf(", total=")
        printf(self.total)
        printf(", promedio=")
        int avg = self.promedio()
        printf(avg)
        printf(" }\n")
    }
}

// ---- Funciones tipadas C-style ----
int add(int a, int b) {
    return a + b
}

int multiply(int x, int y) {
    return x * y
}

// ---- Factorial recursivo ----
int factorial(int n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

// ---- Demo control de flujo ----
void control_flow_demo() {
    printf("Control Flow Demo:\n")

    int x = 10
    if x > 5 {
        printf("x > 5: verdadero\n")
    } else {
        printf("x > 5: falso\n")
    }

    // While loop
    printf("While loop:\n")
    int count = 0
    while count < 3 {
        printf("  count = ")
        printf(count)
        printf("\n")
        count += 1
    }
}

// ---- Demo operaciones con compound assignment ----
void compound_demo() {
    printf("\nCompound Assignments:\n")

    int counter = 100
    counter += 50
    printf("100 += 50 = ")
    printf(counter)
    printf("\n")

    counter -= 30
    printf("150 -= 30 = ")
    printf(counter)
    printf("\n")

    counter *= 2
    printf("120 *= 2 = ")
    printf(counter)
    printf("\n")

    counter /= 4
    printf("240 /= 4 = ")
    printf(counter)
    printf("\n")
}

// ---- Demo else-if y do-while ----
void advanced_flow_demo() {
    // else if chains
    int score = 85
    printf("\nScore ")
    printf(score)
    if score >= 90 {
        printf(" = A\n")
    } else if score >= 80 {
        printf(" = B\n")
    } else if score >= 70 {
        printf(" = C\n")
    } else {
        printf(" = F\n")
    }

    // do-while loop
    printf("\ndo-while countdown: ")
    int n = 5
    do {
        printf(n)
        printf(" ")
        n -= 1
    } while n > 0
    printf("\n")
}

// ---- Función principal ----
int main() {
    printf("=== ADead-BIB v3.4 - Tipos C + OOP ===\n\n")

    // Variables con tipos explicitos
    int i = 42
    printf("int i = ")
    printf(i)
    printf("\n")

    long l = 100000
    printf("long l = ")
    printf(l)
    printf("\n")

    // Funciones tipadas
    int resultado = add(10, 20)
    printf("\nadd(10, 20) = ")
    printf(resultado)
    printf("\n")

    int producto = multiply(5, 6)
    printf("multiply(5, 6) = ")
    printf(producto)
    printf("\n")

    int fact5 = factorial(5)
    printf("factorial(5) = ")
    printf(fact5)
    printf("\n")

    // Demos
    control_flow_demo()
    compound_demo()
    advanced_flow_demo()

    // Struct Stats
    printf("\nStruct Stats:\n")
    let s = Stats::nuevo()
    s.agregar(10)
    s.agregar(20)
    s.agregar(30)
    s.mostrar()

    printf("\n=== Compilado sin NASM, sin LLVM ===\n")
    return 0
}
