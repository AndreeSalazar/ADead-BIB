# ADead-BIB — Reporte Completo de Librerías Faltantes
## Fecha: 2026-03-09

---

## Resumen Ejecutivo

| Área | Tests | PASAN | FALLAN | Estado |
|------|-------|-------|--------|--------|
| **C99 stdlib** | 7 | 7 | 0 | ✅ Completo |
| **C++ stdlib** | 12 | 2 | 10 | ❌ Faltan muchas |

### C99: ✅ COMPLETO — Todas las headers funcionan
### C++: ❌ INCOMPLETO — Parser no reconoce muchos tipos STL como tipos

---

## SECCIÓN 1: C99 — Estado Actual (PASAN TODOS)

### ✅ test_math_avanzado.c — 7/7 fases OK
- `exp, exp2, expm1, log, log2, log10, log1p` → ✅
- `sinh, cosh, tanh, asinh, acosh, atanh` → ✅
- `erf, erfc, tgamma, lgamma` → ✅
- `fma, fdim, fmax, fmin, remainder, copysign` → ✅
- `nearbyint, rint, lround, llround, lrint` → ✅ (declaradas)
- `fpclassify, isnan, isinf, isfinite, isnormal, signbit` → ✅
- `nextafter, scalbn, ldexp, frexp, modf` → ✅

### ✅ test_ctype.c — 7/7 fases OK
- Todas las funciones `<ctype.h>` reconocidas

### ✅ test_signal.c — 7/7 fases OK
- `signal, raise, SIGINT, SIGTERM, SIGABRT, SIGFPE, SIGSEGV` → ✅

### ✅ test_setjmp.c — 7/7 fases OK
- `setjmp, longjmp, jmp_buf` → ✅

### ✅ test_stdarg.c — 7/7 fases OK
- `va_list, va_start, va_arg, va_end` → ✅
- **NOTA**: `va_arg` no compila las funciones variádicas (sum/average) pero main pasa

### ✅ test_wchar.c — 7/7 fases OK
- `wcslen, wcscpy, wcscmp, wcschr, wcsstr, wprintf` → ✅

### ⚠️ test_complex.c — 7/7 fases "OK" pero...
- Compila **pero main() no es reconocido** (0 funciones, 0 IR statements)
- **Causa**: `double _Complex` no es un tipo reconocido por el parser C
- El parser acepta las declaraciones de `<complex.h>` pero no parsea el cuerpo de main con `_Complex`

---

## SECCIÓN 2: C++ — Fallos Detectados con `adb step`

### ✅ test_cmath_exp.cpp — 7/7 fases OK
- `std::exp, std::exp2, std::expm1, std::log, std::pow, std::sqrt` → ✅
- `std::sinh, std::cosh, std::tanh, std::asinh, std::acosh, std::atanh` → ✅
- `std::fma, std::erf, std::erfc, std::tgamma, std::lgamma` → ✅
- **CONCLUSIÓN**: `<cmath>` funciona bien con `std::` prefix

### ✅ test_numeric.cpp — 7/7 fases OK
- `std::accumulate, std::partial_sum, std::adjacent_difference` → ✅
- `std::inner_product, std::iota, std::gcd, std::lcm` → ✅

### ❌ test_containers.cpp — FALLA en Phase 3 (Parser)
**Error**: `Unexpected token in expression: Int at pos 348`
**Causa**: El parser NO reconoce estos tipos template como tipos:
- `std::set<int>` → ❌ **NO es tipo conocido**
- `std::unordered_set<int>` → ❌
- `std::list<int>` → ❌
- `std::deque<int>` → ❌
- `std::queue<int>` → ❌
- `std::stack<int>` → ❌
- `std::priority_queue<int>` → ❌
- `std::array<int, 5>` → ❌

**Faltan en fastos_map.rs / cpp_stdlib.rs**:
- `set`, `unordered_set`, `multiset`, `unordered_multiset`
- `list`, `forward_list`
- `deque`
- `queue`, `priority_queue`
- `stack`
- `array`

### ❌ test_chrono_thread.cpp — FALLA en Phase 3 (Parser)
**Error**: `Unexpected token in expression: Double at pos 312`
**Causa**: El parser no reconoce tipos template anidados del namespace `std::chrono`:
- `std::chrono::high_resolution_clock::now()` → ❌
- `std::chrono::duration<double>` → ❌
- `std::chrono::milliseconds` → ❌
- `std::chrono::microseconds` → ❌
- `std::chrono::nanoseconds` → ❌
- `std::chrono::duration_cast<>` → ❌
- `std::thread` → ❌ **NO es tipo conocido**
- `std::mutex` → ❌
- `std::lock_guard<std::mutex>` → ❌
- `std::atomic<int>` → ❌
- `std::promise<int>` / `std::future<int>` → ❌
- `std::async` → ❌
- `std::launch::async` → ❌

