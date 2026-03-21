# ADead-BIB — Reporte de Análisis Completo
## "Respetar Bits" — Type Strictness ULTRA Agresivo
### Fecha: Marzo 2026

---

## 📊 RESUMEN EJECUTIVO

| Categoría | Estado Actual | Objetivo | Gap |
|-----------|---------------|----------|-----|
| Type Checker Estricto | ⚠️ Parcial | 100% | **60%** |
| Overflow Detection | ✅ Básico | Compile-time | **40%** |
| Signed/Unsigned Mix | ❌ No existe | Bloquear | **100%** |
| Implicit Cast Detection | ⚠️ Parcial | Bloquear | **70%** |
| Step Mode Phases | 7 fases | 13 fases | **6 fases** |
| Error Messages | Básico | FORTRAN-style | **80%** |

---

## 🔍 ANÁLISIS DETALLADO

### 1. SISTEMA DE TIPOS ACTUAL

**Archivo:** `src/rust/frontend/types.rs`

**✅ LO QUE EXISTE:**
```rust
pub enum Type {
    I8, I16, I32, I64,        // Signed integers
    U8, U16, U32, U64,        // Unsigned integers
    F32, F64,                  // Floats
    Bool, Void, Str,          // Primitives
    Pointer(Box<Type>),       // Pointers
    Array(Box<Type>, Option<usize>),
    Struct(String), Class(String),
    // ...
}
```

**Métodos existentes:**
- `is_signed()` → detecta I8/I16/I32/I64
- `is_unsigned()` → detecta U8/U16/U32/U64
- `is_float()` → detecta F32/F64
- `is_numeric()` → detecta todos los numéricos
- `size_bytes()` → tamaño en bytes

**❌ LO QUE FALTA:**
- `cpu_unit()` → "ALU" vs "FPU" (para mensajes de error)
- `bit_representation()` → "IEEE 754" vs "complemento a 2"
- `check_compatible(other: &Type)` → verificación estricta
- `is_implicitly_convertible_to(target: &Type)` → DEBE RETORNAR FALSE SIEMPRE

---

### 2. TYPE CHECKER ACTUAL

**Archivo:** `src/rust/frontend/type_checker.rs` (701 líneas)

**✅ LO QUE EXISTE:**
```rust
pub struct TypeChecker {
    symbol_table: HashMap<String, Type>,
    function_registry: HashMap<String, FuncSig>,
    struct_registry: HashMap<String, StructFields>,
    current_return_type: Type,
}
```

**Funciones existentes:**
- `check_program()` → analiza programa completo
- `check_function()` → verifica funciones
- `infer_expr()` → infiere tipos de expresiones
- `types_compatible()` → **PROBLEMA: ES PERMISIVO**

**❌ PROBLEMA CRÍTICO en `types_compatible()`:**
```rust
fn types_compatible(&self, declared: &Type, inferred: &Type) -> bool {
    // ...
    // Numérico ↔ numérico es compatible (conversión implícita)
    if declared.is_numeric() && inferred.is_numeric() {
        return true;  // 💀 ESTO PERMITE int + float
    }
    // ...
}
```

**❌ LO QUE FALTA:**
1. **TypeCompatResult enum** con:
   - `Ok(CType)`
   - `Mismatch { left, right, op, suggestions }`
   - `SignedUnsignedMix { signed, unsigned, suggestions }`
   - `NarrowingConversion { from, to, suggestion }`

2. **check_types_compatible()** función estricta que:
   - BLOQUEA int + float
   - BLOQUEA signed + unsigned
   - BLOQUEA int32 + int64
   - BLOQUEA float + double

3. **Mensajes de error estilo FORTRAN 1957**

---

### 3. UB DETECTOR ACTUAL

**Directorio:** `src/rust/middle/ub_detector/`

**Archivos existentes (14):**
```
ub_detector/
├── mod.rs              # Coordinador principal
├── analyzer.rs         # Análisis general
├── bounds_check.rs     # Array bounds
├── cache.rs            # Cache de análisis
├── format_check.rs     # Format strings
├── lifetime.rs         # Lifetime analysis
├── null_check.rs       # Null pointer
├── overflow_check.rs   # Integer overflow ⚠️
├── race_check.rs       # Data races
├── report.rs           # Reportes UB
├── type_check.rs       # Type safety ⚠️
├── uninit_check.rs     # Uninitialized vars
├── unsequenced_check.rs # Unsequenced mods
└── useafter_check.rs   # Use-after-free
```

