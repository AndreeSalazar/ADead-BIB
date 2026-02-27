# ADead-BIB Examples — Tres Modos de Compilación

ADead-BIB soporta **tres modos de compilación** para diferentes casos de uso:

---

## 🔧 MODO 1: Boot/OS — Machine Code Directo

**Uso:** Bootloaders, kernels, drivers, firmware, código bare-metal.

**Características:**
- Genera código máquina puro (sin headers PE/ELF)
- Acceso directo a registros y puertos I/O
- Instrucciones privilegiadas: `cli`, `sti`, `hlt`, `cpuid`
- Control total sobre cada byte generado
- Soporte para 16-bit (real mode), 32-bit (protected), 64-bit (long mode)

### Compilar

```bash
# Boot sector (512 bytes con firma 0x55AA)
adeadc raw MODE1_boot_minimal.adB -o boot.bin --boot

# Driver raw
adeadc raw MODE1_kernel_driver.adB -o driver.bin

# Modos de CPU
adeadc raw file.adB -o out.bin --mode 16   # Real mode
adeadc raw file.adB -o out.bin --mode 32   # Protected mode
adeadc raw file.adB -o out.bin --mode 64   # Long mode (default)
```

### Probar con QEMU

```bash
qemu-system-x86_64 -drive format=raw,file=boot.bin
```

### Sintaxis Modo 1

```adB
org 0x7C00             // Dirección de carga

cli                    // Deshabilitar interrupciones
sti                    // Habilitar interrupciones
hlt                    // Halt CPU

raw { 0x31, 0xC0 }     // Bytes de máquina inline (xor ax, ax)

int_call(0x10)         // INT 0x10 (BIOS video)
port_out(0x20, 0x20)   // Escribir a I/O port
let val = port_in(0x60) // Leer de I/O port

@interrupt             // Handler de interrupción (auto push/pop + iretq)
fn keyboard_handler() { ... }

@exception             // Handler de excepción (con error code en stack)
fn page_fault_handler() { ... }

@naked                 // Sin prologue/epilogue (control total)
fn init_pic() { ... }
```

### Ejemplos Modo 1

| Archivo | Descripción |
|---------|-------------|
| `MODE1_boot_minimal.adB` | Boot sector que imprime "OS OK" via BIOS |
| `MODE1_kernel_driver.adB` | Driver 64-bit con PIC, teclado, timer, CPUID |
| `boot_sector.adB` | Boot sector completo con mensaje |
| `os_kernel_setup.adB` | Setup de kernel 64-bit |

---

## 📦 MODO 2: C + Rust OOP — Sintaxis Tipada

**Uso:** Aplicaciones, herramientas CLI, scripts compilados.

**Características:**
- Genera ejecutables PE (Windows) o ELF (Linux)
- Tipos C: `int`, `char`, `short`, `long`, `float`, `double`, `bool`
- Punteros reales: `int*`, `&var`, `*ptr`
- Arrays reales: `int arr[5] = [...]`, `arr[i]`
- **OOP ligero**: `struct` + `impl` + `&self` (Rust-style)
- Control de flujo: `if/else if/else`, `while`, `do-while`, `switch`, `break`
- Compound assignments: `+=`, `-=`, `*=`, `/=`, `%=`, `&=`, `|=`, `^=`
- Bitwise: `&`, `|`, `^`, `~`, `<<`, `>>`
- `sizeof(type)` para introspección de memoria

### Compilar

```bash
adeadc MODE2_app_typed.adB -o app.exe          # Windows PE
adeadc MODE2_app_typed.adB -o app --linux      # Linux ELF
adeadc MODE2_app_typed.adB -o app.exe -O2      # Con optimizaciones
```

### Sintaxis Modo 2 — OOP (struct + impl)

```adB
// Definición
struct Punto {
    x: int
    y: int
}

impl Punto {
    fn nuevo(x: int, y: int) -> Punto {
        Punto { x: x, y: y }
    }

    fn mostrar(&self) {
        printf("Punto(")
        printf(self.x)
        printf(", ")
        printf(self.y)
        printf(")\n")
    }

    fn mover(&mut self, dx: int, dy: int) {
        self.x = self.x + dx
        self.y = self.y + dy
    }
}

// Uso
int main() {
    let p = Punto::nuevo(10, 20)
    p.mostrar()
    p.mover(5, -3)
    p.mostrar()
    return 0
}
```

### Sintaxis Modo 2 — Punteros y Arrays

```adB
// Punteros
int valor = 42
int* ptr = &valor
*ptr = 100           // Modifica valor a través del puntero

// Arrays
int arr[5] = [10, 20, 30, 40, 50]
arr[2] = 99          // Acceso por índice

// sizeof
printf(sizeof(int))  // 4 bytes
```

### Sintaxis Modo 2 — Control de flujo

```adB
// else if chain
if score >= 90 {
    printf("A\n")
} else if score >= 80 {
    printf("B\n")
} else {
    printf("F\n")
}

// do-while
int n = 5
do {
    printf(n)
    n -= 1
} while n > 0
```

### Ejemplos Modo 2

