# TESTEO v2 - Tests de ADead-BIB v2.0.0+

> **Binary Is Binary** - Tests para la arquitectura HEX-First

## Estructura

```
v2/
├── hex/                    # Literales HEX y binarios
│   ├── test_hex_literal.adB
│   └── test_binary_literal.adB
│
├── raw/                    # Modo raw binary (futuro)
│   └── (pendiente)
│
├── cpu/                    # Instrucciones CPU directas (futuro)
│   └── (pendiente)
│
├── gpu/                    # GPU HEX (futuro)
│   └── (pendiente)
│
├── clean/                  # Post-procesamiento
│   └── test_size_comparison.adB
│
└── integrados/             # Tests completos
    └── test_v2_0_0_hex_first.adB
```

## Ejecutar Tests

```bash
# Test de literales HEX
cargo run --bin adeadc -- run TESTEO/v2/hex/test_hex_literal.adB

# Test de literales binarios
cargo run --bin adeadc -- run TESTEO/v2/hex/test_binary_literal.adB

# Test completo v2.0.0
cargo run --bin adeadc -- run TESTEO/v2/integrados/test_v2_0_0_hex_first.adB

# Test de tamaño (post-procesamiento)
cargo run --bin adeadc -- run TESTEO/v2/clean/test_size_comparison.adB
```

## Estado de Tests v2.0.0

| Feature | Test | Estado |
|---------|------|--------|
| Valores HEX | test_hex_literal.adB | ✅ **PASA** |
| Valores Binarios | test_binary_literal.adB | ✅ **PASA** |
| Test Integrado | test_v2_0_0_hex_first.adB | ✅ **PASA** |
| Post-procesamiento | test_size_comparison.adB | ✅ **PASA** |
| Literales 0x/0b | (en lexer) | ⏳ Pendiente implementar |
| emit![] macro | test_emit_macro.adB | ⏳ Pendiente |
| Modo raw | test_raw_mode.adB | ⏳ Pendiente |
| CPU directo | test_cpu_*.adB | ⏳ Pendiente |
| GPU HEX | test_gpu_*.adB | ⏳ Pendiente |

### Nota sobre Literales HEX/Binarios
Los tests actuales usan **valores decimales equivalentes** porque los literales `0x...` y `0b...` aún no están implementados en el lexer. Esta es una característica pendiente de v2.0.0.

## Filosofía de Tests

1. **Sintaxis Humana** - Tests usan sintaxis simple y legible
2. **Verificación de Bytes** - Cada test verifica que los valores son correctos
3. **Progresivo** - Tests van de simple a complejo
4. **Documentado** - Cada test explica qué está probando

---

*ADead-BIB: Código → Bytes → Binario*
