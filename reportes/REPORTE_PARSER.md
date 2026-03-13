# Reporte de Parser — ADead-BIB v8.0

## Estado: ✅ Todos los problemas críticos resueltos

---

## Fixes Implementados

### 1. ✅ Structured Bindings (C++17)
- `auto [a, b, c] = expr;` detectado ANTES de `parse_type()` 
- Evita conflicto con array syntax `auto[]`
- Lowered a múltiples `CppDeclarator` con `CppType::Auto`

### 2. ✅ Namespace Alias vs Function Call disambiguation
- `is_type_start()` ahora verifica si `ns::name` donde `name` es tipo conocido
- `fs::exists()` → expression, `fs::path` → type declaration
- Soporta `peek_at(3) == Lt` para template types después de scope

### 3. ✅ Type Traits
- `<type_traits>` usa HEADER_EMPTY (tipos via prescan)
- 30+ type traits registrados como tipos conocidos
- `::type` y `::value` member access funcional
- `_v` suffix traits via template expression parsing en `parse_postfix_on`

### 4. ✅ Template Fields en Struct
- `std::shared_ptr<Node> next;` dentro de struct → funcional

### 5. ✅ Non-type Template Args
- `std::get<0>(t)`, `std::array<int, 5>` → int literals como template args

### 6. ✅ Template Function Calls
- `std::any_cast<int>(a)` → mangled identifier + function call

### 7. ✅ Multi-level Scope
- `std::chrono::milliseconds` → 3+ niveles de scope resolution

### 8. ✅ Local Namespace Alias
- `namespace fs = std::filesystem;` dentro de funciones

### 9. ✅ Local Typedef con ::type
- `typedef std::remove_const<const int>::type no_const;` dentro de funciones

### 10. ✅ uintmax_t/intmax_t como tipos

---

## Resumen de Impacto

| Fix | Tests Desbloqueados |
|-----|---------------------|
| Structured bindings | `test_tuple.cpp` |
| Namespace vs function | `test_filesystem.cpp` |
| Type traits + ::value | `test_type_traits.cpp` |
| Template fields | `test_smart_ptr_adv.cpp` |
| Non-type template | `test_containers.cpp`, `test_tuple.cpp` |
| Template calls | `test_optional_variant_any.cpp` |
| Multi-level scope | `test_chrono_thread.cpp` |

**Resultado: 12/12 tests C++ stdlib pasan — 0 fallos**

---

*Reporte actualizado el 2026-03-13*
