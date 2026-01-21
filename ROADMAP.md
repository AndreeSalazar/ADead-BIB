# ADead-BIB v2.5 — Roadmap de Desarrollo

> **ADead-BIB** = **A**SM **Dead** - **B**inary **I**s **B**inary
> 
> Lenguaje **OOP Puro + ASM Simbionte + Compute Unificado** que compila **DIRECTO a BINARIO**.
> Sin ASM intermedio. Sin LLVM. Sin linker externo. Sin NASM.
> 
> **Código → AST → BYTES DIRECTOS → Ejecutable**
>
> ## 🎯 Visión a Largo Plazo
> 
> ```
> ┌─────────────────────────────────────────────────────────────────┐
> │                    ADead-BIB — Visión Completa                   │
> ├─────────────────────────────────────────────────────────────────┤
> │                                                                  │
> │  v2.x  →  Compilador HEX-First (CPU + GPU directo)              │
> │  v2.5  →  🆕 Compute Unificado (CUDA/HIP-CPU/Vulkan)            │
> │  v3.x  →  OOP Avanzado (structs, traits, herencia)              │
> │  v4.x  →  ASM Simbionte (interop con Python/Java/C#/Rust)       │
> │  v5.x  →  ADead-OS (Sistema Operativo alternativo)              │
> │  v6.x  →  Ecosistema (IDE, paquetes, comunidad)                 │
> │                                                                  │
> └─────────────────────────────────────────────────────────────────┘
> ```

---

## 🚀 Guía Rápida: Crear y Ejecutar Proyectos

### Comparación con Rust (cargo)

| Rust (cargo) | ADead-BIB (adB) | Descripción |
|--------------|-----------------|-------------|
| `cargo new hola` | `adB new hola` | Crear proyecto nuevo |
| `cargo run` | `adB run main.adB` | Compilar y ejecutar |
| `cargo build` | `adB build main.adB` | Solo compilar |
| `cargo check` | `adB check main.adB` | Verificar sintaxis |
| `cargo init` | `adB init` | Inicializar en directorio actual |

### 📦 Crear Proyecto Nuevo

#### Windows (PowerShell)

```powershell
# Opción 1: Si tienes adeadc instalado globalmente
adB new hola
cd hola
adB run main.adB

# Opción 2: Desde el repositorio ADead-BIB (desarrollo)
cd C:\Users\andre\OneDrive\Documentos\ADead-BIB
cargo run --bin adeadc -- new hola
cd hola
cargo run --bin adeadc -- run main.adB
```

#### Linux / macOS

```bash
# Opción 1: Si tienes adeadc instalado globalmente
adB new hola
cd hola
adB run main.adB

# Opción 2: Desde el repositorio ADead-BIB (desarrollo)
cd ~/ADead-BIB
cargo run --bin adeadc -- new hola
cd hola
cargo run --bin adeadc -- run main.adB
```

### 📁 Estructura del Proyecto Generado

Cuando ejecutas `adB new hola`, se crea:

```
hola/
├── main.adB          # 🎯 Punto de entrada (tu código aquí)
├── call.adB          # 📦 Lógica OOP (structs, traits, impl)
├── build.adB         # ⚙️ Configuración de compilación
├── README.md         # 📖 Documentación del proyecto
├── core/
│   └── mod.adB       # 🔧 init(), shutdown()
├── cpu/
│   └── mod.adB       # 💻 Instrucciones x86-64 directas
└── gpu/
    └── mod.adB       # 🎮 Opcodes GPU (0xC0DA...)
```

### 📝 Contenido de main.adB (generado automáticamente)

```rust
// ============================================================================
// hola - ADead-BIB Project
// ============================================================================
// Ejecutar: adB run main.adB
// ============================================================================

fn main() {
    println("========================================")
    println("     hola - ADead-BIB")
    println("     Binary Is Binary")
    println("========================================")
    println("")
    
    // Tu código aquí
    println("Hello, hola!")
    println("")
    
    // Variables
    let x = 42
    let y = 10
    let result = x + y
    
    print("Resultado: ")
    println(result)
    println("")
    
    println("========================================")
    println("     Proyecto listo!")
    println("========================================")
}
```

### 🎮 Ejemplo: Proyecto con OOP

```bash
# Crear proyecto
adB new mi_juego
cd mi_juego
```

Edita `call.adB` para agregar OOP:

```rust
// call.adB - Lógica OOP del juego

struct Player {
    name: string,
    x: f32,
    y: f32,
    health: i32
}

impl Player {
    fn new(name: string) -> Player {
        return Player {
            name: name,
            x: 0.0,
            y: 0.0,
            health: 100
        }
    }
    
    fn move_to(mut self, x: f32, y: f32) {
        self.x = x
        self.y = y
    }
    
    fn info(self) {
        print("Player: ")
        println(self.name)
        print("Position: (")
        print(self.x)
        print(", ")
        print(self.y)
        println(")")
    }
}

pub fn run() {
    let player = Player::new("Hero")
    player.info()
    
    player.move_to(10.0, 20.0)
    println("Moved!")
    player.info()
}
```

Edita `main.adB` para usar la lógica:

```rust
// main.adB
#![imports(call: run)]

fn main() {
    println("=== Mi Juego ===")
    call::run()
    println("=== Fin ===")
}
```

Ejecutar:
```bash
adB run main.adB
```

### 📋 Todos los Comandos Disponibles

```bash
# CREAR PROYECTO
adB new <nombre>              # Crear proyecto nuevo
adB create <nombre>           # Alias de new
adB init                      # Inicializar en directorio actual

# COMPILAR Y EJECUTAR
adB run <archivo.adB>         # Compilar y ejecutar
adB build <archivo.adB>       # Solo compilar
adB check <archivo.adB>       # Verificar sintaxis sin compilar

# OPTIMIZACIÓN
adB opt <archivo.adB>         # Compilación ultra-optimizada
adB build <archivo.adB> --size   # Optimización agresiva
adB build <archivo.adB> --ultra  # Optimización máxima (<1KB)

# INFORMACIÓN
adB help                      # Mostrar ayuda
adB version                   # Mostrar versión
adB gpu                       # Info de GPU detectada

# MODO INTERACTIVO
adB play                      # REPL interactivo
```

### 🔧 Ejemplo Completo: De Cero a Ejecutable

```bash
# 1. Crear proyecto
cargo run --bin adeadc -- new mi_app

# 2. Ver estructura
ls mi_app/

# 3. Ejecutar (modo desarrollo)
cargo run --bin adeadc -- run mi_app/main.adB

# 4. Compilar a ejecutable
cargo run --bin adeadc -- build mi_app/main.adB

# 5. Ejecutar el binario generado
./mi_app.exe    # Windows
./mi_app        # Linux
```

---

## 🚀 Optimización: Más Pequeño que ASM

ADead-BIB genera binarios **más pequeños que ensamblador tradicional** porque:

1. **Sin linker externo** — No hay overhead de linking
2. **PE directo** — Generamos el ejecutable byte a byte
3. **Optimización binaria** — Patrones de código compactos
4. **Headers mínimos** — Solo lo necesario para Windows/Linux

### Comparación de Tamaños (Hello World)

| Herramienta | Tamaño | vs ADead-BIB |
|-------------|--------|--------------|
| **ADead-BIB Ultra** | **~1 KB** | — |
| **ADead-BIB Normal** | **~1.5 KB** | — |
| NASM + link | ~4 KB | 4x más grande |
| MASM + link | ~4 KB | 4x más grande |
| GCC (C) | ~50 KB | 50x más grande |
| Rust | ~150 KB | 150x más grande |
| Go | ~2 MB | 2000x más grande |

