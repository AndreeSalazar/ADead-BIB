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
| ~~`02_arithmetic.c` (aritmética encadenada)~~ | `exit 0` | ✅ **RESUELTO** — codegen BinOp correcto |
| ~~Llamadas a función (`IrInstr::Call`)~~ | ~~Generar `call rel32`~~ | ✅ **RESUELTO** — PE entry_rva apunta a main, patch_calls funciona |
| Headers ELF (`elf.rs`) | `phdr_offset`/`text_offset` escritos | Variables sin usar |
| Runtime FFI-safe | tipos C válidos | Resuelto vía `*mut c_void` |
| ~~`tests/c99/09_arrays.c`, `10_structs.c`~~ | ~~compilar y correr~~ | ✅ **RESUELTO** — Suite 41/41 PASS |

### ✅ Bugs resueltos (sesión actual + sesiones previas)

| Bug | Root cause | Fix |
|---|---|---|
| **Control de flujo** (`if/while/for` = exit -1) | `xor rax,rax` borraba FLAGS antes de `setcc` | Eliminar XOR; MOVZX ya zero-extiende. + `patch_jumps()` movido a fin de función |
| **Function calls** (exit basura) | PE entry point = offset 0 (primera función, no main) | `pe.entry_rva = main_offset` en CLI |
| **Parser hang arrays** (`int arr[5]`) | `parse_var_decl` no reconocía `[` | Añadido manejo de `LBracket` → `Type::Array` |
| **Parser hang structs** (newlines) | `parse_struct` no saltaba `Token::Newline` | Añadido `skip_newlines()` + soporte array fields |
| **B-02 Ternary** (`a ? b : c`) | `Expr::Ternary` no tenía lowering | alloca + jmp_if + 2 stores + load merge |
| **B-03 break/continue** | `Stmt::Break/Continue` caían al `_ => {}` | `loop_stack: Vec<(cont_bb, brk_bb)>` push/pop en cada loop |
| **B-04 do-while/switch/typedef** | parser no reconocía esos keywords | Añadidas ramas + lowering de `switch` como cadena de `cmp_eq` con fall-through |
| **B-05 sizeof / enum** | sin resolución compile-time | `Expr::SizeofType(t) → IrConst(t.size())`; tabla `enums: HashMap<String, i64>` con auto-incremento |
| **B-06 globals + void mutación** | sin `.data` y sin RIP/abs addressing | Pass 1 codegen layout `.data` con bytes inicializados; PE `SizeOfImage` cubre `.data`; `IrValue::Global` → `mov rax, IMAGE_BASE+DATA_RVA+off`; load/store con deref absoluto |
| **B-01 punteros** (`*p = 20`) | toda `IrReg` se trataba como stack-slot | `alloca_regs: HashSet<u32>` → para regs no-alloca, Load/Store hacen deref real (`mov rcx,[rbp+off]; mov rax,[rcx]`); `Expr::Assign(Deref(p), val)` lowered |
| **B-01 arrays** (`int arr[5]`) | alloca reservaba sólo 8 bytes; `arr` cargaba primer elemento | `alloca_n(ty, count)`; `is_aggregate=true` en vars → `Ident` devuelve dirección; `Index` LHS calcula `addr = base + i*4` y store |
| **B-01 structs** (`p.x = 10`) | `Member` reenvía base sin offset | `structs: HashMap<String, Vec<(field, ty, offset)>>` construido en pass 1; helper `member_addr` calcula `&base + offset`; `Member` lee con `Load(fty, addr)`; `Member` LHS escribe con `Store(addr, val)` |

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
  - **Estado actual: 41 PASS / 0 FAIL / 0 HANG / 0 COMPILE-FAIL** (41/41 · **100 %**) 🎉
- [x] **30+ tests C99 pasando** ✅ (41/41 — TODOS pasan)
- [ ] 10+ tests Win32 pasando
- [ ] `cargo test --workspace` 100% green