| Archivo | Descripción |
|---------|-------------|
| `MODE2_app_typed.adB` | App con tipos C, OOP (Stats), compound, do-while |
| `MODE2_oop_classes.adB` | OOP: Punto, Rectangulo y Circulo con struct+impl |
| `01_hello.adB` | Hello World básico |
| `02_variables.adB` | Variables con tipos C explícitos |
| `03_conditions.adB` | if / else if / else |
| `04_loops.adB` | while, do-while, break |
| `05_functions.adB` | Funciones tipadas y recursión |
| `06_operators.adB` | Aritmética, compound, bitwise |
| `07_class_basic.adB` | OOP: Punto y Contador con struct+impl |
| `08_arrays.adB` | Arrays reales con índice |
| `09_math.adB` | abs, max, min, factorial, potencia, gcd |
| `10_pointers.adB` | Punteros reales (&, *, sizeof) |
| `11_pointers_real.adB` | Punteros + todas las operaciones bitwise |

---

## 🎮 MODO 3: GPU Compute — SPIR-V Directo

**Uso:** Machine learning, procesamiento paralelo, shaders.

**Características:**
- Genera SPIR-V directamente (sin GLSL/HLSL)
- Kernels con `@gpu`
- Buffers: `buffer<f32>`
- Memoria compartida: `shared`
- Sincronización: `barrier()`

```bash
adeadc gpu MODE3_gpu_compute.adB -o compute.spv
```

---

## 📊 Comparación de Modos

| Característica       | Modo 1 (Boot/OS) | Modo 2 (App)       | Modo 3 (GPU) |
|----------------------|------------------|--------------------|--------------|
| **Output**           | Raw bytes        | PE/ELF             | SPIR-V       |
| **Tipos**            | Implícitos (64b) | C explícitos       | f32/vec      |
| **OOP**              | ❌               | ✅ struct+impl    | ❌           |
| **Punteros**         | ✅ (raw)         | ✅ int* / &x / *p | ❌           |
| **Arrays**           | ❌               | ✅ int arr[N]     | ✅ buffer<T> |
| **I/O Ports**        | ✅ port_in/out   | ❌                | ❌           |
| **Interrupciones**   | ✅               | ❌                | ❌           |
| **Compound (+=)**    | ❌               | ✅                | ❌           |
| **Bitwise**          | ✅ (raw)         | ✅ & \| ^ ~ << >> | ❌           |

---

## 🛠️ Comandos del Compilador

```bash
# Modo 1: Raw binary
adeadc raw <input.adB> -o <output.bin>
adeadc raw <input.adB> -o <output.bin> --boot    # Firma 0x55AA
adeadc raw <input.adB> -o <output.bin> --mode 16 # 16-bit

# Modo 2: Aplicación
adeadc <input.adB> -o <output.exe>               # Windows PE
adeadc <input.adB> -o <output> --linux           # Linux ELF
adeadc <input.adB> -o <output.exe> -O2           # Optimizado

# Modo 3: GPU
adeadc gpu <input.adB> -o <output.spv>

# Herramientas
adeadc info <input.adB>                          # Mostrar AST
adeadc disasm <binary>                           # Desensamblar
```

---

## 📁 Estructura de Ejemplos

```
examples/
├── README.md
├── MODE1_boot_minimal.adB       # Boot sector → "OS OK"
├── MODE1_kernel_driver.adB      # Driver 64-bit (PIC, keyboard, timer)
├── MODE2_app_typed.adB          # App tipada con OOP (Stats)
├── MODE2_oop_classes.adB        # OOP: struct+impl (Punto, Rect, Circulo)
├── MODE3_gpu_compute.adB        # GPU: SPIR-V (vector_add, matmul, softmax)
├── boot_sector.adB
├── boot_labels_test.adB
├── os_kernel_setup.adB
└── Ejemplos de guias en .adB/
    ├── 01_hello.adB             # Hello World
    ├── 02_variables.adB         # int, long, char, modulo
    ├── 03_conditions.adB        # if / else if / else
    ├── 04_loops.adB             # while, do-while, break
    ├── 05_functions.adB         # funciones + recursión
    ├── 06_operators.adB         # aritmética + compound + bitwise
    ├── 07_class_basic.adB       # struct+impl OOP (Punto, Contador)
    ├── 08_arrays.adB            # arrays reales int arr[N]
    ├── 09_math.adB              # abs, factorial, potencia, gcd
    ├── 10_pointers.adB          # int* ptr = &x, *ptr = val
    └── 11_pointers_real.adB     # punteros + & | ^ ~ << >> + sizeof
```

---

## 🎯 Filosofía ADead-BIB

> **"Sin NASM, Sin LLVM, Sin headers innecesarios"**

```
Código .adB → Parser → AST → TypeChecker → ISA (ADeadOp) → Encoder → Bytes
```

- **TypeChecker**: Verifica structs, métodos, arrays, punteros, firmas de funciones
- **CPU**: IR completo con optimizaciones en el compilador
- **GPU**: SPIR-V directo (optimización en el driver)

---

**Autor:** Eddi Andreé Salazar Matos  
**Versión:** ADead-BIB v3.4  
**Licencia:** MIT