### Técnicas de Optimización

```
NIVEL 1 (Basic):
  • Eliminación de NOPs
  • Optimización de MOV (mov rax,0 → xor eax,eax)

NIVEL 2 (Aggressive):
  • Fusión de instrucciones (mov rsp,rbp; pop rbp → leave)
  • Compresión de constantes
  • Saltos cortos (jmp rel32 → jmp rel8)

NIVEL 3 (Ultra):
  • Eliminación de stack ops redundantes
  • Encodings más cortos
  • Headers PE mínimos
```

### Comparación con Rust

| Rust | ADead-BIB |
|------|-----------|
| `cargo new hola` | `adB create hola` |
| `cargo run` | `adB run main.adB` |
| `cargo build` | `adB build main.adB` |
| `cargo check` | `adB check main.adB` |

---

## Filosofía Core

```
COMPILADORES TRADICIONALES (7+ capas):
  Código → Tokens → AST → IR → Optimizer → ASM → Assembler → Linker → Binario

ADead-BIB (2-3 capas):
  Código → AST → BYTES DIRECTOS → Binario/HEX
```

### Principios Fundamentales

1. **Sin ASM intermedio** — Emitimos bytes x86-64 directamente
2. **Sin linker externo** — Generamos PE/ELF completos en memoria
3. **Sin runtime pesado** — El binario es autosuficiente
4. **HEX es ciudadano de primera clase** — Puedes escribir bytes literales
5. **OOP Puro** — Objetos como memoria plana, métodos como funciones
6. **CPU y GPU trabajan por separado** — Contratos directos para cada uno

---

## Estado Actual del Proyecto

### Componentes Implementados

| Componente | Estado | Archivos Principales |
|------------|--------|---------------------|
| **Lexer** | ✅ Completo | `frontend/lexer.rs` |
| **Parser** | ✅ Completo | `frontend/parser.rs` |
| **AST** | ✅ Completo | `frontend/ast.rs` |
| **Type Checker** | ⚠️ Básico | `frontend/type_checker.rs` |
| **CPU Codegen** | ✅ Funcional | `backend/cpu/codegen_v2.rs` |
| **Binary Raw** | ✅ Funcional | `backend/cpu/binary_raw.rs` |
| **PE Generator** | ✅ Funcional | `backend/cpu/pe.rs`, `pe_tiny.rs` |
| **ELF Generator** | ✅ Funcional | `backend/cpu/elf.rs` |
| **GPU HEX** | ✅ Funcional | `backend/gpu/hex/` |
| **SPIR-V** | ✅ Funcional | `backend/gpu/spirv/` |
| **CUDA** | ✅ Funcional | `backend/gpu/cuda/` |
| **Vulkan Runtime** | ✅ Funcional | `backend/gpu/vulkan_runtime.rs` |
| **🆕 HIP-CPU** | ✅ Funcional | `backend/gpu/hip/hip_cpu.rs` |
| **🆕 HIP Runtime** | ✅ Funcional | `backend/gpu/hip/hip_runtime.rs` |
| **🆕 CUDA→HIP** | ✅ Funcional | `backend/gpu/hip/cuda_to_hip.rs` |
| **🆕 Compute API** | ✅ Funcional | `backend/gpu/compute.rs` |

### Estructura del Proyecto

```
ADead-BIB/
├── src/rust/
│   ├── frontend/           # Lexer, Parser, AST, TypeChecker
│   │   ├── lexer.rs
│   │   ├── parser.rs
│   │   ├── ast.rs
│   │   └── type_checker.rs
│   │
│   ├── backend/
│   │   ├── cpu/            # Binario x86-64 directo
│   │   │   ├── codegen_v2.rs
│   │   │   ├── binary_raw.rs
│   │   │   ├── pe.rs / pe_tiny.rs
│   │   │   ├── elf.rs
│   │   │   └── syscalls.rs
│   │   │
│   │   └── gpu/            # HEX/SPIR-V/CUDA
│   │       ├── hex/
│   │       ├── spirv/
│   │       ├── cuda/
│   │       ├── vulkan_runtime.rs
│   │       └── unified_pipeline.rs
│   │
│   ├── optimizer/
│   ├── runtime/
│   ├── main.rs
│   └── builder.rs
│
├── TESTEO/
│   ├── CPU/                # Tests CPU (Binario)
│   ├── GPU/                # Tests GPU (HEX)
│   └── v2/                 # Tests v2.0
│
├── examples/
├── docs/
├── Project/                # Template de proyecto (Arquitectura Dual)
│   ├── main.adB            # Binario base (entrypoint)
│   ├── call.adB            # Lógica OOP pura
│   ├── core/               # Intrínsecos del sistema
│   ├── cpu/                # Módulos CPU
│   ├── gpu/                # Módulos GPU
│   └── build.adB           # Configuración de build
└── Metal_Dead/             # Proyecto AI personal
```

---

## Arquitectura Binaria Dual (main.adB + call.adB)

### Concepto

Separación de **BINARIO FUNDAMENTAL** de **BINARIO DE COMPORTAMIENTO**.

| Archivo | Rol | Contenido |
|---------|-----|-----------|
| `main.adB` | Binario **estable** | Entrypoint, init, shutdown |
| `call.adB` | Binario **evolutivo** | OOP, lógica, comportamiento |

### Flujo de Ejecución

```
main.adB::_start()
    ↓
core::init()
    ↓
call::run()  ──→  [OOP puro en call.adB]
    ↓
core::shutdown()
    ↓
exit
```

### Beneficios

- **Código limpio** — Separación clara de responsabilidades
- **OOP sin runtime** — VTable = tabla binaria
- **Binarios estables** — main.adB cambia poco
- **Evolución segura** — Cambias lógica sin tocar core

### Template

Ver carpeta `Project/` para un ejemplo completo de esta arquitectura.

---

## Versiones Completadas

### v0.5.0 - v1.6.0 ✅ (Fundamentos)

- [x] Sintaxis híbrida Rust/Python
- [x] Compilación directa a bytes x86-64
- [x] Control de flujo (if, while, for)
- [x] Funciones con calling convention Windows x64
- [x] OOP básico (structs, impl)
- [x] Arrays y módulos
- [x] `input()` para entrada de usuario
- [x] GPU básico (Vulkan/CUDA)

### v2.0.0 ✅ (HEX-First Architecture)

- [x] **Literales HEX**: `0xFF`, `0x1234`, `0xFF_FF`
- [x] **Literales Binarios**: `0b11110000`, `0b1111_0000`
- [x] **Literales Octales**: `0o755`, `0o777`
- [x] **Separadores estilo Rust**: `0xFF_FF`, `0b1111_0000`
- [x] **Backend CPU reorganizado**: codegen_v2, binary_raw, pe_tiny
- [x] **Backend GPU reorganizado**: hex/, spirv/, cuda/
- [x] **Tests organizados**: CPU/, GPU/, v2/

---

## Roadmap de Desarrollo

### Fase 1: Instrucciones Directas

#### v2.1.0 — Módulo `cpu::` ✅

**Objetivo:** Funciones que emiten instrucciones x86-64 directamente.

```rust
fn optimized_loop() {
    cpu::mov(rcx, 1000000)   // Emite: 48 B9 [imm64]
    cpu::xor(rax, rax)       // Emite: 48 31 C0
    
    loop {
        cpu::inc(rax)        // Emite: 48 FF C0
        cpu::dec(rcx)        // Emite: 48 FF C9
        if rcx == 0 { break }
    }
}
```