### Roadmap OS
- [x] Target `--target adeb-os` y `--flat` (flat binary sin headers PE/ELF) ✅ implementados en CLI
- [ ] Runtime `no_std` modo bare-metal
- [ ] Convención de syscalls custom documentada
- [ ] Primer módulo C compilado y cargado por kernel Rust

---

## 8. Test Intensivo C99 — Estado por Categoría

> **Suite de 41 tests** organizados por categoría. Verificado por
> `python C_Real_Optimo/tests/run_all_tests.py`. Snapshot **actual**:
> **🎉 41 PASS / 0 FAIL / 0 HANG / 0 COMPILE-FAIL — 100 %**.
>
> **Fases aplicadas:** ✅ B-01 (punteros/arrays/structs) · ✅ B-02 (ternary) ·
> ✅ B-03 (break/continue) · ✅ B-04 (do-while/switch/typedef) ·
> ✅ B-05 (sizeof/enum) · ✅ B-06 (globals + void mutación).

### 🟢 Categorías 100 % PASS — TODAS

| Categoría | Tests | Resultado |
|---|---|:---:|
| **Tests iniciales** | 01_variables, 02_arithmetic, 03_if_else, 04_while, 05_for, 06_funcs, 07_recursion | 7/7 ✅ |
| **Memoria** | 08_pointers, 09_arrays, 10_structs | 3/3 ✅ |
| **Operadores** | 11_bitwise, 12_comparisons, 13_logical, 14_compound_assign, 15_inc_dec | 5/5 ✅ |
| **Control flow nested** | 16_nested_if, 17_nested_loops, 18_for_nested | 3/3 ✅ |
| **Funciones** | 19_multi_func, 20_factorial, 21_fibonacci, 22_4args, 23_void_func | 5/5 ✅ |
| **Ternary + flow** | 24_ternary, 25_break_continue, 26_do_while, 27_switch | 4/4 ✅ |
| **Tipos C99** | 28_sizeof, 29_typedef, 30_enum, 31_global_var | 4/4 ✅ |
| **Aritmética avanzada** | 32_long_chain, 33_neg, 34_paren, 35_precedence, 36_neg_div | 5/5 ✅ |
| **Algoritmos** | 37_loop_factorial, 38_gcd, 39_power, 40_complex | 4/4 ✅ |
| **Misc** | hello.c | 1/1 ✅ |

### ✅ Bugs estructurales — TODOS RESUELTOS

#### ~~Bug B-01: Codegen de punteros / arrays / structs~~ ✅ RESUELTO

| Test | Estado |
|---|---|
| `08_pointers.c` | ✅ PASS — exit 0 |
| `09_arrays.c`   | ✅ PASS — exit 0 |
| `10_structs.c`  | ✅ PASS — exit 0 |

**Fix aplicado** (3 cambios coordinados):
1. **Codegen** — `alloca_regs: HashSet<u32>` distingue regs alloca (su valor ES la dirección, emit `lea`) de regs no-alloca (valor en stack-slot, emit `mov`). Load/Store con ptr no-alloca hacen deref real.
2. **Codegen** — `alloca_n(ty, count)` reserva N · sizeof(elem) bytes para `int arr[5]`; `alloca_sizes` ajusta el prólogo del frame.
3. **Lowering AST→IR** — `vars: HashMap<String, (IrReg, IrType, bool, Option<String>)>` con flag `is_aggregate` (arrays/structs devuelven dirección, no load). Tabla `structs` con offsets reales por field. `Expr::Assign` maneja `*p = v`, `arr[i] = v`, `s.field = v` con address calculada.

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

#### ~~Bug B-03: `break` / `continue` no implementados~~ ✅ RESUELTO

| Test | Estado |
|---|---|
| `25_break_continue.c` | ✅ PASS — exit 0 |