**✅ LO QUE EXISTE en `overflow_check.rs`:**
- Detección de overflow en compile-time para constantes
- Detección de división por cero
- Detección de shift overflow
- Detección de signed promotion overflow

**❌ LO QUE FALTA en `overflow_check.rs`:**
- `needs_runtime_check()` para generar checks en runtime
- Detección de INT_MAX + 1 en variables
- Sugerencias de wrapping explícito

**✅ LO QUE EXISTE en `type_check.rs`:**
- Detección de casts peligrosos (literal → pointer)
- Detección de strict aliasing violation
- `is_compatible_for_aliasing()` básico

**❌ LO QUE FALTA en `type_check.rs`:**
- **TypeMismatch** para int + float
- **SignedUnsignedMix** para signed vs unsigned
- **NarrowingConversion** para double → int
- **ImplicitCast** para void* → int*

---

### 4. STEP MODE ACTUAL

**Archivo:** `src/rust/main.rs` (función `step_compile_c`)

**✅ FASES ACTUALES (7):**
1. SOURCE — líneas, bytes, includes
2. PREPROCESSOR — macros, expansión
3. LEXER — tokens, distribución
4. PARSER — funciones, structs, typedefs
5. IR — statements, análisis
6. UB DETECTOR — errores, warnings
7. CODEGEN — bytes, IAT, hex dump

**❌ FASES FALTANTES (6):**
8. **IMPORT RESOLVER** — headers procesados
9. **TYPE CHECKER** — verificación estricta de tipos
10. **STRICT TYPE ANALYSIS** — ALU vs FPU, bits
11. **OVERFLOW DETECTOR** — compile-time overflow
12. **SIGNED/UNSIGNED ANALYZER** — mezcla detectada
13. **PE LINKER** — secciones, relocaciones

---

### 5. ARCHIVOS A CREAR

#### 5.1 `src/rust/middle/type_checker.rs` (NUEVO)

```rust
// ============================================================
// Type Checker ULTRA Estricto — "Respetar Bits"
// FORTRAN 1957 + Ada 1983 + ADead-BIB 2025
// ============================================================

#[derive(Debug, Clone, PartialEq)]
pub enum CType {
    Int8, Int16, Int32, Int64,
    UInt8, UInt16, UInt32, UInt64,
    Float32, Float64,
    Char, Bool, Void,
    Pointer(Box<CType>),
    Array(Box<CType>, usize),
    Struct(String),
}

impl CType {
    pub fn cpu_unit(&self) -> &str {
        match self {
            CType::Float32 | CType::Float64 => "FPU (XMM)",
            _ => "ALU (RAX/EAX)",
        }
    }
    
    pub fn bit_representation(&self) -> &str {
        match self {
            CType::Float32 => "IEEE 754 32-bit",
            CType::Float64 => "IEEE 754 64-bit",
            CType::Int32 => "complemento a 2, 32-bit",
            CType::Int64 => "complemento a 2, 64-bit",
            CType::UInt32 => "binario sin signo, 32-bit",
            _ => "representación específica",
        }
    }
}

#[derive(Debug)]
pub enum TypeCompatResult {
    Ok(CType),
    Mismatch {
        left: CType,
        right: CType,
        op: String,
        suggestions: Vec<String>,
    },
    SignedUnsignedMix {
        signed: CType,
        unsigned: CType,
        suggestions: Vec<String>,
    },
    NarrowingConversion {
        from: CType,
        to: CType,
        suggestion: String,
    },
    ImplicitCast {
        from: CType,
        to: CType,
        suggestion: String,
    },
}

pub fn check_types_compatible(
    left: &CType,
    right: &CType,
    op: &str,
) -> TypeCompatResult {
    // IMPLEMENTAR: reglas estrictas
}
```

#### 5.2 `src/rust/middle/overflow_detector.rs` (NUEVO)

```rust
// ============================================================
// Overflow Detector — Compile-time + Runtime
// ============================================================

pub struct OverflowDetector;

impl OverflowDetector {
    pub fn check_compile_time(
        typ: &CType,
        op: &str,
        left: Option<i64>,
        right: Option<i64>,
        line: usize,
    ) -> Option<UBError> {
        // IMPLEMENTAR: checked_add, checked_sub, checked_mul
    }
    
    pub fn needs_runtime_check(typ: &CType, op: &str) -> bool {
        matches!(
            (typ, op),
            (CType::Int32 | CType::Int64, "+" | "-" | "*")
        )
    }
}
```

