# TESTEO - Tests de ADead-BIB v1.4.0

Esta carpeta contiene tests para las funcionalidades de ADead-BIB.

## Estructura

```
TESTEO/
├── arrays/
│   ├── test_array_basico.adB    - Declaracion de arrays
│   └── test_foreach.adB         - for x in arr
├── input/
│   └── test_input.adB           - input() real
├── conversiones/
│   ├── test_int.adB
│   ├── test_float.adB
│   └── test_bool.adB
├── len/
│   ├── test_len.adB
│   └── test_len_array.adB
└── integrados/
    ├── test_v1_3_0_completo.adB - Test v1.3.0
    └── test_v1_4_0_input.adB    - Test v1.4.0 input()
```

## Como ejecutar tests

```bash
# Test de for x in arr
cargo run --bin adeadc -- run TESTEO/arrays/test_foreach.adB

# Test de len()
cargo run --bin adeadc -- run TESTEO/len/test_len_array.adB

# Test completo v1.3.0
cargo run --bin adeadc -- run TESTEO/integrados/test_v1_3_0_completo.adB

# Test input() v1.4.0 (requiere entrada de usuario)
cargo run --bin adeadc -- build TESTEO/integrados/test_v1_4_0_input.adB -o test.exe
echo 5 10 | .\test.exe
```

## Estado de Tests

| Feature | Test | Estado |
|---------|------|--------|
| Arrays | test_array_basico.adB | OK |
| for x in arr | test_foreach.adB | OK |
| len(arr) | test_len_array.adB | OK |
| int() | test_int.adB | OK |
| float() | test_float.adB | OK |
| bool() | test_bool.adB | OK |
| Test v1.3.0 | test_v1_3_0_completo.adB | OK |
| **input()** | test_v1_4_0_input.adB | **OK** |

## Resultados de Tests (Diciembre 2024)

### test_v1_3_0_completo.adB
```
+========================================+
|   ADead-BIB v1.3.0 - Test Completo    |
+========================================+

[1] Arrays - Array creado: [10, 20, 30, 40, 50]
[2] len() - len(numeros) = 5
[3] for x in arr - x = 10, 20, 30, 40, 50
[4] Conversiones - bool(42) = 1, bool(0) = 0
[5] Funciones - sumar_array(100, 200, 300) = 600
[6] Control de Flujo - valor > 5: true
[7] for i in 0..3 - i = 0, 1, 2

+========================================+
|     OK - Todos los tests pasaron!     |
+========================================+
```

### test_v1_4_0_input.adB (con echo 5 10)
```
+========================================+
|   ADead-BIB v1.4.0 - Test input()     |
+========================================+

Ingresa un numero: Leiste: 5
El doble es: 10
El cuadrado es: 25
Ingresa otro numero: La suma es: 15

+========================================+
|     OK - input() funciona!            |
+========================================+
```
