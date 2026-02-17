# ADead-BIB v3.0

**El Lenguaje que Divide CPU y GPU por Verdad Binaria**

> CPU = IR Completo | GPU = SPIR-V Directo

```
┌─────────────────────────────────────────────────────────────────┐
│                      Tu Código (.adB)                           │
│                            ↓                                    │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                   ADead-BIB Compiler                      │  │
│  │                                                           │  │
│  │   ┌─────────────────┐         ┌─────────────────────┐     │  │
│  │   │      CPU        │         │        GPU          │     │  │
│  │   │   (IR Completo) │         │   (SPIR-V Directo)  │     │  │
│  │   │                 │         │                     │     │  │
│  │   │  AST → IR → x86 │         │  AST → SPIR-V bytes │     │  │
│  │   │  Optimización   │         │  Sin intermediarios │     │  │
│  │   │  completa       │         │                     │     │  │
│  │   └────────┬────────┘         └──────────┬──────────┘     │  │
│  └────────────┼──────────────────────────────┼───────────────┘  │
│               ↓                              ↓                  │
│         .exe / .elf                      .spv / .ahyb           │
│      (Binario Nativo)                 (Bytecode GPU)            │
└─────────────────────────────────────────────────────────────────┘
```

---

## Filosofía: División por Verdad Binaria

| Aspecto | CPU | GPU |
|---------|-----|-----|
| **Representación** | IR (Intermediate Representation) | SPIR-V (bytecode directo) |
| **Optimización** | Completa (DCE, inlining, etc.) | Mínima (driver optimiza) |
| **Ejecución** | Secuencial + SIMD | Masivamente paralela |
| **Memoria** | Stack + Heap | Buffers + Shared Memory |
| **Control** | Branches, loops | Workgroups, barriers |

**No estás dividiendo por comodidad, estás dividiendo por VERDAD BINARIA.**

---

## Inicio Rápido

```bash
# Clonar e instalar
git clone https://github.com/AndreeSalazar/ADead-BIB.git
cd ADead-BIB
cargo build --release

# Ejecutar CPU
adB run main.adB

# Ejecutar GPU
adB gpu matmul 1024
```

---

## Autor

**Eddi Andreé Salazar Matos**  
eddi.salazar.dev@gmail.com  
Hecho en Perú

**Licencia:** GPLv2

---

## CPU Backend: IR Completo

El backend CPU usa **Intermediate Representation** para optimización completa antes de emitir bytes x86-64.

### Pipeline CPU

```
Código ADead → AST → IR → Optimizador → x86-64 bytes → PE/ELF
```

### IR Operations

| IR Op | Descripción | x86-64 |
|-------|-------------|--------|
| `IR_CONST` | Cargar constante | `mov rax, imm` |
| `IR_ADD` | Suma | `add rax, rbx` |
| `IR_MUL` | Multiplicación | `imul rax, rbx` |
| `IR_LOAD` | Cargar de memoria | `mov rax, [rbp+off]` |
| `IR_STORE` | Guardar en memoria | `mov [rbp+off], rax` |
| `IR_CALL` | Llamar función | `call rel32` |
| `IR_RET` | Retornar | `ret` |
| `IR_JMP` | Salto incondicional | `jmp rel32` |
| `IR_JZ` | Salto si cero | `jz rel32` |

### Optimizaciones CPU

- **Dead Code Elimination (DCE)** — Elimina código inalcanzable
- **Constant Folding** — Evalúa constantes en compilación
- **Inlining** — Expande funciones pequeñas
- **Register Allocation** — Minimiza accesos a memoria
- **Peephole** — Optimiza patrones locales

### Ejemplo CPU

```rust
fn factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}

fn main() {
    let result = factorial(10)
    println(result)  // 3628800
}
```

**Genera ~1.5 KB de binario nativo.**

---

## GPU Backend: SPIR-V Directo

El backend GPU emite **SPIR-V bytecode directamente** sin IR intermedio.

### Pipeline GPU

```
Código ADead → AST → SPIR-V bytes (directo)
```

### ADead GPU Opcodes (4 bits)

| Opcode | Valor | Operación |
|--------|-------|-----------|
| `EXIT` | 0x0 | Terminar kernel |
| `LOAD` | 0x1 | acc = buffer[gid] |
| `STORE` | 0x2 | buffer[gid] = acc |
| `LOAD_IMM` | 0x3 | acc = immediate |
| `ADD` | 0x4 | acc += buffer[gid] |
| `SUB` | 0x5 | acc -= buffer[gid] |
| `MUL` | 0x6 | acc *= buffer[gid] |
| `DIV` | 0x7 | acc /= buffer[gid] |
| `VEC_ADD` | 0x8 | Vector add |
| `VEC_MUL` | 0x9 | Vector multiply |
| `DOT` | 0xA | Dot product |
| `MATMUL` | 0xB | Matrix multiply |
| `SYNC` | 0xC | Barrier |