---

### 6. MODIFICACIONES REQUERIDAS

#### 6.1 `src/rust/middle/ub_detector/type_check.rs`

**AGREGAR:**
```rust
// En check_expr_types, agregar:

Expr::BinaryOp { left, op, right, .. } => {
    let lt = infer_c_type(left, env);
    let rt = infer_c_type(right, env);
    
    match check_types_compatible(&lt, &rt, op) {
        TypeCompatResult::Mismatch { left, right, op, suggestions } => {
            reports.push(UBReport::new(
                UBSeverity::Error,
                UBKind::TypeMismatch,  // NUEVO UBKind
                format!("TypeMismatch: {} {} {}", left, op, right),
            ).with_suggestions(suggestions));
        },
        TypeCompatResult::SignedUnsignedMix { signed, unsigned, suggestions } => {
            reports.push(UBReport::new(
                UBSeverity::Error,
                UBKind::SignedUnsignedMix,  // NUEVO UBKind
                format!("SignedUnsignedMix: {:?} vs {:?}", signed, unsigned),
            ).with_suggestions(suggestions));
        },
        // ...
    }
}
```

#### 6.2 `src/rust/middle/ub_detector/report.rs`

**AGREGAR UBKinds:**
```rust
pub enum UBKind {
    // Existentes...
    NullPointerDereference,
    UseAfterFree,
    DoubleFree,
    ArrayOutOfBounds,
    IntegerOverflow,
    IntegerUnderflow,
    DivisionByZero,
    // ...
    
    // NUEVOS para "Respetar Bits":
    TypeMismatch,           // int + float
    SignedUnsignedMix,      // signed vs unsigned
    NarrowingConversion,    // double → int
    ImplicitCast,           // void* → int*
    ImplicitConstruction,   // C++ Vec2 v = 5.0f
}
```

#### 6.3 `src/rust/main.rs` — Step Mode

**EXPANDIR a 13 fases:**
```rust
fn step_compile_c(input_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Phase 0: SOURCE (existente)
    // Phase 1: PREPROCESSOR (existente)
    // Phase 2: LEXER (existente)
    // Phase 3: PARSER (existente)
    // Phase 4: IMPORT RESOLVER (NUEVO)
    // Phase 5: TYPE CHECKER (NUEVO - estricto)
    // Phase 6: IR (existente, renumerado)
    // Phase 7: STRICT TYPE ANALYSIS (NUEVO)
    // Phase 8: OVERFLOW DETECTOR (NUEVO)
    // Phase 9: SIGNED/UNSIGNED ANALYZER (NUEVO)
    // Phase 10: UB DETECTOR (existente, renumerado)
    // Phase 11: CODEGEN (existente, renumerado)
    // Phase 12: PE LINKER (NUEVO)
    // Phase 13: OUTPUT (existente, renumerado)
}
```

---

### 7. TESTS A CREAR

**Directorio:** `tests/c_abi/`

#### 7.1 `test_type_strictness.c`

```c
// === DEBEN FALLAR ===
void test_int_plus_float() {
    int x = 5;
    float y = 3.14f;
    float z = x + y;      // 💀 TypeMismatch
}

void test_signed_unsigned_compare() {
    int s = -1;
    unsigned int u = 5;
    if (s < u) { }        // 💀 SignedUnsignedMix
}

void test_implicit_ptr_cast() {
    void* ptr = malloc(4);
    int* iptr = ptr;      // 💀 ImplicitCast
}

void test_integer_overflow() {
    int x = 2147483647;
    x = x + 1;            // 💀 IntegerOverflow
}

// === DEBEN PASAR ===
void test_int_plus_int() {
    int x = 5 + 3;        // ✅
}

void test_explicit_cast() {
    int x = 5;
    float y = (float)x + 3.14f; // ✅
}
```

#### 7.2 `test_type_strictness_cpp.cpp`

```cpp
// === DEBEN FALLAR ===
template<typename T>
T suma(T a, T b) { return a + b; }

void test_template_mix() {
    suma(5, 3.14f);        // 💀 TypeMismatch
}

class Vec2 {
public:
    Vec2(float val) {}
};

void test_implicit_constructor() {
    Vec2 v = 5.0f;         // 💀 ImplicitConstruction
}

// === DEBEN PASAR ===
void test_explicit_constructor() {
    Vec2 v(5.0f);          // ✅
}
```

