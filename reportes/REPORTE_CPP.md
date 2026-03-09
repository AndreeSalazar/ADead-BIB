# Reporte C++ stdlib — ADead-BIB v7.0

## Estado: ❌ INCOMPLETO (2/12 tests pasan, 10 fallan)

---

## Problema Raíz #1: TIPOS STL NO REGISTRADOS EN EL PARSER

El parser C++ tiene un set de "tipos conocidos" que reconoce como tipos template.
Actualmente reconoce: `vector`, `string`, `map`, `unordered_map`, `unique_ptr`,
`shared_ptr`, `weak_ptr`, `optional`, `variant`, `pair`, `tuple`, `cout`, `cin`.

**Pero NO reconoce estos como tipos**:
- `set`, `multiset`, `unordered_set`, `unordered_multiset`
- `list`, `forward_list`
- `deque`
- `stack`, `queue`, `priority_queue`
- `array` (C++ `std::array`, no C array)
- `regex`, `smatch`, `cmatch`
- `mt19937`, `uniform_int_distribution`, `normal_distribution`
- `thread`, `mutex`, `lock_guard`, `unique_lock`
- `atomic`
- `future`, `promise`
- `condition_variable`
- `any`
- `string_view`
- `span`
- `initializer_list`
- `filesystem::path`, `directory_iterator`
- `chrono::duration`, `chrono::time_point`, etc.

## Problema Raíz #2: FEATURES DEL PARSER NO SOPORTADAS

| Feature | Ejemplo | Estado |
|---------|---------|--------|
| Template fields en struct/class | `std::shared_ptr<Node> next;` | ❌ |
| `decltype()` en template args | `tuple_size<decltype(t)>::value` | ❌ |
| `::type` / `::value` acceso | `remove_const<int>::type` | ❌ |
| `_v` suffix variable templates | `is_integral_v<int>` | ❌ |
| Non-type template args | `get<0>(t)`, `array<int, 5>` | ❌ |
| Namespace alias en función | `namespace fs = std::filesystem;` | ❌ |
| Structured bindings | `auto [a, b] = pair;` | ❌ |
| Template function calls | `any_cast<int>(a)` | ❌ |
| Multi-level scope | `std::chrono::milliseconds` | ⚠️ Parcial |

---

## Módulos stdlib que EXISTEN y FUNCIONAN

| Módulo | Header C++ | Estado |
|--------|------------|--------|
| `fastos_iostream.rs` | `<iostream>` | ✅ cout, cin, cerr, endl |
| `fastos_vector.rs` | `<vector>` | ✅ push_back, size, begin/end... |
| `fastos_string_cpp.rs` | `<string>` | ✅ find, substr, c_str... |
| `fastos_map.rs` | `<map>` `<unordered_map>` | ✅ insert, find, operator[]... |
| `fastos_memory.rs` | `<memory>` | ⚠️ Declarado pero no como struct fields |
| `fastos_algorithm.rs` | `<algorithm>` | ✅ sort, find, copy, transform... |
| `fastos_functional.rs` | `<functional>` | ✅ function, bind, hash... |
| `fastos_utility.rs` | `<utility>` | ✅ pair, move, forward... |
| `fastos_exceptions.rs` | `<exception>` | ✅ exception, runtime_error... |

## Módulos stdlib que NO EXISTEN

| Módulo necesario | Header C++ | Prioridad |
|------------------|------------|-----------|
| `fastos_set.rs` | `<set>` `<unordered_set>` | 🔴 ALTA |
| `fastos_list.rs` | `<list>` `<forward_list>` | 🔴 ALTA |
| `fastos_deque.rs` | `<deque>` | 🔴 ALTA |
| `fastos_stack_queue.rs` | `<stack>` `<queue>` | 🔴 ALTA |
| `fastos_array_cpp.rs` | `<array>` | 🔴 ALTA |
| `fastos_chrono.rs` | `<chrono>` | 🔴 ALTA |
| `fastos_thread.rs` | `<thread>` | 🔴 ALTA |
| `fastos_mutex.rs` | `<mutex>` | 🔴 ALTA |
| `fastos_atomic.rs` | `<atomic>` | 🔴 ALTA |
| `fastos_optional.rs` | `<optional>` | 🟡 MEDIA |
| `fastos_variant.rs` | `<variant>` | 🟡 MEDIA |
| `fastos_any.rs` | `<any>` | 🟡 MEDIA |
| `fastos_tuple.rs` | `<tuple>` | 🟡 MEDIA |
| `fastos_future.rs` | `<future>` | 🟡 MEDIA |
| `fastos_condition_variable.rs` | `<condition_variable>` | 🟡 MEDIA |
| `fastos_filesystem.rs` | `<filesystem>` | 🟡 MEDIA |
| `fastos_regex.rs` | `<regex>` | 🟢 BAJA |
| `fastos_random.rs` | `<random>` | 🟢 BAJA |
| `fastos_string_view.rs` | `<string_view>` | 🟢 BAJA |
| `fastos_span.rs` | `<span>` | 🟢 BAJA |
| `fastos_initializer_list.rs` | `<initializer_list>` | 🟢 BAJA |
| `fastos_iterator.rs` | `<iterator>` | 🟢 BAJA |
| `fastos_numeric.rs` | `<numeric>` | 🟢 BAJA (funciona vía algorithm) |

---

## Acción Requerida (2 frentes)

### Frente 1: Registrar tipos en el parser (`cpp_parser.rs`)
Agregar al set de tipos conocidos: `set`, `list`, `deque`, `stack`, `queue`,
`priority_queue`, `array`, `thread`, `mutex`, `atomic`, `future`, `promise`,
`regex`, `smatch`, `any`, `string_view`, `span`, `initializer_list`,
`lock_guard`, `unique_lock`, `condition_variable`

### Frente 2: Crear módulos fastos_*.rs faltantes
22 módulos nuevos con registros de métodos/funciones/tipos, similar a
los existentes `fastos_vector.rs`, `fastos_map.rs`, etc.

*Reporte generado el 2026-03-09 usando `adb step` en cada test*