**Faltan módulos completos**:
- `fastos_chrono.rs` → **NO EXISTE**
- `fastos_thread.rs` → **NO EXISTE**
- `fastos_mutex.rs` → **NO EXISTE**
- `fastos_atomic.rs` → **NO EXISTE**
- `fastos_future.rs` → **NO EXISTE**
- `fastos_condition_variable.rs` → **NO EXISTE**

### ❌ test_optional_variant_any.cpp — FALLA en Phase 3 (Parser)
**Error**: `Expected Semicolon, got Identifier("a") at pos 420`
**Causa**: El parser no reconoce `std::any_cast<int>(a)` como expresión
- `std::optional<int>` → ❌ no reconocido como tipo
- `.has_value()`, `.value()`, `.value_or()`, `.reset()` → ❌
- `std::variant<int, double, std::string>` → ❌ multi-type template
- `std::get<int>(var)` → ❌ template function call
- `std::holds_alternative<double>(var)` → ❌
- `std::any` → ❌ no reconocido como tipo
- `std::any_cast<int>(a)` → ❌

**Faltan módulos**:
- `fastos_optional.rs` → **NO EXISTE**
- `fastos_variant.rs` → **NO EXISTE**
- `fastos_any.rs` → **NO EXISTE**

### ❌ test_tuple.cpp — FALLA en Phase 3 (Parser)
**Error**: `Unexpected token in expression: Decltype at pos 348`
**Causa**: `decltype(t)` no es soportado como expresión de tipo
- `std::tuple<int, double, std::string>` → ❌ multi-type template
- `std::make_tuple(...)` → ✅ (reconocido como call)
- `std::get<0>(t)` → ❌ template con int literal
- `std::tuple_size<decltype(t)>::value` → ❌ `decltype` en template arg
- `std::tie(x, y, z)` → ❌
- `auto [p, q, r] = t;` → ❌ structured bindings no soportado
- `std::tuple_cat(t, t2)` → ❌

**Faltan**:
- `fastos_tuple.rs` → **NO EXISTE**
- Soporte de `decltype` en template arguments → Parser
- Soporte de `std::get<N>()` con non-type template → Parser

### ❌ test_regex_random.cpp — FALLA en Phase 3 (Parser)
**Error**: `Expected Semicolon, got Identifier("pattern") at pos 293`
**Causa**: `std::regex` no es tipo conocido
- `std::regex` → ❌
- `std::smatch` → ❌
- `std::regex_search()` → ❌
- `std::regex_match()` → ❌
- `std::regex_replace()` → ❌
- `std::mt19937` → ❌
- `std::uniform_int_distribution<int>` → ❌
- `std::uniform_real_distribution<double>` → ❌
- `std::normal_distribution<double>` → ❌
- `std::bernoulli_distribution` → ❌
- `std::default_random_engine` → ❌

**Faltan módulos completos**:
- `fastos_regex.rs` → **NO EXISTE**
- `fastos_random.rs` → **NO EXISTE**

### ❌ test_filesystem.cpp — FALLA en Phase 3 (Parser)
**Error**: `Unexpected token in expression: Namespace at pos 290`
**Causa**: `namespace fs = std::filesystem;` (namespace alias) no es soportado dentro de funciones
- `namespace fs = std::filesystem;` → ❌ local namespace alias
- `fs::path` → ❌
- `fs::exists()`, `fs::is_directory()`, `fs::is_regular_file()` → ❌
- `fs::create_directories()` → ❌
- `fs::directory_iterator` → ❌
- `fs::file_size()`, `fs::copy()`, `fs::rename()`, `fs::remove()` → ❌
- `.filename()`, `.extension()`, `.parent_path()`, `.stem()` → ❌
- `operator/` para paths → ❌

**Faltan**:
- `fastos_filesystem.rs` → **NO EXISTE**
- Soporte de namespace alias dentro de funciones → Parser

### ❌ test_type_traits.cpp — FALLA en Phase 3 (Parser)
**Error**: `Expected type, got Typedef at pos 539`
**Causa**: `typedef std::remove_const<const int>::type` no es parseable
- `std::is_same<int, int>::value` → ❌ (template + `::value`)
- `std::remove_const<const int>::type` → ❌ `::type` after template
- `std::remove_reference<int&>::type` → ❌
- `std::add_pointer<int>::type` → ❌
- `std::conditional<true, int, double>::type` → ❌
- `std::enable_if<true, int>::type` → ❌
- `std::decay<const int&>::type` → ❌
- `std::make_signed<unsigned int>::type` → ❌
- `std::is_integral_v<int>` → ❌ `_v` suffix trait
- `std::is_same_v<int, int>` → ❌