**Tareas:**
- [x] Implementar módulo `cpu::` con funciones de instrucciones ✅
- [x] Registros como constantes tipadas (rax, rbx, rcx, etc.) ✅
- [x] Validación de operandos en tiempo de compilación ✅
- [x] Tests para cada instrucción ✅

**Implementado en:** `Project/cpu/mod.adB`, `TESTEO/cpu/test_cpu_instructions.adB`

#### v2.2.0 — Módulo `gpu::` ✅

**Objetivo:** Funciones que emiten opcodes GPU directamente.

```rust
fn gpu_compute() {
    gpu::init()                          // 0xC0DA0001
    gpu::alloc(4096, reg0)               // 0xC0DA0010
    gpu::matmul(reg0, reg1, reg2)        // 0xC0DA0020
    gpu::sync()                          // 0xC0DA00F0
}
```

**Tareas:**
- [x] Implementar módulo `gpu::` con funciones de opcodes ✅
- [x] Registros GPU como constantes ✅
- [x] Generación automática de command buffer ✅
- [x] Tests para cada opcode ✅

**Implementado en:** `Project/gpu/mod.adB`, `TESTEO/gpu/test_gpu_opcodes.adB`

---

### Fase 2: Bytes Directos

#### v2.3.0 — Macro `emit![]` ✅

**Objetivo:** Insertar bytes directamente en el flujo de código.

```rust
fn fast_function() {
    emit![0x48, 0x31, 0xC0]  // xor rax, rax
    emit![0xC3]              // ret
}
```

**Tareas:**
- [x] Implementar macro `emit![]` en el parser ✅
- [x] Validación de bytes en tiempo de compilación ✅
- [x] Integración con el flujo de código existente ✅
- [x] Bloque `unsafe` requerido para emit![] ✅

**Implementado en:** `Project/cpu/mod.adB` y `Project/call.adB`

#### v2.4.0 — Modo Raw Binary ✅

**Objetivo:** Compilar a bytes puros sin headers PE/ELF.

```rust
#![mode(raw)]
#![base(0x1000)]

fn _start() {
    // Genera solo los bytes de código
}
```

**Tareas:**
- [x] Implementar atributo `#![mode(raw)]` ✅
- [x] Implementar atributo `#![base(addr)]` ✅
- [x] Generador de .bin sin headers ✅
- [ ] Soporte para bootloaders y bare metal (futuro)

**Implementado en:** `src/rust/frontend/ast.rs` (ProgramAttributes, OutputMode)

---

### Fase 3: Compute Unificado

#### v2.5.0 — API Compute Unificada ✅ (NUEVO!)

**Objetivo:** API de alto nivel que abstrae GPU (CUDA/HIP) y CPU (SIMD) con auto-dispatch.

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    ADead-BIB Compute Backend v2.5                       │
├─────────────────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                   API Unificada (compute::)                      │   │
│  │   compute::vector_add(a, b, c, n)                               │   │
│  │   compute::matmul(A, B, C, m, n, k)                             │   │
│  │   compute::parallel_for(n, |i| { ... })                         │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│              ┌───────────────┼───────────────┐                         │
│              ▼               ▼               ▼                         │
│  ┌──────────────────┐ ┌──────────────┐ ┌──────────────┐                │
│  │   CUDA Backend   │ │  HIP-CPU     │ │   Vulkan     │                │
│  │   (RTX 3060)     │ │  (Fallback)  │ │   (Portable) │                │
│  └──────────────────┘ └──────────────┘ └──────────────┘                │
└─────────────────────────────────────────────────────────────────────────┘
```

**Sintaxis ADead-BIB:**
```rust
// Auto-detecta el mejor backend (CUDA si hay GPU, HIP-CPU si no)
let runtime = Compute::auto()

// Operaciones vectoriales
runtime.vector_add(a, b, c, n)
runtime.saxpy(2.5, x, y, n)
let dot = runtime.dot_product(a, b, n)

// Multiplicación de matrices (usa cuBLAS en CUDA)
runtime.matmul(A, B, C, m, n, k)

// Tensor Cores FP16 (RTX 3060)
runtime.matmul_fp16(A_fp16, B_fp16, C, m, n, k)

// Deep Learning activations
runtime.relu(input, output, n)
runtime.softmax(input, output, n)

// Async Streams
let stream = runtime.create_stream()
runtime.async_matmul(stream, a, b, c, m, n, k)
runtime.sync_stream(stream)

// Parallel for personalizado
runtime.parallel_for(n, |i| {
    result[i] = a[i] * b[i]
})

// Benchmark
let results = runtime.benchmark()
results.print()
```

**Tareas v2.5.0:**
- [x] HIP-CPU Runtime (fallback paralelo con SIMD AVX2/512) ✅
- [x] Detección automática de backend (CUDA/ROCm/CPU) ✅
- [x] Traductor CUDA → HIP para portabilidad ✅
- [x] API unificada ComputeRuntime ✅
- [x] Operaciones vectoriales (vector_add, saxpy, dot_product) ✅
- [x] Multiplicación de matrices (matmul, matmul_tiled) ✅
- [x] Reducciones (reduce_sum, reduce_max, reduce_min) ✅
- [x] SendPtr<T> wrapper thread-safe para closures paralelas ✅
- [x] Benchmark integrado ✅
- [x] Documentación: `docs/HIP_CUDA_GUIDE.md` ✅
- [x] Módulo ADead-BIB: `Project/compute/mod.adB` ✅

**Implementado en:**
- `src/rust/backend/gpu/hip/` - HIP backend completo
- `src/rust/backend/gpu/compute.rs` - API unificada Rust
- `Project/compute/mod.adB` - API ADead-BIB
- `docs/HIP_CUDA_GUIDE.md` - Documentación

**Características avanzadas (en progreso):**
- [ ] cuBLAS integration para MatMul ultra-optimizado
- [ ] Tensor Cores FP16 para RTX 3060
- [ ] Async Streams para overlap compute+transfer
- [ ] Multi-GPU support

---

### Fase 4: Formatos Avanzados

#### v2.6.0 — Formato AHYB (ADead Hybrid Binary)

**Objetivo:** Binario que contiene código CPU + GPU en un solo archivo.

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

**Tareas:**
- [ ] Definir especificación AHYB completa
- [ ] Generador de archivos .ahyb
- [ ] Loader de archivos .ahyb
- [ ] Runtime mínimo para dispatch CPU/GPU

#### v2.6.0 — Intel HEX Output

**Objetivo:** Generar archivos .hex estándar para programadores.

**Tareas:**
- [ ] Implementar generador Intel HEX
- [ ] Soporte para múltiples segmentos
- [ ] Checksums automáticos

---

### Fase 4: Optimización

#### v2.7.0 — Post-Procesamiento ✅

**Objetivo:** Eliminar ruido del binario final.

| Optimización | Descripción | Ahorro Estimado | Estado |
|--------------|-------------|-----------------|--------|
| **Strip padding** | Eliminar bytes de relleno | ~20% | ✅ |
| **Dead code removal** | Eliminar código no alcanzable | ~10% | ✅ |
| **Constant folding** | `2 + 3` → `5` en compilación | ~5% | ✅ |
| **String dedup** | Strings duplicados → una copia | ~5% | ✅ |
| **NOP elimination** | Eliminar NOPs innecesarios | ~3% | ✅ |

```rust
#![clean(normal)]      // Default
#![clean(aggressive)]  // Binario más pequeño
#![clean(none)]        // Sin limpieza (debug)
```

**Implementado en:** `src/rust/optimizer/binary_optimizer.rs`

**Comandos CLI:**
```bash
adB opt archivo.adB          # Optimización ultra
adB build archivo.adB --size # Optimización agresiva
adB build archivo.adB --ultra # Optimización máxima
```

#### v2.8.0 — Peephole Optimizer ✅

**Objetivo:** Optimizaciones locales de secuencias de bytes.

- [x] Patrones comunes de instrucciones ✅
- [x] Reemplazo de secuencias ineficientes ✅
- [x] Alineación inteligente ✅

**Implementado en:** `src/rust/optimizer/binary_optimizer.rs`

---

### Fase 5: OOP Avanzado — El Corazón de ADead-BIB 💎

> **"OOP sin runtime pesado. Objetos como memoria plana. Métodos como funciones puras."**

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    ADead-BIB OOP — Arquitectura Binaria                      │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│   ┌─────────────┐     ┌─────────────┐     ┌─────────────┐                   │
│   │   struct    │───▶│    impl     │────▶│   trait     │                   │
│   │  (Memoria)  │     │  (Métodos)  │     │ (Contrato)  │                   │
│   └─────────────┘     └─────────────┘     └─────────────┘                   │
│         │                   │                   │                            │
│         ▼                   ▼                   ▼                            │
│   ┌─────────────────────────────────────────────────────┐                   │
│   │              BYTES DIRECTOS (sin GC)                │                   │
│   │   • Struct = Layout de memoria contigua             │                   │
│   │   • Método = Función con self como primer arg       │                   │
│   │   • Vtable = Tabla de punteros a funciones          │                   │
│   └─────────────────────────────────────────────────────┘                   │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### v3.0.0 — OOP Core Spec ✅

**Objetivo:** Sistema OOP completo sin runtime pesado.

##### 📦 Structs — Datos como Memoria Plana

```rust
// Struct simple - Layout contiguo en memoria
struct Vec2 {
    x: f32,    // offset 0
    y: f32     // offset 4
}

