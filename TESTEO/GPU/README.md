# GPU - Contratos HEX Directos

> **ADead-BIB GPU Backend** - Código → Opcodes HEX → GPU
> 
> La GPU **NO interpreta**. La GPU **ejecuta contratos binarios**.

---

## 🎯 Filosofía: Contrato HEX Directo

```
COMPILADORES TRADICIONALES:
  Código → GLSL/HLSL → Compilador Shader → SPIR-V → Driver → GPU
  (5+ capas de "traducción")

ADead-BIB GPU:
  Código → Opcodes HEX → SPIR-V/CUDA directo → GPU
  (Sin shaders textuales. Sin reinterpretación.)
```

### ¿Por qué importa?

La GPU **no entiende lenguajes**. La GPU entiende:
- **Opcodes** (instrucciones binarias)
- **Buffers** (memoria)
- **Contratos** (command buffers)

Si el compilador:
- Elimina operaciones "sin efecto"
- Reordena accesos a memoria
- "Optimiza" sincronización

👉 **Rompe el contrato**

ADead-BIB **respeta el contrato** porque emite HEX directamente.

---

## 📁 Estructura

```
GPU/
├── hex/               # Literales HEX (0x...)
│   └── test_*.adB
├── opcodes/           # Opcodes GPU directos (0xC0DA...)
│   └── test_*.adB
├── contratos/         # Command buffers, sincronización
│   └── test_*.adB
└── README.md          # Esta guía
```

---

## 🔢 Tabla de Opcodes GPU ADead-BIB

| Opcode | HEX | Descripción |
|--------|-----|-------------|
| `GPU_INIT` | `0xC0DA0001` | Inicializar contexto |
| `GPU_SHUTDOWN` | `0xC0DA0002` | Cerrar contexto |
| `GPU_ALLOC` | `0xC0DA0010` | Reservar memoria |
| `GPU_FREE` | `0xC0DA0011` | Liberar memoria |
| `GPU_COPY_H2D` | `0xC0DA0012` | Host → Device |
| `GPU_COPY_D2H` | `0xC0DA0013` | Device → Host |
| `GPU_MATMUL` | `0xC0DA0020` | Multiplicación matrices |
| `GPU_ADD` | `0xC0DA0021` | Suma tensores |
| `GPU_MUL` | `0xC0DA0023` | Multiplicación elemento |
| `GPU_SYNC` | `0xC0DA00F0` | Sincronizar |
| `GPU_END` | `0xC0DAFFFF` | Fin programa |

---

## 📝 Sintaxis ADead-BIB para GPU

### Literales HEX
```rust
// Literales HEX directos
let byte = 0xFF            // 255
let word = 0x1234          // 4660
let big = 0xFF_FF          // 65535 (con separadores)

// Opcodes GPU como valores
let GPU_INIT = 0xC0DA0001
let GPU_MATMUL = 0xC0DA0020
let GPU_SYNC = 0xC0DA00F0
```

### Operaciones GPU (Concepto)
```rust
// Futuro: funciones gpu::*
// gpu::init()                    // 0xC0DA0001
// gpu::alloc(4096, reg0)         // 0xC0DA0010
// gpu::matmul(reg0, reg1, reg2)  // 0xC0DA0020
// gpu::sync()                    // 0xC0DA00F0
```

### Formato de Instrucción GPU
```
[opcode:8][dst:8][src1:8][src2:8] = 4 bytes por instrucción

Ejemplo:
  0xC0DA0020 = GPU_MATMUL
  Siguiente 3 bytes = registros destino, fuente1, fuente2
```

---

## 🔥 Contratos GPU

### 1. Command Buffer
```
CPU escribe:
  [comando 1]
  [comando 2]
  [comando N]
  [GPU_END]

GPU lee:
  ejecuta comando 1
  ejecuta comando 2
  ...
  encuentra GPU_END → termina
```

### 2. Sincronización
```
GPU_SYNC = barrera
  - Todos los threads esperan
  - Memoria coherente
  - Continúa después

Sin GPU_SYNC:
  - Ejecución paralela
  - Orden no garantizado
  - Más rápido pero peligroso
```

### 3. Memoria
```
Host (CPU):     RAM del sistema
Device (GPU):   VRAM de la GPU

GPU_COPY_H2D:   Host → Device (subir datos)
GPU_COPY_D2H:   Device → Host (bajar resultados)

⚠️ La GPU NO puede leer RAM directamente
⚠️ La CPU NO puede leer VRAM directamente
```

---

## 🧪 Ejecutar Tests GPU

```bash
# Test de literales HEX
cargo run --bin adeadc -- run TESTEO/GPU/hex/test_hex.adB

# Test de opcodes GPU
cargo run --bin adeadc -- run TESTEO/GPU/opcodes/test_opcodes.adB

# Test de contratos (command buffer)
cargo run --bin adeadc -- run TESTEO/GPU/contratos/test_command_buffer.adB
```

---

## ⚠️ Reglas de Oro

1. **No confíes en el driver** - Emite HEX explícito
2. **Respeta el contrato** - Command buffer es ley
3. **Sincroniza cuando necesites** - GPU_SYNC no es gratis
4. **HEX es HEX** - No hay "interpretación"

---

## 🔗 Relación con CPU

### La GPU NO "piensa" sola
### La GPU PUEDE trabajar sin intervención continua de CPU

```
CPU:
  1. Prepara datos
  2. Construye comandos
  3. Los escribe en memoria
  4. Le dice a la GPU: "ahí está el trabajo"
  5. Se aparta

GPU:
  1. Lee comandos
  2. Lee memoria
  3. Ejecuta kernels
  4. Sin volver a preguntar
```

### Mientras:
- No se quede sin comandos
- No necesite sincronización
- No ocurra un fault

🔥 **No hay "loop CPU→GPU→CPU" constante**

---

## 💡 Por qué ADead-BIB es diferente

### LLVM-style
```
for i in 0..N:
  do_nothing(i)
```
LLVM: "esto no tiene efectos observables" → **ELIMINA**

### GPU-style
```
for i in 0..N:
  write memory-mapped register
```
👉 **Cada iteración ES un efecto**
👉 **Eliminarlo rompe el sistema**

ADead-BIB entiende eso **por diseño**.

---

## 📊 Formato AHYB (ADead Hybrid Binary)

```
┌─────────────────────────────────┐
│ Header AHYB (8 bytes)           │
│   Magic: "AHYB"                 │
│   Version: u8                   │
│   Flags: u8                     │
│   CPU_size: u16                 │
│   GPU_size: u16                 │
├─────────────────────────────────┤
│ CPU Section (bytes x86-64)      │
├─────────────────────────────────┤
│ GPU Section (opcodes HEX)       │
└─────────────────────────────────┘
```

---

*ADead-BIB GPU: HEX directo. Sin mentiras.*
