# ADead-BIB Examples — Dos Modos de Compilación

ADead-BIB soporta **dos modos de compilación** para diferentes casos de uso:

---

## 🔧 MODO 1: Boot/OS — Machine Code Directo

**Uso:** Bootloaders, kernels, drivers, firmware, código bare-metal.

**Características:**
- Genera código máquina puro (sin headers PE/ELF)
- Acceso directo a registros y puertos I/O
- Instrucciones privilegiadas (cli, sti, hlt, cpuid)
- Control total sobre cada byte generado
- Soporte para 16-bit (real mode), 32-bit (protected), 64-bit (long mode)

### Compilar

```bash
# Boot sector (512 bytes con firma 0x55AA)
adeadc raw MODE1_boot_minimal.adB -o boot.bin --boot

# Código raw sin firma
adeadc raw MODE1_kernel_driver.adB -o driver.bin

# Especificar modo de CPU
adeadc raw file.adB -o out.bin --mode 16   # Real mode
adeadc raw file.adB -o out.bin --mode 32   # Protected mode
adeadc raw file.adB -o out.bin --mode 64   # Long mode (default)
```

### Probar con QEMU

```bash
# Boot sector
qemu-system-x86_64 -drive format=raw,file=boot.bin

# Kernel con FastOS
qemu-system-x86_64 -drive format=raw,file=fastos.bin -serial stdio
```

### Sintaxis Modo 1

```adB
// Dirección de carga
org 0x7C00

// Instrucciones privilegiadas
cli                    // Deshabilitar interrupciones
sti                    // Habilitar interrupciones
hlt                    // Halt CPU

// Bytes de máquina inline
raw { 0x31, 0xC0 }     // xor ax, ax
raw { 0xB4, 0x0E }     // mov ah, 0x0E

// Interrupciones BIOS
int_call(0x10)         // INT 0x10 (video)
int_call(0x13)         // INT 0x13 (disco)

// I/O Ports
port_out(0x20, 0x20)   // Escribir a puerto
let val = port_in(0x60) // Leer de puerto

// Atributos de función
@interrupt             // Handler de interrupción (auto push/pop + iretq)
@naked                 // Sin prologue/epilogue
@exception             // Handler de excepción (con error code)
```

### Ejemplos Modo 1

| Archivo | Descripción |
|---------|-------------|
| `MODE1_boot_minimal.adB` | Boot sector mínimo que imprime "OS OK" |
| `MODE1_kernel_driver.adB` | Driver de kernel con PIC, keyboard, timer |
| `boot_sector.adB` | Boot sector completo con mensaje |
| `os_kernel_setup.adB` | Setup de kernel 64-bit |

---

## 📦 MODO 2: C+Python — Sintaxis Tipada

**Uso:** Aplicaciones, herramientas CLI, scripts compilados.

**Características:**
- Genera ejecutables PE (Windows) o ELF (Linux)
- Tipos C: `int`, `char`, `short`, `long`, `float`, `double`
- Punteros y referencias: `int*`, `char*`, `&var`
- Arrays: `int arr[10]`
- Clases y OOP (sintaxis Python)
- Control de flujo completo

### Compilar

```bash
# Windows PE (.exe)
adeadc MODE2_app_typed.adB -o app.exe

# Linux ELF
adeadc MODE2_app_typed.adB -o app --linux

# Con optimizaciones
adeadc MODE2_app_typed.adB -o app.exe -O2
```

### Ejecutar

```bash
# Windows
.\app.exe

# Linux
./app
```

### Sintaxis Modo 2

```adB
// Tipos C (sin punto y coma)
int main() {
    int i = 42
    int l = 100000
    
    printf("Hello, World!\n")
    return 0
}

// Funciones tipadas
int add(int a, int b) {
    return a + b
}

int factorial(int n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

// Control de flujo (sin paréntesis en condición)
void demo() {
    int x = 10
    if x > 5 {
        printf("x > 5\n")
    } else {
        printf("x <= 5\n")
    }
    
    int count = 0
    while count < 3 {
        printf(count)
        count = count + 1
    }
}

// Funciones matemáticas
int max(int a, int b) {
    if a > b {
        return a
    }
    return b
}

int abs_val(int x) {
    if x < 0 {
        return 0 - x
    }
    return x
}
```

### Ejemplos Modo 2

| Archivo | Descripción |
|---------|-------------|
| `MODE2_app_typed.adB` | Aplicación con tipos C, funciones, control de flujo |
| `MODE2_oop_classes.adB` | Funciones para geometría (punto, rectángulo, círculo) |
| `01_hello.adB` | Hello World básico |
| `05_functions.adB` | Funciones y recursión |
| `11_pointers_real.adB` | Punteros y operaciones bitwise |

---

## 📊 Comparación de Modos

| Característica | Modo 1 (Boot/OS) | Modo 2 (App) |
|----------------|------------------|--------------|
| **Output** | Raw bytes | PE/ELF |
| **Headers** | Ninguno | Completos |
| **Tipos** | Implícitos (64-bit) | Explícitos (C-style) |
| **Registros** | Acceso directo | Automático |
| **I/O Ports** | ✅ `port_in/out` | ❌ |
| **Interrupciones** | ✅ `int_call`, `@interrupt` | ❌ |
| **Clases/OOP** | ❌ | ✅ |
| **printf** | ❌ (usar BIOS) | ✅ |
| **Tamaño típico** | 512 bytes - 64KB | 1KB - 1MB |

---

## 🛠️ Comandos del Compilador

```bash
# Ayuda
adeadc --help

# Modo 1: Raw binary
adeadc raw <input.adB> -o <output.bin>
adeadc raw <input.adB> -o <output.bin> --boot    # Con firma 0x55AA
adeadc raw <input.adB> -o <output.bin> --mode 16 # 16-bit

# Modo 2: Aplicación
adeadc <input.adB> -o <output.exe>               # Windows PE
adeadc <input.adB> -o <output> --linux           # Linux ELF
adeadc <input.adB> -o <output.exe> -O2           # Optimizado

# Información
adeadc info <input.adB>                          # Mostrar AST
adeadc disasm <binary>                           # Desensamblar
```

---

## 📁 Estructura de Ejemplos

```
examples/
├── README.md                    # Este archivo
├── MODE1_boot_minimal.adB       # Boot sector mínimo
├── MODE1_kernel_driver.adB      # Driver de kernel
├── MODE2_app_typed.adB          # App con tipos C
├── MODE2_oop_classes.adB        # OOP con clases
├── boot_sector.adB              # Boot sector completo
├── boot_labels_test.adB         # Test de labels
├── os_kernel_setup.adB          # Setup de kernel
└── Ejemplos de guias en .adB/   # Tutoriales básicos
    ├── 01_hello.adB
    ├── 02_variables.adB
    ├── 03_conditions.adB
    ├── 04_loops.adB
    ├── 05_functions.adB
    ├── 06_operators.adB
    ├── 07_class_basic.adB
    ├── 08_arrays.adB
    ├── 09_math.adB
    ├── 10_pointers.adB
    └── 11_pointers_real.adB
```

---

## 🎯 Filosofía ADead-BIB

> **"Sin NASM, Sin LLVM, Sin headers innecesarios"**

ADead-BIB genera código máquina **directo** desde el AST:

```
Código .adB → Parser → AST → ISA (ADeadOp) → Encoder → Bytes
```

- **CPU**: IR completo con optimizaciones en el compilador
- **GPU**: SPIR-V directo (optimización en el driver)

---

**Autor:** Eddi Andreé Salazar Matos  
**Versión:** ADead-BIB v3.2  
**Licencia:** MIT
