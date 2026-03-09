# Reporte de Problemas del Parser — ADead-BIB v7.0

## Problemas que NO son de stdlib sino del Parser C/C++

Estos son bugs/limitaciones del parser que causan fallos independientemente
de si las headers están bien declaradas.

---

### 1. ❌ `_Complex` no es tipo reconocido (C Parser)
**Archivo**: `c_parser.rs`
**Ejemplo**: `double _Complex z = 1.0 + 2.0 * I;`
**Impacto**: `<complex.h>` parsea pero main() no genera código
**Fix**: Agregar `_Complex` como type qualifier (como `const`, `volatile`)

### 2. ❌ Template types como campos de struct/class (C++ Parser)
**Archivo**: `cpp_parser.rs`
**Ejemplo**:
```cpp
struct Node {
    std::shared_ptr<Node> next;  // ← FALLA
    std::weak_ptr<Node> parent;  // ← FALLA
};
```
**Impacto**: No se pueden declarar structs/classes con smart pointers
**Fix**: Al parsear campos de struct, reconocer `std::template<T>` como tipo

### 3. ❌ `decltype()` en posición de tipo (C++ Parser)
**Archivo**: `cpp_parser.rs`
**Ejemplo**: `std::tuple_size<decltype(t)>::value`
**Fix**: Parsear `decltype(expr)` como expression-type

### 4. ❌ `::type` / `::value` access después de template (C++ Parser)
**Ejemplo**: `std::remove_const<int>::type x;`
**Impacto**: Type traits completamente inutilizables
**Fix**: Después de parsear `Template<Args>`, check for `::identifier`

### 5. ❌ `_v` variable templates (C++ Parser)
**Ejemplo**: `bool b = std::is_integral_v<int>;`
**Fix**: Reconocer como `std::is_integral_v < int >` = template variable access

### 6. ❌ Non-type template arguments (C++ Parser)
**Ejemplo**: `std::get<0>(tuple)`, `std::array<int, 5>`
**Fix**: Permitir int literals como template args

### 7. ❌ Namespace alias dentro de funciones (C++ Parser)
**Ejemplo**: `namespace fs = std::filesystem;` dentro de `main()`
**Fix**: Permitir `namespace X = Y;` como statement dentro de función body

### 8. ❌ Structured bindings no funciona (C++ Expander)
**Ejemplo**: `auto [a, b, c] = tuple;`
**Nota**: Listado en `expander.rs` como feature C++17 pero no implementado en parser
**Fix**: Implementar en parser o en expander realmente

### 9. ❌ Template function calls como expresiones (C++ Parser)
**Ejemplo**: `std::any_cast<int>(value)`, `std::get<int>(variant)`
**Fix**: Cuando se ve `identifier < type > ( args )`, tratar como template function call

### 10. ⚠️ Multi-level scope access parcial (C++ Parser)
**Ejemplo**: `std::chrono::high_resolution_clock::now()`
**Estado**: Funciona para 2 niveles (`std::cout`), falla para 3+ niveles
**Fix**: Loop en scope resolution: while `::` consume next identifier

---

## Resumen de Impacto

| Prioridad | Fix | Tests que desbloquea |
|-----------|-----|---------------------|
| 🔴 #2 | Template fields en struct | `test_smart_ptr_adv.cpp` |
| 🔴 #9 | Template function calls | `test_optional_variant_any.cpp`, `test_tuple.cpp` |
| 🔴 #6 | Non-type template args | `test_tuple.cpp`, `test_containers.cpp` |
| 🟡 #4 | `::type` / `::value` | `test_type_traits.cpp` |
| 🟡 #7 | Namespace alias en func | `test_filesystem.cpp` |
| 🟡 #10 | Multi-level scope | `test_chrono_thread.cpp` |
| 🟢 #1 | `_Complex` tipo | `test_complex.c` |
| 🟢 #3 | `decltype()` | `test_tuple.cpp` |
| 🟢 #5 | `_v` traits | `test_type_traits.cpp` |
| 🟢 #8 | Structured bindings | `test_tuple.cpp` |

*Reporte generado el 2026-03-09*
