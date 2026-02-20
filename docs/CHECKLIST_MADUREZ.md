# ADead-BIB — Checklist de Madurez del Compilador

**Fecha:** 2026-02-20  
**Versión:** v3.2

---

## Estado Actual

| # | Criterio | Estado | Notas |
|---|----------|--------|-------|
| 1 | ¿Compila programas grandes sin fallos? | ⚠️ Parcial | Compila ejemplos pequeños/medianos. Falta probar con programas >1000 líneas |
| 2 | ¿Puede compilarse a sí mismo (self-hosting)? | ❌ No | El compilador está en Rust, no en ADead-BIB |
| 3 | ¿Tiene suite de tests automatizada? | ✅ Sí | 145 tests unitarios pasando |
| 4 | ¿Tiene tests de regresión? | ⚠️ Parcial | Tests unitarios, pero no tests de regresión de compilación |
| 5 | ¿Has hecho fuzz testing? | ❌ No | No implementado |
| 6 | ¿Has documentado la ISA layer? | ⚠️ Parcial | Código documentado, falta doc externa |
| 7 | ¿Puedes portar el backend a otra ISA fácilmente? | ✅ Sí | Diseño modular con ADeadOp abstracción |

---

## 1. Compilación de Programas Grandes

### Estado Actual
- ✅ Compila `MODE2_app_typed.adB` (118 líneas)
- ✅ Compila `MODE2_oop_classes.adB` (134 líneas)
- ✅ Compila `MODE1_boot_minimal.adB` (boot sector)
- ✅ Compila `MODE3_gpu_compute.adB` (SPIR-V)

### Pendiente
- [ ] Crear programa de prueba >500 líneas
- [ ] Crear programa de prueba >1000 líneas
- [ ] Medir tiempo de compilación
- [ ] Medir tamaño de binario generado

---

## 2. Self-Hosting

### Estado Actual
El compilador ADead-BIB está escrito en **Rust**, no en ADead-BIB.

### Camino hacia Self-Hosting
Para lograr self-hosting, necesitaríamos:

1. **Fase 1:** Implementar subset de Rust en ADead-BIB
   - Structs, enums, impl blocks
   - Pattern matching
   - Generics básicos
   - Traits

2. **Fase 2:** Reescribir componentes críticos
   - Lexer → ADead-BIB
   - Parser → ADead-BIB
   - AST → ADead-BIB

3. **Fase 3:** Bootstrap
   - Compilar el compilador con Rust
   - Compilar el compilador con ADead-BIB
   - Verificar que ambos producen el mismo output

### Estimación
- **Complejidad:** Alta
- **Tiempo estimado:** 6-12 meses
- **Prioridad:** Baja (no crítico para funcionalidad)

---

## 3. Suite de Tests Automatizada

### Estado Actual
```
cargo test --lib → 145 tests pasando
```

### Tests por Módulo
- `backend::gpu::compute` — Tests de compute GPU
- `backend::gpu::unified_pipeline` — Tests de pipeline
- `backend::gpu::vulkan_runtime` — Tests de Vulkan
- `runtime::gpu_misuse_detector` — Tests de detección
- `isa::reg_alloc` — Tests de register allocator

### Pendiente
- [ ] Tests de integración (compilar → ejecutar → verificar output)
- [ ] Tests de parser con casos edge
- [ ] Tests de codegen con verificación de bytes
- [ ] Coverage report

---

## 4. Tests de Regresión

### Estado Actual
No hay tests de regresión formales.

### Plan de Implementación
1. Crear directorio `tests/regression/`
2. Guardar outputs esperados de compilación
3. Script que compila y compara con expected
4. Integrar en CI/CD

### Estructura Propuesta
```
tests/
├── regression/
│   ├── compile_hello.rs      # Test: compilar hello.adB
│   ├── compile_functions.rs  # Test: compilar functions.adB
│   ├── compile_loops.rs      # Test: compilar loops.adB
│   └── expected/
│       ├── hello.bin         # Output esperado
│       ├── functions.bin
│       └── loops.bin
└── integration/
    ├── run_hello.rs          # Ejecutar y verificar output
    └── run_math.rs
```

---

## 5. Fuzz Testing

### Estado Actual
No implementado.

### Plan de Implementación
1. Usar `cargo-fuzz` o `afl`
2. Targets:
   - Lexer: input aleatorio → no debe crashear
   - Parser: tokens aleatorios → no debe crashear
   - Codegen: AST aleatorio → no debe crashear

### Ejemplo de Fuzz Target
```rust
// fuzz/fuzz_targets/fuzz_lexer.rs
#![no_main]
use libfuzzer_sys::fuzz_target;
use adead_bib::frontend::lexer::Lexer;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let mut lexer = Lexer::new(s);
        let _ = lexer.tokenize(); // No debe crashear
    }
});
```

