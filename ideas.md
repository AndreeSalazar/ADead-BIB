# ADead-BIB v3.0 — Documento de Arquitectura

> **Versión:** 3.0  
> **Autor:** Eddi Andreé Salazar Matos  
> **Filosofía:** División por Verdad Binaria — CPU = IR | GPU = SPIR-V

---

## Manifiesto v3.0

**ADead-BIB divide CPU y GPU por VERDAD BINARIA, no por comodidad.**

```
┌─────────────────────────────────────────────────────────────────┐
│                         ADead-BIB v3.0                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   ┌─────────────────────┐       ┌─────────────────────┐         │
│   │        CPU          │       │        GPU          │         │
│   │   (IR Completo)     │       │   (SPIR-V Directo)  │         │
│   │                     │       │                     │         │
│   │  AST → IR → x86-64  │       │  AST → SPIR-V bytes │         │
│   │  Optimización       │       │  Sin intermediarios │         │
│   │  completa           │       │                     │         │
│   └─────────────────────┘       └─────────────────────┘         │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Parte I: División CPU / GPU

### 1.1 Por qué dividir

| Aspecto | CPU | GPU |
|---------|-----|-----|
| **Modelo de ejecución** | Secuencial + SIMD | Masivamente paralelo |
| **Memoria** | Stack + Heap | Buffers + Shared |
| **Control de flujo** | Branches, loops | Workgroups, barriers |
| **Optimización** | En compilador | En driver |
| **Representación** | IR (optimizable) | SPIR-V (directo) |

### 1.2 Principio Fundamental

> **No estás dividiendo por comodidad.**
> **Estás dividiendo por VERDAD BINARIA.**

La CPU necesita IR porque:
- Tiene registros limitados que asignar
- Tiene branches que optimizar
- Tiene cache que considerar
- El compilador DEBE optimizar

La GPU no necesita IR porque:
- El driver ya optimiza
- Los workgroups son fijos
- La memoria es explícita
- SPIR-V es suficiente

---

## Parte II: CPU Backend — IR Completo

### 2.1 Pipeline CPU

```
Código ADead (.adB)
       ↓
    Lexer
       ↓
    Parser
       ↓
     AST
       ↓
  ┌────────────────────────────────────┐
  │              IR                    │
  │  (Intermediate Representation)     │
  │                                    │
  │  - SSA form                        │
  │  - Type-annotated                  │
  │  - Control flow graph              │
  └────────────────────────────────────┘
       ↓
  ┌────────────────────────────────────┐
  │          Optimizador               │
  │                                    │
  │  - Dead Code Elimination           │
  │  - Constant Folding                │
  │  - Inlining                        │
  │  - Register Allocation             │
  │  - Peephole                        │
  └────────────────────────────────────┘
       ↓
   x86-64 bytes
       ↓
   PE / ELF
```

### 2.2 IR Operations

| IR Op | Descripción | x86-64 Emission |
|-------|-------------|-----------------|
| `IR_CONST(val)` | Cargar constante | `mov rax, imm64` |
| `IR_ADD(a, b)` | Suma | `add rax, rbx` |
| `IR_SUB(a, b)` | Resta | `sub rax, rbx` |
| `IR_MUL(a, b)` | Multiplicación | `imul rax, rbx` |
| `IR_DIV(a, b)` | División | `idiv rbx` |
| `IR_LOAD(addr)` | Cargar memoria | `mov rax, [rbp+off]` |
| `IR_STORE(addr, val)` | Guardar memoria | `mov [rbp+off], rax` |
| `IR_CALL(fn)` | Llamar función | `call rel32` |
| `IR_RET(val)` | Retornar | `mov rax, val; ret` |
| `IR_JMP(label)` | Salto | `jmp rel32` |
| `IR_JZ(cond, label)` | Salto condicional | `test rax, rax; jz rel32` |
| `IR_CMP(a, b)` | Comparar | `cmp rax, rbx` |
| `IR_PHI(...)` | SSA phi node | (resolved in regalloc) |

### 2.3 Optimizaciones CPU

| Optimización | Descripción | Impacto |
|--------------|-------------|---------|
| **DCE** | Dead Code Elimination | -10% código |
| **Constant Folding** | Evaluar constantes | -5% instrucciones |
| **Inlining** | Expandir funciones pequeñas | +velocidad, -calls |
| **Register Allocation** | Minimizar spills | +velocidad |
| **Peephole** | Patrones locales | -instrucciones |
| **Loop Unrolling** | Desenrollar loops pequeños | +velocidad |

### 2.4 Ejemplo CPU

```rust
fn factorial(n: i64) -> i64 {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}

