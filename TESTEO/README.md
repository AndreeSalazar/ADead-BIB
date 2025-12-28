# TESTEO - Tests de ADead-BIB v1.3.0

Esta carpeta contiene tests para las nuevas funcionalidades de ADead-BIB.

## Estructura

```
TESTEO/
├── arrays/
│   ├── test_array_basico.adB    - Declaración de arrays
│   └── test_foreach.adB         - for x in arr ✅
├── strings/                      - (pendiente)
├── conversiones/
│   ├── test_int.adB
│   ├── test_float.adB
│   └── test_bool.adB            - ✅ Funciona
├── len/
│   ├── test_len.adB
│   └── test_len_array.adB       - ✅ Funciona
└── integrados/
    ├── test_completo.adB
    └── test_v1_3_0_completo.adB - ✅ Test completo
```

## Cómo ejecutar tests

```bash
# Test de for x in arr
cargo run --bin adeadc -- run TESTEO/arrays/test_foreach.adB

# Test de len()
cargo run --bin adeadc -- run TESTEO/len/test_len_array.adB

# Test completo v1.3.0
cargo run --bin adeadc -- run TESTEO/integrados/test_v1_3_0_completo.adB
```

## Estado de Tests

| Feature | Test | Estado |
|---------|------|--------|
| Arrays declaración | test_array_basico.adB | ✅ Funciona |
| **for x in arr** | test_foreach.adB | ✅ **FUNCIONA** |
| **len(arr)** | test_len_array.adB | ✅ **FUNCIONA** |
| int() | test_int.adB | ✅ Funciona |
| float() | test_float.adB | ✅ Funciona |
| **bool()** | test_bool.adB | ✅ **FUNCIONA** |
| Test v1.3.0 | test_v1_3_0_completo.adB | ✅ **FUNCIONA** |

## Resultados de Tests (Diciembre 2024)

### test_foreach.adB ✅
```
=== Test for x in arr ===
Iterando sobre [10, 20, 30]:
x = 10
x = 20
x = 30
Test foreach completado!
```

### test_len_array.adB ✅
```
=== Test len() con Arrays ===
len([1,2,3,4,5]) = 5
len([10,20]) = 2
Test len() completado!
```

### test_v1_3_0_completo.adB ✅
```
╔══════════════════════════════════════╗
║   ADead-BIB v1.3.0 - Test Completo   ║
╚══════════════════════════════════════╝

▶ 1. Arrays
  Array creado: [10, 20, 30, 40, 50]
▶ 2. len()
  len(numeros) = 5
▶ 3. for x in arr
  Iterando sobre numeros:
    x = 10, 20, 30, 40, 50
▶ 4. Conversiones
  bool(42) = 1
  bool(0) = 0
▶ 5. Funciones
  sumar_array(100, 200, 300) = 600
▶ 6. Control de Flujo
  valor > 5: true
▶ 7. for i in 0..3
  i = 0, 1, 2

╔══════════════════════════════════════╗
║      ✓ Todos los tests pasaron!      ║
╚══════════════════════════════════════╝
```
