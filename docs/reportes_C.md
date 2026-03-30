# 📋 Reporte C: estructura de carpetas, organización y validación de referencias

> **Fecha:** 28 de Marzo de 2026  
> **Proyecto:** ADead-BIB  
> **Objetivo del documento:** definir una estructura lógica, escalable y mantenible para todo el ecosistema C del repositorio, validando carpetas, configuración, dependencias y rutas relevantes

---

## 1. Alcance

Este documento cubre el **ecosistema C completo del repositorio**, no solo el frontend del compilador. En ADead-BIB, el “proyecto C” está distribuido entre:

- el **compilador C** implementado en Rust
- la **stdlib C / catálogos de símbolos y headers**
- los **tests y fixtures C**
- el **código C del kernel y runtime de FastOS**
- la **documentación técnica y archivos de build**

Esto significa que la estructura profesional del proyecto debe contemplar tanto las carpetas de código C consumido por el compilador como las carpetas del compilador que implementan soporte para C.

---

## 2. Resumen ejecutivo

La estructura actual del repositorio ya contiene los bloques fundamentales correctos, pero estaban dispersos en la documentación previa. A nivel organizativo, los componentes C necesarios hoy son:

| Bloque | Estado | Función |
|---|---|---|
| `src/rust/crates/frontend/c/adeb-frontend-c/` | ✅ | frontend C del compilador |
| `src/rust/crates/shared/adeb-stdlib/src/c/` | ✅ | módulos lógicos de stdlib C |
| `src/rust/crates/middle/adeb-middle/` | ✅ | IR, passes y validación intermedia |
| `src/rust/crates/backend/cpu/adeb-backend-x64/` | ✅ | backend nativo para salida PE/x64 |
| `src/rust/crates/app/ADead-BIB-Main/` | ✅ | CLI, driver y modo `step` |
| `tests/c/` | ✅ | suite de pruebas C e integración |
| `FastOS_v2/kernel/` | ✅ | código C real de sistema operativo / runtime |
| `docs/` | ✅ | documentación técnica del ecosistema C |

La recomendación principal es **mantener esta separación por dominio**:

1. **frontend C**
2. **stdlib y headers**
3. **middle-end y backend**
4. **tests C**
5. **código C del sistema / runtime**
6. **documentación y build**

---

## 3. Jerarquía completa de directorios relevantes para C

La siguiente jerarquía recoge las carpetas necesarias del repositorio para un proyecto C serio dentro de ADead-BIB, usando la estructura real encontrada y organizada por responsabilidad.

```text
ADead-BIB/
├── docs/
│   └── reportes_C.md
│
├── tests/
│   └── c/
│       ├── README.md
│       └── fixtures/
│           ├── 01_ctype_basic.c
│           ├── 02_ctype_extended.c
│           ├── 03_ctype_loop_parser.c
│           └── 04_ctype_edge_cases.c
│
├── FastOS_v2/
│   ├── kernel/
│   │   ├── include/
│   │   │   ├── hal.h
│   │   │   └── kernel.h
│   │   ├── lib/
│   │   │   ├── printf.c
│   │   │   └── string.c
│   │   ├── gdt.c
│   │   ├── heap.c
│   │   ├── idt.c
│   │   ├── kernel.c
│   │   ├── keyboard.c
│   │   ├── pic.c
│   │   ├── pmm.c
│   │   ├── scheduler.c
│   │   ├── shell.c
│   │   ├── timer.c
│   │   ├── vga.c
│   │   └── vmm.c
│   ├── legacy/
│   │   ├── boot.asm
│   │   └── stage2.asm
│   ├── Makefile
│   ├── build.ps1
│   └── kernel.ld
│
├── src/
│   └── rust/
│       ├── Cargo.toml
│       ├── resources/
│       │   └── errors.json
│       └── crates/
│           ├── app/
│           │   └── ADead-BIB-Main/
│           ├── frontend/
│           │   ├── c/
│           │   │   └── adeb-frontend-c/
│           │   ├── cpp/
│           │   │   └── adeb-frontend-cpp/
│           │   ├── cuda/
│           │   │   └── adeb-frontend-cuda/
│           │   └── js/
│           ├── middle/
│           │   └── adeb-middle/
│           │       └── src/
│           │           ├── analysis/
│           │           ├── ir/
│           │           ├── optimizer/
│           │           ├── passes.rs
│           │           └── ub_detector/
│           ├── backend/
│           │   ├── cpu/
│           │   │   └── adeb-backend-x64/
│           │   └── gpu/
│           │       └── adeb-backend-gpu/
│           ├── shared/
│           │   ├── adeb-core/
│           │   ├── adeb-platform/
│           │   └── adeb-stdlib/
│           └── security/
│               └── adeb-bg/
│
├── Cargo.toml
├── ARCHITECTURE.md
├── README.md
└── .gitignore
```

