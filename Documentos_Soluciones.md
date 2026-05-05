# 📘 Documentos_Soluciones.md — ADead-BIB

> **Plan de mejora total del compilador C independiente para integración futura en OS Rust.**
>
> Estado actual: el pipeline `Lexer → Parser → AST→IR → Optimizer → UB → Codegen → PE` **compila** y genera `.exe` reales (≈1 KB), pero tiene bugs concretos en codegen y un runtime sobre-generado con tipos incorrectos. Este documento llena cada gap.

---

## 📑 Índice

1. [Estado actual verificado](#1-estado-actual-verificado)
2. [Mapa de completitud por componente](#2-mapa-de-completitud-por-componente)
3. [Problemas detectados (con evidencia)](#3-problemas-detectados-con-evidencia)
4. [Soluciones por prioridad](#4-soluciones-por-prioridad)
5. [Roadmap hacia OS Rust](#5-roadmap-hacia-os-rust)
6. [Plan de tests automatizado](#6-plan-de-tests-automatizado)
7. [Checklist global](#7-checklist-global)
8. [Test Intensivo C99 — Estado por Categoría](#8-test-intensivo-c99--estado-por-categoría)
9. [Plan Inmediato — Tests C99 al 100 %](#9-plan-inmediato--tests-c99-al-100-)
10. [Resumen Ejecutivo](#10--resumen-ejecutivo)

---

## 1. Estado Actual Verificado

### ✅ Lo que **sí funciona** (verificado por mí)

| Acción | Comando | Resultado |
|---|---|---|
| Build del compilador | `cargo build --release` (en `C_Real_Optimo/`) | ✅ Compila en 2.85 s |
| Generación de `.exe` PE | `adB cc hello.c -o hello.exe` | ✅ 1024 bytes, exit 0 |
| Pipeline 7 fases | source → lexer → parser → IR → opt → UB → codegen → PE | ✅ Todas reportan |
| CLI `adB` | `adB version`, `adB cc`, `adB run` | ✅ Funcionan |
| Workspace dual | `compiler/` + `runtime/` | ✅ Cargo workspace OK |

### ❌ Lo que **no funciona** (verificado por mí)

| Caso | Esperado | Observado |
|---|---|---|
| `02_arithmetic.c` (aritmética encadenada) | `exit 0` | `exit 24` |
| ~~Llamadas a función (`IrInstr::Call`)~~ | ~~Generar `call rel32`~~ | ✅ **RESUELTO** — PE entry_rva apunta a main, patch_calls funciona |
| Headers ELF (`elf.rs`) | `phdr_offset`/`text_offset` escritos | Variables sin usar |
| Runtime FFI-safe | tipos C válidos | 29 warnings con `()` (callback sin tipo) |
| ~~`tests/c99/09_arrays.c`, `10_structs.c`~~ | ~~compilar y correr~~ | ✅ **RESUELTO** — Parser soporta `int arr[N]` y structs con newlines |

### ✅ Bugs resueltos (sesión actual)

| Bug | Root cause | Fix |
|---|---|---|
| **Control de flujo** (`if/while/for` = exit -1) | `xor rax,rax` borraba FLAGS antes de `setcc` | Eliminar XOR; MOVZX ya zero-extiende. + `patch_jumps()` movido a fin de función |
| **Function calls** (exit basura) | PE entry point = offset 0 (primera función, no main) | `pe.entry_rva = main_offset` en CLI |
| **Parser hang arrays** (`int arr[5]`) | `parse_var_decl` no reconocía `[` | Añadido manejo de `LBracket` → `Type::Array` |
| **Parser hang structs** (newlines) | `parse_struct` no saltaba `Token::Newline` | Añadido `skip_newlines()` + soporte array fields |

---

## 2. Mapa de Completitud por Componente

```diagram
╭───────────────────────────────────────────────────────────────╮
│                  ADead-BIB: Madurez por capa                  │
╰───────────────────────────────────────────────────────────────╯

Frontend  ████████░░  80% — lexer y parser sólidos para C99 base
Middle    ██████░░░░  60% — IR y optimizer existen, falta SSA real
Backend   ████░░░░░░  40% — codegen incompleto en BinOp y Call
Linker    █████░░░░░  50% — PE OK, ELF a medias, sin DLL/SO
Runtime   ███░░░░░░░  30% — auto-generado, tipos pobres, duplicado
Stdlib    █░░░░░░░░░  10% — directorio existe pero vacío
Tests     ███░░░░░░░  30% — 12 tests C99, sin assertion harness
GPU       ░░░░░░░░░░   0% — Vulkan/DX12 sólo headers en runtime
```

### Detalle numérico por archivo (C_Real_Optimo)

| Archivo | KB | ¿Maduro? | Comentario |
|---|---:|---|---|
| `compiler/frontend/lexer.rs` | 12.7 | 🟢 Sí | Tokens C99 cubiertos |
| `compiler/frontend/parser.rs` | 25.2 | 🟡 Casi | Falta: bitfields, VLA, designated init |
| `compiler/frontend/ast.rs` | 4.3 | 🟡 Casi | Falta nodos para `_Generic`, `_Static_assert` |
| `compiler/frontend/token.rs` | 2.7 | 🟢 Sí | OK |
| `compiler/middle/ir.rs` | 15.9 | 🟡 Casi | Sin SSA, sin CFG explícito |
| `compiler/middle/ast_to_ir.rs` | 9.7 | 🟡 Casi | No traduce todas las exprs |
| `compiler/middle/optimizer.rs` | 12.7 | 🟡 Casi | DCE/fold OK, falta inline real |
| `compiler/middle/ub_detector.rs` | 11.2 | 🟢 Sí | Solo emite reports |
| `compiler/backend/codegen.rs` | 15.1 | 🔴 Bug | BinOp encadenado mal, Call vacío |
| `compiler/backend/encoder.rs` | 17.6 | 🟢 Sí | Bytes x86-64 directos OK |
| `compiler/backend/pe.rs` | 11.2 | 🟢 Sí | PE válido, console subsystem |
| `compiler/backend/elf.rs` | 6.2 | 🟡 Casi | Offsets de headers no se usan |
| `compiler/cli/main.rs` | 4.0 | 🟡 Casi | Falta `--dll`, `--so`, `--flat`, `step` |
| `runtime/core/mod.rs` | 2.8 | 🟢 Sí | init/shutdown/memcpy/memset OK |
| `runtime/win32/{lib,mod}.rs` | 5993 | 🔴 No | 6 MB duplicados, callbacks `()` |
| `runtime/vulkan/{lib,mod}.rs` | 1855 | 🔴 No | Idem |
| `runtime/dx12/{lib,mod}.rs` | 892 | 🔴 No | Idem |
| `runtime/io/{lib,mod}.rs` | 744 | 🔴 No | Idem |
| `runtime/math/{lib,mod}.rs` | 576 | 🟡 Casi | Más limpio, pero duplicado |
| `runtime/string/{lib,mod}.rs` | 21 | 🟢 Sí | Pequeño, manual |
| `runtime/thread/{lib,mod}.rs` | 848 | 🔴 No | Tipos pobres |
| `runtime/memory/{lib,mod}.rs` | 4820 | 🔴 No | 4.8 MB duplicados |
| `runtime/opengl/{lib,mod}.rs` | 8.7 | 🟡 Casi | Pequeño, casi listo |
| `stdlib/` | — | 🔴 No | **No existe** (README lo documenta) |
| `asm/` (ASM-BIB) | — | 🔴 No | **No existe** |

**Totales:**
- Compilador: ~115 KB de Rust real → **60% maduro**
- Runtime: ~14 MB autogenerado → **30% maduro** (mucho ruido)
- Tests: 12 archivos C99 + 7 ejecutables compilados → **30% maduro**

---

## 3. Problemas Detectados (con evidencia)

### 🔴 P-01 · Codegen aritmético encadenado falla

**Evidencia:**
```c
int sum = a + b;      // 24
int diff = a - b;     // 16
int prod = a * b;     // 80
int quot = a / b;     // 5
int mod = a % b;      // 0
int result = sum + diff + prod + quot + mod;  // 125
return result - 125;  // esperado 0
```
**Resultado real:** `exit 24` → solo se evalúa `sum`.

**Causa probable:** En `compiler/backend/codegen.rs` la traducción de `IrInstr::BinaryOp` sobrescribe el destino sin acumular en una temporal. El último `mov rax, ...` retorna sólo el primer resultado.

**Ubicación:** `compiler/backend/codegen.rs:~200-260`

---

### 🔴 P-02 · `IrInstr::Call` no implementado

**Evidencia:**
```rust
warning: unused variable: `func`
   --> compiler\backend\codegen.rs:255:34
    |
255 |             IrInstr::Call { dst, func, args } => {
    |                                  ^^^^ help: try ignoring the field: `func: _`
```

**Impacto:** Cualquier programa C que llame a otra función (incluyendo `printf`) no genera el `call rel32` correcto.

---

### 🟠 P-03 · Runtime con `extern "C" fn(... ()) ...` (29 warnings)

**Evidencia (muestra):**
```rust
pub unsafe extern "C" fn channel_register(name: *mut i8, flags: usize, callback: ()) -> *mut c_void
```
`()` no es FFI-safe. Estos provienen de campos en `knowledge.json` donde el extractor Python no detectó el tipo del callback.

**Solución:** mapear callbacks a `Option<unsafe extern "C" fn() -> ()>` o `*mut c_void` (puntero opaco).

---

### 🟠 P-04 · Runtime duplicado `lib.rs` ≈ `mod.rs`

**Evidencia:**
| Subdir | `lib.rs` | `mod.rs` | Duplicación |
|---|---:|---:|---|
| win32 | 3 380 731 | 2 612 758 | ~80% |
| vulkan | 1 069 869 | 785 878 | ~75% |
| memory | 2 572 570 | 2 247 186 | ~85% |
| io | 412 066 | 332 704 | ~80% |

**Total redundante:** ~7 MB.

**Solución:** generar **sólo `mod.rs`** desde Python, dejar `lib.rs` como `pub mod xxx;` simple en el subdir padre.

---

### 🟠 P-05 · `knowledge.json` con metadatos pobres

**Evidencia:**
```json
{
  "name": "void",
  "return": "extern",
  "params": ["*argp_program_version_hook"],
  "source": "glibc",
  "header": "argp.h",
  "category": "memory"
}
```
- `name = "void"` → no es un nombre, es el tipo
- `return = "extern"` → no es un tipo, es un linkage modifier
- `category = "memory"` → debería ser "callback global"
- Win32 `GetExplicitEntriesFromAclA` con `category = "math"` → mal categorizado

**Solución:** re-extracción con clasificador mejorado en Python (ver §4.5).

---

### 🟡 P-06 · ELF no escribe headers (variables sin usar)

**Evidencia:**
```rust
warning: unused variable: `phdr_offset`
  --> compiler\backend\elf.rs:72:13
warning: unused variable: `text_offset`
  --> compiler\backend\elf.rs:79:13
```

Esto significa que el `.elf` generado es estructuralmente inválido en Linux.

---

### 🟡 P-07 · CLI sin flags documentados en README

El README v12.0 promete `adB cc --dll`, `--so`, `--flat`, `step`, `gpu`, `spirv`, `create`, `build`, `--target fastos256`. **Realidad:** `cli/main.rs` solo implementa `cc`, `run`, `version`.

---

### 🟡 P-08 · Falta `stdlib/` y `asm/`

README documenta:
```
├── stdlib/   # C ABI headers — Win32, Vulkan, DX12, opengl
├── asm/      # ASM-BIB — ensamblador
```
No existen.

---

### 🟡 P-09 · No hay harness de tests

Hay 12 `.c` y 7 `.exe` en `tests/c99/`, pero ningún script verifica que `02_arithmetic.exe` deba dar exit 0. El `run_tests.ps1` existe pero no asserta exit codes esperados.

---

### 🟡 P-10 · Cargo.toml raíz vs C_Real_Optimo/Cargo.toml — versiones inconsistentes

- Raíz: `version = "8.0.0"` (proyecto madre)
- `C_Real_Optimo`: `version = "1.0.0"` (workspace hijo)

Decisión pendiente: ¿C_Real_Optimo es independiente o sub-proyecto?

---

## 4. Soluciones por Prioridad

### 🟥 4.1 (P-01, P-02) Arreglar codegen — **CRÍTICO**

**Pasos:**
1. Abrir `compiler/middle/ir.rs` y verificar que `IrInstr::BinaryOp { dst, op, lhs, rhs }` use `dst` distinto por instrucción (SSA o al menos temporal contador).
2. En `compiler/backend/codegen.rs`, para `BinaryOp`:
   ```
   mov rax, [lhs]      ; cargar primer operando
   <op> rax, [rhs]     ; aplicar operador
   mov [dst], rax      ; guardar en destino único
   ```
3. Implementar `IrInstr::Call`:
   ```
   ; argumentos en RCX, RDX, R8, R9 (Win64 fastcall)
   sub rsp, 32         ; shadow space
   call <func_offset>  ; rel32 hacia tabla de símbolos
   add rsp, 32
   mov [dst], rax
   ```
4. Re-correr `02_arithmetic.exe` y exigir `exit 0`.

**Validación:**
```powershell
.\C_Real_Optimo\target\release\adB.exe cc tests/c99/02_arithmetic.c -o t.exe
.\t.exe; if ($LASTEXITCODE -ne 0) { throw "FAIL: arith" }
```

---

### 🟥 4.2 (P-03) Limpiar tipos `()` en runtime — **CRÍTICO**

**Estrategia:** post-procesar los `mod.rs` generados con un script Python que reemplace:
```rust
callback: ()
```
por:
```rust
callback: Option<unsafe extern "C" fn()>
```
o si el contexto sugiere puntero genérico:
```rust
callback: *mut core::ffi::c_void
```

Crear `Tareas_Para_Python/fix_runtime_callbacks.py`:
```python
import re, pathlib
PAT = re.compile(r'(\w+):\s*\(\)')
for p in pathlib.Path("../C_Real_Optimo/runtime").rglob("*.rs"):
    txt = p.read_text(encoding="utf-8")
    new = PAT.sub(r'\1: *mut core::ffi::c_void', txt)
    if new != txt:
        p.write_text(new, encoding="utf-8")
        print(f"fixed {p}")
```

---

### 🟧 4.3 (P-04) Deduplicar runtime — **ALTA**

**Plan:**
1. Decidir: cada subdir tendrá **solo `mod.rs`**.
2. Eliminar todos los `lib.rs` dentro de subdirs (`runtime/win32/lib.rs`, etc).
3. En `runtime/lib.rs`, declarar:
   ```rust
   pub mod core;
   pub mod memory;
   pub mod io;
   pub mod string;
   pub mod math;
   pub mod thread;
   pub mod win32;
   pub mod vulkan;
   pub mod dx12;
   pub mod opengl;
   ```
   (ya está así, perfecto)
4. Modificar `Tareas_Para_Python/generate_runtime_v2.py` para emitir **únicamente `mod.rs`** por subdir.

**Ahorro estimado:** ~7 MB.

---

### 🟧 4.4 (P-06) Completar `backend/elf.rs` — **ALTA**

Escribir program headers reales:
```rust
let phdr_offset = elf.len() as u64;
// ... emitir Elf64_Phdr aquí ...
elf.extend_from_slice(&phdr_data);

let text_offset = elf.len();
// ... patch en ehdr para apuntar phdr_offset y text_offset ...
```

Sin esto el `.elf` generado no carga en Linux.

---

### 🟧 4.5 (P-05) Regenerar `knowledge.json` con clasificador maduro — **ALTA**

**Crear** `Tareas_Para_Python/extract_apis_v2.py` que:
1. Use `tree-sitter-c` para parsear los headers reales en `Real_compiler/wine/`, `glibc/`, `reactos/`.
2. Extraiga firmas con tipos reales (no strings).
3. Categorice por header path:
   - `windows.h` → `category: "win32"`
   - `gl/gl.h` → `category: "opengl"`
   - `vulkan/vulkan.h` → `category: "vulkan"`
   - `string.h` → `category: "string"`
   - `stdio.h` → `category: "io"`
   - `math.h` → `category: "math"`
4. Para callbacks (function pointers), guardar firma:
   ```json
   {
     "name": "callback",
     "type": "fn_ptr",
     "signature": "fn(*mut c_void) -> i32"
   }
   ```

---

### 🟨 4.6 (P-07) Implementar flags de CLI faltantes — **MEDIA**

Extender `compiler/cli/main.rs`:

```rust
match args[1].as_str() {
    "cc"      => cmd_compile_c(&args[2..]),
    "cxx"     => cmd_compile_cxx(&args[2..]),
    "run"     => cmd_run(&args[2..]),
    "step"    => cmd_step(&args[2..]),       // NUEVO
    "create"  => cmd_create(&args[2..]),     // NUEVO
    "build"   => cmd_build(&args[2..]),      // NUEVO (lee adb.toml)
    "gpu"     => cmd_gpu(&args[2..]),        // NUEVO
    "spirv"   => cmd_spirv(&args[2..]),      // NUEVO
    "version" => cmd_version(),
    _ => print_usage(),
}
```

Flags por archivo:
- `--dll` / `--so` → cambiar `PeBuilder::new().console()` por `.dll()` / `ElfBuilder::new().shared()`
- `--flat` → emitir solo `code` sin headers PE/ELF
- `-Wstrict` → `ub_detector` promueve a error
- `-step` → imprimir AST/IR/asm en cada fase

---

### 🟨 4.7 (P-08) Crear `stdlib/` con headers C ABI — **MEDIA**

Estructura sugerida:
```
C_Real_Optimo/stdlib/
├── adeb_stdio.h       # printf, fprintf, fopen...
├── adeb_stdlib.h      # malloc, free, exit...
├── adeb_string.h      # memcpy, strlen, strcmp...
├── adeb_math.h        # sin, cos, sqrt...
├── adeb_win32.h       # CreateWindowEx, MessageBoxA...
├── adeb_vulkan.h      # vkCreateInstance, vkQueueSubmit...
├── adeb_dx12.h        # ID3D12Device, ID3D12Resource...
├── adeb_opengl.h      # glDrawArrays, glClear...
└── adeb_main.h        # incluye todo
```

Cada header declara `extern` las funciones del runtime (ej. `adeb_win32_*`).

---

### 🟨 4.8 (P-08) Crear `asm/` (ASM-BIB) — **MEDIA**

Empezar mínimo:
```
C_Real_Optimo/asm/
├── asm_bib.rs           # parser .pasm → COFF .obj
├── functions/
│   ├── strlen.pasm
│   ├── strcpy.pasm
│   ├── memcpy.pasm
│   └── ...              # las 21 funciones del README
└── bridge.rs            # merge COFF .obj con PE
```

---

### 🟨 4.9 (P-09) Harness de tests — **MEDIA**

Reemplazar `tests/c99/run_tests.ps1` por:
```powershell
$tests = @{
    "01_variables.c" = 0
    "02_arithmetic.c" = 0
    "03_if_else.c" = 0
    "04_while_loop.c" = 0
    "05_for_loop.c" = 0
    "06_functions.c" = 0
    "07_recursion.c" = 0
    "08_pointers.c" = 0
    "09_arrays.c" = 0
    "10_structs.c" = 0
}

$pass = 0; $fail = 0
foreach ($t in $tests.GetEnumerator()) {
    $exe = $t.Key -replace '\.c$', '.exe'
    & "..\..\target\release\adB.exe" cc $t.Key -o $exe 2>&1 | Out-Null
    & ".\$exe" 2>&1 | Out-Null
    if ($LASTEXITCODE -eq $t.Value) { $pass++; "✓ $($t.Key)" }
    else { $fail++; "✗ $($t.Key) (exit $LASTEXITCODE, expected $($t.Value))" }
}
"`n$pass passed, $fail failed"
```

---

### 🟩 4.10 (P-10) Decidir versión y modularización — **BAJA**

**Recomendación:** alinear ambas a `12.0.0` (la versión documentada en README v12.0). Editar:
- `Cargo.toml` raíz → `version = "12.0.0"`
- `C_Real_Optimo/Cargo.toml` → `version = "12.0.0"`

---

## 5. Roadmap hacia OS Rust

> El compilador independiente debe poder generar binarios que un kernel Rust pueda cargar y ejecutar sin libc, sin Win32, sin Linux.

```diagram
╭──────────────────────────────────────────────────────────────╮
│ FASE A · Compilador robusto (4 sem)                          │
│   - Arreglar codegen (P-01, P-02)                            │
│   - Tests harness (P-09)                                     │
│   - ELF correcto (P-06)                                      │
╰──────────────────────────────────────────────────────────────╯
                          ▼
╭──────────────────────────────────────────────────────────────╮
│ FASE B · Runtime limpio (3 sem)                              │
│   - Deduplicar runtime (P-04)                                │
│   - Tipos FFI-safe (P-03)                                    │
│   - knowledge.json v2 (P-05)                                 │
╰──────────────────────────────────────────────────────────────╯
                          ▼
╭──────────────────────────────────────────────────────────────╮
│ FASE C · Stdlib y ASM-BIB (4 sem)                            │
│   - Headers C ABI (P-08)                                     │
│   - ASM-BIB con 21 funciones (P-08)                          │
│   - CLI completo (P-07)                                      │
╰──────────────────────────────────────────────────────────────╯
                          ▼
╭──────────────────────────────────────────────────────────────╮
│ FASE D · Target `--flat` para OS Rust (4 sem)                │
│   - Emitir flat binary sin PE/ELF                            │
│   - Convención: entry point en offset 0                      │
│   - Sin syscalls Win32/Linux                                 │
│   - Sólo runtime/core/* (memcpy, memset, alloc bare-metal)   │
│   - Modo `no_std` runtime                                    │
╰──────────────────────────────────────────────────────────────╯
                          ▼
╭──────────────────────────────────────────────────────────────╮
│ FASE E · Integración kernel Rust (∞)                         │
│   - El kernel Rust mapea .flat en memoria física             │
│   - El kernel resuelve syscalls custom (no Linux/Win32)      │
│   - El compilador genera código C → flat binary del OS       │
│   - El OS arranca con módulos C compilados por adB           │
╰──────────────────────────────────────────────────────────────╯
```

### Convenciones para target OS-Rust (`--target adeb-os`)

```
adB cc kernel_module.c --target adeb-os -o module.bin

Resultado:
- Sin headers PE/ELF
- Entry point: offset 0
- Calling convention: Win64 fastcall (RCX, RDX, R8, R9)
- Sin libc: solo adeb_runtime_core
- Heap manual: módulo proporciona malloc/free
- Syscalls: int 0x80 con números definidos por el kernel Rust
```

---

## 6. Plan de Tests Automatizado

### 6.1 Tests unitarios Rust

Ya existen 14 tests Rust en `runtime/core` y otros. Ejecutar:
```bash
cargo test --workspace
```

Cobertura objetivo:
- `compiler/frontend/lexer.rs` → 20 tests
- `compiler/frontend/parser.rs` → 30 tests
- `compiler/middle/ir.rs` → 15 tests
- `compiler/middle/optimizer.rs` → 10 tests
- `compiler/backend/codegen.rs` → 25 tests
- `compiler/backend/encoder.rs` → 40 tests (uno por instrucción x86-64)

### 6.2 Tests de integración C

```diagram
tests/
├── c99/         # 30+ casos C99 (estándar)
├── win32/       # 10+ casos: ventanas, archivos, threads
├── gpu/         # 5+ casos: triángulo OpenGL, computeshader Vulkan
└── kernel/      # 5+ casos: módulos OS Rust target
```

### 6.3 CI script

`scripts/test_all.ps1`:
```powershell
cargo build --release --workspace
cargo test --workspace
.\tests\c99\run_tests.ps1
.\tests\win32\run_tests.ps1
```

---

## 7. Checklist Global

Marcar cuando esté hecho.

### Bugs críticos
- [x] **P-01** Codegen aritmético encadenado funciona (`02_arithmetic.exe → exit 0`) ✅ VERIFICADO
- [x] **P-02** `IrInstr::Call` genera `call rel32` con shadow space + tabla `func_offsets` + `patch_calls()` ✅
- [x] **P-06** `elf.rs` usa `phdr_offset` y `text_offset` con `debug_assert_eq!` ✅

### Limpieza
- [x] **P-03** 28 callbacks `: ()` reemplazados por `*mut c_void` (script `fix_runtime_callbacks.py`) ✅
- [x] **P-04** 9 archivos `lib.rs` duplicados eliminados — **8.35 MB liberados** ✅
- [ ] **P-05** `knowledge.json` regenerado con tipos correctos (pendiente — script `extract_apis_v2.py`)
- [x] **P-10** Versión unificada `12.0.0` (raíz · workspace · CLI const) ✅

### Funcionalidad nueva
- [x] **P-07** CLI completo: `cc`, `cxx`, `run`, `step`, `create`, `build`, `gpu`, `spirv`, `version` ✅
  - Flags: `--dll`, `--so`, `--elf`, `--flat`, `--target`, `-Wstrict`, `-step`
  - Targets: `pe-exe`, `pe-dll`, `elf`, `elf-so`, `flat`, `adeb-os`, `fastos256`
  - Banner ASCII al ejecutar `adB version`
- [x] **P-08** `stdlib/` con 9 headers C ABI ✅
  - `adeb_main.h` (master) · `adeb_types.h` · `adeb_stdio.h` · `adeb_stdlib.h`
  - `adeb_string.h` · `adeb_math.h` · `adeb_win32.h` · `adeb_vulkan.h` · `adeb_dx12.h` · `adeb_opengl.h`
- [x] **P-08** `asm/` con 21 funciones ASM-BIB en 5 archivos `.pasm` ✅
  - `asm_strlen.pasm` · `asm_memcpy.pasm` · `asm_memset.pasm` · `asm_math.pasm` · `asm_bit.pasm`
  - README con pipeline `.pasm → COFF → PE`

### Tests
- [x] **P-09** Harness automatizado en Python (`run_all_tests.py`) con timeout de 5 s ✅
  - Soporta: `[PASS]`, `[FAIL]`, `[HANG]`, `[FAIL-COMPILE]`, `[skip]`
  - **Estado actual: 3 PASS / 5 FAIL / 1 HANG / 2 COMPILE-FAIL** (sobre 11 tests)
- [ ] 30+ tests C99 pasando (control flow y calls aún rotos — ver §8)
- [ ] 10+ tests Win32 pasando
- [ ] `cargo test --workspace` 100% green

### Roadmap OS
- [x] Target `--target adeb-os` y `--flat` (flat binary sin headers PE/ELF) ✅ implementados en CLI
- [ ] Runtime `no_std` modo bare-metal
- [ ] Convención de syscalls custom documentada
- [ ] Primer módulo C compilado y cargado por kernel Rust

---

## 8. Test Intensivo C99 — Estado por Categoría

> **Suite ampliada a 41 tests** organizados por categoría. Verificado por
> `python C_Real_Optimo/tests/run_all_tests.py`. Snapshot:
> **30 PASS / 8 FAIL / 0 HANG / 3 COMPILE-FAIL** (73 % pasando · ↑ desde 68 %).
>
> **Última fase aplicada:** ✅ **B-02 — Ternary lowering** (FASE T2). Tests 24 y 40 ahora PASS.

### 🟢 Categorías 100 % PASS

| Categoría | Tests | Resultado |
|---|---|:---:|
| **Operadores** | 11_bitwise, 12_comparisons, 13_logical, 14_compound_assign, 15_inc_dec | 5/5 ✅ |
| **Control flow nested** | 16_nested_if, 17_nested_loops, 18_for_nested | 3/3 ✅ |
| **Aritmética avanzada** | 32_long_chain, 33_neg, 34_paren, 35_precedence, 36_neg_div | 5/5 ✅ |
| **Algoritmos básicos** | 37_loop_factorial, 38_gcd, 39_power | 3/3 ✅ |
| **Funciones básicas** | 19_multi_func, 20_factorial, 21_fibonacci, 22_4args | 4/4 ✅ |
| **Misc** | hello.c | 1/1 ✅ |

### 🟡 Categorías parcialmente OK

| Categoría | PASS | FAIL | Detalle |
|---|---|---|---|
| Tests iniciales (01–07) | 7 | 0 | ✅ todos pasan |
| Recursión + funcs (19–22) | 4 | 0 | ✅ |
| Algoritmos | 3 | 1 | `40_complex` falla (ternary anidado) |

### 🔴 Categorías con bugs estructurales

#### Bug B-01: Codegen de punteros / arrays / structs

| Test | Exit | Causa |
|---|---:|---|
| `08_pointers.c`  | -10  | `Expr::AddrOf`, `Expr::Deref` sin codegen real |
| `09_arrays.c`    | -15  | `Expr::Index` sin codegen |
| `10_structs.c`   | -30  | `Expr::Member`, `Expr::Arrow` sin codegen |

**Solución (en `compiler/middle/ast_to_ir.rs` + `backend/codegen.rs`):**
- `&x` → `IrInstr::Alloca` ya devuelve un puntero. Convertir `Expr::AddrOf(Ident)` a usar el ptr almacenado en `vars`.
- `*p` → `IrInstr::Load` con el valor del puntero.
- `arr[i]` → `Add base, i*sizeof(elem)` + `Load`.
- `s.field` → `Add base, offset(field)` + `Load` (requiere tabla de structs).

#### ~~Bug B-02: Ternary y operadores condicionales sin lowering~~ ✅ RESUELTO

| Test | Estado |
|---|---|
| `24_ternary.c`  | ✅ PASS — exit 0 |
| `40_complex.c`  | ✅ PASS — exit 0 |

**Fix aplicado** en [`compiler/middle/ast_to_ir.rs`](file:///c%3A/Users/andre/OneDrive/Documentos/ADead-BIB/C_Real_Optimo/compiler/middle/ast_to_ir.rs):
añadido brazo `Expr::Ternary(c, t, e)` que genera:
```
%tmp = alloca i32
if c { jmp then_bb } else { jmp else_bb }
then_bb: store %tmp, t; jmp merge
else_bb: store %tmp, e; jmp merge
merge:   %r = load %tmp
```
**Build:** OK · **Tests desbloqueados:** 24, 40 · **Tiempo real:** ~5 min

#### Bug B-03: `break` / `continue` no implementados

| Test | Causa |
|---|---|
| `25_break_continue.c` | `Stmt::Break` y `Stmt::Continue` caen al `_ => {}` final |

**Solución:** Mantener un stack de `(continue_bb, break_bb)` durante `convert_stmt` y emitir `jmp` correspondiente.

#### Bug B-04: Parser hang en `do-while`, `switch`, `typedef`

| Test | Causa |
|---|---|
| `26_do_while.c`  | `parse_stmt` no soporta token `do` |
| `27_switch.c`    | `parse_stmt` no soporta `switch/case/default` |
| `29_typedef.c`   | `parse_top_level` consume mal `typedef int Number;` |

**Solución:** Añadir ramas en parser para los tres keywords + `parse_switch` con `parse_case`.

#### Bug B-05: `sizeof` y `enum` no se resuelven a constante

| Test | Exit | Causa |
|---|---:|---|
| `28_sizeof.c` | -15 | `Expr::SizeofType` no resuelve a `IrConst` |
| `30_enum.c`   |  -7 | `enum` no registra constantes en symbol table |

**Solución:** En `convert_expr`, `SizeofType(t)` → `IrConst::I32(t.size())`. Para `enum`, registrar variantes con su valor numérico en una tabla global.

#### Bug B-06: Globals y void functions

| Test | Exit | Causa |
|---|---:|---|
| `23_void_func.c` | -3 | función void muta `counter` global pero load no actualiza |
| `31_global_var.c`| -50 | global `int global = 42` no inicializa en data section |

**Solución:** El backend debe emitir un `.data` con globals inicializados, y `Expr::Ident` para nombres globales debe generar `Load [rip+offset]` real (no `mov ri 0`).

---

## 9. Plan Inmediato — Tests C99 al 100 %

```diagram
╭──────────────────────────────────────────────────────────────╮
│ FASE T1 · Codegen punteros/arrays/structs (B-01)             │
│   - Lower Expr::AddrOf, Deref, Index, Member, Arrow          │
│   - Tabla de offsets de struct fields                        │
│   - Tests +3:  08, 09, 10                                    │
│   ETA: 1-2 días                                              │
╰──────────────────────────────────────────────────────────────╯
                          ▼
╭──────────────────────────────────────────────────────────────╮
│ FASE T2 · Ternary + control extra (B-02, B-03)               │
│   - Lower Expr::Ternary                                      │
│   - Stack de break/continue targets                          │
│   - Tests +3:  24, 25, 40                                    │
│   ETA: 1 día                                                 │
╰──────────────────────────────────────────────────────────────╯
                          ▼
╭──────────────────────────────────────────────────────────────╮
│ FASE T3 · Parser do/switch/typedef (B-04)                    │
│   - parse_do_while, parse_switch, parse_typedef              │
│   - Tests +3:  26, 27, 29                                    │
│   ETA: 1-2 días                                              │
╰──────────────────────────────────────────────────────────────╯
                          ▼
╭──────────────────────────────────────────────────────────────╮
│ FASE T4 · sizeof + enum + globals (B-05, B-06)               │
│   - Resolver sizeof en compile time                          │
│   - Tabla global de enums                                    │
│   - PE .data con globals inicializados                       │
│   - Tests +4:  23, 28, 30, 31                                │
│   ETA: 2 días                                                │
╰──────────────────────────────────────────────────────────────╯
                          ▼
╭──────────────────────────────────────────────────────────────╮
│ TARGET: 41/41 PASS (100 %)                                   │
│   El compilador C99 base estará COMPLETO.                    │
╰──────────────────────────────────────────────────────────────╯
                          ▼
╭──────────────────────────────────────────────────────────────╮
│ FASE T5 · Tests AVANZADOS (futuro)                           │
│   - Strings literales + printf                               │
│   - Function pointers                                        │
│   - Pointers to pointers                                     │
│   - Multi-dim arrays                                         │
│   - Nested structs                                           │
│   - Variadic functions                                       │
│   - 50+ tests adicionales                                    │
╰──────────────────────────────────────────────────────────────╯
```

### 📈 Tracker de progreso C99 — visual

```diagram
Tests C99 PASS rate evolution:
  v1.0    ██░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  3/11   (27 %)
  v12.0   ████████████████████████░░░░░░░░  8/11   (73 %)
  v13.0   ████████████████████████████░░░░ 28/41   (68 %)
  T2 done █████████████████████████████░░░ 30/41   (73 %)  ← AHORA
  Goal T4 ████████████████████████████████ 41/41   (100 %) ← META
```

### 🔧 Orden recomendado de ataque

| Orden | Bug | Esfuerzo | Tests desbloqueados | Estado |
|:-:|---|---|---|:-:|
| 1 | **B-02** Ternary | 2 h | 24, 40 | ✅ HECHO |
| 2 | **B-03** break/continue | 2 h | 25 | ⏳ siguiente |
| 3 | **B-04** parse do/switch/typedef | 4 h | 26, 27, 29 | ⏳ |
| 4 | **B-05** sizeof/enum constantes | 3 h | 28, 30 | ⏳ |
| 5 | **B-06** globals + void mutación | 4 h | 23, 31 | ⏳ |
| 6 | **B-01** punteros/arrays/structs | 1-2 d | 08, 09, 10 | ⏳ |

Total estimado para **41/41 PASS**: **3-4 días** de trabajo enfocado.

---


## 10. 📊 Resumen Ejecutivo

**¿Qué tan completo está ADead-BIB hoy?** → **~70 %** del README v12/13.

### Métricas verificadas (suite intensiva C99)

| Métrica | Valor | Detalle |
|---|---|---|
| Tests C99 totales | **41** | Suite ampliada de 11 → 41 (categorizada en 7 áreas) |
| Tests PASS | **28 / 41** (68 %) | Operadores, control flow nested, aritmética, recursión |
| Tests FAIL | 10 | punteros/arrays/structs/ternary/break/continue/globals |
| Tests COMPILE-FAIL | 3 | parser hangs en do-while / switch / typedef |
| Tamaño .exe típico | 1–2 KB | sin CRT, sin runtime overhead |
| Build time release | 5.92 s | `cargo build --release` workspace completo |
| Warnings build | < 10 | post-cleanup |
| Runtime liberado | 8.35 MB | dedup `lib.rs` ↔ `mod.rs` |

### Lo que ya tienes (real, verificado)

- ✅ Pipeline 7 fases generando PE x86-64 válidos
- ✅ **Operadores 100 %**: bitwise, lógicos, comparaciones, compound assign, inc/dec
- ✅ **Aritmética 100 %**: encadenada, paréntesis, precedencia, división negativa
- ✅ **Control flow 100 %**: if/else (nested), while, for, anidados
- ✅ **Funciones 100 %**: 4 args, recursión (factorial, fib, gcd, power)
- ✅ Parser C99 sin hangs en arrays/structs
- ✅ ASM-BIB bridge — `coff_reader.rs` + `bridge.rs` + `--link-obj` CLI
- ✅ 14 KB de runtime auto-generado desde 18 DLLs
- ✅ Suite intensiva 41 tests + harness Python con timeout

### Lo que falta para 100 % C99

| Bloqueador | Tests afectados | Esfuerzo |
|---|---|---|
| B-01 punteros/arrays/structs codegen | 08, 09, 10 | 1-2 d |
| B-02 ternary lowering | 24, 40 | 2 h |
| B-03 break/continue | 25 | 2 h |
| B-04 parser do/switch/typedef | 26, 27, 29 | 4 h |
| B-05 sizeof/enum constexpr | 28, 30 | 3 h |
| B-06 globals + void mutation | 23, 31 | 4 h |

**Tiempo total para llegar a 41/41 PASS:** **3-4 días** de trabajo enfocado.

### Lo que falta para integración OS Rust

- Codegen `--target adeb-os` produciendo flat binary correcto (entry@0)
- Runtime `no_std` modo bare-metal
- Convención de syscalls custom documentada
- Primer módulo C compilado y cargado por kernel Rust

**Tiempo para integración mínima viable con OS Rust:** **+4 semanas** sobre el 100 % C99.

---

> *"El compilador respira, camina, y ya hace álgebra y recursión.*  
> *Le falta tocar memoria con dedos finos (punteros), aprender condicionales tres-en-uno (ternary),*  
> *y romperse el lazo cuando se cansa (break). Después, solo le queda olvidar Windows*  
> *y volar libre dentro de tu propio OS."*