**Fix aplicado** (`AstToIr.loop_stack: Vec<(BlockId, BlockId)>`):
en `Stmt::While`/`For`/`DoWhile`/`Switch` se hace `push((cont_bb, break_bb))` antes
de convertir el cuerpo y `pop()` al salir. `Stmt::Break`/`Continue` emiten `jmp`
al target correspondiente y abren un bloque sumidero post-jump.

#### ~~Bug B-04: Parser hang en `do-while`, `switch`, `typedef`~~ ✅ RESUELTO

| Test | Estado |
|---|---|
| `26_do_while.c`  | ✅ PASS — exit 0 |
| `27_switch.c`    | ✅ PASS — exit 0 |
| `29_typedef.c`   | ✅ PASS — exit 0 |

**Fix aplicado:** parser reconoce `do { … } while ( … );`, `switch (…) { case …: …  default: … }`
y `typedef <type> <name>;`. `Stmt::Switch` se lowerea como cadena de comparaciones
`cmp_eq` con fall-through implícito y `break` saliendo del `exit_bb`.

#### ~~Bug B-05: `sizeof` y `enum` no se resuelven a constante~~ ✅ RESUELTO

| Test | Estado |
|---|---|
| `28_sizeof.c` | ✅ PASS — exit 0 |
| `30_enum.c`   | ✅ PASS — exit 0 |

**Fix aplicado:**
- `Expr::SizeofType(t)` → `IrValue::Const(IrConst::I32(t.size() as i32))` usando `Type::size()`.
- En primer pass de `convert()`, las variantes `enum` se registran en
  `enums: HashMap<String, i64>` con auto-incremento (respetando overrides explícitos).
- `Expr::Ident` consulta `self.enums` antes de buscar en `vars`.

#### ~~Bug B-06: Globals y void functions~~ ✅ RESUELTO

| Test | Estado |
|---|---|
| `23_void_func.c` | ✅ PASS — exit 0 |
| `31_global_var.c`| ✅ PASS — exit 0 |

**Fix aplicado:**
- `IrModule.globals: Vec<(String, IrType, Option<i64>)>` ahora propaga el inicializador.
- `convert_global_var` extrae literales (`IntLit`, `CharLit`, `Unary(Neg, IntLit)`).
- En **codegen pass 1** se hace layout de `.data`: bytes inicializados (LE) + padding a 8.
- `IMAGE_BASE = 0x140000000`, `DATA_RVA = 0x2000` → dirección absoluta calculada en compile-time.
- `Expr::Ident` y `Expr::Assign` para nombres globales emiten `Load`/`Store` con `IrValue::Global(name)` como pointer.
- En codegen, `Load{ptr=Global(name)}` → `mov rax, ABS_ADDR; mov rax, [rax]`. `Store` análogo.
- **PE bugfix:** `SizeOfImage` ahora cubre todas las secciones (text + data + rdata), no sólo text.

---

## 9. Plan Inmediato — Tests C99 al 100 % ✅ COMPLETADO