// Struct con múltiples tipos
struct Player {
    name: string,      // offset 0  (puntero)
    position: Vec2,    // offset 8  (inline)
    health: i32,       // offset 16
    mana: i32,         // offset 20
    level: u8,         // offset 24
    is_alive: bool     // offset 25
}

// Struct genérico (v3.2+)
struct Container<T> {
    data: T,
    size: u64
}
```

##### 🔧 Impl — Métodos sin Magia

```rust
impl Vec2 {
    // Constructor estático
    fn new(x: f32, y: f32) -> Vec2 {
        return Vec2 { x: x, y: y }
    }
    
    // Constructor con valores por defecto
    fn zero() -> Vec2 {
        return Vec2 { x: 0.0, y: 0.0 }
    }
    
    // Método de instancia (self = primer argumento)
    fn length(self) -> f32 {
        return sqrt(self.x * self.x + self.y * self.y)
    }
    
    // Método que modifica (self mutable)
    fn normalize(mut self) {
        let len = self.length()
        if len > 0.0 {
            self.x = self.x / len
            self.y = self.y / len
        }
    }
    
    // Método que retorna nuevo valor
    fn add(self, other: Vec2) -> Vec2 {
        return Vec2 {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
    
    // Operadores sobrecargados (v3.1+)
    fn __add__(self, other: Vec2) -> Vec2 {
        return self.add(other)
    }
    
    fn __mul__(self, scalar: f32) -> Vec2 {
        return Vec2 { x: self.x * scalar, y: self.y * scalar }
    }
}

impl Player {
    fn new(name: string) -> Player {
        return Player {
            name: name,
            position: Vec2::zero(),
            health: 100,
            mana: 50,
            level: 1,
            is_alive: true
        }
    }
    
    fn take_damage(mut self, amount: i32) {
        self.health = self.health - amount
        if self.health <= 0 {
            self.health = 0
            self.is_alive = false
            println("Player died!")
        }
    }
    
    fn heal(mut self, amount: i32) {
        if self.is_alive {
            self.health = self.health + amount
            if self.health > 100 {
                self.health = 100
            }
        }
    }
    
    fn move_to(mut self, x: f32, y: f32) {
        self.position.x = x
        self.position.y = y
    }
    
    fn info(self) {
        print("Player: ")
        println(self.name)
        print("  Position: (")
        print(self.position.x)
        print(", ")
        print(self.position.y)
        println(")")
        print("  Health: ")
        print(self.health)
        print("/100  Mana: ")
        print(self.mana)
        print("/50  Level: ")
        println(self.level)
    }
}
```

##### 📜 Traits — Contratos sin Herencia

```rust
// Trait básico
trait Drawable {
    fn draw(self)
    fn get_bounds(self) -> (f32, f32, f32, f32)
}

// Trait con método por defecto
trait Updatable {
    fn update(mut self, delta_time: f32)
    
    // Método con implementación por defecto
    fn should_update(self) -> bool {
        return true
    }
}

// Trait para serialización
trait Serializable {
    fn to_bytes(self) -> [u8]
    fn from_bytes(data: [u8]) -> Self
}

// Trait para comparación
trait Comparable {
    fn equals(self, other: Self) -> bool
    fn less_than(self, other: Self) -> bool
    
    fn greater_than(self, other: Self) -> bool {
        return other.less_than(self)
    }
}

// Implementar trait para struct
impl Drawable for Player {
    fn draw(self) {
        print("Drawing player at (")
        print(self.position.x)
        print(", ")
        print(self.position.y)
        println(")")
    }
    
    fn get_bounds(self) -> (f32, f32, f32, f32) {
        return (self.position.x - 16.0, 
                self.position.y - 16.0,
                self.position.x + 16.0,
                self.position.y + 16.0)
    }
}

impl Updatable for Player {
    fn update(mut self, delta_time: f32) {
        // Regenerar mana con el tiempo
        if self.mana < 50 {
            self.mana = self.mana + 1
        }
    }
    
    fn should_update(self) -> bool {
        return self.is_alive
    }
}
```

##### 🎮 Ejemplo Completo: Sistema de Juego

```rust
// ============================================================================
// SISTEMA DE ENTIDADES COMPLETO
// ============================================================================

struct Entity {
    id: u64,
    position: Vec2,
    velocity: Vec2,
    active: bool
}

struct Enemy {
    entity: Entity,
    health: i32,
    damage: i32,
    ai_state: u8
}

struct Projectile {
    entity: Entity,
    owner_id: u64,
    damage: i32,
    lifetime: f32
}

// Trait para entidades del juego
trait GameEntity {
    fn get_id(self) -> u64
    fn get_position(self) -> Vec2
    fn set_position(mut self, pos: Vec2)
    fn is_active(self) -> bool
    fn destroy(mut self)
}

impl GameEntity for Enemy {
    fn get_id(self) -> u64 { return self.entity.id }
    fn get_position(self) -> Vec2 { return self.entity.position }
    fn set_position(mut self, pos: Vec2) { self.entity.position = pos }
    fn is_active(self) -> bool { return self.entity.active }
    fn destroy(mut self) { self.entity.active = false }
}

// Sistema de colisiones
trait Collidable {
    fn get_hitbox(self) -> (f32, f32, f32, f32)
    fn on_collision(mut self, other_id: u64)
}

impl Collidable for Player {
    fn get_hitbox(self) -> (f32, f32, f32, f32) {
        return (self.position.x - 8.0, self.position.y - 8.0,
                self.position.x + 8.0, self.position.y + 8.0)
    }
    
    fn on_collision(mut self, other_id: u64) {
        println("Player collided with entity!")
    }
}

// GameState - Contenedor principal
struct GameState {
    player: Player,
    enemies: [Enemy; 100],
    enemy_count: u32,
    projectiles: [Projectile; 500],
    projectile_count: u32,
    score: u64,
    game_time: f32,
    is_running: bool
}

impl GameState {
    fn new() -> GameState {
        return GameState {
            player: Player::new("Hero"),
            enemies: [Enemy::default(); 100],
            enemy_count: 0,
            projectiles: [Projectile::default(); 500],
            projectile_count: 0,
            score: 0,
            game_time: 0.0,
            is_running: true
        }
    }
    
    fn spawn_enemy(mut self, x: f32, y: f32) {
        if self.enemy_count < 100 {
            self.enemies[self.enemy_count] = Enemy {
                entity: Entity {
                    id: self.enemy_count as u64,
                    position: Vec2::new(x, y),
                    velocity: Vec2::zero(),
                    active: true
                },
                health: 50,
                damage: 10,
                ai_state: 0
            }
            self.enemy_count = self.enemy_count + 1
        }
    }
    
    fn update(mut self, delta_time: f32) {
        self.game_time = self.game_time + delta_time
        
        // Actualizar player
        if self.player.should_update() {
            self.player.update(delta_time)
        }
        
        // Actualizar enemigos
        for i in 0..self.enemy_count {
            if self.enemies[i].entity.active {
                // AI simple: moverse hacia el player
                let dx = self.player.position.x - self.enemies[i].entity.position.x
                let dy = self.player.position.y - self.enemies[i].entity.position.y
                let dist = sqrt(dx * dx + dy * dy)
                
                if dist > 0.0 {
                    self.enemies[i].entity.velocity.x = (dx / dist) * 50.0
                    self.enemies[i].entity.velocity.y = (dy / dist) * 50.0
                }
                
                self.enemies[i].entity.position.x += self.enemies[i].entity.velocity.x * delta_time
                self.enemies[i].entity.position.y += self.enemies[i].entity.velocity.y * delta_time
            }
        }
    }
    
    fn render(self) {
        // Dibujar player
        self.player.draw()
        
        // Dibujar enemigos
        for i in 0..self.enemy_count {
            if self.enemies[i].entity.active {
                print("Enemy at (")
                print(self.enemies[i].entity.position.x)
                print(", ")
                print(self.enemies[i].entity.position.y)
                println(")")
            }
        }
    }
}

// Main del juego
fn main() {
    let mut game = GameState::new()
    
    // Spawn algunos enemigos
    game.spawn_enemy(100.0, 100.0)
    game.spawn_enemy(200.0, 50.0)
    game.spawn_enemy(150.0, 200.0)
    
    // Game loop simulado
    let delta_time = 0.016  // ~60 FPS
    
    for frame in 0..100 {
        game.update(delta_time)
        
        if frame % 10 == 0 {
            game.render()
        }
    }
    
    println("Game finished!")
    print("Final score: ")
    println(game.score)
}
```

**Tareas v3.0.0:**
- [x] Especificación formal de structs ✅
- [x] Especificación formal de impl ✅
- [x] Especificación formal de traits ✅
- [x] Vtables simples para polimorfismo ✅
- [x] Documentación completa ✅

**Implementado en:** `Project/call.adB`

---

#### v3.1.0 — Herencia y Composición

**Objetivo:** Herencia simple + composición preferida.

```rust
// Herencia simple (un solo nivel)
struct Entity {
    id: u64,
    x: f32,
    y: f32
}

struct Player extends Entity {
    health: i32,
    name: string
}

// Player hereda id, x, y de Entity
let player = Player { id: 1, x: 0.0, y: 0.0, health: 100, name: "Hero" }

// Composición (PREFERIDA en ADead-BIB)
struct PlayerComposed {
    entity: Entity,    // Composición explícita
    health: i32,
    name: string
}

// Acceso: player.entity.x vs player.x (herencia)
```

##### Sobrecarga de Operadores

```rust
impl Vec2 {
    // Operadores aritméticos
    fn __add__(self, other: Vec2) -> Vec2 { ... }
    fn __sub__(self, other: Vec2) -> Vec2 { ... }
    fn __mul__(self, scalar: f32) -> Vec2 { ... }
    fn __div__(self, scalar: f32) -> Vec2 { ... }
    fn __neg__(self) -> Vec2 { ... }
    
    // Operadores de comparación
    fn __eq__(self, other: Vec2) -> bool { ... }
    fn __ne__(self, other: Vec2) -> bool { ... }
    
    // Indexación
    fn __index__(self, i: u32) -> f32 { ... }
    fn __index_mut__(mut self, i: u32) -> mut f32 { ... }
}

// Uso natural
let a = Vec2::new(1.0, 2.0)
let b = Vec2::new(3.0, 4.0)
let c = a + b           // __add__
let d = c * 2.0         // __mul__
let e = -d              // __neg__
let x = c[0]            // __index__ -> 4.0
```

**Tareas v3.1.0:**
- [ ] Herencia simple (extends)
- [ ] Sobrecarga de operadores (__add__, __mul__, etc.)
- [ ] Indexación personalizada (__index__)
- [ ] Conversiones implícitas (Into, From traits)

---

#### v3.2.0 — Genéricos y Tipos Avanzados

**Objetivo:** Tipos genéricos sin monomorphization pesado.

```rust
// Struct genérico
struct Option<T> {
    has_value: bool,
    value: T
}

impl<T> Option<T> {
    fn some(value: T) -> Option<T> {
        return Option { has_value: true, value: value }
    }
    
    fn none() -> Option<T> {
        return Option { has_value: false, value: T::default() }
    }
    
    fn unwrap(self) -> T {
        if !self.has_value {
            panic("Unwrap on None!")
        }
        return self.value
    }
    
    fn unwrap_or(self, default: T) -> T {
        if self.has_value {
            return self.value
        }
        return default
    }
    
    fn map<U, F>(self, f: F) -> Option<U> 
    where F: Fn(T) -> U {
        if self.has_value {
            return Option::some(f(self.value))
        }
        return Option::none()
    }
}

// Result para manejo de errores
struct Result<T, E> {
    is_ok: bool,
    ok_value: T,
    err_value: E
}

impl<T, E> Result<T, E> {
    fn ok(value: T) -> Result<T, E> {
        return Result { is_ok: true, ok_value: value, err_value: E::default() }
    }
    
    fn err(error: E) -> Result<T, E> {
        return Result { is_ok: false, ok_value: T::default(), err_value: error }
    }
    
    fn is_ok(self) -> bool { return self.is_ok }
    fn is_err(self) -> bool { return !self.is_ok }
    
    fn unwrap(self) -> T {
        if !self.is_ok {
            panic("Unwrap on Err!")
        }
        return self.ok_value
    }
}

// Vec dinámico
struct Vec<T> {
    data: ptr<T>,
    len: u64,
    capacity: u64
}

impl<T> Vec<T> {
    fn new() -> Vec<T> {
        return Vec { data: null, len: 0, capacity: 0 }
    }
    
    fn with_capacity(cap: u64) -> Vec<T> {
        return Vec {
            data: alloc(cap * sizeof(T)),
            len: 0,
            capacity: cap
        }
    }
    
    fn push(mut self, item: T) {
        if self.len >= self.capacity {
            self.grow()
        }
        self.data[self.len] = item
        self.len = self.len + 1
    }
    
    fn pop(mut self) -> Option<T> {
        if self.len == 0 {
            return Option::none()
        }
        self.len = self.len - 1
        return Option::some(self.data[self.len])
    }
    
    fn get(self, index: u64) -> Option<T> {
        if index >= self.len {
            return Option::none()
        }
        return Option::some(self.data[index])
    }
}

// HashMap básico
struct HashMap<K, V> {
    buckets: [Option<(K, V)>; 256],
    len: u64
}

impl<K: Hashable, V> HashMap<K, V> {
    fn new() -> HashMap<K, V> { ... }
    fn insert(mut self, key: K, value: V) { ... }
    fn get(self, key: K) -> Option<V> { ... }
    fn remove(mut self, key: K) -> Option<V> { ... }
}
```

**Tareas v3.2.0:**
- [ ] Structs genéricos (`struct Option<T>`)
- [ ] Impl genéricos (`impl<T> Option<T>`)
- [ ] Traits con tipos asociados
- [ ] Where clauses
- [ ] Tipos built-in: Option, Result, Vec, HashMap

---

#### v3.3.0 — Pattern Matching y Enums

**Objetivo:** Enums con datos y pattern matching exhaustivo.

```rust
// Enum simple
enum Direction {
    Up,
    Down,
    Left,
    Right
}

// Enum con datos (tagged union)
enum Event {
    KeyPress { key: u8, modifiers: u8 },
    MouseMove { x: i32, y: i32 },
    MouseClick { button: u8, x: i32, y: i32 },
    WindowResize { width: u32, height: u32 },
    Quit
}

// Pattern matching
fn handle_event(event: Event) {
    match event {
        Event::KeyPress { key, modifiers } => {
            print("Key pressed: ")
            println(key)
        },
        Event::MouseMove { x, y } => {
            print("Mouse at: ")
            print(x)
            print(", ")
            println(y)
        },
        Event::MouseClick { button, x, y } => {
            print("Click button ")
            print(button)
            print(" at ")
            print(x)
            print(", ")
            println(y)
        },
        Event::WindowResize { width, height } => {
            print("Window resized to ")
            print(width)
            print("x")
            println(height)
        },
        Event::Quit => {
            println("Quitting...")
        }
    }
}

// Pattern matching con guards
fn categorize_number(n: i32) -> string {
    match n {
        0 => "zero",
        1..=9 => "single digit",
        10..=99 => "double digit",
        n if n < 0 => "negative",
        _ => "large"
    }
}

// Destructuring en let
let point = (10, 20)
let (x, y) = point

let player = Player::new("Hero")
let Player { name, health, .. } = player
```

**Tareas v3.3.0:**
- [ ] Enums simples
- [ ] Enums con datos (tagged unions)
- [ ] Pattern matching exhaustivo
- [ ] Guards en match
- [ ] Destructuring

---

### Fase 6: Ecosistema

#### v3.2.0 — Sistema de Módulos

**Objetivo:** Importar código de otros archivos.

```rust
use math::Vector2
use graphics::Sprite

fn main() {
    let pos = Vector2::new(10, 20)
}
```

#### v3.3.0 — Gestor de Paquetes

**Objetivo:** Sistema simple de dependencias.

```toml
# adead.toml
[package]
name = "my_game"
version = "1.0.0"

[dependencies]
math = "0.1.0"
graphics = "0.2.0"
```

---

### Fase 7: ASM Simbionte — Interoperabilidad Universal

#### v4.0.0 — ASM Simbionte Core

**Objetivo:** Crear un sistema de "ASM Simbionte" que permita a ADead-BIB combinarse con **cualquier lenguaje OOP** de forma nativa, sin dependencias externas.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        ASM SIMBIONTE — Arquitectura                          │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────┐   ┌─────────────┐   ┌─────────────┐   ┌─────────────┐      │
│  │   Python    │   │    Java     │   │     C#      │   │    Rust     │      │
│  │   (OOP)     │   │   (OOP)     │   │   (OOP)     │   │   (OOP)     │      │
│  └──────┬──────┘   └──────┬──────┘   └──────┬──────┘   └──────┬──────┘      │
│         │                 │                 │                 │              │
│         ▼                 ▼                 ▼                 ▼              │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                    ADead-BIB ASM Simbionte                          │    │
│  │                                                                     │    │
│  │   • FFI Universal (Foreign Function Interface)                     │    │
│  │   • ABI Estándar (Application Binary Interface)                    │    │
│  │   • Vtables Compatibles (Polimorfismo cross-language)              │    │
│  │   • Memory Layout Definido (Structs binarios)                      │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│         │                                                                    │
│         ▼                                                                    │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                    BYTES DIRECTOS (x86-64 / ARM64)                  │    │
│  │                    Sin NASM. Sin LLVM. Sin GAS.                     │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Características del ASM Simbionte:**

| Característica | Descripción |
|----------------|-------------|
| **FFI Universal** | Llamar funciones de cualquier lenguaje OOP |
| **ABI Estándar** | Convención de llamada compatible (cdecl, stdcall, fastcall) |
| **Vtables Simbionte** | Polimorfismo que funciona entre lenguajes |
| **Memory Layout** | Structs con layout binario definido |
| **Sin NASM** | Emitimos bytes x86-64 directamente |
| **Sin LLVM** | No dependemos de infraestructura externa |

**Sintaxis propuesta:**

```rust
// Exportar función para otros lenguajes
#[export("C")]
fn calculate(x: i32, y: i32) -> i32 {
    return x + y
}

// Importar función de otro lenguaje
#[import("python", "numpy.add")]
extern fn numpy_add(a: ptr, b: ptr) -> ptr

// Struct con layout binario compatible
#[repr(C)]
struct Point {
    x: f32,
    y: f32
}

// Clase simbionte (compatible con OOP de otros lenguajes)
#[symbiont]
class Entity {
    x: i32
    y: i32
    
    fn new(x: i32, y: i32) -> Entity
    fn move(dx: i32, dy: i32)
}
```

**Tareas:**
- [ ] Definir ABI Simbionte estándar
- [ ] Implementar FFI para C/C++
- [ ] Implementar FFI para Python (ctypes)
- [ ] Implementar FFI para Rust
- [ ] Vtables compatibles cross-language
- [ ] Documentación de interoperabilidad

#### v4.1.0 — Bindings Automáticos

**Objetivo:** Generar bindings automáticamente para otros lenguajes.

```bash
# Generar bindings
adB bind mylib.adB --python    # Genera mylib.py
adB bind mylib.adB --rust      # Genera mylib.rs
adB bind mylib.adB --c         # Genera mylib.h
adB bind mylib.adB --csharp    # Genera mylib.cs
```

---

### Fase 8: FastOS — Sistema Operativo Rápido y Directo

#### v5.0.0 — FastOS Kernel Core ✅ (En Desarrollo)

**Objetivo:** Crear un sistema operativo alternativo a Windows, **virgen y directo**, usando el stack **ADead-BIB + Rust + wgpu**.

**Ubicación:** `Sistema operativo/FastOS/`

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         FastOS — Arquitectura                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                      APLICACIONES (ADead-BIB)                       │    │
│  │   • Juegos (GAME/)                                                  │    │
│  │   • Utilidades                                                      │    │
│  │   • IDE nativo                                                      │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                    │                                         │
│                                    ▼                                         │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                      ADead-OS API (Syscalls)                        │    │
│  │   • File I/O                                                        │    │
│  │   • Memory Management                                               │    │
│  │   • Process Control                                                 │    │
│  │   • Graphics (GPU Direct)                                           │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                    │                                         │
│                                    ▼                                         │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                      ADead-OS KERNEL                                │    │
│  │                                                                     │    │
│  │   ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │    │
│  │   │   Scheduler  │  │    Memory    │  │   Drivers    │              │    │
│  │   │  (ADead-BIB) │  │  (ADead-BIB) │  │ (Rust/wgpu)  │              │    │
│  │   └──────────────┘  └──────────────┘  └──────────────┘              │    │
│  │                                                                     │    │
│  │   ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │    │
│  │   │  Filesystem  │  │   Network    │  │     GPU      │              │    │
│  │   │  (ADead-BIB) │  │  (ADead-BIB) │  │ (Rust/wgpu)  │              │    │
│  │   └──────────────┘  └──────────────┘  └──────────────┘              │    │
│  │                                                                     │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                    │                                         │
│                                    ▼                                         │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                      HARDWARE (x86-64 / ARM64)                      │    │
│  │   • CPU (bytes directos ADead-BIB)                                  │    │
│  │   • GPU (wgpu/Vulkan cuando necesario)                              │    │
│  │   • Memoria, Disco, Red                                             │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Filosofía FastOS:**

| Principio | Descripción |
|-----------|-------------|
| **Virgen** | Sin código legacy de Windows/Linux |
| **Directo** | Bytes directos al hardware, sin capas innecesarias |
| **Claro** | Código legible y documentado |
| **Mínimo** | Solo lo necesario, nada más |
| **Sin NASM** | Todo en ADead-BIB (bytes directos) |
| **Rust/wgpu opcional** | Solo para drivers GPU complejos |

**Componentes del Kernel:**

```
FastOS/
├── boot/
│   ├── bootloader.adB      # Bootloader en ADead-BIB (bytes directos)
│   ├── stage1.adB          # Modo real → Modo protegido
│   └── stage2.adB          # Modo protegido → Modo largo (64-bit)
│
├── kernel/
│   ├── main.adB            # Entry point del kernel
│   ├── memory.adB          # Gestión de memoria (paging, heap)
│   ├── scheduler.adB       # Planificador de procesos
│   ├── syscalls.adB        # Llamadas al sistema
│   └── interrupts.adB      # Manejo de interrupciones (IDT)
│
├── drivers/
│   ├── keyboard.adB        # Driver de teclado (PS/2, USB)
│   ├── display.adB         # Driver de pantalla (framebuffer)
│   ├── disk.adB            # Driver de disco (AHCI, NVMe)
│   ├── gpu.rs              # Driver GPU (Rust + wgpu, opcional)
│   └── network.adB         # Driver de red (básico)
│
├── fs/
│   ├── vfs.adB             # Virtual File System
│   ├── fat32.adB           # Soporte FAT32
│   └── adeadfs.adB         # Filesystem propio (simple)
│
└── userspace/
    ├── shell.adB           # Shell básica
    ├── init.adB            # Proceso init
    └── apps/               # Aplicaciones de usuario
```

**Tareas v5.0.0:**
- [x] Estructura del proyecto ✅
- [x] Bootloader básico (MBR → Modo largo) ✅
- [x] Kernel mínimo (VGA print) ✅
- [x] Gestión de memoria (bump allocator) ✅
- [x] IDT y manejo de interrupciones ✅
- [x] Driver de teclado PS/2 ✅
- [x] Shell básica con comandos ✅
- [ ] Scheduler simple (round-robin)
- [ ] Syscalls básicos (read, write, exit)

#### v5.1.0 — FastOS Graphics

**Objetivo:** Sistema gráfico nativo usando GPU directo.

```rust
// ADead-OS Graphics API
fn main() {
    let screen = Screen::init(1920, 1080)
    
    screen.clear(Color::BLACK)
    screen.draw_rect(100, 100, 200, 150, Color::RED)
    screen.draw_text(10, 10, "ADead-OS v5.1", Color::WHITE)
    screen.present()
}
```

**Tareas:**
- [ ] Framebuffer básico (VESA/GOP)
- [ ] Driver GPU con wgpu (opcional)
- [ ] Window manager simple
- [ ] Compositor básico

#### v5.2.0 — ADead-OS Userspace

**Objetivo:** Entorno de usuario completo.

- [ ] Shell interactiva
- [ ] Editor de texto
- [ ] Compilador ADead-BIB nativo
- [ ] Gestor de archivos
- [ ] Juegos de ejemplo (Flappy Bird portado)

---

### Fase 9: Ecosistema Completo

#### v6.0.0 — ADead-IDE

**Objetivo:** IDE nativo para ADead-BIB.

```
┌─────────────────────────────────────────────────────────────┐
│  ADead-IDE                                         [─][□][×]│
├─────────────────────────────────────────────────────────────┤
│  File  Edit  View  Build  Run  Help                         │
├─────────────────────────────────────────────────────────────┤
│  ┌──────────┐ ┌────────────────────────────────────────────┐│
│  │ Explorer │ │ main.adB                                   ││
│  │          │ │                                            ││
│  │ ▼ src/   │ │  fn main() {                               ││
│  │  main.adB│ │      println("Hello, ADead-OS!")           ││
│  │  lib.adB │ │  }                                         ││
│  │          │ │                                            ││
│  └──────────┘ └────────────────────────────────────────────┘│
│  ┌──────────────────────────────────────────────────────────┐
│  │ Output: ✅ Compiled successfully (0.02s, 1.2KB)          │
│  └──────────────────────────────────────────────────────────┘
└─────────────────────────────────────────────────────────────┘
```

---

## Arquitectura de Backends

### CPU Backend (Binario)

```
┌────────────────────────────────────────────────────────────┐
│                    CPU Backend                             │
├────────────────────────────────────────────────────────────┤
│                                                            │
│  Código ADead-BIB                                          │
│       ↓                                                    │
│  ┌─────────────────────────────────────────────────────┐   │
│  │              BINARY EMITTER                         │   │
│  │                                                     │   │
│  │  codegen_v2.rs  →  Genera bytes x86-64              │   │
│  │  binary_raw.rs  →  Emisor de bytes directos         │   │
│  │  pe_tiny.rs     →  PE ultra-compacto (<500 bytes)   │   │
│  │  pe.rs          →  Windows PE estándar              │   │
│  │  elf.rs         →  Linux ELF                        │   │
│  └─────────────────────────────────────────────────────┘   │
│       ↓                                                    │
│  .exe / .elf (Binario Nativo)                              │
└────────────────────────────────────────────────────────────┘
```

### GPU Backend (HEX)

```
┌─────────────────────────────────────────────────────────────┐
│                    GPU Backend                              │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Nivel 1: Opcodes ADead-BIB (0xC0DA...)                     │
│    - Formato propio y portable                              │
│    - Documentado                                            │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Nivel 2: Backend por target                                │
│    ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│    │    SPIR-V    │  │     CUDA     │  │    Vulkan    │     │
│    │  (Todas GPU) │  │   (NVIDIA)   │  │   (Runtime)  │     │
│    └──────────────┘  └──────────────┘  └──────────────┘     │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## Tablas de Referencia

### Opcodes x86-64

| Instrucción | Bytes | Descripción |
|-------------|-------|-------------|
| `push rbp` | `0x55` | Guardar base pointer |
| `mov rbp, rsp` | `0x48 0x89 0xE5` | Setup stack frame |
| `pop rbp` | `0x5D` | Restaurar base pointer |
| `ret` | `0xC3` | Retornar |
| `xor rax, rax` | `0x48 0x31 0xC0` | Limpiar rax |
| `call rel32` | `0xE8 [4 bytes]` | Llamar función |
| `jmp rel32` | `0xE9 [4 bytes]` | Salto incondicional |

### Opcodes GPU

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

### Calling Convention Windows x64

```
Parámetros: RCX, RDX, R8, R9 (primeros 4)
            Stack (resto)
Retorno:    RAX
Preservar:  RBX, RBP, RDI, RSI, R12-R15
Alineación: Stack a 16 bytes antes de call
```

---

## Comandos CLI

```bash
# ═══════════════════════════════════════════════════════════════
# CREAR PROYECTO (estilo Rust)
# ═══════════════════════════════════════════════════════════════
adB create mi_proyecto          # Crear proyecto nuevo
adB new mi_proyecto             # Alias de create
adB init                        # Inicializar en directorio actual

# ═══════════════════════════════════════════════════════════════
# COMPILAR Y EJECUTAR
# ═══════════════════════════════════════════════════════════════
adB run archivo.adB             # Compilar y ejecutar
adB build archivo.adB           # Compilar a ejecutable
adB build archivo.adB -o app.exe # Compilar con nombre específico
adB check archivo.adB           # Verificar sintaxis

# ═══════════════════════════════════════════════════════════════
# MODO INTERACTIVO
# ═══════════════════════════════════════════════════════════════
adB play                        # REPL interactivo (playground)

# ═══════════════════════════════════════════════════════════════
# MODOS AVANZADOS
# ═══════════════════════════════════════════════════════════════
adB tiny archivo.adB            # PE ultra-compacto (<500 bytes)
adB nano output.exe             # PE más pequeño posible
adB micro output.exe            # PE32 sub-256 bytes

# ═══════════════════════════════════════════════════════════════
# GPU
# ═══════════════════════════════════════════════════════════════
adB gpu                         # Detectar GPU
adB spirv matmul 1024           # Generar shader SPIR-V
adB cuda matmul 1024            # Generar código CUDA
adB unified matmul 1000000      # Pipeline unificado CPU↔GPU
```

---

## Tests

### Estructura de Tests

```
TESTEO/
├── CPU/                     # Tests CPU (Binario)
│   ├── binario/             # Literales 0b...
│   ├── opcodes/             # Opcodes x86-64
│   └── contratos/           # Calling conventions
│
├── GPU/                     # Tests GPU (HEX)
│   ├── hex/                 # Literales 0x...
│   ├── opcodes/             # Opcodes GPU
│   └── contratos/           # Command buffers
│
└── v2/                      # Tests v2.0.0
    ├── hex/
    ├── raw/
    ├── cpu/
    ├── gpu/
    └── integrados/
```

### Comandos de Test

```bash
# Tests CPU
cargo run --bin adeadc -- run TESTEO/CPU/binario/test_binary_literals.adB
cargo run --bin adeadc -- run TESTEO/CPU/opcodes/test_x86_opcodes.adB

# Tests GPU
cargo run --bin adeadc -- run TESTEO/GPU/hex/test_hex_literals.adB
cargo run --bin adeadc -- run TESTEO/GPU/opcodes/test_gpu_opcodes.adB

# Test integrado v2.0
cargo run --bin adeadc -- run TESTEO/v2/integrados/test_v2_0_0_hex_first.adB
```

---

## Tamaños de Binario

| Modo | Tamaño | Comando | Descripción |
|------|--------|---------|-------------|
| Standard | ~1.5 KB | `adeadc build` | Binario completo |
| Tiny | < 500 bytes | `adeadc tiny` | PE ultra-compacto |
| Raw | Variable | `adeadc raw` | Solo código |

### Comparación

| Lenguaje | Hello World | Runtime |
|----------|-------------|---------|
| **ADead-BIB** | **~1.5 KB** | **Ninguno** |
| Assembly | ~500 bytes | Ninguno |
| C | ~50 KB | libc |
| Rust | ~150 KB | std |
| Go | ~2 MB | Go Runtime |

---

## 🔧 Tareas Pendientes del Parser/Compilador

### Completadas Recientemente ✅

| Tarea | Estado | Archivo |
|-------|--------|---------|
| Operador módulo `%` | ✅ | `parser.rs` |
| Comparaciones `>`, `<`, `>=`, `<=` | ✅ | `parser.rs` |
| Atributos de programa `#![mode()]` | ✅ | `ast.rs` |
| Optimizador binario | ✅ | `binary_optimizer.rs` |

### Pendientes para Mejorar 🚧

| Tarea | Prioridad | Descripción |
|-------|-----------|-------------|
| **Constantes globales** | Alta | `const X = 10` no se evalúa correctamente en runtime |
| **Operadores bit a bit** | Media | `>>`, `<<`, `&`, `\|`, `^` en expresiones |
| **Operador ternario** | Baja | `x = a > b ? a : b` |
| **Asignación múltiple** | Baja | `let (a, b) = (1, 2)` |
| **Strings interpolados** | Media | `"Valor: {x}"` |
| **Arrays dinámicos** | Media | `let arr = [1, 2, 3]` con push/pop |
| **Match expressions** | Media | Pattern matching completo |
| **Closures/Lambdas** | Baja | `let f = \|x\| x * 2` |

### Motor de Juegos (GAME/) ✅

| Componente | Estado | Descripción |
|------------|--------|-------------|
| Engine Core | ✅ | Window, Renderer, Input, Time |
| ECS | ✅ | Entity, Components, World |
| Systems | ✅ | Physics, Collision, Render |
| Flappy Game | ✅ | Juego funcional de ejemplo |
| ADead-BIB Logic | ✅ | `game_logic.adB` integrado |
| Ventana redimensionable | ✅ | Soporte para maximizar |

---

## Documentación

- [README.md](README.md) — Documentación principal
- [ideas.md](ideas.md) — Documento de diseño del lenguaje
- [GUIA_ES.md](GUIA_ES.md) — Guía en español
- [docs/ESTRUCTURA.md](docs/ESTRUCTURA.md) — Estructura del proyecto
- [docs/gpu_hex_opcodes.md](docs/gpu_hex_opcodes.md) — Opcodes GPU

---

## Contribuir

1. Fork del repositorio
2. Crear rama feature: `git checkout -b feature/nueva-funcionalidad`
3. Commit cambios: `git commit -m "Agregar nueva funcionalidad"`
4. Push a la rama: `git push origin feature/nueva-funcionalidad`
5. Crear Pull Request

---

## Licencia

### GPLv2 — Resumen Claro

**ADead-BIB** está licenciado bajo **GNU General Public License v2.0 (GPLv2)**.

#### ✅ Puedes:

| Acción | Descripción |
|--------|-------------|
| **Usar** | Usar ADead-BIB para cualquier propósito (personal, comercial, educativo) |
| **Estudiar** | Leer y aprender del código fuente |
| **Modificar** | Cambiar el código para tus necesidades |
| **Distribuir** | Compartir copias del código original |
| **Distribuir modificaciones** | Compartir tus versiones modificadas |

#### ⚠️ Condiciones:

| Condición | Descripción |
|-----------|-------------|
| **Misma licencia** | Si distribuyes modificaciones, DEBEN ser GPLv2 |
| **Código fuente** | Si distribuyes binarios, DEBES incluir el código fuente |
| **Aviso de copyright** | Mantener los avisos de copyright originales |
| **Cambios documentados** | Documentar los cambios que hagas |

#### ❌ No puedes:

| Restricción | Descripción |
|-------------|-------------|
| **Cerrar el código** | No puedes hacer versiones propietarias cerradas |
| **Sublicenciar** | No puedes cambiar la licencia a otra diferente |
| **Quitar atribución** | No puedes quitar los créditos del autor original |

#### 📋 En términos simples:

> **Usa ADead-BIB libremente, pero si lo modificas y distribuyes, comparte el código.**

```
Copyright (C) 2024-2026 Eddi Andreé Salazar Matos
Email: eddi.salazar.dev@gmail.com

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; version 2 of the License.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.
```

---

## Autor

**Eddi Andreé Salazar Matos**  
📧 eddi.salazar.dev@gmail.com  
🇵🇪 Hecho con ❤️ en Perú

---

**ADead-BIB v2.0: Código → Bytes → Binario**
**OOP Puro + ASM Simbionte = El Nuevo Lenguaje**
**Sin ASM. Sin LLVM. Sin mentiras.**