### SPIR-V Generation

```rust
// ADead bytecode
let kernel = [
    (LOAD, 0),      // acc = A[gid]
    (ADD, 1),       // acc += B[gid]
    (STORE, 2),     // C[gid] = acc
    (EXIT, 0),
]

// Genera SPIR-V válido directamente
// Magic: 0x07230203
// Version: 1.0
// ...compute shader completo
```

### FFI GPU (Python)

```python
from FFI_GPU import GPU

gpu = GPU()

# Crear buffers
A = gpu.buffer(data_a)
B = gpu.buffer(data_b)
C = gpu.buffer(size=N)

# Cargar y ejecutar kernel
kernel = gpu.load_spirv("vecadd.spv")
gpu.dispatch(kernel, A, B, C, groups=(N//256, 1, 1))
gpu.wait()

# Leer resultado
result = C.read()
```

---

## Comparación CPU vs GPU

| Operación | CPU (IR) | GPU (SPIR-V) |
|-----------|----------|--------------|
| MatMul 1024x1024 | ~200ms | ~5ms |
| VecAdd 1M | ~10ms | ~0.5ms |
| Reduce 1M | ~15ms | ~1ms |
| Compilación | Optimizada | Directa |
| Tamaño binario | ~1.5 KB | ~2 KB shader |

### Cuándo usar cada uno

| Caso | Recomendación |
|------|---------------|
| Lógica de control | CPU |
| Cálculo masivo paralelo | GPU |
| I/O, archivos | CPU |
| Matrices grandes | GPU |
| Código secuencial | CPU |
| Procesamiento de imágenes | GPU |

---

## Estructura del Proyecto

```
ADead-BIB/
├── src/rust/
│   ├── frontend/           # Lexer, Parser, AST
│   ├── backend/
│   │   ├── cpu/            # IR → x86-64
│   │   │   ├── ir.rs       # Intermediate Representation
│   │   │   ├── codegen.rs  # x86-64 emission
│   │   │   └── pe.rs       # PE/ELF generation
│   │   └── gpu/            # AST → SPIR-V
│   │       ├── spirv/      # SPIR-V bytecode
│   │       ├── vulkan/     # Vulkan runtime
│   │       └── compute.rs  # Unified compute API
│   └── optimizer/          # CPU optimizations
│
├── FFI GPU/                # Python GPU runtime
│   └── python/
│       ├── gpu_runtime.py
│       ├── gpu_buffer.py
│       ├── gpu_kernel.py
│       └── gpu_optimizer.py
│
├── Metal_Dead/             # IA Personal CPU-first
│   └── core/
│       ├── cpu_compute.py
│       └── metal_dead_cpu.py
│
└── examples/
```

---

## Metal_Dead: IA con ADead-BIB

Metal_Dead es una IA personal que usa ADead-BIB FFI para cómputo CPU-first.

```python
from Metal_Dead.core import MetalDeadCPU

ai = MetalDeadCPU()
response = ai.chat("Hola, soy el desarrollador")
print(response)
ai.shutdown()
```

### Características

- **CPU-First**: Optimizado para CPU con SIMD
- **ADead-BIB FFI**: Integración nativa
- **Transformer**: 2 capas, 128 dim embedding
- **Memoria**: ~1.2 MB en RAM

---

## Comandos CLI

```bash
# CPU
adB run main.adB              # Compilar y ejecutar
adB build main.adB            # Compilar a .exe
adB opt main.adB              # Optimización máxima

# GPU
adB gpu                       # Detectar GPU
adB spirv kernel.adB          # Generar SPIR-V
adB unified matmul 1024       # CPU+GPU unificado

# Proyecto
adB create proyecto           # Nuevo proyecto
adB check main.adB            # Verificar sintaxis
```

---

## Tamaños de Binario

| Modo | Tamaño | Descripción |
|------|--------|-------------|
| CPU Normal | ~1.5 KB | Con IR optimizado |
| CPU Ultra | ~1 KB | Optimización máxima |
| CPU Tiny | <500 bytes | PE mínimo |
| GPU Shader | ~2 KB | SPIR-V completo |

---

## Licencia

**GNU General Public License v2.0**

- Usar libremente
- Modificar libremente
- Distribuir con misma licencia
- Incluir código fuente

```
Copyright (C) 2024-2026 Eddi Andreé Salazar Matos
eddi.salazar.dev@gmail.com
```

---

**ADead-BIB v3.0: CPU = IR | GPU = SPIR-V**
**División por Verdad Binaria**