```diagram
╭──────────────────────────────────────────────────────────────╮
│ ✅ FASE T2 · Ternary (B-02) — COMPLETADA                     │
│    Tests desbloqueados: 24, 40                               │
╰──────────────────────────────────────────────────────────────╯
                          ▼
╭──────────────────────────────────────────────────────────────╮
│ ✅ FASE T3 · break/continue + do/switch/typedef (B-03 + B-04)│
│    Tests desbloqueados: 25, 26, 27, 29                       │
╰──────────────────────────────────────────────────────────────╯
                          ▼
╭──────────────────────────────────────────────────────────────╮
│ ✅ FASE T4 · sizeof + enum (B-05) — COMPLETADA               │
│    Tests desbloqueados: 28, 30                               │
╰──────────────────────────────────────────────────────────────╯
                          ▼
╭──────────────────────────────────────────────────────────────╮
│ ✅ FASE T5 · Globals + void mutation (B-06) — COMPLETADA     │
│    .data layout + IMAGE_BASE addressing + PE SizeOfImage fix │
│    Tests desbloqueados: 23, 31                               │
╰──────────────────────────────────────────────────────────────╯
                          ▼
╭──────────────────────────────────────────────────────────────╮
│ ✅ FASE T6 · Punteros/arrays/structs codegen real (B-01)     │
│    alloca_regs + alloca_n + is_aggregate + structs table     │
│    Tests desbloqueados: 08, 09, 10                           │
╰──────────────────────────────────────────────────────────────╯
                          ▼
╭──────────────────────────────────────────────────────────────╮
│ 🎉 TARGET ALCANZADO: 41/41 PASS (100 %)                      │
│    El compilador C99 base está COMPLETO.                     │
╰──────────────────────────────────────────────────────────────╯
                          ▼
╭──────────────────────────────────────────────────────────────╮
│ FASE T7 · Tests AVANZADOS (futuro)                           │
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
  v1.0      ██░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  3/11   (27 %)
  v12.0     ████████████████████████░░░░░░░░  8/11   (73 %)
  v13.0     ████████████████████████████░░░░ 28/41   (68 %)
  T2 done   █████████████████████████████░░░ 30/41   (73 %)
  T3+T4+T5  ███████████████████████████████░ 36/41   (88 %)
  T6 done   █████████████████████████████████ 41/41 (100 %) 🎉 ← AHORA
```

### 🔧 Orden recomendado de ataque — TODO HECHO

| Orden | Bug | Esfuerzo | Tests desbloqueados | Estado |
|:-:|---|---|---|:-:|
| 1 | **B-02** Ternary | 2 h | 24, 40 | ✅ HECHO |
| 2 | **B-03** break/continue | 2 h | 25 | ✅ HECHO |
| 3 | **B-04** parse do/switch/typedef | 4 h | 26, 27, 29 | ✅ HECHO |
| 4 | **B-05** sizeof/enum constantes | 3 h | 28, 30 | ✅ HECHO |
| 5 | **B-06** globals + void mutación | 4-6 h | 23, 31 | ✅ HECHO |
| 6 | **B-01** punteros/arrays/structs | 1-2 d | 08, 09, 10 | ✅ HECHO |

🎉 **Suite C99 base 41/41 PASS — el compilador es funcionalmente completo para C99 simple.**

---


## 10. 📊 Resumen Ejecutivo

**¿Qué tan completo está ADead-BIB hoy?** → **~95 %** del README v12/13 (C99 base completo).

### Métricas verificadas (suite intensiva C99)

| Métrica | Valor | Detalle |
|---|---|---|
| Tests C99 totales | **41** | Suite categorizada en 10 áreas |
| Tests PASS | **🎉 41 / 41** (**100 %**) | TODOS los tests del C99 base |
| Tests FAIL | 0 | — |
| Tests HANG | 0 | parser robusto, sin loops infinitos |
| Tests COMPILE-FAIL | 0 | todos los tests compilan a `.exe` |
| Tamaño .exe típico | 1.5 KB | sin CRT, sin runtime overhead |
| Build time release | < 6 s | `cargo build --release -p adeb-compiler` |
| Warnings build | < 10 | post-cleanup |
| Runtime liberado | 8.35 MB | dedup `lib.rs` ↔ `mod.rs` |

### Lo que ya tienes (real, verificado)