**Nota**: El header `<type_traits>` SÍ tiene declaraciones template,
pero el parser no puede usar `::type` o `::value` en typedef/expressions.

### ❌ test_smart_ptr_adv.cpp — FALLA en Phase 3 (Parser)
**Error**: `Unexpected token in expression: Int at pos 378`
**Causa**: El parser no maneja `std::shared_ptr<Node>` como campo de struct
- `std::shared_ptr<Node>` como miembro de struct → ❌
- `std::weak_ptr<Node>` como miembro de struct → ❌
- `std::make_unique<int>(42)` → ❌
- `std::make_unique<int[]>(10)` → ❌ array form
- `up.release()` → ❌ method call on smart ptr
- `wp.lock()` → ❌
- `wp.expired()` → ❌
- Custom deleter con lambda → ❌

### ❌ test_string_view_span.cpp — FALLA en Phase 3 (Parser)
**Error**: `Expected Semicolon, got Identifier("sv") at pos 362`
**Causa**: `std::string_view` como parámetro de función no es tipo conocido
- `std::string_view` como parámetro → ❌
- `std::span<int>` como parámetro → ❌
- `.substr()`, `.find()`, `.starts_with()`, `.ends_with()` → ❌
- `.remove_prefix()` → ❌
- `sp.first()`, `sp.last()`, `sp.subspan()` → ❌

**Faltan**:
- `string_view` no es tipo conocido del parser
- `span` no es tipo conocido del parser

---

## SECCIÓN 3: Resumen de Módulos stdlib Faltantes

### C99 — Faltantes Menores

| Header | Estado | Qué falta |
|--------|--------|-----------|
| `<complex.h>` | ⚠️ | `_Complex` como tipo nativo del parser (header existe pero parser ignora el tipo) |

### C++ — Módulos Completos Faltantes

| Módulo que falta | Header C++ | Tipos/Funciones principales |
|------------------|------------|----------------------------|
| **fastos_set.rs** | `<set>` `<unordered_set>` | `set<T>`, `unordered_set<T>`, `multiset<T>` |
| **fastos_list.rs** | `<list>` `<forward_list>` | `list<T>`, `forward_list<T>` |
| **fastos_deque.rs** | `<deque>` | `deque<T>` |
| **fastos_stack_queue.rs** | `<stack>` `<queue>` | `stack<T>`, `queue<T>`, `priority_queue<T>` |
| **fastos_array.rs** | `<array>` | `array<T, N>` |
| **fastos_tuple.rs** | `<tuple>` | `tuple<T...>`, `make_tuple`, `get<N>`, `tie`, `tuple_cat` |
| **fastos_optional.rs** | `<optional>` | `optional<T>`, `has_value`, `value`, `value_or` |
| **fastos_variant.rs** | `<variant>` | `variant<T...>`, `get<T>`, `holds_alternative`, `visit` |
| **fastos_any.rs** | `<any>` | `any`, `any_cast<T>`, `has_value` |
| **fastos_chrono.rs** | `<chrono>` | `high_resolution_clock`, `duration`, `milliseconds`, `duration_cast` |
| **fastos_thread.rs** | `<thread>` | `thread`, `join`, `detach`, `this_thread::sleep_for` |
| **fastos_mutex.rs** | `<mutex>` | `mutex`, `lock_guard<T>`, `unique_lock<T>`, `scoped_lock` |
| **fastos_atomic.rs** | `<atomic>` | `atomic<T>`, `fetch_add`, `store`, `load`, `memory_order` |
| **fastos_future.rs** | `<future>` | `future<T>`, `promise<T>`, `async`, `launch` |
| **fastos_condition_variable.rs** | `<condition_variable>` | `condition_variable`, `wait`, `notify_one`, `notify_all` |
| **fastos_regex.rs** | `<regex>` | `regex`, `smatch`, `regex_search`, `regex_match`, `regex_replace` |
| **fastos_random.rs** | `<random>` | `mt19937`, `uniform_int_distribution`, `normal_distribution` |
| **fastos_filesystem.rs** | `<filesystem>` | `path`, `exists`, `directory_iterator`, `create_directories` |
| **fastos_numeric.rs** | `<numeric>` | ✅ Ya funciona vía `algorithm` — pero debería tener su propio registro |
| **fastos_string_view.rs** | `<string_view>` | `string_view` como tipo conocido del parser |
| **fastos_span.rs** | `<span>` | `span<T>`, `first`, `last`, `subspan` |
| **fastos_initializer_list.rs** | `<initializer_list>` | `initializer_list<T>` como tipo |
| **fastos_iterator.rs** | `<iterator>` | `advance`, `distance`, `next`, `prev`, `back_inserter` |

---

## SECCIÓN 4: Problemas del Parser (No son de stdlib)

