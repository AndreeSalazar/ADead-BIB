# TESTEO v2 - Tests de ADead-BIB v2.0.0+

> **Binary Is Binary** - Tests para la arquitectura HEX-First

## Estructura

```
v2/
├── hex/                    # Literales HEX y binarios
│   ├── test_hex_literal.adB      ✅
│   ├── test_binary_literal.adB   ✅
│   └── test_emit_macro.adB       ✅
│
├── raw/                    # Modo raw binary
│   └── test_raw_mode.adB         ✅
│
├── cpu/                    # Instrucciones CPU directas
│   └── test_cpu_opcodes.adB      ✅
│
├── gpu/                    # GPU HEX
│   └── test_gpu_opcodes.adB      ✅
│
├── clean/                  # Post-procesamiento
│   └── test_size_comparison.adB  ✅
│
└── integrados/             # Tests completos
    └── test_v2_0_0_hex_first.adB ✅
```

## Ejecutar Tests

```bash
# Test de literales HEX (0x...)
cargo run --bin adeadc -- run TESTEO/v2/hex/test_hex_literal.adB

# Test de literales binarios (0b...)
cargo run --bin adeadc -- run TESTEO/v2/hex/test_binary_literal.adB

# Test de emit![] macro (concepto)
cargo run --bin adeadc -- run TESTEO/v2/hex/test_emit_macro.adB

# Test de modo raw (concepto)
cargo run --bin adeadc -- run TESTEO/v2/raw/test_raw_mode.adB

# Test de opcodes CPU x86-64
cargo run --bin adeadc -- run TESTEO/v2/cpu/test_cpu_opcodes.adB

# Test de opcodes GPU
cargo run --bin adeadc -- run TESTEO/v2/gpu/test_gpu_opcodes.adB

# Test de post-procesamiento
cargo run --bin adeadc -- run TESTEO/v2/clean/test_size_comparison.adB

# Test completo v2.0.0
cargo run --bin adeadc -- run TESTEO/v2/integrados/test_v2_0_0_hex_first.adB
```

## Estado de Tests v2.0.0

| Feature | Test | Estado |
|---------|------|--------|
| **Literales HEX (0x...)** | test_hex_literal.adB | ✅ **PASA** |
| **Literales Binarios (0b...)** | test_binary_literal.adB | ✅ **PASA** |
| **emit![] macro** | test_emit_macro.adB | ✅ **PASA** (concepto) |
| **Modo raw** | test_raw_mode.adB | ✅ **PASA** (concepto) |
| **CPU Opcodes** | test_cpu_opcodes.adB | ✅ **PASA** |
| **GPU Opcodes** | test_gpu_opcodes.adB | ✅ **PASA** |
| **Post-procesamiento** | test_size_comparison.adB | ✅ **PASA** |
| **Test Integrado** | test_v2_0_0_hex_first.adB | ✅ **PASA** |

### ✅ Implementado en Lexer v2.0.0
- **Literales HEX**: `0xFF`, `0x1234`, `0xFF_FF` (con separadores)
- **Literales Binarios**: `0b11110000`, `0b1111_0000` (con separadores)
- **Literales Octales**: `0o777`, `0o755` (bonus)

## Filosofía de Tests

1. **Sintaxis Humana** - Tests usan sintaxis simple y legible
2. **Verificación de Bytes** - Cada test verifica que los valores son correctos
3. **Progresivo** - Tests van de simple a complejo
4. **Documentado** - Cada test explica qué está probando

## Características Futuras (v2.1.0+)
- `emit![]` macro para insertar bytes directamente
- `#![mode(raw)]` para compilar a bytes puros
- `cpu::*` funciones para instrucciones directas
- `gpu::*` funciones para opcodes GPU

---

*ADead-BIB: Código → Bytes → Binario*
*Todos los tests v2.0.0 pasando ✅*
