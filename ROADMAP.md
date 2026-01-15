# ADead-BIB v2.0 — Roadmap de Desarrollo

> **ADead-BIB** = **A**SM **Dead** - **B**inary **I**s **B**inary
> 
> Lenguaje **OOP Puro + ASM Simbionte** que compila **DIRECTO a BINARIO**.
> Sin ASM intermedio. Sin LLVM. Sin linker externo.
> 
> **Código → AST → BYTES DIRECTOS → Ejecutable**

---

## Flujo de Trabajo Estándar (estilo Rust)

```bash
# Crear proyecto nuevo
adB create mi_proyecto      # Crea estructura completa
adB new mi_proyecto         # Alias de create
adB init                    # Inicializa en directorio actual

# Compilar y ejecutar
adB run main.adB            # Compila y ejecuta
adB build main.adB          # Solo compila
adB check main.adB          # Verifica sintaxis

# Modo interactivo
adB play                    # REPL interactivo
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
- [ ] Tests para cada instrucción

**Implementado en:** `Project/cpu/mod.adB`

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
- [ ] Tests para cada opcode

**Implementado en:** `Project/gpu/mod.adB`

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

#### v2.4.0 — Modo Raw Binary

**Objetivo:** Compilar a bytes puros sin headers PE/ELF.

```rust
#![mode(raw)]
#![base(0x1000)]

fn _start() {
    // Genera solo los bytes de código
}
```

**Tareas:**
- [ ] Implementar atributo `#![mode(raw)]`
- [ ] Implementar atributo `#![base(addr)]`
- [ ] Generador de .bin sin headers
- [ ] Soporte para bootloaders y bare metal

---

### Fase 3: Formatos Avanzados

#### v2.5.0 — Formato AHYB (ADead Hybrid Binary)

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

#### v2.7.0 — Post-Procesamiento

**Objetivo:** Eliminar ruido del binario final.

| Optimización | Descripción | Ahorro Estimado |
|--------------|-------------|-----------------|
| **Strip padding** | Eliminar bytes de relleno | ~20% |
| **Dead code removal** | Eliminar código no alcanzable | ~10% |
| **Constant folding** | `2 + 3` → `5` en compilación | ~5% |
| **String dedup** | Strings duplicados → una copia | ~5% |
| **NOP elimination** | Eliminar NOPs innecesarios | ~3% |

```rust
#![clean(normal)]      // Default
#![clean(aggressive)]  // Binario más pequeño
#![clean(none)]        // Sin limpieza (debug)
```

#### v2.8.0 — Peephole Optimizer

**Objetivo:** Optimizaciones locales de secuencias de bytes.

- [ ] Patrones comunes de instrucciones
- [ ] Reemplazo de secuencias ineficientes
- [ ] Alineación inteligente

---

### Fase 5: OOP Avanzado

#### v3.0.0 — OOP Core Spec ✅

**Objetivo:** Sistema OOP completo y documentado.

```rust
struct Player {
    x: i32,
    y: i32,
    health: u8
}

impl Player {
    fn new(x, y) {
        return Player { x: x, y: y, health: 100 }
    }
    
    fn move(self, dx, dy) {
        self.x += dx
        self.y += dy
    }
}

trait Drawable {
    fn draw(self)
}

impl Drawable for Player {
    fn draw(self) {
        // ...
    }
}
```

**Tareas:**
- [x] Especificación formal de structs ✅
- [x] Especificación formal de impl ✅
- [x] Especificación formal de traits ✅
- [x] Vtables simples para polimorfismo ✅
- [x] Documentación completa ✅

**Implementado en:** `Project/call.adB` con ejemplos completos:
- `Vec2`, `Vec3`, `Matrix4` — Tipos matemáticos
- `Entity`, `Player`, `Enemy` — Entidades de juego
- `GameState` — Sistema completo
- `Drawable`, `Updatable` — Traits con implementaciones

#### v3.1.0 — Herencia Simple

**Objetivo:** Herencia de un solo nivel (sin herencia profunda).

```rust
struct Entity {
    x: i32,
    y: i32
}

struct Player extends Entity {
    health: u8
}
```

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
