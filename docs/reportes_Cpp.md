# Reporte C++ — ADead-BIB Frontend & Stdlib Analysis

> **Fecha:** 30 de Marzo de 2026  
> **Versión:** ADead-BIB v8.0  
> **Autor:** Eddi Andreé Salazar Matos  
> **Licencia:** Techne v1.0 (τέχνη)  
> **Objetivo:** Auditoría completa del estado C++ — parser, lowering, driver, stdlib — con plan de extensión C++98→C++17/20 selectivo

---

## 1. Estado Real del Frontend C++ — Auditoría de Código

### 1.1 Resumen Ejecutivo

| Componente | Archivo | Líneas | Estado |
|---|---|---|---|
| **Lexer** | `parse/lexer.rs` | 969 | ✅ Completo — C++20 tokens |
| **Parser** | `parse/parser.rs` | 3624 | ✅ Completo — C++98/11/14/17/20 |
| **AST** | `ast.rs` | 644 | ✅ Completo — nodos para todo C++ moderno |
| **Preprocessor** | `preprocessor.rs` | 1031 | ✅ Completo — #include, #define, #ifdef, macros |
| **Stdlib headers** | `stdlib.rs` | 1148 | ✅ Completo — 45+ headers C++ resueltos |
| **cpp_to_ir.rs** | — | 0 | ❌ **NO EXISTE** — archivo declarado pero no creado |
| **lower/** | — | 0 | ❌ **VACÍO** — directorio sin archivos |
| **oop/** | — | 0 | ❌ **VACÍO** — directorio sin archivos |
| **sema/** | — | 0 | ❌ **VACÍO** — directorio sin archivos |
| **lib.rs** | `lib.rs` | 17 | ❌ **NO COMPILA** — 37 errores (módulos no encontrados) |
| **Driver** | `cpp_driver.rs` | 51 | ❌ **STUB** — retorna error siempre |

### 1.2 Estado de Compilación

```
cargo build -p adeb-frontend-cpp → 37 ERRORES

Errores principales:
  E0583: file not found for module `lexer`       → lib.rs dice `mod lexer` pero el archivo es `parse/lexer.rs`
  E0583: file not found for module `parser`       → lib.rs dice `mod parser` pero el archivo es `parse/parser.rs`
  E0583: file not found for module `cpp_to_ir`    → ARCHIVO NO EXISTE
  E0432: unresolved import `crate::stdlib::cpp`   → stdlib.rs no tiene submódulo `cpp`
  E0432: unresolved import `regex`                → dependencia `regex` no declarada en Cargo.toml
```

### 1.3 Diagnóstico: El parser funciona pero no está conectado

El parser C++ tiene **3624 líneas de código funcional** con tests unitarios que pasan.
El problema es que `lib.rs` declara módulos con paths incorrectos y `cpp_to_ir.rs` nunca fue creado.

**Lo que FUNCIONA internamente (vía `cpp_mod.rs`):**
- `cpp_mod.rs` → declara `pub mod cpp_lexer`, `pub mod cpp_parser`, `pub mod cpp_to_ir`
- `parse/lexer.rs` y `parse/parser.rs` → existen y son funcionales
- Los tests del parser pasan: funciones, clases, templates, namespaces, enums, using

**Lo que FALTA para que compile:**
1. Fijar `lib.rs` para usar los paths correctos (`parse::lexer`, `parse::parser`)
2. Crear `cpp_to_ir.rs` — el lowering de CppAST → Program IR
3. Agregar `regex` al `Cargo.toml` (o eliminar el import)
4. Conectar el driver C++ al pipeline real

---

## 2. Parser C++ — Análisis de Features por Estándar

### 2.1 C++98 — Base (Canon ADead-BIB)

| Feature | AST Node | Parser | Lowering |
|---|---|---|---|
| Classes con herencia | `ClassDef`, `CppBaseClass` | ✅ | ❌ |
| Virtual / pure virtual | `CppFuncQualifiers::is_virtual` | ✅ | ❌ |
| Constructores / Destructores | `Constructor`, `Destructor` | ✅ | ❌ |
| Operator overloading | `parse_operator_name()` | ✅ | ❌ |
| Templates `<typename T>` | `CppTemplateParam` | ✅ | ❌ |
| Namespaces | `Namespace` | ✅ | ❌ |
| References `T&` | `CppType::Reference` | ✅ | ❌ |
| `new` / `delete` | `CppExpr::New`, `Delete` | ✅ | ❌ |
| `friend` declarations | `CppClassMember::FriendDecl` | ✅ | ❌ |
| `explicit` constructors | `is_explicit` field | ✅ | ❌ |
| C-style / `static_cast` / etc. | `CppCastKind` (5 tipos) | ✅ | ❌ |
| `sizeof(type)` / `sizeof(expr)` | `CppSizeOfArg` | ✅ | ❌ |
| Exception handling `try/catch/throw` | `CppStmt::Try` | ✅ | ❌ |
| `enum` / `enum class` | `CppTopLevel::EnumDef` | ✅ | ❌ |
| `typedef` / type alias | `TypeAlias` | ✅ | ❌ |

### 2.2 C++11 — Movimiento, Lambdas, Auto

| Feature | AST Node | Parser | Lowering |
|---|---|---|---|
| `auto` type deduction | `CppType::Auto` | ✅ | ❌ |
| `nullptr` | `CppExpr::NullptrLiteral` | ✅ | ❌ |
| Lambda expressions | `CppExpr::Lambda` | ✅ | ❌ |
| R-value references `T&&` | `CppType::RValueRef` | ✅ | ❌ |
| Range-for `for(auto x : v)` | `CppStmt::RangeFor` | ✅ | ❌ |
| `constexpr` functions | `CppType::Constexpr` | ✅ | ❌ |
| `decltype(expr)` | `CppType::Decltype` | ✅ | ❌ |
| `static_assert(cond, msg)` | `CppTopLevel::StaticAssert` | ✅ | ❌ |
| `enum class` con tipo base | `underlying_type: Option<CppType>` | ✅ | ❌ |
| Initializer lists `{1,2,3}` | `CppExpr::InitList` | ✅ | ❌ |
| `= default` / `= delete` | `is_default`, `is_delete` | ✅ | ❌ |
| `noexcept` | `is_noexcept` | ✅ | ❌ |
| `override` / `final` | `is_override`, `is_final` | ✅ | ❌ |
| `thread_local` | `CppToken::Thread_local` | ✅ | ❌ |

### 2.3 C++14 — Mejoras menores

| Feature | AST Node | Parser | Lowering |
|---|---|---|---|
| Generic lambdas `[](auto x)` | Via `CppType::Auto` en params | ✅ | ❌ |
| Relaxed `constexpr` | Misma infraestructura | ✅ | ❌ |
| Variable templates | Via `CppTemplateParam` | ⚠️ Parcial | ❌ |
| `decltype(auto)` | Combinación existente | ✅ | ❌ |

### 2.4 C++17 — Features Selectivas Importantes

| Feature | AST Node | Parser | Lowering | Prioridad |
|---|---|---|---|---|
| Structured bindings `auto [a,b]` | Via `VarDecl` descompuesto | ✅ | ❌ | 🔴 Alta |
| `if constexpr` | `CppStmt::If::is_constexpr` | ✅ | ❌ | 🔴 Alta |
| If with initializer `if(auto x=f(); x>0)` | `CppStmt::If::init` | ✅ | ❌ | 🟡 Media |
| `std::optional<T>` | `CppType::StdOptional` | ✅ | ❌ | 🟡 Media |
| `std::variant<Ts...>` | `CppType::StdVariant` | ✅ | ❌ | 🟡 Media |
| `std::string_view` | `CppType::StdStringView` | ✅ | ❌ | 🟡 Media |
| `std::any` | `CppType::StdAny` | ✅ | ❌ | 🟢 Baja |
| Nested namespaces `A::B::C` | Parseado | ✅ | ❌ | 🟢 Baja |
| Fold expressions `(args + ...)` | `CppExpr::FoldExpr` | ✅ | ❌ | 🟢 Baja |
| Template specialization | `TemplateSpecialization` | ✅ | ❌ | 🟡 Media |
| `inline` variables | Via qualifiers | ✅ | ❌ | 🟢 Baja |

### 2.5 C++20 — Selectivo (solo lo importante)

| Feature | AST Node | Parser | Lowering | Prioridad |
|---|---|---|---|---|
| Spaceship operator `<=>` | `CppToken::Spaceship` | ✅ | ❌ | 🟡 Media |
| `co_await` / `co_yield` | `CppExpr::CoAwait/CoYield` | ✅ | ❌ | 🟢 Baja |
| `co_return` | `CppStmt::CoReturn` | ✅ | ❌ | 🟢 Baja |
| `concept` / `requires` | `CppToken::Concept/Requires` | ⚠️ Token only | ❌ | 🟢 Baja |
| `consteval` / `constinit` | `CppToken::Consteval/Constinit` | ✅ | ❌ | 🟢 Baja |
| `char8_t` | `CppType::Char8` | ✅ | ❌ | 🟢 Baja |

---

## 3. Plan de Extensión: C heredando a C++

### 3.1 Principio: C++ hereda de C automáticamente

El pipeline actual de C es:
```
.c → CPreprocessor → CLexer → CParser → CToIR → Program → IsaCompiler → PE
```

El pipeline C++ debe ser:
```
.cpp → CppPreprocessor → CppLexer → CppParser → CppToIR → Program → IsaCompiler → PE
```

**Ambos convergen en `Program` IR** — el ISA compiler y PE builder son compartidos.
El CppToIR solo necesita convertir los nodos C++ extra (classes, templates, namespaces, lambdas) a las mismas primitivas que C usa (functions, structs, statements).

### 3.2 Qué puede reutilizar C++ de C

| Componente C | C++ lo hereda |
|---|---|
| ISA compiler (`isa_compiler.rs`) | ✅ 100% — mismo backend |
| PE builder (`pe.rs`) | ✅ 100% — mismo output |
| Program IR (`ast.rs` de adeb-core) | ✅ 100% — mismas structs |
| IAT registry (msvcrt.dll) | ✅ 100% — mismo printf/malloc |
| UB detector patterns | ✅ ~80% — C++ agrega reglas OOP |
| Data section & strings | ✅ 100% — misma mecánica |

### 3.3 Qué es nuevo en CppToIR (lo que falta crear)

| Transformación | Complejidad | Prioridad |
|---|---|---|
| **Classes → Structs + Functions** | Media | 🔴 Fase 1 |
| **Constructores → init functions** | Media | 🔴 Fase 1 |
| **Destructores → cleanup functions** | Media | 🔴 Fase 1 |
| **`this` → primer parámetro implícito** | Baja | 🔴 Fase 1 |
| **Herencia → struct embedding** | Media | 🔴 Fase 1 |
| **Virtual → vtable (array de punteros)** | Alta | 🟡 Fase 2 |
| **Templates → monomorphización** | Alta | 🟡 Fase 2 |
| **Namespaces → name mangling** | Baja | 🔴 Fase 1 |
| **Lambdas → closure structs** | Media | 🟡 Fase 2 |
| **Operator overload → function rename** | Baja | 🔴 Fase 1 |
| **`new`/`delete` → malloc/free** | Baja | 🔴 Fase 1 |
| **References → pointers** | Baja | 🔴 Fase 1 |
| **`auto` → type inference** | Media | 🟡 Fase 2 |
| **Range-for → while con iterador** | Media | 🟡 Fase 2 |
| **Structured bindings → individual vars** | Baja | 🟡 Fase 2 |
| **`if constexpr` → dead branch elimination** | Media | 🟡 Fase 2 |
| **STL containers → C inline impls** | Alta | 🟡 Fase 3 |
| **Exceptions → error codes** | Media | 🟢 Fase 4 |

---

## 4. Fases de Implementación

### 🔴 Fase 1 — Hacer que C++ compile (Fundación)

**Objetivo:** `adB cxx hello.cpp -o hello.exe` funciona para C++ básico.

1. **Fijar `lib.rs`** — corregir paths de módulos para que compile
2. **Crear `cpp_to_ir.rs`** — lowering básico:
   - Functions → `Program::functions` (idéntico a C)
   - Classes → `Program::structs` + methods como functions
   - `this` → primer parámetro ptr
   - Namespaces → flatten con `::` en nombre
   - `new T(args)` → `malloc(sizeof(T))` + constructor call
   - `delete p` → destructor call + `free(p)`
   - References `T&` → pointers
   - Operator overload → function `operator_add`, `operator_eq`, etc.
3. **Conectar driver** — reemplazar stub con pipeline real
4. **Tests básicos** — hello world C++, clase simple, herencia simple

### 🟡 Fase 2 — C++ Moderno (C++11/14/17)

1. **Templates** — monomorphización (solo instanciaciones usadas)
2. **Lambdas** — closure como struct con `operator()` 
3. **Virtual / vtable** — array de function pointers en struct
4. **`auto`** — deducción de tipo basada en inicializador
5. **Range-for** — expandir a while con begin/end
6. **Structured bindings** — descomponer a variables individuales
7. **`if constexpr`** — evaluar condición en compile time, eliminar rama muerta
8. **Move semantics** — transferencia de ownership sin copia

### 🟡 Fase 3 — STL con implementación C inline

1. **`std::string`** — ya existe `__adb_string` (SSO 22-byte)
2. **`std::vector`** — ya existe `__adb_vector` (push_back, at, resize)
3. **`std::cout`/`std::cin`** — ya existe via printf/scanf
4. **`std::map`** — ya existe `__adb_map` (sorted array)
5. **`std::unique_ptr`** — raw ptr + free en destructor
6. **`std::array`** — fixed array con bounds check
7. **`std::optional`** — has_value + union storage
8. **`std::string_view`** — ptr + size sin ownership

### 🟢 Fase 4 — Avanzado (selectivo)

1. **Exceptions → error codes** (ADead-BIB elimina excepciones)
2. **RTTI → eliminado** (typeid/dynamic_cast no soportados)
3. **Coroutines** — stackless transform
4. **Concepts** — compile-time constraints
5. **Concurrencia** — wrappers Win32/pthread

---

## 5. Stdlib C++ — Estado de Módulos fastos_*.rs

### Módulos CON Implementación C Inline (IMPL)

| Módulo | Header C++ | Métodos | Impl C | Calidad |
|---|---|---|---|---|
| `fastos_vector.rs` | `<vector>` | 25 | ✅ `__adb_vector` (init, push_back, pop_back, at, resize, reserve, move, free) | 9/10 |
| `fastos_string_cpp.rs` | `<string>` | 32 | ✅ `__adb_string` con SSO 22-byte | 9/10 |
| `fastos_iostream.rs` | `<iostream>` | 36 | ✅ `__adb_ostream/__adb_istream` (cout/cin via printf/scanf) | 7/10 |
| `fastos_map.rs` | `<map>` `<unordered_map>` | 14 | ✅ `__adb_map` sorted + `__adb_umap` hash | 8/10 |
| `fastos_algorithm.rs` | `<algorithm>` | 58 | ✅ `__alg_*` (sort, find, accumulate, reverse, binary_search) | 7/10 |

### Módulos SOLO Registro de Símbolos (27)

| Prioridad | Módulos |
|---|---|
| 🔴 Alta | `fastos_memory.rs`, `fastos_utility.rs`, `fastos_array.rs`, `fastos_string_view.rs`, `fastos_initializer_list.rs`, `fastos_iterator.rs` |
| 🟡 Media | `fastos_set.rs`, `fastos_list.rs`, `fastos_deque.rs`, `fastos_stack_queue.rs`, `fastos_tuple.rs`, `fastos_numeric.rs`, `fastos_optional.rs`, `fastos_variant.rs`, `fastos_any.rs`, `fastos_span.rs` |
| 🟡 Media | `fastos_chrono.rs`, `fastos_thread.rs`, `fastos_mutex.rs`, `fastos_atomic.rs` |
| 🟢 Baja | `fastos_functional.rs`, `fastos_exceptions.rs`, `fastos_future.rs`, `fastos_condition_variable.rs`, `fastos_regex.rs`, `fastos_random.rs`, `fastos_filesystem.rs` |

### Headers C++ FALTANTES (sin módulo alguno)

| Header | Prioridad | Razón |
|---|---|---|
| `<new>` | 🔴 Alta | placement new — fundamental C++ |
| `<bitset>` | 🔴 Alta | usado frecuentemente |
| `<typeinfo>` | 🟡 Media | RTTI (solo si se habilita) |
| `<complex>` | 🟡 Media | números complejos |
| `<valarray>` | 🟡 Media | operaciones SIMD-friendly |
| `<ratio>` | 🟡 Media | dependencia de `<chrono>` |
| `<charconv>` | 🟡 Media | to_chars/from_chars rápido (C++17) |
| `<type_traits>` | 🟡 Media | is_same, enable_if — compile-time |
| `<locale>` | 🟢 Baja | localización |
| `<codecvt>` | 🟢 Baja | deprecated C++17 |
| `<typeindex>` | 🟢 Baja | wrapper type_info |
| `<scoped_allocator>` | 🟢 Baja | allocator propagation |
| `<execution>` | 🟢 Baja | parallel policies |
| `<memory_resource>` | 🟢 Baja | PMR allocators |

---

## 6. Cobertura Global

### Por Estándar

| Estándar | Features en AST | Parseadas | Con Lowering | Cobertura Real |
|---|---|---|---|---|
| C++98 | 15 | 15 | 0 | ❌ 0% funcional |
| C++11 | 14 | 14 | 0 | ❌ 0% funcional |
| C++14 | 4 | 3 | 0 | ❌ 0% funcional |
| C++17 | 11 | 11 | 0 | ❌ 0% funcional |
| C++20 | 6 | 4 | 0 | ❌ 0% funcional |

### Por Capa

```
Lexer (tokens):         ████████████████████████████████████ 100% ✅
Parser (AST):           ████████████████████████████████░░░  ~90% ✅ 
Preprocessor:           ████████████████████████████████████ 100% ✅
Stdlib headers:         ████████████████████████████████░░░  ~85% ✅
Lowering (CppToIR):     ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   0% ❌
Driver (E2E):           ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   0% ❌
lib.rs (compila):       ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   NO ❌
```

### Conclusión

**El 80% del trabajo de parsing ya está hecho.** El parser C++ (3624 líneas) cubre prácticamente todo C++98 hasta C++17 con tokens C++20.

**El bloqueante es el lowering:** `cpp_to_ir.rs` no existe. Sin él, ningún programa C++ puede compilar.

**La buena noticia:** El backend (ISA compiler + PE builder) ya funciona perfectamente para C. C++ solo necesita convertir sus constructos a las mismas primitivas que C usa:
- Classes → Structs + Functions
- Templates → Monomorphized functions
- Namespaces → Prefixed names
- new/delete → malloc/free
- References → Pointers

---

*ADead-BIB v8.0 — C++ Complete Analysis — Marzo 2026*  
*Eddi Andreé Salazar Matos — Lima, Perú*  
*Licencia: Techne v1.0 (τέχνη)*  
*"El parser ya sabe C++. Solo falta enseñarle a hablar IR." 💀🦈*