---

## 4. Definición clara de cada carpeta

### 4.1 Documentación

| Carpeta | Propósito |
|---|---|
| `docs/` | documentación técnica, reportes de estado, decisiones de arquitectura y guías de mantenimiento |

### 4.2 Tests y muestras C

| Carpeta | Propósito |
|---|---|
| `tests/c/` | fixtures y pruebas funcionales del compilador C, enfocados en headers, parsing y regresiones |

### 4.3 Código C real de plataforma

| Carpeta | Propósito |
|---|---|
| `FastOS_v2/kernel/` | código C de kernel y runtime bare-metal |
| `FastOS_v2/kernel/include/` | headers públicos/compartidos del kernel |
| `FastOS_v2/kernel/lib/` | utilidades C reusables de bajo nivel |
| `FastOS_v2/legacy/` | arranque heredado y piezas de compatibilidad temprana |

### 4.4 Núcleo del compilador C

| Carpeta | Propósito |
|---|---|
| `src/rust/crates/frontend/c/adeb-frontend-c/` | frontend C: preprocesado, lexer, parser, AST y lowering |
| `src/rust/crates/frontend/c/adeb-frontend-c/src/parse/` | análisis léxico y sintáctico |
| `src/rust/crates/frontend/c/adeb-frontend-c/src/lower/` | traducción del AST C al IR interno |

### 4.5 Infraestructura compartida del compilador

| Carpeta | Propósito |
|---|---|
| `src/rust/crates/shared/adeb-core/` | tipos base, AST compartido, símbolos, diagnósticos y toolchain helpers |
| `src/rust/crates/middle/adeb-middle/` | IR, optimizaciones, chequeos estrictos y detección de UB |
| `src/rust/crates/backend/cpu/adeb-backend-x64/` | selección de instrucciones, codegen, encoder y backend nativo |
| `src/rust/crates/shared/adeb-platform/` | formatos de salida y soporte de plataforma (`PE`, `ELF`, etc.) |

### 4.6 Biblioteca estándar y catálogos C

| Carpeta | Propósito |
|---|---|
| `src/rust/crates/shared/adeb-stdlib/src/c/` | módulos lógicos de stdlib C y runtime asociado |
| `src/rust/crates/frontend/c/adeb-frontend-c/src/stdlib.rs` | resolución de headers e inyección de prototipos/definiciones para el frontend |
| `src/rust/crates/frontend/c/adeb-frontend-c/src/compiler_extensions.rs` | compatibilidad GCC/MSVC y headers especiales/extensiones |

### 4.7 Driver y CLI

| Carpeta | Propósito |
|---|---|
| `src/rust/crates/app/ADead-BIB-Main/` | entrypoint del compilador, comandos CLI, modo `step`, build driver |
| `src/rust/crates/app/ADead-BIB-Main/src/cli/` | utilidades de terminal, formato y salida de inspección |

---

## 5. Estructura lógica y profesional recomendada

La estructura real del repositorio es válida, pero para documentarla profesionalmente conviene agruparla en capas:

### Capa 1 — Producto y documentación

- `docs/`
- `README.md`
- `ARCHITECTURE.md`

### Capa 2 — Código C objetivo y runtime

- `FastOS_v2/`
- `tests/c/`

### Capa 3 — Implementación del compilador

- `src/rust/crates/frontend/c/adeb-frontend-c/`
- `src/rust/crates/shared/adeb-core/`
- `src/rust/crates/middle/adeb-middle/`
- `src/rust/crates/backend/cpu/adeb-backend-x64/`
- `src/rust/crates/shared/adeb-platform/`
- `src/rust/crates/app/ADead-BIB-Main/`

### Capa 4 — Biblioteca estándar y headers

