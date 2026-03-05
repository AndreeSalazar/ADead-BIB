# UB_Detector Architecture — Undefined Behavior Detection

## Pipeline Completo

```
C99/C++98 Source Code
        ↓
    Parser/AST
        ↓
    IR (ADeadOp)
        ↓
  ┌─────────────┐
  │ UB_Detector │ ← ANÁLISIS ANTES DE CODEGEN
  └─────────────┘
    - Null pointer dereference
    - Array bounds violations
    - Integer overflow/underflow
    - Division by zero
    - Use-after-free
    - Double free
    - Uninitialized variables
        ↓
   ISA Compiler
        ↓
     Encoder
        ↓
   PE/ELF/Po
```

## Arquitectura del UB_Detector

### Ubicación
`src/rust/middle/ub_detector/`

### Módulos

#### 1. `mod.rs` — Coordinador Principal
- **UBDetector**: Estructura principal que coordina todos los análisis
- Ejecuta análisis en orden:
  1. Null safety
  2. Bounds checking
  3. Overflow detection
  4. Lifetime analysis
- Ordena reportes por severidad
- Determina si hay errores que bloquean compilación

#### 2. `report.rs` — Sistema de Reportes
- **UBSeverity**: Error | Warning | Info
- **UBKind**: 13 tipos de UB detectables
- **UBReport**: Reporte con ubicación, mensaje, sugerencia
- Salida con colores ANSI (rojo/amarillo/cyan)

#### 3. `null_check.rs` — Detección de Null Pointer
- Analiza `Deref`, `ArrowAccess`, `ArrowAssign`
- Detecta uso de `Nullptr` o `Number(0)` como puntero
- Reporta con sugerencia de agregar null check

#### 4. `bounds_check.rs` — Verificación de Límites de Array
- Analiza `IndexAssign` con índices constantes
- Compara índice con tamaño de array (si conocido)
- Detecta accesos fuera de rango [0..size)

#### 5. `overflow_check.rs` — Detección de Overflow/Underflow
- Analiza operaciones aritméticas: Add, Sub, Mul
- Usa `checked_add`, `checked_sub`, `checked_mul`
- Detecta división/módulo por cero
- Reporta overflow en tiempo de compilación para constantes

#### 6. `lifetime.rs` — Análisis de Lifetime
- Rastrea variables liberadas con `Free`
- Detecta use-after-free
- Detecta double-free
- Mantiene conjunto de variables freed por función

#### 7. `analyzer.rs` — Analizador General
- Módulo extensible para análisis adicionales futuros
- Coordinación de sub-analizadores

## Uso en el Compilador

### Integración en Builder

```rust
use adead_bib::middle::UBDetector;

// En builder.rs, después de generar IR:
let mut ub_detector = UBDetector::new();
let reports = ub_detector.analyze(&program);

if ub_detector.has_errors() {
    ub_detector.print_reports();
    return Err("Compilation aborted due to undefined behavior");
}

// Si solo warnings, continuar con ISA Compiler
if !reports.is_empty() {
    ub_detector.print_reports();
}
```

### Ejemplo de Salida

```
=== UB_Detector Report ===
[ERROR] Division by Zero: Division by zero: 10 / 0 in main:0
  → Suggestion: Add zero check before division
[WARN] Null Pointer Dereference: Dereferencing potentially null pointer in process_data:42
  → Suggestion: Add null check before dereference
[INFO] Array Out of Bounds: Array index 10 out of bounds [0..8) in compute:15
  → Suggestion: Index must be in range [0..8)
=========================
```

## Tipos de UB Detectados

| UB Kind | Severidad | Descripción |
|---------|-----------|-------------|
| NullPointerDereference | Warning | Deref de puntero potencialmente null |
| UseAfterFree | Error | Uso de variable después de free() |
| DoubleFree | Error | Llamada a free() dos veces |
| ArrayOutOfBounds | Error | Índice fuera de rango |
| IntegerOverflow | Error | Overflow en suma/multiplicación |
| IntegerUnderflow | Error | Underflow en resta |
| DivisionByZero | Error | División o módulo por cero |
| UninitializedVariable | Warning | Variable usada sin inicializar |
| TypeConfusion | Warning | Cast inválido entre tipos |
| StackOverflow | Warning | Recursión profunda detectada |
| DataRace | Error | Acceso concurrente sin sincronización |
| InvalidCast | Warning | Cast de puntero inválido |
| DanglingPointer | Warning | Puntero a memoria liberada |

## Extensibilidad

### Añadir Nuevo Análisis

1. Crear módulo en `ub_detector/new_check.rs`
2. Implementar función `analyze_new_pattern(program: &Program) -> Vec<UBReport>`
3. Añadir llamada en `UBDetector::analyze()`
4. Añadir nuevo `UBKind` en `report.rs`

### Ejemplo: Detectar Recursión Infinita

```rust
// En ub_detector/recursion_check.rs
pub fn analyze_recursion(program: &Program) -> Vec<UBReport> {
    let mut reports = Vec::new();
    // Construir call graph
    // Detectar ciclos sin caso base
    // Reportar como StackOverflow
    reports
}
```

## Tests

Cada módulo incluye tests unitarios:
- `test_ub_detector_creation`
- `test_null_detection`
- `test_bounds_detection`
- `test_overflow_detection`
- `test_lifetime_analysis`

Ejecutar: `cargo test ub_detector`

## Roadmap

### Fase 1 (Actual) ✓
- [x] Null pointer dereference
- [x] Array bounds (constantes)
- [x] Integer overflow (constantes)
- [x] Division by zero
- [x] Use-after-free básico
- [x] Double free

### Fase 2 (Próxima)
- [ ] Array bounds con análisis de flujo
- [ ] Overflow con análisis simbólico
- [ ] Uninitialized variable tracking
- [ ] Type confusion detection
- [ ] Stack overflow (recursión profunda)

### Fase 3 (Futuro)
- [ ] Data race detection (multi-threading)
- [ ] Taint analysis (seguridad)
- [ ] Memory leak detection
- [ ] Buffer overflow detection
- [ ] Format string vulnerabilities

## Referencias

- **LLVM UBSan**: Undefined Behavior Sanitizer
- **Rust Borrow Checker**: Lifetime analysis
- **Clang Static Analyzer**: Pattern detection
- **Coverity**: Commercial static analysis
- **Infer (Facebook)**: Separation logic

## Contribuir

Para mejorar UB_Detector:
1. Identificar patrón de UB no detectado
2. Crear test case que lo reproduzca
3. Implementar detector en módulo apropiado
4. Añadir tests y documentación
5. Verificar con casos reales de C99/C++98

---

**ADead-BIB v3.0** — Compilador con detección de UB integrada