---

## 6. Documentación ISA Layer

### Estado Actual
- Código tiene comentarios
- No hay documentación externa

### Documentación Necesaria

#### 6.1 Registros (Reg enum)
| Registro | Tamaño | Uso |
|----------|--------|-----|
| RAX | 64-bit | Acumulador, retorno |
| RBX | 64-bit | Callee-saved |
| RCX | 64-bit | Arg 1 (Windows) |
| RDX | 64-bit | Arg 2 (Windows) |
| RSI | 64-bit | Arg 2 (Linux) |
| RDI | 64-bit | Arg 1 (Linux) |
| RSP | 64-bit | Stack pointer |
| RBP | 64-bit | Frame pointer |
| R8-R15 | 64-bit | General purpose |

#### 6.2 Operandos (Operand enum)
| Variante | Descripción | Ejemplo |
|----------|-------------|---------|
| `Reg(Reg)` | Registro | `RAX` |
| `Imm8(i8)` | Inmediato 8-bit | `42` |
| `Imm16(i16)` | Inmediato 16-bit | `1000` |
| `Imm32(i32)` | Inmediato 32-bit | `100000` |
| `Imm64(u64)` | Inmediato 64-bit | `0x1234567890` |
| `Mem { base, disp }` | Memoria | `[RBP-8]` |
| `MemSIB { base, index, scale, disp }` | SIB | `[RAX+RBX*4+8]` |
| `RipRel(i32)` | RIP-relative | `[RIP+0x100]` |

#### 6.3 Instrucciones (ADeadOp enum)
| Instrucción | Descripción | Encoding |
|-------------|-------------|----------|
| `Push { src }` | Push a stack | `50+r` |
| `Pop { dst }` | Pop de stack | `58+r` |
| `Mov { dst, src }` | Mover datos | `89/8B` |
| `Add { dst, src }` | Suma | `01/03` |
| `Sub { dst, src }` | Resta | `29/2B` |
| `Mul { src }` | Multiplicación | `F7 /4` |
| `Div { src }` | División | `F7 /6` |
| `Cmp { left, right }` | Comparar | `39/3B` |
| `Jmp { target }` | Salto incondicional | `EB/E9` |
| `Je/Jne/Jl/Jg/...` | Saltos condicionales | `74/75/7C/7F/...` |
| `Call { target }` | Llamar función | `E8` |
| `Ret` | Retornar | `C3` |

---

## 7. Portabilidad del Backend

### Estado Actual
El diseño es modular:

```
AST → IsaCompiler → Vec<ADeadOp> → Encoder → Vec<u8>
```

### Portar a ARM64
Para portar a ARM64, solo necesitamos:

1. **Nuevo encoder:** `encoder_arm64.rs`
2. **Mapeo de registros:** `Reg::RAX → Reg::X0`
3. **Encoding de instrucciones:** ARM64 encoding

### Ejemplo de Abstracción
```rust
// Actual (x86-64)
ADeadOp::Mov { dst: Reg(RAX), src: Imm64(42) }
// → 48 B8 2A 00 00 00 00 00 00 00

// ARM64 (futuro)
ADeadOp::Mov { dst: Reg(X0), src: Imm64(42) }
// → D2 80 05 40 (MOVZ X0, #42)
```

### Estimación para ARM64
- **Complejidad:** Media
- **Tiempo estimado:** 2-4 semanas
- **Archivos a crear:**
  - `src/rust/isa/encoder_arm64.rs`
  - `src/rust/isa/reg_arm64.rs`

---

## Resumen

| Criterio | Estado | Acción Requerida |
|----------|--------|------------------|
| Programas grandes | ⚠️ | Crear tests con >500 líneas |
| Self-hosting | ❌ | Roadmap a largo plazo |
| Tests automatizados | ✅ | Mantener y expandir |
| Tests de regresión | ⚠️ | Crear `tests/regression/` |
| Fuzz testing | ❌ | Implementar con cargo-fuzz |
| Documentación ISA | ⚠️ | Crear `docs/ISA.md` |
| Portabilidad | ✅ | Diseño ya es modular |

### Conclusión

**ADead-BIB está FUNCIONAL, no "lista".**

Para considerarla "lista" necesita:
1. ✅ Tests de regresión automatizados
2. ✅ Fuzz testing básico
3. ✅ Documentación ISA completa
4. ⚠️ Pruebas con programas grandes
5. ❌ Self-hosting (opcional, largo plazo)

---

**Próximos Pasos Inmediatos:**
1. Crear `tests/regression/` con tests de compilación
2. Crear `docs/ISA.md` con documentación completa
3. Implementar fuzz testing básico para lexer/parser