fn main() {
    let result = factorial(10)
    println(result)  // 3628800
}
```

**IR generado:**
```
factorial:
  IR_PARAM n
  IR_CMP n, 1
  IR_JG .recurse
  IR_RET 1
.recurse:
  IR_SUB n, 1
  IR_CALL factorial
  IR_MUL n, result
  IR_RET result
```

**x86-64 generado:** ~50 bytes

---

## Parte III: GPU Backend — SPIR-V Directo

### 3.1 Pipeline GPU

```
Código ADead (.adB)
       ↓
    Lexer
       ↓
    Parser
       ↓
     AST
       ↓
  ┌────────────────────────────────────┐
  │         SPIR-V Emission            │
  │         (DIRECTO)                  │
  │                                    │
  │  - Header SPIR-V                   │
  │  - Declaraciones de tipos          │
  │  - Variables globales              │
  │  - Función main                    │
  │  - Instrucciones compute           │
  └────────────────────────────────────┘
       ↓
   .spv bytecode
```

**NO HAY IR INTERMEDIO.** El AST se traduce directamente a SPIR-V.

### 3.2 ADead GPU Opcodes

| Opcode | Valor | Operación | SPIR-V |
|--------|-------|-----------|--------|
| `EXIT` | 0x0 | Terminar | OpReturn |
| `LOAD` | 0x1 | acc = buf[gid] | OpLoad |
| `STORE` | 0x2 | buf[gid] = acc | OpStore |
| `LOAD_IMM` | 0x3 | acc = imm | OpConstant |
| `ADD` | 0x4 | acc += buf[gid] | OpFAdd |
| `SUB` | 0x5 | acc -= buf[gid] | OpFSub |
| `MUL` | 0x6 | acc *= buf[gid] | OpFMul |
| `DIV` | 0x7 | acc /= buf[gid] | OpFDiv |
| `VEC_ADD` | 0x8 | Vector add | OpFAdd (vec4) |
| `VEC_MUL` | 0x9 | Vector mul | OpFMul (vec4) |
| `DOT` | 0xA | Dot product | OpDot |
| `MATMUL` | 0xB | Matrix multiply | Loop + OpFMul |
| `SYNC` | 0xC | Barrier | OpControlBarrier |

### 3.3 SPIR-V Structure

```
┌─────────────────────────────────────┐
│  SPIR-V Header                      │
│  Magic: 0x07230203                  │
│  Version: 1.0                       │
│  Generator: ADead-BIB               │
├─────────────────────────────────────┤
│  Capabilities                       │
│  OpCapability Shader                │
├─────────────────────────────────────┤
│  Memory Model                       │
│  OpMemoryModel Logical GLSL450      │
├─────────────────────────────────────┤
│  Entry Point                        │
│  OpEntryPoint GLCompute %main       │
├─────────────────────────────────────┤
│  Execution Mode                     │
│  OpExecutionMode %main LocalSize    │
│  256 1 1                            │
├─────────────────────────────────────┤
│  Types & Variables                  │
│  %void, %float, %uint               │
│  %buffer_type, %ptr_type            │
├─────────────────────────────────────┤
│  Function                           │
│  OpFunction %main                   │
│  ... compute instructions ...       │
│  OpReturn                           │
│  OpFunctionEnd                      │
└─────────────────────────────────────┘
```

### 3.4 Ejemplo GPU

```rust
// Kernel: C[i] = A[i] + B[i]
@kernel
fn vecadd(A: buffer<f32>, B: buffer<f32>, C: buffer<f32>) {
    let gid = global_id()
    C[gid] = A[gid] + B[gid]
}
```

**ADead bytecode:**
```
[LOAD, 0]     // acc = A[gid]
[ADD, 1]      // acc += B[gid]
[STORE, 2]    // C[gid] = acc
[EXIT, 0]
```

**SPIR-V generado:** ~200 bytes de bytecode válido

---

## Parte IV: FFI GPU (Python)

### 4.1 API Simple

```python
from FFI_GPU import GPU