- `src/rust/crates/shared/adeb-stdlib/src/c/`
- `src/rust/crates/frontend/c/adeb-frontend-c/src/stdlib.rs`
- `src/rust/crates/frontend/c/adeb-frontend-c/src/compiler_extensions.rs`

Esta organización es la más conveniente porque separa:

1. **el compilador**
2. **las bibliotecas soportadas**
3. **el código C real**
4. **la documentación**
5. **la validación**

---

## 6. Convenciones de nomenclatura

### 6.1 Carpetas

| Tipo | Convención | Ejemplo |
|---|---|---|
| Módulos Rust del compilador | `kebab-case` con prefijo de dominio | `adeb-frontend-c`, `adeb-backend-x64` |
| Submódulos internos | nombre corto y semántico | `parse`, `lower`, `optimizer`, `isa` |
| Fixtures/tests C | jerarquía por dominio y propósito | `tests/c/fixtures` |
| Código C de plataforma | nombre de producto o subsistema | `FastOS_v2`, `kernel`, `include`, `lib` |

### 6.2 Archivos

| Tipo | Convención | Ejemplo |
|---|---|---|
| Archivos C | `snake_case.c` | `scheduler.c`, `keyboard.c` |
| Headers C | `snake_case.h` | `kernel.h`, `hal.h` |
| Módulos Rust | `snake_case.rs` | `preprocessor.rs`, `strict_type_checker.rs` |
| Tests C ordenados | prefijo numérico + área + caso | `01_ctype_basic.c` |
| Configuración | nombre estándar de herramienta | `Cargo.toml`, `Makefile`, `kernel.ld` |

### 6.3 Reglas recomendadas

1. Usar `snake_case` en archivos C y Rust
2. Reservar prefijos numéricos solo para fixtures ordenados
3. Mantener `include/` exclusivamente para headers públicos
4. Mantener `lib/` para implementaciones C reutilizables, no para headers
5. Separar archivos de plataforma (`kernel`, `boot`, `linker`) de tests y fixtures

---

## 7. Archivos de configuración requeridos

Los siguientes archivos existen y son necesarios para un mantenimiento correcto del ecosistema C del proyecto:

| Archivo | Ubicación | Propósito |
|---|---|---|
| `Cargo.toml` | raíz del repo | configuración general del proyecto principal |
| `src/rust/Cargo.toml` | workspace Rust | define el workspace de crates del compilador |
| `src/rust/crates/app/ADead-BIB-Main/Cargo.toml` | driver CLI | binario `adB` y dependencias del pipeline |
| `src/rust/crates/frontend/c/adeb-frontend-c/Cargo.toml` | frontend C | dependencias del frontend (`adeb-core`, `adeb-middle`) |
| `src/rust/crates/shared/adeb-stdlib/Cargo.toml` | stdlib | catálogo de stdlib C/C++ |
| `src/rust/crates/backend/cpu/adeb-backend-x64/Cargo.toml` | backend | backend nativo x64 |
| `FastOS_v2/Makefile` | build C/ASM del kernel | build principal estilo Unix |
| `FastOS_v2/build.ps1` | build Windows | automatización en PowerShell |
| `FastOS_v2/kernel.ld` | linker script | layout del kernel/binario |
| `.gitignore` | raíz | exclusión de binarios, builds y artefactos |

### Configuración adicional recomendada

Si el proyecto C de FastOS crece, la estructura soporta incorporar sin romper la jerarquía:

- `FastOS_v2/tests/`
- `FastOS_v2/tools/`
- `FastOS_v2/scripts/`
- `FastOS_v2/config/`
- `FastOS_v2/docs/`

Estas carpetas no son obligatorias hoy, pero son las extensiones más naturales siguiendo mejores prácticas.

---

## 8. Ejemplos de rutas relativas

### 8.1 Desde la raíz del repositorio

| Objetivo | Ruta relativa |
|---|---|
| Reporte C | `docs/reportes_C.md` |
| Test básico de `ctype` | `tests/c/fixtures/01_ctype_basic.c` |
| Frontend C | `src/rust/crates/frontend/c/adeb-frontend-c/src/` |
| Parser C | `src/rust/crates/frontend/c/adeb-frontend-c/src/parse/parser.rs` |
| Lowering C a IR | `src/rust/crates/frontend/c/adeb-frontend-c/src/lower/to_ir.rs` |
| Stdlib C lógica | `src/rust/crates/shared/adeb-stdlib/src/c/` |
| Driver CLI | `src/rust/crates/app/ADead-BIB-Main/src/main.rs` |
| Kernel headers | `FastOS_v2/kernel/include/` |
| Librería C del kernel | `FastOS_v2/kernel/lib/` |

