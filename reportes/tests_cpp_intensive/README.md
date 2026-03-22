# ADead-BIB C++ Intensive Tests

## "Respetar Bits" — Type Strictness ULTRA

Tests intensivos de C++ para validar el compilador ADead-BIB desde C++98 hasta C++17.

---

## Archivos de Test

### C++98 Features

| Archivo | Descripción | Tests |
|---------|-------------|-------|
| `test_cpp98_classes.cpp` | Clases, herencia, polimorfismo, operadores | 6 tests |
| `test_cpp98_templates.cpp` | Templates de función y clase, especialización | 6 tests |

### C++11 Features

| Archivo | Descripción | Tests |
|---------|-------------|-------|
| `test_cpp11_features.cpp` | auto, nullptr, lambdas, constexpr, enum class, move | 10 tests |

### C++14 Features

| Archivo | Descripción | Tests |
|---------|-------------|-------|
| `test_cpp14_features.cpp` | Generic lambdas, return deduction, variable templates | 10 tests |

### C++17 Features

| Archivo | Descripción | Tests |
|---------|-------------|-------|
| `test_cpp17_features.cpp` | Structured bindings, if constexpr, fold expressions | 10 tests |

### STL & Algorithms

| Archivo | Descripción | Tests |
|---------|-------------|-------|
| `test_cpp_stl_containers.cpp` | Vector, Stack, Queue, Map, Set, List | 6 tests |
| `test_cpp_smart_pointers.cpp` | unique_ptr, shared_ptr, weak_ptr, RAII | 6 tests |
| `test_cpp_algorithms.cpp` | Sorting, searching, math, strings, graphs, DP | 6 tests |

### Type Strictness

| Archivo | Descripción | Tests |
|---------|-------------|-------|
| `test_cpp_type_strictness.cpp` | Type mismatch, signed/unsigned, narrowing, overflow | 10 tests |

---

## Cómo Ejecutar

```bash
# Compilar y ejecutar un test específico
adb cpp reportes/tests_cpp_intensive/test_cpp98_classes.cpp -o test.exe
./test.exe

# Ver paso a paso
adb step reportes/tests_cpp_intensive/test_cpp11_features.cpp

# Ejecutar todos los tests
for file in reportes/tests_cpp_intensive/*.cpp; do
    echo "=== Testing: $file ==="
    adb cpp "$file" -o test.exe && ./test.exe
done
```

---

## Características Testeadas

### C++98

- [x] Classes y constructores/destructores
- [x] Herencia y polimorfismo
- [x] Encapsulación (private/public/protected)
- [x] Miembros estáticos
- [x] Sobrecarga de operadores
- [x] Copy constructor
- [x] Templates de función
- [x] Templates de clase
- [x] Especialización de templates
- [x] Non-type template parameters

### C++11

- [x] `auto` keyword
- [x] `nullptr`
- [x] Range-based for
- [x] Lambda expressions
- [x] `constexpr`
- [x] `enum class`
- [x] Initializer lists
- [x] `static_assert`
- [x] `decltype`
- [x] Move semantics

### C++14

- [x] Generic lambdas
- [x] Return type deduction
- [x] Variable templates
- [x] Relaxed constexpr
- [x] Binary literals
- [x] Digit separators
- [x] `[[deprecated]]` attribute
- [x] Lambda capture expressions
- [x] `decltype(auto)`
- [x] Aggregate member initialization

### C++17

- [x] Structured bindings
- [x] `if constexpr`
- [x] Fold expressions
- [x] Inline variables
- [x] `[[nodiscard]]` attribute
- [x] `[[maybe_unused]]` attribute
- [x] Nested namespaces
- [x] Init statements in if/switch
- [x] Class template argument deduction (CTAD)

---

## Type Strictness Rules ("Respetar Bits")

1. **NO** implicit int + float mixing
2. **NO** implicit signed + unsigned mixing
3. **NO** implicit narrowing conversions
4. **NO** implicit void* to T* casts
5. **NO** silent integer overflow
6. **NO** implicit constructor calls
7. **ALWAYS** use explicit casts
8. **ALWAYS** use explicit constructors

---

## Filosofía

> "Los bits merecen respeto"
> "FORTRAN lo supo en 1957"
> "Ada lo reforzó en 1983"
> "ADead-BIB lo aplica en 2025"
> "Binary Is Binary" 💀🦈

---

## Versión

ADead-BIB V9.0 — Marzo 2026
