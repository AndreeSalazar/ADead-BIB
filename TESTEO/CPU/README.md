# CPU - Contratos Binarios x86-64

> **ADead-BIB CPU Backend** - Código → Bytes x86-64 → Ejecutable
> 
> La CPU **NO interpreta**. La CPU **ejecuta bytes**.

---

## 🎯 Filosofía: Contrato Binario Directo

```
COMPILADORES TRADICIONALES:
  Código → IR → Optimizador → ASM → Assembler → Linker → Binario
  (7+ capas de "interpretación")

ADead-BIB CPU:
  Código → AST → BYTES DIRECTOS → PE/ELF
  (Sin intermediarios. Sin reinterpretación.)
```

### ¿Por qué importa?

La CPU **no entiende lenguajes**. La CPU entiende:
- **Bytes** (instrucciones x86-64)
- **Direcciones** (memoria)
- **Contratos** (calling conventions)

Si el compilador:
- Elimina código "innecesario"
- Reordena instrucciones
- "Optimiza" accesos a memoria

👉 **Rompe el contrato**

ADead-BIB **respeta el contrato** porque emite bytes directamente.

---

## 📁 Estructura

```
CPU/
├── binario/           # Literales binarios (0b...)
│   └── test_*.adB
├── opcodes/           # Opcodes x86-64 directos
│   └── test_*.adB
├── contratos/         # Calling conventions, stack
│   └── test_*.adB
└── README.md          # Esta guía
```

---

## 🔢 Tabla de Bytes x86-64

| Instrucción | Bytes | Descripción |
|-------------|-------|-------------|
| `push rbp` | `0x55` | Guardar base pointer |
| `mov rbp, rsp` | `0x48 0x89 0xE5` | Setup stack frame |
| `pop rbp` | `0x5D` | Restaurar base pointer |
| `ret` | `0xC3` | Retornar |
| `xor rax, rax` | `0x48 0x31 0xC0` | Limpiar rax (return 0) |
| `mov rax, imm64` | `0x48 0xB8 [8 bytes]` | Cargar inmediato |
| `call rel32` | `0xE8 [4 bytes]` | Llamar función |
| `jmp rel32` | `0xE9 [4 bytes]` | Salto incondicional |

---

## 📝 Sintaxis ADead-BIB para CPU

### Literales Binarios
```rust
// Literales binarios directos
let mask = 0b11110000      // 240
let bits = 0b1010_1010     // 170 (con separadores)

// Operaciones
let result = mask + bits   // Aritmética directa
```

### Literales HEX (para bytes CPU)
```rust
// Opcodes como valores
let push_rbp = 0x55        // push rbp
let ret = 0xC3             // ret
let call = 0xE8            // call rel32

// Verificación
if push_rbp == 85 {
    println("Opcode correcto")
}
```

### Funciones (Contrato de Stack)
```rust
fn mi_funcion(a, b) {
    // ADead-BIB genera:
    // push rbp           (0x55)
    // mov rbp, rsp       (0x48 0x89 0xE5)
    // ... código ...
    // pop rbp            (0x5D)
    // ret                (0xC3)
    return a + b
}
```

---

## 🔥 Contratos CPU

### 1. Calling Convention Windows x64
```
Parámetros: RCX, RDX, R8, R9 (primeros 4)
            Stack (resto)
Retorno:    RAX
Preservar:  RBX, RBP, RDI, RSI, R12-R15
```

### 2. Stack Frame
```
[RBP+16]  → Parámetro 2 (si > 4 params)
[RBP+8]   → Return address
[RBP]     → Old RBP (saved)
[RBP-8]   → Variable local 1
[RBP-16]  → Variable local 2
```

### 3. Alineación
- Stack debe estar alineado a 16 bytes antes de `call`
- Datos deben respetar alineación natural

---

## 🧪 Ejecutar Tests CPU

```bash
# Test de literales binarios
cargo run --bin adeadc -- run TESTEO/CPU/binario/test_binary.adB

# Test de opcodes x86-64
cargo run --bin adeadc -- run TESTEO/CPU/opcodes/test_opcodes.adB

# Test de contratos (calling convention)
cargo run --bin adeadc -- run TESTEO/CPU/contratos/test_stack.adB
```

---

## ⚠️ Reglas de Oro

1. **No confíes en optimizaciones** - Cada byte cuenta
2. **Respeta el contrato** - Calling convention es ley
3. **Verifica alineación** - Stack a 16 bytes
4. **Bytes son bytes** - No hay "interpretación"

---

## 🔗 Relación con GPU

```
CPU prepara → GPU ejecuta → CPU recibe

CPU:
  1. Escribe datos en memoria
  2. Escribe comandos GPU
  3. Dispara ejecución
  4. Se aparta (o hace otra cosa)

GPU:
  1. Lee comandos
  2. Ejecuta kernels
  3. Escribe resultados
  4. Sin volver a preguntar
```

La CPU **no mira cada iteración**.
La GPU **no pide permiso**.

---

*ADead-BIB CPU: Bytes directos. Sin mentiras.*