### 8.2 Desde `src/rust/`

| Objetivo | Ruta relativa |
|---|---|
| Frontend C | `crates/frontend/c/adeb-frontend-c/src/` |
| Stdlib C | `crates/shared/adeb-stdlib/src/c/` |
| Backend x64 | `crates/backend/cpu/adeb-backend-x64/src/` |
| Driver CLI | `crates/app/ADead-BIB-Main/src/` |
| Fixture C | `../../tests/c/fixtures/01_ctype_basic.c` |

### 8.3 Desde `FastOS_v2/`

| Objetivo | Ruta relativa |
|---|---|
| Header principal | `kernel/include/kernel.h` |
| Biblioteca auxiliar | `kernel/lib/printf.c` |
| Linker script | `kernel.ld` |
| Build script PowerShell | `build.ps1` |

---

## 9. Validación de dependencias y bibliotecas

### 9.1 Dependencias entre crates del pipeline C

Validación realizada contra los `Cargo.toml` reales del repositorio:

| Componente | Dependencias validadas | Estado |
|---|---|---|
| `adeb-frontend-c` | `adeb-core`, `adeb-middle` | ✅ |
| `adeb-stdlib` | `adeb-core` | ✅ |
| `adeb-backend-x64` | `adeb-core`, `adeb-middle`, `adeb-platform` | ✅ |
| `ADead-BIB-Main` | `adeb-frontend-c`, `adeb-middle`, `adeb-backend-x64`, `adeb-stdlib`, `adeb-core` | ✅ |

### 9.2 Referencias de bibliotecas C internas

| Referencia lógica | Ubicación validada | Estado |
|---|---|---|
| Módulos `fastos_*` de C | `src/rust/crates/shared/adeb-stdlib/src/c/` | ✅ |
| Resolución de headers C | `src/rust/crates/frontend/c/adeb-frontend-c/src/stdlib.rs` | ✅ |
| Extensiones GCC/MSVC y headers especiales | `src/rust/crates/frontend/c/adeb-frontend-c/src/compiler_extensions.rs` | ✅ |
| AST / lexer / parser / lowering C | `src/rust/crates/frontend/c/adeb-frontend-c/src/` | ✅ |
| IR / optimizadores | `src/rust/crates/middle/adeb-middle/src/` | ✅ |
| Generación x64 | `src/rust/crates/backend/cpu/adeb-backend-x64/src/` | ✅ |

### 9.3 Referencias de código C real y pruebas

| Área | Ubicación validada | Estado |
|---|---|---|
| Fixtures de regresión | `tests/c/fixtures/` | ✅ |
| Código C de kernel | `FastOS_v2/kernel/*.c` | ✅ |
| Headers de kernel | `FastOS_v2/kernel/include/` | ✅ |
| Librerías auxiliares C del kernel | `FastOS_v2/kernel/lib/` | ✅ |
| Configuración de build del kernel | `FastOS_v2/Makefile`, `FastOS_v2/build.ps1`, `FastOS_v2/kernel.ld` | ✅ |

### 9.4 Interpretación de la validación

La validación confirma que:

1. las carpetas y archivos clave **existen**
2. las dependencias entre crates del pipeline C están **declaradas**
3. las bibliotecas y módulos C están **ubicados de forma coherente**
4. las rutas de tests, kernel, frontend, stdlib y backend son **consistentes**

No debe confundirse esta validación con soporte funcional total del estándar C: aquí se valida la **organización y referencia estructural**, no la cobertura semántica completa.

---

## 10. Estructura objetivo recomendada para escalabilidad

La estructura actual es buena; la estructura objetivo profesional para crecer sin deuda debería mantenerse así:

