# TESTEO - Tests de ADead-BIB v1.3.0

Esta carpeta contiene tests para las nuevas funcionalidades de ADead-BIB.

## Estructura

```
TESTEO/
├── arrays/           - Tests de arrays
│   └── test_array_basico.adB
├── strings/          - Tests de strings (pendiente)
├── conversiones/     - Tests de conversión de tipos
│   ├── test_int.adB
│   ├── test_float.adB
│   └── test_bool.adB
├── len/              - Tests de función len()
│   └── test_len.adB
└── integrados/       - Tests que combinan todo
    └── test_completo.adB
```

## Cómo ejecutar tests

```bash
# Ejecutar un test específico
cargo run --bin adeadc -- run TESTEO/arrays/test_array_basico.adB

# Verificar sintaxis
cargo run --bin adeadc -- check TESTEO/arrays/test_array_basico.adB

# Ejecutar test integrado completo
cargo run --bin adeadc -- run TESTEO/integrados/test_completo.adB
```

## Estado de Tests

| Feature | Test | Estado |
|---------|------|--------|
| Arrays declaración | test_array_basico.adB | ✅ Funciona |
| Arrays indexación | test_array_basico.adB | ⚠️ Sintaxis OK, codegen pendiente |
| len() | test_len.adB | ✅ Sintaxis OK |
| int() | test_int.adB | ✅ Funciona |
| float() | test_float.adB | ✅ Funciona |
| bool() | test_bool.adB | ✅ Funciona |
| Test integrado | test_completo.adB | ✅ Funciona |

## Resultados de Tests (Diciembre 2024)

### test_completo.adB ✅
```
=== Test Integrado v1.3.0 ===
--- Arrays ---
Array creado: [1, 2, 3]
--- Conversiones ---
bool(100) = 1
bool(0) = 0
--- Funciones ---
suma_array(10, 20, 30) = 60
--- Control de Flujo ---
valor es positivo
--- Loops ---
i = 0
i = 1
i = 2
=== Test Integrado Completado! ===
```

### test_bool.adB ✅
```
=== Test bool() ===
bool(42) = 1
bool(0) = 0
bool(1) = 1
Test bool() completado!
```