---

### 8. MENSAJES DE ERROR — FORMATO EXACTO

```
[UB] TypeMismatch detectado — ADead-BIB
     archivo: src/main.c
     línea: 47, columna: 12

     operación: int32 + float32

     lado izquierdo:
       tipo: int32 (complemento a 2)
       unidad CPU: ALU
       registro: EAX/RAX

     lado derecho:
       tipo: float32 (IEEE 754)
       unidad CPU: FPU
       registro: XMM0

     problema:
       ALU y FPU son unidades diferentes
       bits con representación incompatible
       mezclar = UB garantizado

     soluciones:
       (float)x + y      ← convierte int a float
       x + (int)y        ← convierte float a int

     filosofía:
       "los bits merecen respeto"
       "FORTRAN lo supo en 1957"
       "ADead-BIB lo aplica en 2025"

     compilación bloqueada 💀
     Binary Is Binary — ADead-BIB 💀🦈
```

---

## 📋 CHECKLIST DE IMPLEMENTACIÓN

### Prioridad ALTA (Bloquean compilación):
- [ ] Crear `src/rust/middle/strict_type_checker.rs`
- [ ] Agregar `TypeMismatch`, `SignedUnsignedMix` a UBKind
- [ ] Modificar `type_check.rs` para usar reglas estrictas
- [ ] Bloquear int + float en `check_types_compatible()`
- [ ] Bloquear signed + unsigned en comparaciones
- [ ] Bloquear void* → T* sin cast explícito

### Prioridad MEDIA (Mejoran detección):
- [ ] Crear `src/rust/middle/overflow_detector.rs` mejorado
- [ ] Agregar `needs_runtime_check()` para overflow
- [ ] Detectar INT_MAX + 1 en variables
- [ ] Agregar sugerencias de wrapping explícito

### Prioridad BAJA (UX):
- [ ] Expandir step mode a 13 fases
- [ ] Mejorar mensajes de error estilo FORTRAN
- [ ] Agregar `cpu_unit()` y `bit_representation()` a Type
- [ ] Crear tests en `tests/c_abi/`

---

## 🎯 OBJETIVO FINAL

```
╔══════════════════════════════════════════════════════════════╗
║      "Respetar Bits" — ADead-BIB C/C++                       ║
╠══════════════════════════════════════════════════════════════╣
║  FORTRAN 1957: tipos estrictos    ✅                          ║
║  Ada 1983:     más estricto       ✅                          ║
║  ADead-BIB:    el más estricto    💀                          ║
╠══════════════════════════════════════════════════════════════╣
║  int   + int   = respeto          ✅                          ║
║  float + float = respeto          ✅                          ║
║  int   + float = irrespeto        💀 BLOQUEADO               ║
║  signed + unsigned = irrespeto    💀 BLOQUEADO               ║
║  overflow silencioso = irrespeto  💀 BLOQUEADO               ║
║  implicit cast = irrespeto        💀 BLOQUEADO               ║
╠══════════════════════════════════════════════════════════════╣
║  cast explícito = respeto         ✅                          ║
║  mismo tipo = respeto             ✅                          ║
║  dev consciente = respeto         ✅                          ║
╠══════════════════════════════════════════════════════════════╣
║  Binary Is Binary 💀🦈 — Lima, Perú 🇵🇪                        ║
╚══════════════════════════════════════════════════════════════╝
```

---

## 📁 ESTRUCTURA DE ARCHIVOS FINAL

```
src/rust/
├── middle/
│   ├── strict_type_checker.rs    # NUEVO — Type strictness ULTRA
│   ├── overflow_detector.rs      # NUEVO — Compile-time overflow
│   └── ub_detector/
│       ├── mod.rs                # Actualizar con nuevos UBKinds
│       ├── type_check.rs         # Modificar para usar strict checker
│       └── report.rs             # Agregar TypeMismatch, SignedUnsignedMix
├── frontend/
│   ├── types.rs                  # Agregar cpu_unit(), bit_representation()
│   └── type_checker.rs           # Modificar types_compatible() → ESTRICTO
└── main.rs                       # Expandir step mode a 13 fases

tests/
└── c_abi/
    ├── test_type_strictness.c    # NUEVO
    └── test_type_strictness_cpp.cpp # NUEVO
```

---

**Generado por:** ADead-BIB Analysis System  
**Versión:** v8.0  
**Filosofía:** "Respetar Bits" — FORTRAN 1957 Heritage
