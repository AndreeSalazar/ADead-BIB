# ADead-BIB - Estructura del Proyecto

> **Binary Is Binary** - Código → Bytes Directos → Ejecutable
> 
> CPU = Binario x86-64 | GPU = HEX/SPIR-V

---

## 📁 Estructura Principal

```
ADead-BIB/
│
├── src/rust/                    # 🔵 COMPILADOR PRINCIPAL
│   ├── main.rs                  # CLI (adeadc)
│   ├── lib.rs                   # Librería
│   ├── builder.rs               # Builder de proyectos
│   │
│   ├── frontend/                # Parsing
│   │   ├── lexer.rs             # Tokenizer (HEX/BIN literals)
│   │   ├── parser.rs            # AST builder
│   │   ├── ast.rs               # Definiciones AST
│   │   ├── types.rs             # Sistema de tipos
│   │   └── type_checker.rs      # Verificador
│   │
│   ├── backend/                 # Generación de código
│   │   ├── cpu/                 # CPU - Binario x86-64
│   │   │   ├── codegen_v2.rs    # Generador principal
│   │   │   ├── binary_raw.rs    # Emisor de bytes
│   │   │   ├── pe_tiny.rs       # PE ultra-compacto
│   │   │   ├── pe.rs            # Windows PE
│   │   │   ├── elf.rs           # Linux ELF
│   │   │   └── syscalls.rs      # Syscalls OS
│   │   │
│   │   └── gpu/                 # GPU - HEX/SPIR-V
│   │       ├── hex/             # Opcodes HEX (0xC0DA...)
│   │       ├── spirv/           # Backend SPIR-V (Vulkan)
│   │       ├── cuda/            # Backend CUDA (NVIDIA)
│   │       ├── vulkan/          # Runtime Vulkan
│   │       ├── scheduler.rs     # CPU↔GPU scheduler
│   │       └── memory.rs        # Memoria GPU
│   │
│   ├── optimizer/               # Optimizaciones
│   │   ├── branchless.rs
│   │   └── simd.rs
│   │
│   └── runtime/                 # Runtime
│       ├── cpu_detect.rs
│       ├── dispatcher.rs
│       └── gpu_dispatcher.rs
│
├── TESTEO/                      # 🧪 TESTS
│   ├── CPU/                     # Tests CPU (Binario)
│   │   ├── binario/             # Literales 0b...
│   │   ├── opcodes/             # Opcodes x86-64
│   │   └── contratos/           # Calling conventions
│   │
│   ├── GPU/                     # Tests GPU (HEX)
│   │   ├── hex/                 # Literales 0x...
│   │   ├── opcodes/             # Opcodes GPU
│   │   └── contratos/           # Command buffers
│   │
│   └── v2/                      # Tests v2.0.0
│
├── examples/                    # 📝 EJEMPLOS
│   ├── hello.adB
│   ├── aritmetica.adB
│   └── engines/                 # Templates de engines
│       ├── ComputeEngine/
│       ├── GameEngine/
│       └── GraphicsEngine/
│
├── docs/                        # 📚 DOCUMENTACIÓN
│   ├── ESTRUCTURA.md            # Este archivo
│   ├── gpu_hex_opcodes.md       # Opcodes GPU
│   └── heredar_*.md             # Templates docs
│
├── Metal_Dead/                  # 🤖 IA PERSONAL (no tocar)
│
├── python/                      # 🐍 HERRAMIENTAS PYTHON
│   ├── ia_personal*.py          # IA personal
│   └── benchmark_*.py           # Benchmarks
│
├── integrations/                # 🔗 INTEGRACIONES
│   ├── javascript/
│   ├── python/
│   ├── rust/
│   └── ...
│
├── ASM simbionte/               # 📊 COMPARACIONES ASM
│
├── GAME/                        # 🎮 PROYECTO JUEGO
│
├── hex/                         # GPU HEX (Python tools)
│
└── runtime/                     # Runtime alternativo
```

---

## 🔵 Arquitectura del Compilador

```
┌─────────────────────────────────────────────────────────────┐
│                    ADead-BIB Compiler                       │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Código .adB                                                │
│      ↓                                                      │
│  ┌─────────────┐                                            │
│  │   Lexer     │  → Tokens (HEX/BIN literals)               │
│  └─────────────┘                                            │
│      ↓                                                      │
│  ┌─────────────┐                                            │
│  │   Parser    │  → AST                                     │
│  └─────────────┘                                            │
│      ↓                                                      │
│  ┌─────────────┐                                            │
│  │ TypeChecker │  → AST tipado                              │
│  └─────────────┘                                            │
│      ↓                                                      │
│  ┌─────────────────────────────────────────────────────┐    │
│  │              Backend (Generación Directa)           │    │
│  ├─────────────────────┬───────────────────────────────┤    │
│  │        CPU          │            GPU                │    │
│  │  ┌───────────────┐  │  ┌─────────────────────────┐  │    │
│  │  │ codegen_v2.rs │  │  │ hex/ (Opcodes 0xC0DA)   │  │    │
│  │  │ binary_raw.rs │  │  │ spirv/ (Vulkan/OpenCL) │  │    │
│  │  │ pe_tiny.rs    │  │  │ cuda/ (NVIDIA PTX)     │  │    │
│  │  └───────────────┘  │  └─────────────────────────┘  │    │
│  └─────────────────────┴───────────────────────────────┘    │
│      ↓                          ↓                           │
│  .exe/.elf (CPU)          .spv/.ptx (GPU)                   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## 🟢 Arquitectura GPU (Dos Niveles)

```
Nivel 1: Opcodes ADead-BIB (0xC0DA...)
  - Tu contrato
  - Tu formato
  - Portable
  - Documentado

Nivel 2: Backend por target
  - spirv/   → Vulkan/OpenCL (TODAS las GPUs)
  - cuda/    → NVIDIA (PTX directo)
  - vulkan/  → Runtime Vulkan
```

---

## 🧪 Comandos de Test

```bash
# Tests CPU
cargo run --bin adeadc -- run TESTEO/CPU/binario/test_binary_literals.adB
cargo run --bin adeadc -- run TESTEO/CPU/opcodes/test_x86_opcodes.adB

# Tests GPU
cargo run --bin adeadc -- run TESTEO/GPU/hex/test_hex_literals.adB
cargo run --bin adeadc -- run TESTEO/GPU/opcodes/test_gpu_opcodes.adB

# Tests v2.0
cargo run --bin adeadc -- run TESTEO/v2/integrados/test_v2_0_0_hex_first.adB
```

---

## 📋 Comandos CLI

```bash
adeadc run archivo.adB      # Compilar y ejecutar
adeadc build archivo.adB    # Compilar
adeadc check archivo.adB    # Verificar sintaxis
adeadc tiny archivo.adB     # PE ultra-compacto (<500 bytes)
```

---

*ADead-BIB: Código → Bytes → Binario*
*Sin ASM. Sin LLVM. Sin mentiras.*