```text
docs/                  → documentación y reportes
tests/c/               → tests y fixtures C
FastOS_v2/             → código C real del sistema
src/rust/crates/
  app/ADead-BIB-Main/                  → CLI y driver
  frontend/c/adeb-frontend-c/          → frontend C
  frontend/cpp/adeb-frontend-cpp/      → frontend C++
  frontend/cuda/adeb-frontend-cuda/    → frontend CUDA
  frontend/js/                         → reservado para frontend JS
  middle/adeb-middle/                  → IR, análisis, UB detector y optimización
  backend/cpu/adeb-backend-x64/        → backend nativo
  backend/gpu/adeb-backend-gpu/        → backend GPU
  shared/adeb-core/                    → tipos base y símbolos
  shared/adeb-platform/                → formatos de salida
  shared/adeb-stdlib/                  → stdlib y catálogos C
  security/adeb-bg/                    → Binary Guardian
```

### Razones por las que esta estructura es correcta

| Criterio | Cumplimiento |
|---|---|
| Modularidad | ✅ separa frontend, stdlib, middle-end y backend |
| Escalabilidad | ✅ permite crecer por crate y por dominio |
| Mantenibilidad | ✅ cada capa tiene una responsabilidad clara |
| Reusabilidad | ✅ tests, kernel y stdlib no se mezclan con CLI |
| Profesionalismo | ✅ sigue separación por subsistema, no por “archivos sueltos” |
| Portabilidad | ✅ permite convivir con build C tradicional y pipeline Rust |

---

## 11. Reglas de mantenimiento recomendadas

1. No mezclar fixtures C en carpetas del compilador
2. No colocar headers públicos dentro de `lib/`
3. No colocar archivos de build del kernel dentro del workspace Rust
4. Mantener todos los módulos de soporte C dentro de `adeb-frontend-c` o `adeb-stdlib`, nunca repartidos arbitrariamente
5. Centralizar nuevas referencias de headers en una sola capa de resolución
6. Documentar toda carpeta nueva en `docs/reportes_C.md`
7. Añadir tests de regresión en `tests/c/fixtures/` para cada nuevo header, extensión o bug corregido

---

## 12. Conclusión

La estructura actual del ecosistema C de ADead-BIB ya dispone de los bloques esenciales para un proyecto profesional:

- compilador C modular
- stdlib y resolución de headers
- middle-end y backend separados
- tests C dedicados
- código C real de plataforma
- documentación y build scripts en ubicaciones coherentes

La mejora principal aportada por este reporte es dejar explícita una **jerarquía completa, validada y escalable**, con propósito por carpeta, convención de nombres, archivos de configuración y rutas relativas de referencia.

En su estado actual, la estructura es **modular, mantenible y apta para crecer**, siempre que se preserve esta separación de responsabilidades y se continúe documentando cualquier nueva carpeta o dependencia del ecosistema C.

---

## 13. Mejoras implementadas — Sesión de auditoría C99/C11

> **Fecha:** Julio 2026  
> **Tests:** 98 → 124 (26 nuevos tests, 4 bugs corregidos)

### 13.1 Bugs corregidos en el parser

| Bug | Causa raíz | Fix |
|---|---|---|
| `_Static_assert` dentro de funciones fallaba | `parse_expression()` consumía la coma como operador comma | Usar `parse_assign_expr()` en lugar de `parse_expression()` |
| Inicializadores anidados `{{1,2},{3,4}}` fallaban | `parse_brace_init()` no recursaba al encontrar `{` interno | Agregar rama recursiva para `CToken::LBrace` en `parse_brace_init()` |
| Arrays multidimensionales con init anidado | Mismo problema de brace init | Mismo fix recursivo |
| Patrón real-world con struct arrays | Cascada del bug de init anidado | Mismo fix recursivo |

### 13.2 Lowering (to_ir.rs) — Funcionalidad restaurada

| Feature | Estado anterior | Estado actual |
|---|---|---|
| `goto` / labels | No-op (silenciosamente descartados) | Emit `Stmt::JumpTo` / `Stmt::LabelDef` |
| `CInitializer::List` | `TODO: None` en 3 ubicaciones | `convert_init_list()` recursivo → `Expr::Array` |
| `scanf` / `sscanf` / `fscanf` | Pasaba por ruta genérica sin reconocimiento | Ruta explícita como statement call |

### 13.3 Headers C99/C11 agregados a stdlib.rs