gpu = GPU()

# Crear buffers
A = gpu.buffer(data_a)           # CPU → GPU
B = gpu.buffer(data_b)
C = gpu.buffer(size=N)           # Solo GPU

# Cargar y ejecutar kernel
kernel = gpu.load_spirv("vecadd.spv")
gpu.dispatch(kernel, A, B, C, groups=(N//256, 1, 1))
gpu.wait()

# Leer resultado
result = C.read()                 # GPU → CPU
```

### 4.2 Componentes FFI GPU

| Componente | Archivo | Función |
|------------|---------|---------|
| `GPU` | gpu_runtime.py | API principal |
| `GPUBuffer` | gpu_buffer.py | Gestión memoria |
| `GPUKernel` | gpu_kernel.py | Carga kernels |
| `GPUOptimizer` | gpu_optimizer.py | Layout optimizer |
| `GPUFence` | gpu_runtime.py | Sincronización |
| `GPUStream` | gpu_runtime.py | Queues async |

### 4.3 Kernels Predefinidos

```python
from FFI_GPU import kernel_vector_add, kernel_matmul

# Vector add: C = A + B
kernel = kernel_vector_add()

# Matrix multiply: C = A @ B
kernel = kernel_matmul()
```

---

## Parte V: Metal_Dead — IA CPU-First

### 5.1 Arquitectura

```
Metal_Dead/
├── core/
│   ├── cpu_compute.py      # Motor CPU con ADead-BIB FFI
│   ├── metal_dead_cpu.py   # IA principal
│   ├── tokenizer.py        # Tokenizador
│   └── memory.py           # Memoria persistente
```

### 5.2 Uso

```python
from Metal_Dead.core import MetalDeadCPU

ai = MetalDeadCPU()
response = ai.chat("Hola, soy el desarrollador")
print(response)
ai.shutdown()
```

### 5.3 Características

| Característica | Valor |
|----------------|-------|
| Backend | CPU-First (ADead-BIB FFI) |
| SIMD | AVX2 |
| Cores | 12 |
| Modelo | ~1.2 MB RAM |
| Embedding | 128 dim |
| Layers | 2 transformer |

---

## Parte VI: Comparación CPU vs GPU

### 6.1 Rendimiento

| Operación | CPU (IR) | GPU (SPIR-V) | Speedup |
|-----------|----------|--------------|---------|
| MatMul 1024x1024 | ~200ms | ~5ms | **40x** |
| VecAdd 1M | ~10ms | ~0.5ms | **20x** |
| Reduce 1M | ~15ms | ~1ms | **15x** |
| Factorial(20) | ~0.001ms | N/A | CPU wins |

### 6.2 Cuándo usar cada uno

| Caso de uso | Recomendación | Por qué |
|-------------|---------------|---------|
| Lógica de control | CPU | Branches eficientes |
| Cálculo paralelo | GPU | Miles de threads |
| I/O, archivos | CPU | Acceso secuencial |
| Matrices grandes | GPU | Paralelismo masivo |
| Recursión | CPU | Stack nativo |
| Procesamiento imágenes | GPU | Pixels independientes |

---

## Parte VII: Flujo de Compilación

### 7.1 CPU Flow

```
main.adB
    ↓
┌─────────────────┐
│     Lexer       │  Tokens
└────────┬────────┘
         ↓
┌─────────────────┐
│     Parser      │  AST
└────────┬────────┘
         ↓
┌─────────────────┐
│   IR Builder    │  IR (SSA)
└────────┬────────┘
         ↓
┌─────────────────┐
│   Optimizer     │  IR optimizado
└────────┬────────┘
         ↓
┌─────────────────┐
│    Codegen      │  x86-64 bytes
└────────┬────────┘
         ↓
┌─────────────────┐
│   PE/ELF Gen    │  Binario
└────────┬────────┘
         ↓
    main.exe
```

### 7.2 GPU Flow

```
kernel.adB
    ↓
┌─────────────────┐
│     Lexer       │  Tokens
└────────┬────────┘
         ↓
┌─────────────────┐
│     Parser      │  AST
└────────┬────────┘
         ↓
┌─────────────────┐
│  SPIR-V Emit    │  SPIR-V bytes (DIRECTO)
└────────┬────────┘
         ↓
    kernel.spv
```

**Nota:** GPU NO tiene paso de IR ni optimizador. El driver GPU optimiza.

---

## Parte VIII: Archivos del Proyecto

### 8.1 Estructura

```
ADead-BIB/
├── src/rust/
│   ├── frontend/
│   │   ├── lexer.rs
│   │   ├── parser.rs
│   │   └── ast.rs
│   ├── backend/
│   │   ├── cpu/
│   │   │   ├── ir.rs           # IR definition
│   │   │   ├── ir_builder.rs   # AST → IR
│   │   │   ├── optimizer.rs    # IR optimizations
│   │   │   ├── codegen.rs      # IR → x86-64
│   │   │   └── pe.rs           # PE generation
│   │   └── gpu/
│   │       ├── spirv/
│   │       │   └── bytecode.rs # AST → SPIR-V
│   │       ├── vulkan_runtime.rs
│   │       └── compute.rs
│   └── optimizer/
│
├── FFI GPU/
│   └── python/
│       ├── gpu_runtime.py
│       ├── gpu_buffer.py
│       ├── gpu_kernel.py
│       └── gpu_optimizer.py
│
├── Metal_Dead/
│   └── core/
│       ├── cpu_compute.py
│       └── metal_dead_cpu.py
│
└── examples/
```

---

## Parte IX: Principios de Diseño

### 9.1 CPU: Optimización es Responsabilidad del Compilador

El compilador CPU DEBE:
- Eliminar código muerto
- Propagar constantes
- Asignar registros eficientemente
- Minimizar accesos a memoria

### 9.2 GPU: Optimización es Responsabilidad del Driver

El compilador GPU DEBE:
- Emitir SPIR-V válido
- Declarar tipos correctamente
- Configurar workgroups
- **NO optimizar** (el driver lo hace mejor)

### 9.3 Regla de Oro v3.0

> **CPU: IR completo, optimización completa.**
> **GPU: SPIR-V directo, sin intermediarios.**
> **División por VERDAD BINARIA.**

---

## Parte X: Resumen de Arquitectura

### División por Verdad Binaria

| Archivo | Contenido | Target |
|---------|-----------|--------|
| `main.adB` | Código CPU | IR → x86-64 |
| `kernel.adB` | Código GPU | AST → SPIR-V |

### Flujo de Ejecución Híbrido

```
main.adB::main()
    ↓
CPU: inicializar datos
    ↓
GPU: dispatch kernel
    ↓
GPU: ejecutar paralelo
    ↓
CPU: leer resultados
    ↓
CPU: continuar lógica
```

### Principio Clave

> **No estás dividiendo por comodidad,
> estás dividiendo por VERDAD BINARIA.**

---

**ADead-BIB v3.0: El lenguaje que domestica la máquina sin mentir.**
**CPU = IR Completo | GPU = SPIR-V Directo**