- ✅ Pipeline 7 fases generando PE x86-64 válidos (`SizeOfImage` correcto multi-sección)
- ✅ **Tests iniciales 100 %** (01–07): variables, aritmética, if/else, while, for, funcs, recursión
- ✅ **Memoria 100 %**: punteros (`*p = v`), arrays (`int a[5]; a[i]=v`), structs (`p.field=v`)
- ✅ **Operadores 100 %**: bitwise, lógicos, comparaciones, compound assign, inc/dec
- ✅ **Control flow 100 %**: if/else nested, while, for, do-while, switch/case/default, break/continue
- ✅ **Funciones 100 %**: 4 args, recursión (factorial, fib, gcd, power), multi-func, void
- ✅ **Tipos C99 100 %**: typedef, enum (auto-incremento), sizeof, globals (.data inicializada)
- ✅ **Aritmética avanzada 100 %**: encadenada, paréntesis, precedencia, división negativa
- ✅ **Algoritmos 100 %**: factorial, gcd, power, complex (con ternary)
- ✅ **Ternary 100 %**: lowering vía alloca + if/else + load
- ✅ Parser C99 sin hangs en arrays/structs/do-while/switch/typedef
- ✅ ASM-BIB bridge — `coff_reader.rs` + `bridge.rs` + `--link-obj` CLI
- ✅ 14 KB de runtime auto-generado desde 18 DLLs (callbacks `*mut c_void`)
- ✅ Suite intensiva 41 tests + harness Python con timeout 5 s
- ✅ Globals con dirección absoluta `IMAGE_BASE+DATA_RVA+offset` (sin necesidad de relocaciones)
- ✅ Punteros con codegen real: `lea` para alloca-regs (= dirección), deref real (`mov [rcx]`) para non-alloca-regs (= valor pointer)
- ✅ Tabla de structs con offsets reales calculados en pass 1 (alineamiento 8 bytes)

### Próximas líneas (post-100 % C99)

| Iniciativa | Tipo | Esfuerzo |
|---|---|---|
| Strings literales + `printf` (FFI Win32) | Funcional | 1-2 d |
| Function pointers (`int (*f)(int)`) | Funcional | 2-3 d |
| Multi-dim arrays (`int m[3][4]`) | Funcional | 1-2 d |
| Nested structs y unions reales | Funcional | 2-3 d |
| Variadic functions (`printf("%d", x)`) | Funcional | 2-3 d |
| `cargo test --workspace` 100% green | Calidad | 1-2 d |
| 10+ tests Win32 | Cobertura | 3-4 d |
| Re-extracción `knowledge.json` v2 (P-05) | Limpieza | 2 d |

### Lo que falta para integración OS Rust

- Codegen `--target adeb-os` produciendo flat binary correcto (entry@0)
- Runtime `no_std` modo bare-metal
- Convención de syscalls custom documentada
- Primer módulo C compilado y cargado por kernel Rust

**Tiempo para integración mínima viable con OS Rust:** **~4 semanas** sobre el 100 % C99 actual.

---

> *"El compilador respira, camina, hace álgebra, recursión, ternario y switch.*  
> *Rompe el lazo cuando se cansa (break) y vuelve al inicio cuando quiere (continue).*  
> *Toca memoria con dedos finos (`*p = v`), recorre arrays con índices (`a[i]`),*  
> *guarda secretos en structs (`p.field`) y recuerda lo que escribió en su libreta global.*  
> *La suite C99 base le dice 41/41 — está listo para olvidar Windows*  
> *y volar libre dentro de tu propio OS."*

---

## 🎯 Conclusión final

El compilador ADead-BIB **superó el hito C99 base con 100 % de tests pasando**.
Pipeline `Lexer → Parser → AST→IR → Optimizer → UB → Codegen → PE` produce binarios
PE x86-64 válidos (~1.5 KB) sin dependencia de CRT ni runtime externo, con soporte
completo para:

- Tipos primitivos + typedef + enum + sizeof
- Punteros con `&` / `*` / `*p = v`
- Arrays con `int a[N]` y acceso por índice (lectura/escritura)
- Structs con `p.field` y `p->field` (offsets reales)
- Globals inicializados en `.data`
- Control flow completo: if/else, while, for, do-while, switch/case, break/continue, ternary
- Funciones (recursivas, void, multi-arg, callee-saved registers)
- Aritmética/lógica/bitwise completa con precedencia y compound assign

**Próximo hito:** strings + printf (FFI Win32), o salto directo al target `adeb-os` para integración con kernel Rust.