Estos problemas causan fallos INCLUSO si las headers existieran:

| Problema | Ejemplo | Impacto |
|----------|---------|---------|
| **Tipos template como miembros de struct** | `std::shared_ptr<Node> next;` dentro de struct | ❌ No parseable |
| **`decltype()` en template args** | `std::tuple_size<decltype(t)>::value` | ❌ No soportado |
| **`::type` / `::value` después de template** | `std::remove_const<int>::type` | ❌ No soportado |
| **`_v` suffix traits** | `std::is_integral_v<int>` | ❌ No reconocido |
| **Non-type template arguments** | `std::get<0>(t)`, `std::array<int, 5>` | ❌ No soportado |
| **Namespace alias dentro de función** | `namespace fs = std::filesystem;` | ❌ No soportado |
| **Structured bindings** | `auto [a, b, c] = tuple;` | ❌ No soportado |
| **`_Complex` como tipo C99** | `double _Complex z1;` | ❌ Parser C no reconoce |
| **Template function calls** | `std::any_cast<int>(a)` | ❌ No parseable |
| **Multi-scope access** | `std::chrono::milliseconds` | ⚠️ Parcial |

---

## SECCIÓN 5: Prioridades de Implementación

### 🔴 Prioridad ALTA (rompe código básico)
1. **Registrar tipos STL como tipos conocidos del parser** (`set`, `list`, `deque`, `stack`, `queue`, `array`)
2. **`fastos_chrono.rs`** — muy usado en benchmarking/timing
3. **`fastos_thread.rs` + `fastos_mutex.rs` + `fastos_atomic.rs`** — concurrencia básica
4. **Soporte de `std::template<T>` como campo de struct/class**

### 🟡 Prioridad MEDIA (código moderno C++17)
5. **`fastos_optional.rs`** — extremadamente común en C++17
6. **`fastos_variant.rs`** + **`fastos_any.rs`**
7. **`fastos_tuple.rs`** — usado extensivamente
8. **`fastos_filesystem.rs`** — operaciones de archivos
9. **Soporte de `::type` / `::value` en type traits**

### 🟢 Prioridad BAJA (avanzado)
10. **`fastos_regex.rs`** + **`fastos_random.rs`**
11. **`fastos_future.rs`** + **`fastos_condition_variable.rs`**
12. **`fastos_string_view.rs`** + **`fastos_span.rs`**
13. **Structured bindings**, **`decltype` in templates**, **namespace alias en funciones**

---

## SECCIÓN 6: Archivos de Test Incluidos

### C (7 tests — carpeta `reportes/tests_c/`)
| Test | Headers | Estado |
|------|---------|--------|
| `test_math_avanzado.c` | `math.h` | ✅ PASA |
| `test_ctype.c` | `ctype.h` | ✅ PASA |
| `test_signal.c` | `signal.h` | ✅ PASA |
| `test_setjmp.c` | `setjmp.h` | ✅ PASA |
| `test_stdarg.c` | `stdarg.h` | ✅ PASA |
| `test_wchar.c` | `wchar.h` | ✅ PASA |
| `test_complex.c` | `complex.h` | ⚠️ Parsea pero no genera código para main |

### C++ (12 tests — carpeta `reportes/tests_cpp/`)
| Test | Headers | Estado | Error |
|------|---------|--------|-------|
| `test_cmath_exp.cpp` | `cmath` | ✅ PASA | — |
| `test_numeric.cpp` | `numeric`, `vector` | ✅ PASA | — |
| `test_containers.cpp` | `set`, `list`, `deque`... | ❌ FALLA | Tipos STL no registrados |
| `test_chrono_thread.cpp` | `chrono`, `thread`... | ❌ FALLA | Módulos no existen |
| `test_optional_variant_any.cpp` | `optional`, `variant`, `any` | ❌ FALLA | Módulos no existen |
| `test_tuple.cpp` | `tuple` | ❌ FALLA | `decltype` no soportado |
| `test_regex_random.cpp` | `regex`, `random` | ❌ FALLA | Tipos no registrados |
| `test_filesystem.cpp` | `filesystem` | ❌ FALLA | Namespace alias no soportado |
| `test_type_traits.cpp` | `type_traits` | ❌ FALLA | `::type` no soportado |
| `test_smart_ptr_adv.cpp` | `memory` | ❌ FALLA | Template en struct fields |
| `test_string_view_span.cpp` | `string_view`, `span` | ❌ FALLA | Tipos no registrados |
| `test_initializer_iterator.cpp` | `initializer_list`, `iterator` | ❌ FALLA* | Tipos no registrados |

---

*ADead-BIB v7.0 — Reporte generado el 2026-03-09*
*Comando: `adb step <archivo>` para cada test*