| Header | Contenido |
|---|---|
| `fenv.h` | `fenv_t`, `fexcept_t`, funciones de floating-point environment (C99 §7.6) |
| `iso646.h` | Macros de operadores alternativos (vacío — manejado por keywords) |
| `stdalign.h` | `alignas`/`alignof` (vacío — manejado por keywords `_Alignas`/`_Alignof`) |
| `stdnoreturn.h` | `noreturn` (vacío — manejado por keyword `_Noreturn`) |
| `stdatomic.h` | Tipos atómicos C11, `atomic_flag`, `atomic_load/store/exchange/fetch_*` |
| `threads.h` | Tipos y funciones C11 threads: `thrd_*`, `mtx_*`, `cnd_*`, `tss_*`, `call_once` |

### 13.4 Tests agregados (22 nuevos)

**Lowering (to_ir.rs):**
- `test_goto_label_lowering` — verifica `JumpTo`/`LabelDef` en IR
- `test_fenv_header_available` — `#include <fenv.h>` compila
- `test_stdatomic_header_available` — `#include <stdatomic.h>` compila
- `test_threads_header_available` — `#include <threads.h>` compila
- `test_nested_struct_array_init` — `struct Vec2 pts[4] = {{0,0},...}`
- `test_scanf_as_call` — `scanf("%d", &x)` se convierte correctamente
- `test_multiple_goto_labels` — goto loop con múltiples labels
- `test_do_while_lowering` — do-while genera IR correcto
- `test_switch_with_default` — switch con case + default
- `test_ternary_expression` — operador ternario `?:`
- `test_compound_assignment_ops` — todos los `+=, -=, *=, /=, %=, &=, |=, ^=, <<=, >>=`
- `test_enum_constants_in_expressions` — enum → constante numérica
- `test_typedef_resolution` — typedef se resuelve en declaraciones
- `test_string_concat_lowering` — concatenación de strings adyacentes
- `test_cast_expression` — cast `(int)d`, `(void *)0`
- `test_sizeof_types_lowering` — `sizeof(int)`, `sizeof(char)`, etc.
- `test_nested_function_calls` — `add(mul(2,3), mul(4,5))`
- `test_pointer_arithmetic_patterns` — `*(p + 2)`

**Stdlib (stdlib.rs):**
- `test_c99_c11_additional_headers` — 6 nuevos headers disponibles
- `test_fenv_has_functions` — funciones `feclearexcept`, `fegetround`
- `test_stdatomic_has_types_and_ops` — tipos y operaciones atómicas
- `test_threads_has_types_and_ops` — funciones de threading C11

### 13.5 Backend PE — Bug .text > 5KB corregido

> **Fecha:** Marzo 2026

**Problema:** Los fixtures 02 y 03 generaban PEs que crasheaban al ejecutar porque el `.text` section excedía 4KB (0x1000 bytes). La causa raíz era que `idata_rva` estaba hardcodeado a `0x2000`, dando al `.text` solo el rango RVA `0x1000`–`0x1FFF`. Cuando el código excedía ese espacio virtual, solapaba con `.idata`, corrompiendo la Import Address Table.

**Solución:**
1. **PE builder** (`lib.rs`): `idata_rva` se calcula dinámicamente según el tamaño real del código: `text_rva + align_up(code.len(), section_alignment)`
2. **Patching de código**: Cuando `idata_rva` difiere del valor asumido (`0x2000`), el PE builder parchea todos los `string_imm64_offsets` (direcciones absolutas de strings) y `iat_call_offsets` (desplazamientos RIP-relativos a IAT) con el delta correcto
3. **Constante compartida**: `pe::ASSUMED_IDATA_RVA` centraliza el valor asumido entre ISA compiler y PE builder

**Resultado:** 18/18 fixtures ejecutan correctamente. Sin límite de tamaño de `.text`.

### 13.6 Gaps restantes identificados

| Gap | Prioridad | Notas |
|---|---|---|
| printf %f (float formatting) | Media | Formateo de floats en runtime |
| math.h → msvcrt linking | Media | sin()/cos() retornan 0 en runtime |
| `_Generic` (C11) | Media | Requiere nodo AST + parser + lowering |
| Compound literals `(Type){...}` | Media | Parser y lowering |
| VLA (Variable Length Arrays) | Baja | Solo declaración, no runtime |
| Function pointer calls en lowering | Media | Actualmente emite `__fptr_call` marker |
| Union lowering diferenciado | Baja | Actualmente se trata como struct |
| Designator support en lowering | Baja | `.field=val` parseado pero ignorado en convert_init_list |
