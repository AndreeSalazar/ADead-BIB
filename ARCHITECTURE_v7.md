# ADead-BIB Compiler Architecture v7.0

> Grace Hopper: 'la maquina sirve al humano'
> Dennis Ritchie: 'small is beautiful'
> Ken Thompson: 'trust only code you created'
> Bjarne Stroustrup: 'within C++ a smaller cleaner language struggles to get out'
> Linus Torvalds: 'talk is cheap, show me the code'
> ADead-BIB 2026: cumple los 5 + elimina el linker para siempre 💀🦈 🇵🇪

---

## Filosofía Central

```
SIN LINKER EXTERNO — NUNCA
  ld     ❌ eliminado
  lld    ❌ eliminado
  link.exe ❌ eliminado
  gold   ❌ eliminado
  mold   ❌ eliminado

EL HEADER ES SUFICIENTE
  header_main.h → hereda TODO
  tree shaking automático → binario mínimo
  un comando → un ejecutable
  sin flags → sin dolor → sin Stack Overflow

UB DETECTION ANTES DEL OPTIMIZER
  si fuera después → optimizer elimina evidencia
  exactamente lo que hace GCC ❌
  ADead-BIB: UB primero → optimizer después ✓
```

---

## Pipeline Completo v7.0

```
C99/C++98/C++17 codigo fuente
        │
        ▼
[ PREPROCESSOR ]  ←── preprocessor/
  header_main.h resolution COMPLETA
  hereda TODAS las headers fastos_*.h
  fastos.bib cache (CACHE HIT = nanosegundos)
  symbol deduplication GLOBAL
  C++11/C++14/C++17 completo → C++98 canon (34 features)
  tree shaking preparation (marca símbolos usados)
        │
        ▼
[ LINKER ELIMINATOR ]  ←── preprocessor/resolver.rs
  Sin .o intermedios — NUNCA
  Sin ld/lld/link.exe externos — NUNCA
  Resolución de símbolos INTERNA completa
  Unity build automático (todos los .cpp juntos)
  Dead symbol elimination (no linkea lo no usado)
  = source → IR directo
  = sin programa externo
  = sin "undefined reference" — NUNCA
  = sin flags misteriosos — NUNCA
        │
        ▼
[ PARSER / AST ]  ←── frontend/c/ + frontend/cpp/
  C99 parser separado (c_parser.rs)
  C++98 parser separado (cpp_parser.rs)
  tipos resueltos estáticamente
  macros expandidos completamente
        │
        ▼
[ IR — ADeadOp ]  ←── middle/ir/
  AST → operaciones abstractas SSA-form
  tipos explícitos en cada instrucción
  flujo de control claro (BasicBlocks)
  sin ambigüedad semántica
        │
        ▼
[ UB_DETECTOR ]  ←── middle/ub_detector/ (ÚNICO EN EL MUNDO)
  Analiza IR completo ANTES de codegen
  21 tipos de UB detectados (ver sección completa)
  Modo Estricto (default) → SE DETIENE con reporte
  --warn-ub → avisa con reporte y continua
  Cache de resultados en fastos.bib
        │
        ▼
[ OPTIMIZER ]  ←── optimizer/
  Dead code elimination
  Constant folding
  Constant propagation
  Redundant ops removal
  Inline expansion
  Binary-level size optimization
  Branchless transforms
  SIMD code generation
  SIN explotar UB — nunca
        │
        ▼
[ REGISTER ALLOCATOR ]  ←── isa/reg_alloc.rs
  IR variables → 13 registros físicos x86-64
  TempAllocator (fast path)
  LinearScanAllocator (liveness analysis)
  Spill automático con stack alignment 16-byte
        │
        ▼
[ ISA COMPILER ]  ←── isa/
  c_isa.rs   → C99 layout/sizeof/alignment
  cpp_isa.rs → C++98 vtable/this/constructors
  encoder.rs → bytes x86-64 directos
        │
        ▼
[ OUTPUT DIRECTO ]  ←── output/
  Sin linker externo — genera binario DIRECTAMENTE
  --target fastos  → .po  (24 bytes header propio)
  --target windows → .exe (PE completo)
  --target linux   → .elf (ELF completo)
  --target all     → los 3 simultáneamente
```

---

## Estructura de Directorios v7.0 (Completa)

```
src/rust/
├── lib.rs                        # Exports públicos + re-exports
├── main.rs                       # CLI: adeadc cc/cxx/build/run
├── builder.rs                    # Orquestador del pipeline completo
│
├── preprocessor/                 # SIN CMake, SIN Linker — NUNCA
│   ├── mod.rs                    # Entry point del preprocessor
│   ├── resolver.rs               # Header resolution + LINKER ELIMINATOR
│   │                             #   resolve_symbol(name) → InternalDef
│   │                             #   unity_build(files) → single IR
│   │                             #   eliminate_external_deps() → void
│   │                             #   mark_used_symbols(ast) → SymbolSet
│   ├── dedup.rs                  # Symbol Table deduplication global
│   │                             #   dedup_symbols(table) → UniqueTable
│   │                             #   detect_conflicts(a, b) → Option<Conflict>
│   └── expander.rs               # C++11/C++14/C++17 → C++98 canon
│                                 #   expand_lambda(node) → C++98Closure
│                                 #   expand_range_for(node) → C++98Loop
│                                 #   expand_auto(node) → ExplicitType
│                                 #   expand_nullptr(node) → NullLiteral
│                                 #   expand_structured_binding(node) → C++98
│                                 #   expand_if_constexpr(node) → C++98Branch
│                                 #   expand_fold_expression(node) → C++98
│                                 #   (34 funciones totales — ver sección)
│
├── stdlib/                       # Standard Library PROPIA — Sin libc externa
│   ├── mod.rs                    # Registry de todas las fastos headers
│   ├── header_main.rs            # header_main.h — hereda TODO (ver sección)
│   │
│   ├── c/                        # C99 Standard Library
│   │   ├── fastos_stdio.rs       # printf, scanf, fprintf, fopen, fclose
│   │   │                         #   fastos_printf(fmt, args) → void
│   │   │                         #   fastos_scanf(fmt, args) → i32
│   │   │                         #   fastos_fopen(path, mode) → *File
│   │   │                         #   fastos_fclose(file) → i32
│   │   │                         #   fastos_fread/fwrite(buf, size, n, f) → usize
│   │   │                         #   fastos_fgets(buf, n, file) → *char
│   │   │                         #   fastos_fputs(str, file) → i32
│   │   │                         #   fastos_sprintf/snprintf → i32
│   │   │                         #   fastos_sscanf → i32
│   │   │                         #   fastos_perror(msg) → void
│   │   │                         #   Implementado sobre syscall write/read directo
│   │   │
│   │   ├── fastos_stdlib.rs      # malloc, free, NULL, exit, atoi, rand
│   │   │                         #   fastos_malloc(size) → *void
│   │   │                         #   fastos_calloc(n, size) → *void
│   │   │                         #   fastos_realloc(ptr, size) → *void
│   │   │                         #   fastos_free(ptr) → void
│   │   │                         #   fastos_exit(code) → !
│   │   │                         #   fastos_abort() → !
│   │   │                         #   fastos_atoi(str) → i32
│   │   │                         #   fastos_atof(str) → f64
│   │   │                         #   fastos_itoa(n, buf, base) → *char
│   │   │                         #   fastos_rand() → i32
│   │   │                         #   fastos_srand(seed) → void
│   │   │                         #   fastos_qsort(arr, n, size, cmp) → void
│   │   │                         #   fastos_bsearch(key, arr, n, size, cmp) → *void
│   │   │                         #   NULL definido como ((void*)0)
│   │   │                         #   Implementado sobre syscall mmap/munmap directo
│   │   │
│   │   ├── fastos_string.rs      # strlen, strcpy, strcat, strcmp, memcpy
│   │   │                         #   fastos_strlen(str) → usize
│   │   │                         #   fastos_strcpy(dst, src) → *char
│   │   │                         #   fastos_strncpy(dst, src, n) → *char
│   │   │                         #   fastos_strcat(dst, src) → *char
│   │   │                         #   fastos_strncat(dst, src, n) → *char
│   │   │                         #   fastos_strcmp(a, b) → i32
│   │   │                         #   fastos_strncmp(a, b, n) → i32
│   │   │                         #   fastos_strchr(str, c) → *char
│   │   │                         #   fastos_strrchr(str, c) → *char
│   │   │                         #   fastos_strstr(hay, needle) → *char
│   │   │                         #   fastos_memcpy(dst, src, n) → *void
│   │   │                         #   fastos_memmove(dst, src, n) → *void
│   │   │                         #   fastos_memset(ptr, val, n) → *void
│   │   │                         #   fastos_memcmp(a, b, n) → i32
│   │   │                         #   fastos_strtok(str, delim) → *char
│   │   │                         #   fastos_strdup(str) → *char
│   │   │
│   │   ├── fastos_math.rs        # sin, cos, tan, sqrt, pow, log, PI
│   │   │                         #   fastos_sin(x: f64) → f64
│   │   │                         #   fastos_cos(x: f64) → f64
│   │   │                         #   fastos_tan(x: f64) → f64
│   │   │                         #   fastos_asin/acos/atan(x) → f64
│   │   │                         #   fastos_atan2(y, x) → f64
│   │   │                         #   fastos_sqrt(x: f64) → f64
│   │   │                         #   fastos_cbrt(x: f64) → f64
│   │   │                         #   fastos_pow(base, exp) → f64
│   │   │                         #   fastos_exp(x: f64) → f64
│   │   │                         #   fastos_log(x: f64) → f64
│   │   │                         #   fastos_log2(x: f64) → f64
│   │   │                         #   fastos_log10(x: f64) → f64
│   │   │                         #   fastos_floor/ceil/round(x) → f64
│   │   │                         #   fastos_fabs/fabsf(x) → f64
│   │   │                         #   fastos_fmod(x, y) → f64
│   │   │                         #   fastos_hypot(x, y) → f64
│   │   │                         #   PI  = 3.14159265358979323846
│   │   │                         #   E   = 2.71828182845904523536
│   │   │                         #   TAU = 6.28318530717958647692
│   │   │                         #   Implementado con x87/SSE2 instrucciones
│   │   │
│   │   ├── fastos_time.rs        # clock, time, sleep, gettimeofday
│   │   │                         #   fastos_time(t) → i64
│   │   │                         #   fastos_clock() → u64
│   │   │                         #   fastos_sleep(seconds) → void
│   │   │                         #   fastos_usleep(microseconds) → void
│   │   │                         #   fastos_nanosleep(ns) → void
│   │   │                         #   fastos_gettimeofday(tv, tz) → i32
│   │   │                         #   fastos_localtime(t) → *tm
│   │   │                         #   fastos_gmtime(t) → *tm
│   │   │                         #   fastos_mktime(tm) → i64
│   │   │                         #   fastos_strftime(buf, n, fmt, tm) → usize
│   │   │
│   │   ├── fastos_assert.rs      # assert, static_assert, NDEBUG
│   │   │                         #   assert(cond) → void | abort()
│   │   │                         #   static_assert(cond, msg) → compiletime
│   │   │                         #   NDEBUG desactiva assert en release
│   │   │
│   │   ├── fastos_errno.rs       # errno, strerror, error codes
│   │   │                         #   errno global thread-local
│   │   │                         #   ENOENT, ENOMEM, EINVAL, etc.
│   │   │                         #   fastos_strerror(errno) → *char
│   │   │
│   │   ├── fastos_limits.rs      # INT_MAX, INT_MIN, CHAR_MAX, etc.
│   │   │                         #   INT_MAX    = 2147483647
│   │   │                         #   INT_MIN    = -2147483648
│   │   │                         #   UINT_MAX   = 4294967295
│   │   │                         #   LONG_MAX   = 9223372036854775807
│   │   │                         #   CHAR_MAX   = 127 / 255
│   │   │                         #   SIZE_MAX   = 18446744073709551615
│   │   │
│   │   └── fastos_types.rs       # stdint.h + stddef.h + stdbool.h
│   │                             #   int8_t, int16_t, int32_t, int64_t
│   │                             #   uint8_t, uint16_t, uint32_t, uint64_t
│   │                             #   intptr_t, uintptr_t, ptrdiff_t
│   │                             #   size_t, ssize_t, off_t
│   │                             #   bool, true, false (_Bool nativo C99)
│   │                             #   NULL, offsetof(type, member)
│   │                             #   wchar_t, wint_t
│   │
│   └── cpp/                      # C++ Standard Library
│       ├── fastos_iostream.rs    # std::cout, std::cin, std::cerr, std::endl
│       │                         #   ostream::operator<<(const char*) → ostream&
│       │                         #   ostream::operator<<(int) → ostream&
│       │                         #   ostream::operator<<(long) → ostream&
│       │                         #   ostream::operator<<(long long) → ostream&
│       │                         #   ostream::operator<<(float) → ostream&
│       │                         #   ostream::operator<<(double) → ostream&
│       │                         #   ostream::operator<<(char) → ostream&
│       │                         #   ostream::operator<<(bool) → ostream&
│       │                         #   ostream::operator<<(void*) → ostream&
│       │                         #   ostream::operator<<(ostream& (*)(ostream&))
│       │                         #   istream::operator>>(int&) → istream&
│       │                         #   istream::operator>>(double&) → istream&
│       │                         #   istream::operator>>(char*) → istream& (safe)
│       │                         #   istream::operator>>(char&) → istream&
│       │                         #   endl(ostream&) → ostream& (flush incluido)
│       │                         #   flush(ostream&) → ostream&
│       │                         #   Implementado sobre fastos_stdio internamente
│       │
│       ├── fastos_vector.rs      # std::vector<T>
│       │                         #   vector<T>::vector() → default ctor
│       │                         #   vector<T>::vector(n) → size ctor
│       │                         #   vector<T>::vector(n, val) → fill ctor
│       │                         #   vector<T>::push_back(val) → void
│       │                         #   vector<T>::pop_back() → void
│       │                         #   vector<T>::operator[](i) → T&
│       │                         #   vector<T>::at(i) → T& (bounds checked)
│       │                         #   vector<T>::size() → size_t
│       │                         #   vector<T>::capacity() → size_t
│       │                         #   vector<T>::empty() → bool
│       │                         #   vector<T>::clear() → void
│       │                         #   vector<T>::resize(n) → void
│       │                         #   vector<T>::reserve(n) → void
│       │                         #   vector<T>::begin/end() → iterator
│       │                         #   vector<T>::front/back() → T&
│       │                         #   vector<T>::data() → T*
│       │                         #   vector<T>::insert(pos, val) → iterator
│       │                         #   vector<T>::erase(pos) → iterator
│       │                         #   Rule of Three completo (copy ctor/assign/dtor)
│       │
│       ├── fastos_string_cpp.rs  # std::string
│       │                         #   string::string() → default
│       │                         #   string::string(const char*) → from cstr
│       │                         #   string::string(n, char) → fill
│       │                         #   string::operator=(const string&) → string&
│       │                         #   string::operator+(const string&) → string
│       │                         #   string::operator[](i) → char&
│       │                         #   string::at(i) → char& (bounds checked)
│       │                         #   string::length/size() → size_t
│       │                         #   string::empty() → bool
│       │                         #   string::clear() → void
│       │                         #   string::c_str() → const char*
│       │                         #   string::data() → char*
│       │                         #   string::find(str, pos) → size_t
│       │                         #   string::substr(pos, len) → string
│       │                         #   string::append(str) → string&
│       │                         #   string::compare(str) → i32
│       │                         #   string::operator==/!=/</>
│       │                         #   string::begin/end() → iterator
│       │                         #   string::npos = SIZE_MAX
│       │
│       ├── fastos_map.rs         # std::map<K,V>
│       │                         #   map<K,V>::map() → default
│       │                         #   map<K,V>::operator[](key) → V&
│       │                         #   map<K,V>::at(key) → V& (bounds checked)
│       │                         #   map<K,V>::insert(pair) → pair<iter,bool>
│       │                         #   map<K,V>::erase(key) → size_t
│       │                         #   map<K,V>::find(key) → iterator
│       │                         #   map<K,V>::count(key) → size_t
│       │                         #   map<K,V>::size() → size_t
│       │                         #   map<K,V>::empty() → bool
│       │                         #   map<K,V>::clear() → void
│       │                         #   map<K,V>::begin/end() → iterator
│       │                         #   Implementado como Red-Black Tree
│       │
│       ├── fastos_memory.rs      # std::unique_ptr, std::shared_ptr
│       │                         #   unique_ptr<T>::unique_ptr(T*) → ctor
│       │                         #   unique_ptr<T>::~unique_ptr() → dtor (auto free)
│       │                         #   unique_ptr<T>::get() → T*
│       │                         #   unique_ptr<T>::release() → T*
│       │                         #   unique_ptr<T>::reset(T*) → void
│       │                         #   unique_ptr<T>::operator*() → T&
│       │                         #   unique_ptr<T>::operator->() → T*
│       │                         #   make_unique<T>(args...) → unique_ptr<T>
│       │                         #   shared_ptr<T> + reference counting
│       │                         #   make_shared<T>(args...) → shared_ptr<T>
│       │                         #   weak_ptr<T> para ciclos
│       │
│       ├── fastos_algorithm.rs   # std::sort, find, copy, transform
│       │                         #   sort(first, last) → void
│       │                         #   sort(first, last, cmp) → void
│       │                         #   stable_sort(first, last) → void
│       │                         #   find(first, last, val) → iterator
│       │                         #   find_if(first, last, pred) → iterator
│       │                         #   copy(first, last, dst) → iterator
│       │                         #   transform(first, last, dst, fn) → iterator
│       │                         #   reverse(first, last) → void
│       │                         #   fill(first, last, val) → void
│       │                         #   count(first, last, val) → ptrdiff_t
│       │                         #   count_if(first, last, pred) → ptrdiff_t
│       │                         #   min/max(a, b) → T
│       │                         #   min_element/max_element(first, last)
│       │                         #   swap(a, b) → void
│       │                         #   accumulate(first, last, init) → T
│       │
│       ├── fastos_functional.rs  # std::function, std::bind
│       │                         #   function<R(Args...)> type erasure
│       │                         #   function::operator()(args) → R
│       │                         #   bind(fn, args...) → callable
│       │
│       ├── fastos_utility.rs     # std::pair, std::move, std::forward
│       │                         #   pair<T1,T2>::pair(a, b) → ctor
│       │                         #   pair<T1,T2>::first → T1&
│       │                         #   pair<T1,T2>::second → T2&
│       │                         #   make_pair(a, b) → pair<T1,T2>
│       │                         #   move(val) → T&&
│       │                         #   forward<T>(val) → T&&
│       │                         #   swap(a, b) → void
│       │
│       └── fastos_exceptions.rs  # try/catch/throw C++98
│                                 #   exception base class
│                                 #   runtime_error(msg)
│                                 #   logic_error(msg)
│                                 #   out_of_range(msg)
│                                 #   bad_alloc()
│                                 #   exception::what() → const char*
│                                 #   throw statement
│                                 #   try/catch blocks
│                                 #   catch(...) all handler
│
├── frontend/                     # C/C++ Parsing
│   ├── mod.rs
│   ├── ast.rs                    # Unified AST
│   │                             #   Program { decls: Vec<Decl> }
│   │                             #   Decl: FnDecl | VarDecl | StructDecl | etc.
│   │                             #   Stmt: If | While | For | Return | Block | etc.
│   │                             #   Expr: BinOp | UnOp | Call | Index | etc.
│   ├── types.rs                  # Type system completo
│   │                             #   Primitive: I8/I16/I32/I64/U8/U16/U32/U64
│   │                             #   Float: F32/F64/F80
│   │                             #   Pointer(Box<Type>)
│   │                             #   Array(Box<Type>, usize)
│   │                             #   Struct { name, fields }
│   │                             #   Union { name, fields }
│   │                             #   Enum { name, variants }
│   │                             #   Function { params, ret }
│   │                             #   Const/Volatile/Restrict qualifiers
│   ├── type_checker.rs           # Static analysis
│   │                             #   check_types(ast) → Result<(), TypeErr>
│   │                             #   infer_type(expr) → Type
│   │                             #   check_implicit_conversions(from, to)
│   │
│   ├── c/                        # C99 Frontend COMPLETO
│   │   ├── c_lexer.rs            # Tokenizer C99
│   │   │                         #   token_start_line tracking (Ctrl+Click preciso)
│   │   │                         #   tokens: Ident, Literal, Op, Punct, Keyword
│   │   │                         #   keywords C99: auto, break, case, char, const,
│   │   │                         #     continue, default, do, double, else, enum,
│   │   │                         #     extern, float, for, goto, if, inline, int,
│   │   │                         #     long, register, restrict, return, short,
│   │   │                         #     signed, sizeof, static, struct, switch,
│   │   │                         #     typedef, union, unsigned, void, volatile,
│   │   │                         #     while, _Bool, _Complex, _Imaginary
│   │   │
│   │   ├── c_parser.rs           # Parser C99 completo
│   │   │                         # C99 CANON — features que DEBEN funcionar:
│   │   │                         #
│   │   │                         # DECLARACIONES:
│   │   │                         #   int x = 5;
│   │   │                         #   const int MAX = 100;
│   │   │                         #   static int counter = 0;
│   │   │                         #   extern int global;
│   │   │                         #   register int fast;
│   │   │                         #   volatile int hw_reg;
│   │   │                         #   restrict int *ptr;
│   │   │                         #
│   │   │                         # TIPOS BÁSICOS C99:
│   │   │                         #   char, short, int, long, long long
│   │   │                         #   unsigned variants de todos
│   │   │                         #   float, double, long double
│   │   │                         #   _Bool (bool nativo C99)
│   │   │                         #   void, void*
│   │   │                         #
│   │   │                         # PUNTEROS:
│   │   │                         #   int *ptr = NULL;
│   │   │                         #   int **double_ptr;
│   │   │                         #   void *generic_ptr;
│   │   │                         #   const int *const ptr;
│   │   │                         #   function pointers: int (*fn)(int, int);
│   │   │                         #
│   │   │                         # ARRAYS:
│   │   │                         #   int arr[10];
│   │   │                         #   int arr[] = {1,2,3};
│   │   │                         #   int matrix[3][3];
│   │   │                         #   VLA: int vla[n]; (C99 §6.7.5.2)
│   │   │                         #
│   │   │                         # STRUCTS/UNIONS/ENUMS:
│   │   │                         #   struct Point { int x; int y; };
│   │   │                         #   typedef struct { ... } Name;
│   │   │                         #   union Data { int i; float f; };
│   │   │                         #   enum Color { RED, GREEN, BLUE };
│   │   │                         #   Flexible array: struct { int n; int arr[]; };
│   │   │                         #
│   │   │                         # CONTROL DE FLUJO:
│   │   │                         #   if/else, else if chains
│   │   │                         #   while, do-while
│   │   │                         #   for (init; cond; step)
│   │   │                         #   switch/case/default/break
│   │   │                         #   goto label; (C99 compatible)
│   │   │                         #   return expr;
│   │   │                         #   continue, break
│   │   │                         #
│   │   │                         # EXPRESIONES:
│   │   │                         #   binarias: + - * / % & | ^ << >> && || == != < > <= >=
│   │   │                         #   unarias: - ~ ! ++ -- * & sizeof
│   │   │                         #   ternaria: cond ? a : b
│   │   │                         #   asignación: = += -= *= /= %= &= |= ^= <<= >>=
│   │   │                         #   coma: expr1, expr2
│   │   │                         #   cast: (type)expr
│   │   │                         #   array index: arr[i]
│   │   │                         #   member: struct.field, ptr->field
│   │   │                         #   call: fn(args)
│   │   │                         #
│   │   │                         # FUNCIONES:
│   │   │                         #   declaración forward
│   │   │                         #   definición con body
│   │   │                         #   variadic: fn(int n, ...)
│   │   │                         #   va_list, va_start, va_arg, va_end
│   │   │                         #   inline functions (C99 §6.7.4)
│   │   │                         #   static functions (file scope)
│   │   │                         #
│   │   │                         # C99 ESPECÍFICOS:
│   │   │                         #   Designated initializers: .field = val
│   │   │                         #   Compound literals: (Type){...}
│   │   │                         #   // comentarios (C99 §6.4.9)
│   │   │                         #   Mezclado declaraciones/código
│   │   │                         #   for(int i=0; ...) declaración en for
│   │   │                         #   VLA (Variable Length Arrays)
│   │   │                         #   restrict pointers
│   │   │                         #   _Bool tipo nativo
│   │   │                         #   Flexible array members
│   │   │
│   │   ├── c_ast.rs              # AST nodes C99
│   │   ├── c_preprocessor.rs     # #define, #include, #ifdef, #pragma
│   │   │                         #   #define MACRO value
│   │   │                         #   #define MACRO(args) body
│   │   │                         #   #undef, #ifdef, #ifndef, #endif
│   │   │                         #   #if expr, #elif, #else
│   │   │                         #   #include <system> y "local"
│   │   │                         #   #pragma once
│   │   │                         #   __FILE__, __LINE__, __DATE__, __TIME__
│   │   │                         #   Stringification: #arg
│   │   │                         #   Token pasting: arg1##arg2
│   │   │
│   │   ├── c_stdlib.rs           # Mapping #include → fastos_*.rs
│   │   │                         #   #include <stdio.h>  → fastos_stdio.rs
│   │   │                         #   #include <stdlib.h> → fastos_stdlib.rs
│   │   │                         #   #include <string.h> → fastos_string.rs
│   │   │                         #   #include <math.h>   → fastos_math.rs
│   │   │                         #   #include <time.h>   → fastos_time.rs
│   │   │                         #   #include <assert.h> → fastos_assert.rs
│   │   │                         #   #include <errno.h>  → fastos_errno.rs
│   │   │                         #   #include <limits.h> → fastos_limits.rs
│   │   │                         #   #include <stdint.h> → fastos_types.rs
│   │   │                         #   #include <stddef.h> → fastos_types.rs
│   │   │                         #   #include <stdbool.h>→ fastos_types.rs
│   │   │                         #   SIN buscar en filesystem del OS
│   │   │                         #   SIN linker externo
│   │   │                         #   TODO resuelto internamente
│   │   │
│   │   └── c_to_ir.rs            # C99 AST → ADeadOp IR
│   │                             #   lower_function(fn) → IRFunction
│   │                             #   lower_stmt(stmt) → Vec<IRInstr>
│   │                             #   lower_expr(expr) → IRValue
│   │                             #   lower_struct(s) → IRType
│   │
│   └── cpp/                      # C++98 Frontend COMPLETO
│       ├── cpp_lexer.rs          # Tokenizer C++98
│       │                         #   keywords adicionales C++: class, namespace,
│       │                         #     template, typename, this, virtual, override,
│       │                         #     new, delete, operator, public, private,
│       │                         #     protected, friend, inline, explicit,
│       │                         #     mutable, throw, try, catch, using,
│       │                         #     bool, true, false, nullptr (C++11)
│       │
│       ├── cpp_parser.rs         # Parser C++98 CANON completo
│       │                         # C++98 CANON — features que DEBEN funcionar:
│       │                         #
│       │                         # CLASES:
│       │                         #   class/struct con access control
│       │                         #   public/private/protected members
│       │                         #   constructores: default, parametrizado, copia
│       │                         #   destructor: ~ClassName()
│       │                         #   Rule of Three: copy ctor + copy assign + dtor
│       │                         #   member functions
│       │                         #   static members (variables y funciones)
│       │                         #   const member functions
│       │                         #   mutable members
│       │                         #   friend classes y functions
│       │                         #   explicit constructors
│       │                         #
│       │                         # HERENCIA:
│       │                         #   herencia simple: class B : public A
│       │                         #   herencia múltiple: class C : public A, public B
│       │                         #   herencia virtual: class A : virtual public Base
│       │                         #   public/protected/private inheritance
│       │                         #   constructores de base class
│       │                         #   override de funciones virtuales
│       │                         #
│       │                         # POLIMORFISMO:
│       │                         #   virtual functions
│       │                         #   pure virtual: virtual fn() = 0
│       │                         #   abstract classes (tiene pure virtual)
│       │                         #   vtable generada automáticamente
│       │                         #   dynamic_cast<T*>(ptr) con RTTI
│       │                         #   static_cast, reinterpret_cast, const_cast
│       │                         #   typeid(expr) con RTTI
│       │                         #
│       │                         # TEMPLATES:
│       │                         #   function templates: template<typename T> fn
│       │                         #   class templates: template<typename T> class
│       │                         #   template specialization completa
│       │                         #   template partial specialization
│       │                         #   variadic templates: template<typename... Args>
│       │                         #   non-type template params: template<int N>
│       │                         #   template template params
│       │                         #   SFINAE básico con enable_if
│       │                         #   template instantiation automática
│       │                         #
│       │                         # OPERATOR OVERLOADING:
│       │                         #   operator+, -, *, /, %, &, |, ^
│       │                         #   operator==, !=, <, >, <=, >=
│       │                         #   operator=, +=, -=, *=, /=
│       │                         #   operator[], operator()
│       │                         #   operator++, -- (pre y post)
│       │                         #   operator<< y >> (stream)
│       │                         #   operator new y delete
│       │                         #   operator-> y operator*
│       │                         #   conversion operators: operator int()
│       │                         #
│       │                         # NAMESPACES:
│       │                         #   namespace std { ... }
│       │                         #   using namespace std;
│       │                         #   using std::cout;
│       │                         #   namespace anidados: A::B::fn()
│       │                         #   anonymous namespace
│       │                         #
│       │                         # MEMORY MANAGEMENT:
│       │                         #   new T → allocate + construct
│       │                         #   new T[n] → array allocation
│       │                         #   delete ptr → destruct + free
│       │                         #   delete[] ptr → array delete
│       │                         #   placement new: new(buf) T(args)
│       │                         #
│       │                         # EXCEPTIONS:
│       │                         #   throw expr
│       │                         #   try { } catch(Type& e) { }
│       │                         #   catch(...) { }
│       │                         #   noexcept specification
│       │                         #   exception specifications
│       │                         #   RAII con destructores garantizados
│       │                         #
│       │                         # REFERENCES:
│       │                         #   int& ref = var;
│       │                         #   const int& cref = expr;
│       │                         #   reference parameters
│       │                         #   return by reference
│       │                         #   rvalue references: int&& (C++11)
│       │                         #
│       │                         # INLINE/CONSTEXPR:
│       │                         #   inline functions
│       │                         #   constexpr variables y funciones
│       │                         #   const expressions evaluated at compile time
│       │                         #
│       │                         # INITIALIZER LISTS:
│       │                         #   std::initializer_list<T>
│       │                         #   uniform initialization: Type{args}
│       │
│       ├── cpp_ast.rs            # AST nodes C++98
│       ├── cpp_preprocessor.rs   # Preprocessor C++ (hereda de C + C++ extras)
│       ├── cpp_stdlib.rs         # Mapping #include C++ → fastos_*.rs
│       │                         #   #include <iostream>  → fastos_iostream.rs
│       │                         #   #include <vector>    → fastos_vector.rs
│       │                         #   #include <string>    → fastos_string_cpp.rs
│       │                         #   #include <map>       → fastos_map.rs
│       │                         #   #include <memory>    → fastos_memory.rs
│       │                         #   #include <algorithm> → fastos_algorithm.rs
│       │                         #   #include <functional>→ fastos_functional.rs
│       │                         #   #include <utility>   → fastos_utility.rs
│       │                         #   #include <exception> → fastos_exceptions.rs
│       │                         #   #include <stdexcept> → fastos_exceptions.rs
│       │                         #   SIN buscar en filesystem del OS
│       │                         #   SIN linker externo
│       │
│       └── cpp_to_ir.rs          # C++98 AST → ADeadOp IR
│                                 #   lower_class(cls) → IRStruct + vtable
│                                 #   lower_method(fn, this) → IRFunction
│                                 #   lower_template(tmpl, args) → IRFunction
│                                 #   lower_new(expr) → IRAlloc + IRCall(ctor)
│                                 #   lower_delete(expr) → IRCall(dtor) + IRFree
│                                 #   lower_virtual_call(obj, method) → vtable dispatch
│
├── middle/                       # IR + UB Detection + Analysis
│   ├── mod.rs
│   ├── ir/                       # ADeadOp IR (SSA-form)
│   │   ├── module.rs             # IRModule { functions, globals, types }
│   │   ├── function.rs           # IRFunction { params, blocks, locals }
│   │   ├── basicblock.rs         # BasicBlock { instrs, terminator }
│   │   ├── instruction.rs        # ADeadOp instructions completas:
│   │   │                         #   Alloca(type, align) → ptr
│   │   │                         #   Load(ptr, type) → value
│   │   │                         #   Store(value, ptr)
│   │   │                         #   BinOp(op, lhs, rhs) → value
│   │   │                         #   UnOp(op, val) → value
│   │   │                         #   Call(fn, args) → value
│   │   │                         #   Return(value?)
│   │   │                         #   Branch(cond, true_bb, false_bb)
│   │   │                         #   Jump(target_bb)
│   │   │                         #   Phi([(value, bb)]) → value (SSA)
│   │   │                         #   GEP(ptr, indices) → ptr (GetElementPtr)
│   │   │                         #   Cast(value, from_type, to_type) → value
│   │   │                         #   Syscall(num, args) → value
│   │   │                         #   VTableLoad(obj, method_idx) → fn_ptr
│   │   │                         #   Memcpy(dst, src, size)
│   │   │                         #   Memset(ptr, val, size)
│   │   │
│   │   ├── types.rs              # IR Type system
│   │   ├── value.rs              # IRValue: Const | Reg | Global | Undef
│   │   └── builder.rs            # IRBuilder para construcción del IR
│   │
│   ├── ub_detector/              # 21 UB Types — ÚNICO EN EL MUNDO
│   │   ├── mod.rs                # Orquesta 10 sub-analizadores
│   │   │                         #   detect_all(ir) → Vec<UBReport>
│   │   │                         #   detect_strict(ir) → Result<(), UBReport>
│   │   │                         #   detect_warn(ir) → Vec<UBReport>
│   │   │
│   │   ├── null_check.rs         # NullPointerDereference
│   │   │                         #   check_null_deref(ir) → Vec<UBReport>
│   │   │                         #   track_null_assignments(fn) → NullSet
│   │   │                         #   track_malloc_returns(fn) → MallocSet
│   │   │                         #   track_calloc_returns(fn) → MallocSet
│   │   │                         #   track_realloc_returns(fn) → MallocSet
│   │   │                         #   is_null_checked(ptr, bb) → bool
│   │   │                         #   Detecta: int *p = NULL; *p = 42; ❌
│   │   │                         #   Detecta: int *p = malloc(n); *p = 42; ❌ (sin check)
│   │   │                         #   Permite: if(p) *p = 42; ✓
│   │   │
│   │   ├── bounds_check.rs       # ArrayOutOfBounds
│   │   │                         #   check_array_bounds(ir) → Vec<UBReport>
│   │   │                         #   track_array_sizes(fn) → SizeMap
│   │   │                         #   check_index_range(idx, size) → bool
│   │   │                         #   check_negative_index(idx) → bool
│   │   │                         #   Detecta: arr[10] cuando size=10 ❌
│   │   │                         #   Detecta: arr[-1] ❌
│   │   │                         #   Detecta: memcpy past buffer end ❌
│   │   │                         #   Detecta: strcpy overflow ❌
│   │   │
│   │   ├── overflow_check.rs     # IntegerOverflow/Underflow/DivByZero/ShiftOverflow
│   │   │                         #   check_integer_overflow(ir) → Vec<UBReport>
│   │   │                         #   check_division_by_zero(ir) → Vec<UBReport>
│   │   │                         #   check_shift_overflow(ir) → Vec<UBReport>
│   │   │                         #   check_signed_promotion(ir) → Vec<UBReport>
│   │   │                         #   Detecta: INT_MAX + 1 ❌
│   │   │                         #   Detecta: x / 0 ❌
│   │   │                         #   Detecta: 1 << 32 cuando int=32bit ❌
│   │   │                         #   Detecta: char→int overflow ❌
│   │   │
│   │   ├── uninit_check.rs       # UninitializedVariable (flow-sensitive)
│   │   │                         #   check_uninitialized(ir) → Vec<UBReport>
│   │   │                         #   build_init_sets(fn) → InitMap
│   │   │                         #   check_use_before_def(var, bb) → bool
│   │   │                         #   flow_sensitive_analysis(fn) → DefMap
│   │   │                         #   Detecta: int x; use(x); ❌
│   │   │                         #   Permite: int x = 0; use(x); ✓
│   │   │                         #   Permite: if(cond) x=1; else x=2; use(x); ✓
│   │   │
│   │   ├── useafter_check.rs     # UseAfterFree + DanglingPtr + ReturnLocalAddr
│   │   │                         #   check_use_after_free(ir) → Vec<UBReport>
│   │   │                         #   check_dangling_ptr(ir) → Vec<UBReport>
│   │   │                         #   check_return_local(ir) → Vec<UBReport>
│   │   │                         #   track_free_calls(fn) → FreeSet
│   │   │                         #   track_scope_exits(fn) → ScopeMap
│   │   │                         #   Detecta: free(p); *p = 42; ❌
│   │   │                         #   Detecta: int *p = &local; return p; ❌
│   │   │                         #   Detecta: { int x; ptr = &x; } use(ptr); ❌
│   │   │
│   │   ├── type_check.rs         # TypeConfusion + StrictAliasing + InvalidCast
│   │   │                         #   check_type_confusion(ir) → Vec<UBReport>
│   │   │                         #   check_strict_aliasing(ir) → Vec<UBReport>
│   │   │                         #   check_invalid_cast(ir) → Vec<UBReport>
│   │   │                         #   check_alignment(ir) → Vec<UBReport>
│   │   │                         #   Detecta: (int*)&float_var ❌ (strict aliasing)
│   │   │                         #   Permite: (char*)&any_var ✓ (C99 §6.5/7)
│   │   │                         #   Detecta: Base* → Derived* sin dynamic_cast ❌
│   │   │                         #   Detecta: misaligned pointer cast ❌
│   │   │
│   │   ├── race_check.rs         # StackOverflow (recursión sin base case)
│   │   │                         #   check_stack_overflow(ir) → Vec<UBReport>
│   │   │                         #   detect_infinite_recursion(fn) → bool
│   │   │                         #   build_call_graph(module) → CallGraph
│   │   │                         #   find_recursive_cycles(graph) → Vec<Cycle>
│   │   │                         #   check_base_case_exists(fn) → bool
│   │   │                         #   Detecta: void f() { f(); } ❌
│   │   │                         #   Permite: void f(int n) { if(n>0) f(n-1); } ✓
│   │   │
│   │   ├── unsequenced_check.rs  # UnsequencedModification
│   │   │                         #   check_unsequenced(ir) → Vec<UBReport>
│   │   │                         #   build_sequence_graph(expr) → SeqGraph
│   │   │                         #   track_writes_reads(expr) → WRMap
│   │   │                         #   check_sequence_points(expr) → bool
│   │   │                         #   Detecta: i = i++ + 1 ❌
│   │   │                         #   Detecta: arr[i] = i++ ❌
│   │   │                         #   Detecta: f(i++, i) ❌
│   │   │                         #   Permite: cond ? i++ : i-- ✓ (sequence point)
│   │   │
│   │   ├── lifetime.rs           # DoubleFree + lifetime analysis
│   │   │                         #   check_double_free(ir) → Vec<UBReport>
│   │   │                         #   build_lifetime_map(fn) → LifetimeMap
│   │   │                         #   track_ownership(ptr) → Owner
│   │   │                         #   check_free_count(ptr) → u32
│   │   │                         #   Detecta: free(p); free(p); ❌
│   │   │                         #   Detecta: delete p; delete p; ❌
│   │   │                         #   Permite: free(p); p = NULL; free(p); ✓
│   │   │
│   │   ├── report.rs             # UBReport output
│   │   │                         #   struct UBReport {
│   │   │                         #     kind: UBKind,
│   │   │                         #     file: String,
│   │   │                         #     line: usize,      ← token_start_line preciso
│   │   │                         #     column: usize,
│   │   │                         #     function: String,
│   │   │                         #     message: String,
│   │   │                         #     suggestion: String,
│   │   │                         #   }
│   │   │                         #   format_report(r) → String (human readable)
│   │   │                         #   format_json(r) → String (CI/CD integration)
│   │   │
│   │   ├── cache.rs              # UB results cacheados en fastos.bib
│   │   │                         #   cache_ub_results(file_hash, results)
│   │   │                         #   load_cached_results(file_hash) → Option<Vec<UBReport>>
│   │   │                         #   invalidate_cache(file_hash)
│   │   │
│   │   └── analyzer.rs           # Coordinator general de todos los detectors
│   │                             #   run_all_detectors(ir, mode) → UBResult
│   │                             #   UBMode::Strict → error on first UB
│   │                             #   UBMode::Warn  → collect all, continue
│   │                             #   UBMode::Silent → skip (no recomendado)
│   │
│   ├── analysis/                 # CFG, Dominators, Loops
│   │   ├── cfg.rs                # Control Flow Graph
│   │   │                         #   build_cfg(fn) → CFG
│   │   │                         #   get_predecessors(bb) → Vec<BB>
│   │   │                         #   get_successors(bb) → Vec<BB>
│   │   │
│   │   ├── dominators.rs         # Dominator tree
│   │   │                         #   compute_dominators(cfg) → DomTree
│   │   │                         #   dominates(a, b) → bool
│   │   │                         #   immediate_dominator(bb) → Option<BB>
│   │   │
│   │   └── loops.rs              # Loop detection
│   │                             #   find_loops(cfg, dom) → Vec<Loop>
│   │                             #   Loop { header, body, exit }
│   │
│   ├── lowering/                 # AST → IR lowering
│   └── passes/                   # Optimization passes (LLVM-style)
│       ├── mem2reg.rs            # Stack → SSA registers
│       ├── dce.rs                # Dead code elimination pass
│       ├── instcombine.rs        # Instruction combining
│       └── simplifycfg.rs        # CFG simplification
│
├── optimizer/                    # Multi-level optimizations
│   ├── mod.rs
│   ├── dead_code.rs              # Dead code elimination
│   │                             #   eliminate_dead_code(ir) → IR
│   │                             #   mark_live_instructions(ir) → LiveSet
│   │                             #   sweep_dead(ir, live) → IR
│   │
│   ├── const_fold.rs             # Constant folding
│   │                             #   fold_constants(ir) → IR
│   │                             #   eval_binop(op, lhs, rhs) → Option<Const>
│   │                             #   fold_if_const(cond) → Option<Branch>
│   │
│   ├── const_prop.rs             # Constant propagation
│   │                             #   propagate_constants(ir) → IR
│   │                             #   build_const_map(fn) → ConstMap
│   │                             #   replace_uses_with_const(var, val, ir) → IR
│   │
│   ├── redundant.rs              # Redundant ops removal
│   │                             #   remove_redundant(ir) → IR
│   │                             #   find_redundant_loads(ir) → Vec<Instr>
│   │                             #   find_redundant_stores(ir) → Vec<Instr>
│   │
│   ├── inline_exp.rs             # Inline expansion
│   │                             #   inline_small_functions(ir) → IR
│   │                             #   should_inline(fn) → bool (size heuristic)
│   │                             #   do_inline(call_site, fn) → IR
│   │
│   ├── binary_optimizer.rs       # Binary-level size optimization
│   ├── branch_detector.rs        # Branch pattern detection
│   ├── branchless.rs             # Branchless transforms (cmov)
│   └── simd.rs                   # Auto-vectorization SIMD
│
├── isa/                          # ISA Layer — x86-64 completo
│   ├── mod.rs
│   ├── c_isa.rs                  # C99: sizeof/alignment rules
│   │                             #   sizeof_type(t) → usize
│   │                             #   align_of_type(t) → usize
│   │                             #   layout_struct(s) → StructLayout
│   │                             #   ABI: System V AMD64 (Linux) + Windows x64
│   │
│   ├── cpp_isa.rs                # C++98: vtable/this/constructors
│   │                             #   generate_vtable(cls) → VTable
│   │                             #   generate_ctor(cls) → IRFunction
│   │                             #   generate_dtor(cls) → IRFunction
│   │                             #   this_pointer_convention() → Reg
│   │                             #   vtable_dispatch(obj, method_idx) → Call
│   │
│   ├── isa_compiler.rs           # Main ISA compiler
│   │                             #   compile_function(fn) → Vec<ADeadOp>
│   │                             #   compile_basic_block(bb) → Vec<ADeadOp>
│   │                             #   compile_instruction(instr) → Vec<ADeadOp>
│   │
│   ├── encoder.rs                # ADeadOp → bytes x86-64 directos
│   │                             #   encode_instruction(op) → Vec<u8>
│   │                             #   encode_mov(dst, src) → Vec<u8>
│   │                             #   encode_add/sub/mul/div → Vec<u8>
│   │                             #   encode_cmp/jmp/jcc → Vec<u8>
│   │                             #   encode_call/ret → Vec<u8>
│   │                             #   encode_push/pop → Vec<u8>
│   │                             #   encode_lea → Vec<u8>
│   │                             #   encode_xmm (SSE2) → Vec<u8>
│   │                             #   REX prefix automático
│   │                             #   ModRM/SIB encoding automático
│   │
│   ├── decoder.rs                # bytes → ADeadOp (disassembler)
│   ├── reg_alloc.rs              # Dual register allocator
│   │                             #   TempAllocator:
│   │                             #     13 registros: RBX,RCX,RDX,RSI,RDI,R8-R15
│   │                             #     5 callee-saved: RBX,R12,R13,R14,R15
│   │                             #     8 caller-saved: RCX,RDX,RSI,RDI,R8,R9,R10,R11
│   │                             #     Windows x64 args: RCX,RDX,R8,R9
│   │                             #     Linux x64 args: RDI,RSI,RDX,RCX,R8,R9
│   │                             #     Spill a stack cuando se agotan
│   │                             #
│   │                             #   LinearScanAllocator:
│   │                             #     compute_liveness(fn) → LiveIntervals
│   │                             #     allocate_registers(intervals) → RegMap
│   │                             #     spill_furthest(active) → SpilledReg
│   │                             #     stack_alignment: 16 bytes automático
│   │                             #     métricas: spill_slots_used, spill_stack_size
│   │
│   ├── optimizer.rs              # ISA-level peephole opts
│   └── compiler/                 # Modular compilation stages
│
├── cache/                        # fastos.bib System v2
│   ├── mod.rs                    # ADeadCache struct
│   │                             #   struct ADeadCache {
│   │                             #     header: CacheHeader,
│   │                             #     ast_data: Vec<u8>,
│   │                             #     type_table: TypeTable,
│   │                             #     symbol_table: SymbolTable,
│   │                             #     ub_reports: Vec<UBReport>,
│   │                             #   }
│   │
│   ├── serializer.rs             # Cache → bytes
│   │                             #   serialize(cache) → Vec<u8>
│   │                             #   serialize_type_table(t) → Vec<u8>
│   │                             #   serialize_symbol_table(s) → Vec<u8>
│   │                             #   serialize_ub_reports(u) → Vec<u8>
│   │
│   ├── deserializer.rs           # bytes → Cache (roundtrip completo v2)
│   │                             #   deserialize(bytes) → Result<ADeadCache>
│   │                             #   verify_magic(bytes) → bool
│   │                             #   verify_version(bytes) → bool
│   │
│   ├── hasher.rs                 # FNV-1a header hashing
│   │                             #   hash_source(content) → u64
│   │                             #   hash_header(path) → u64
│   │                             #   FNV_PRIME = 1099511628211u64
│   │                             #   FNV_OFFSET = 14695981039346656037u64
│   │
│   └── validator.rs              # Cache hit/stale/miss/corrupt
│                                 #   validate(path, source_hash) → CacheStatus
│                                 #   CacheStatus::Hit → use cached
│                                 #   CacheStatus::Stale → recompile
│                                 #   CacheStatus::Miss → first compile
│                                 #   CacheStatus::Corrupt → delete + recompile
│
├── output/                       # Binary Output — SIN LINKER EXTERNO
│   ├── mod.rs                    # OutputFormat enum
│   │                             #   OutputFormat::PE      → Windows .exe
│   │                             #   OutputFormat::ELF     → Linux binary
│   │                             #   OutputFormat::FastOS  → .po binary
│   │                             #   OutputFormat::All     → los 3
│   │                             #
│   │                             # FILOSOFÍA OUTPUT:
│   │                             #   ADead-BIB genera binario DIRECTAMENTE
│   │                             #   SIN pasar por linker externo — NUNCA
│   │                             #   SIN .o intermedios — NUNCA
│   │                             #   SIN ld/lld/link.exe — NUNCA
│   │                             #   source → encode → output → DONE
│   │
│   ├── pe.rs                     # Windows PE (.exe) generator
│   │                             #   generate_pe(code, data) → Vec<u8>
│   │                             #   PE Header (IMAGE_DOS_HEADER):
│   │                             #     e_magic: 0x4D5A ("MZ")
│   │                             #     e_lfanew: offset to PE header
│   │                             #   PE Signature: "PE\0\0"
│   │                             #   IMAGE_FILE_HEADER:
│   │                             #     Machine: 0x8664 (AMD64)
│   │                             #     NumberOfSections
│   │                             #     Characteristics
│   │                             #   IMAGE_OPTIONAL_HEADER64:
│   │                             #     Magic: 0x20B (PE32+)
│   │                             #     AddressOfEntryPoint
│   │                             #     ImageBase: 0x400000 default
│   │                             #     SectionAlignment: 0x1000
│   │                             #     FileAlignment: 0x200
│   │                             #   Sections: .text, .data, .rdata, .bss
│   │                             #   Import Table (si hay DLLs externas opcionales)
│   │                             #   Relocations (si necesario)
│   │                             #   Genera PE VÁLIDO sin link.exe ✓
│   │
│   ├── elf.rs                    # Linux ELF generator
│   │                             #   generate_elf(code, data) → Vec<u8>
│   │                             #   ELF Header (Elf64_Ehdr):
│   │                             #     e_ident: magic + class + data + version
│   │                             #     e_type: ET_EXEC (executable)
│   │                             #     e_machine: EM_X86_64 (0x3E)
│   │                             #     e_entry: entry point virtual address
│   │                             #     e_phoff: program header offset
│   │                             #     e_shoff: section header offset
│   │                             #   Program Headers (PT_LOAD):
│   │                             #     LOAD .text: R+X segment
│   │                             #     LOAD .data: R+W segment
│   │                             #   Section Headers: .text, .data, .bss, .strtab
│   │                             #   Genera ELF VÁLIDO sin ld ✓
│   │                             #   chmod +x automático en output
│   │
│   └── po.rs                     # FastOS .po generator (nativo FastOS)
│                                 #   generate_po(code, data) → Vec<u8>
│                                 #   .Po Header (24 bytes):
│                                 #     magic:      "ADEAD.PO" (8 bytes)
│                                 #     version:    u16 (2 bytes)
│                                 #     flags:      u16 (2 bytes)
│                                 #     entry:      u32 (4 bytes) — entry point offset
│                                 #     code_size:  u32 (4 bytes)
│                                 #     data_size:  u32 (4 bytes)
│                                 #   Body:
│                                 #     code section (x86-64 bytes)
│                                 #     data section
│                                 #   Ultra minimal — 24 bytes overhead total
│                                 #   FastOS carga directo en memoria
│                                 #   Sin relocations complejas
│                                 #   Sin import tables
│                                 #   Sin secciones innecesarias
│
├── bg/                           # Binary Guardian (security)
├── runtime/                      # CPU/GPU detection + dispatch
└── toolchain/                    # GCC/LLVM/MSVC compatibility layer
    ├── mod.rs
    ├── gcc_compat.rs             # Emula flags GCC comunes
    │                             #   -O0/-O1/-O2/-O3 → optimizer levels
    │                             #   -Wall → enable all warnings
    │                             #   -g → debug info
    │                             #   -std=c99 / -std=c++11 / etc.
    │                             #   -I path → include path (fastos handles)
    │                             #   -D MACRO → predefined macros
    │
    ├── msvc_compat.rs            # Emula flags MSVC comunes
    │                             #   /W4 → warning level
    │                             #   /O2 → optimization
    │                             #   /MT /MD → runtime (fastos ignora)
    │
    └── clang_compat.rs           # Emula flags Clang comunes
```

---

## header_main.h — Header Universal v7.0

```c
/*
 * header_main.h — ADead-BIB Universal Header
 * Un solo include. Todo disponible. Sin linker.
 * 💀🦈 🇵🇪
 */
#ifndef _ADEAD_HEADER_MAIN
#define _ADEAD_HEADER_MAIN

/* ── C99 Standard Library COMPLETA ── */
#include <fastos_types.h>     /* int8_t, uint64_t, size_t, bool, NULL */
#include <fastos_limits.h>    /* INT_MAX, INT_MIN, SIZE_MAX, CHAR_MAX */
#include <fastos_stdio.h>     /* printf, scanf, fopen, fclose, fread, fwrite */
#include <fastos_stdlib.h>    /* malloc, free, calloc, realloc, exit, qsort */
#include <fastos_string.h>    /* strlen, strcpy, strcat, strcmp, memcpy, memset */
#include <fastos_math.h>      /* sin, cos, sqrt, pow, log, PI, E, TAU */
#include <fastos_time.h>      /* time, clock, sleep, gettimeofday */
#include <fastos_assert.h>    /* assert, static_assert */
#include <fastos_errno.h>     /* errno, strerror, ENOENT, ENOMEM */

/* ── C++ Standard Library COMPLETA (solo en C++ mode) ── */
#ifdef __cplusplus
#include <fastos_iostream>    /* std::cout, std::cin, std::cerr, std::endl */
#include <fastos_vector>      /* std::vector<T> */
#include <fastos_string_cpp>  /* std::string */
#include <fastos_map>         /* std::map<K,V> */
#include <fastos_memory>      /* std::unique_ptr, std::shared_ptr, make_unique */
#include <fastos_algorithm>   /* std::sort, std::find, std::copy */
#include <fastos_functional>  /* std::function, std::bind */
#include <fastos_utility>     /* std::pair, std::move, std::forward, make_pair */
#include <fastos_exception>   /* std::exception, std::runtime_error */
#endif

/* ── FastOS Extensions (multiplataforma) ── */
#include <fastos_thread.h>    /* threads, mutex, condition_variable */
#include <fastos_fs.h>        /* filesystem: read_file, write_file, list_dir */
#include <fastos_net.h>       /* sockets: connect, listen, send, recv */
#include <fastos_window.h>    /* ventanas: create_window, handle_events */
#include <fastos_graphics.h>  /* DirectX12/Vulkan abstraction layer */

/*
 * TREE SHAKING AUTOMÁTICO:
 * ADead-BIB incluye en el binario SOLO lo que usas.
 * Si usas solo printf → solo printf en el binario.
 * Dead Code Elimination garantiza 0 overhead.
 * Hello World con este header → 2KB binario.
 */

#endif /* _ADEAD_HEADER_MAIN */
```

---

## Linker Eliminator — Filosofía y Implementación

```
EL PROBLEMA QUE ELIMINAMOS:

GCC/LLVM/MSVC hoy:
  archivo.c  → compilar → archivo.o
  archivo2.c → compilar → archivo2.o
  archivo3.c → compilar → archivo3.o
  LINKER (programa separado):
    ld/lld/link.exe junta los .o
    busca símbolos no resueltos
    carga librerías externas (.lib/.a/.so)
    genera ejecutable final
  
  Errores clásicos del linker:
    "undefined reference to `printf`" ← falta -lc
    "undefined reference to `sin`"    ← falta -lm
    "multiple definition of `global`" ← duplicado
    "cannot find -lstdc++"            ← falta lib
    "LNK1104: cannot open file"       ← MSVC
  
  = programa separado ❌
  = proceso separado ❌
  = falla por separado ❌
  = flags misteriosos ❌
  = Stack Overflow obligatorio ❌

ADEAD-BIB v7.0:
  archivo.c  ─┐
  archivo2.c ─┤→ unity build → IR único → binario
  archivo3.c ─┘
  
  Sin .o intermedios ✓
  Sin linker externo ✓
  Sin flags de librería ✓
  Sin "undefined reference" ✓
  Resolución de símbolos INTERNA ✓
  
  adeadc cc main.cpp -o output
  = UN comando ✓
  = UN paso ✓
  = UN binario ✓
  = cero dolor ✓

CÓMO FUNCIONA INTERNAMENTE:
  1. preprocessor/resolver.rs lee TODOS los archivos
  2. Construye SymbolTable global unificada
  3. Deduplica símbolos (dedup.rs)
  4. Genera IR unificado (unity build)
  5. UB_Detector analiza IR completo
  6. Optimizer trabaja sobre IR completo
  7. Encoder genera bytes directos
  8. output/pe.rs / elf.rs / po.rs genera binario
  
  = linker nunca invocado
  = linker nunca necesario
  = los headers tienen toda la información
  = ADead-BIB genera la implementación
  = el conocimiento es suficiente
  = "._."
```

---

## C99 Canon — Features Completas

```
C99 DEBE COMPILAR (ISO/IEC 9899:1999):

TIPOS BÁSICOS:
  char                    signed char         unsigned char
  short                   unsigned short
  int                     unsigned int
  long                    unsigned long
  long long               unsigned long long
  float                   double              long double
  _Bool                   (bool via stdbool.h)
  void                    void*
  
TIPOS DERIVADOS:
  int *ptr                puntero simple
  int **dptr              doble puntero
  int arr[N]              array estático
  int vla[n]              VLA (C99 §6.7.5.2) ← IMPORTANTE
  struct { ... }          estructura
  union { ... }           unión
  enum { ... }            enumeración
  int (*fn)(int)          function pointer
  struct { int arr[]; }   flexible array member ← C99 específico

QUALIFIERS:
  const    volatile    restrict    ← restrict es C99 NUEVO

STORAGE CLASS:
  auto    register    static    extern

LITERALES:
  123       0x1F      0777      (int)
  123L      123UL     123LL     123ULL
  3.14f     3.14      3.14L
  'a'       '\n'      '\0'      '\x41'
  "hello"   L"wide"
  (C99) compound literal: (Type){init}
  (C99) designated: .field = val, [idx] = val

EXPRESIONES (orden de precedencia):
  postfix:   () [] -> . ++ --
  unary:     + - ~ ! * & ++ -- sizeof (cast)
  binary:    * / %  + -  << >>  < > <= >=  == !=  &  ^  |  &&  ||
  ternary:   cond ? a : b
  assign:    = += -= *= /= %= &= |= ^= <<= >>=
  comma:     expr1, expr2

CONTROL FLOW:
  if (cond) stmt
  if (cond) stmt else stmt
  while (cond) stmt
  do stmt while (cond);
  for (init; cond; step) stmt
  for (int i=0; ...) ← C99: declaración en for
  switch (expr) { case N: ... default: ... }
  break    continue    return expr    goto label

FUNCIONES:
  int fn(int a, int b) { ... }
  int fn(void) { ... }          ← void explícito
  static int fn(...)            ← file scope
  inline int fn(...)            ← C99 §6.7.4
  int fn(int n, ...) { ... }    ← variadic
  va_list va_start va_arg va_end va_copy

PREPROCESADOR C99:
  #include <system.h>
  #include "local.h"
  #define MACRO value
  #define MACRO(x) (x*x)
  #define MACRO(x, ...) fn(x, __VA_ARGS__)  ← C99 variadic macro
  #undef MACRO
  #ifdef / #ifndef / #if / #elif / #else / #endif
  #pragma once
  #error "message"
  #line N "file"
  __FILE__ __LINE__ __DATE__ __TIME__ __func__
  Stringification: #x → "x"
  Token pasting: a##b → ab
```

---

## C++98 Canon — Features Completas

```
C++98 DEBE COMPILAR (ISO/IEC 14882:1998):

TODO LO DE C99 MÁS:

CLASES:
  class ClassName {
  public:
      ClassName();                    // default ctor
      ClassName(int x);               // param ctor
      ClassName(const ClassName& o);  // copy ctor
      ClassName& operator=(const ClassName& o); // copy assign
      ~ClassName();                   // dtor
      void method();
      void const_method() const;
      static void static_method();
      static int static_member;
      mutable int cache_value;
  private:
      int data;
  protected:
      int shared;
  friend class OtherClass;
  friend void free_function();
  };

HERENCIA:
  class Derived : public Base { ... }
  class Multi : public A, public B { ... }
  class Virtual : virtual public Base { ... }
  Derived::Derived() : Base(args), member(val) { }

POLIMORFISMO:
  virtual void fn();
  virtual void fn() = 0;    ← pure virtual
  virtual ~Base();          ← virtual dtor OBLIGATORIO
  vtable generada automáticamente
  dynamic_cast<Derived*>(base_ptr)
  static_cast<int>(double_val)
  reinterpret_cast<void*>(int_val)
  const_cast<int*>(const_ptr)
  typeid(expr).name()

TEMPLATES:
  template<typename T>
  T max(T a, T b) { return a > b ? a : b; }
  
  template<typename T>
  class Stack {
      T data[100];
      int top;
  public:
      void push(T val);
      T pop();
  };
  
  template<>                        ← specialization
  class Stack<bool> { ... };
  
  template<typename T, int N>       ← non-type param
  class Array { T data[N]; };
  
  template<typename... Args>        ← variadic (C++11 via expander)
  void print(Args... args);
  
  template<template<typename> class C>  ← template template
  void fn(C<int>& container);

OPERATOR OVERLOADING:
  Type operator+(const Type& b) const;
  Type operator[](int i);
  Type& operator=(const Type& b);
  bool operator==(const Type& b) const;
  bool operator<(const Type& b) const;
  Type& operator++();              ← prefix
  Type operator++(int);           ← postfix
  ostream& operator<<(ostream& os, const Type& t);
  void* operator new(size_t size);
  void operator delete(void* ptr);
  explicit operator bool() const;

NAMESPACES:
  namespace mylib { ... }
  using namespace mylib;
  using mylib::function;
  mylib::ClassName obj;
  namespace nested { namespace inner { ... } }
  namespace { ... }               ← anonymous

REFERENCES:
  int& ref = var;
  const int& cref = 42;
  void fn(int& out);
  int& fn();                      ← return by ref

MEMORY:
  int* p = new int(42);
  int* arr = new int[100];
  delete p;
  delete[] arr;
  new(buf) Type(args);            ← placement new

EXCEPTIONS:
  throw std::runtime_error("msg");
  try {
      risky();
  } catch(const std::exception& e) {
      handle(e.what());
  } catch(...) {
      handle_all();
  }
```

---

## C++17 → C++98 Canon (34 Features via expander.rs)

```
MacroExpander convierte syntax moderno a C++98 internamente.
El parser solo necesita entender C++98. Zero overhead.

C++11 — 12 features:
  lambda:               [cap](args) → body
                        → struct __lambda { auto operator()(args) { body } };
  range-for:            for(auto x : container)
                        → for(auto it=begin(c); it!=end(c); ++it)
  auto:                 auto x = expr
                        → tipo inferido en compilación
  nullptr:              nullptr
                        → (void*)0 con type safety
  static_assert:        static_assert(cond, "msg")
                        → compiletime check
  enum class:           enum class Color { Red }
                        → enum con scope
  using alias:          using Vec = vector<int>
                        → typedef vector<int> Vec
  variadic templates:   template<typename... T>
                        → recursión de templates C++98
  constexpr:            constexpr int fn()
                        → evaluado en compilación
  move semantics:       T&&, std::move, std::forward
                        → rvalue references
  initializer_list:     {1, 2, 3} initialization
                        → std::initializer_list<T>
  delegating ctors:     ClassName() : ClassName(0) {}
                        → llamar otro ctor

C++14 — 6 features:
  generic lambda:       [](auto x) { return x; }
                        → template lambda
  [[deprecated]]:       [[deprecated("use new_fn")]]
                        → warning en compilación
  binary literals:      0b1010
                        → decimal equivalent
  digit separators:     1'000'000
                        → 1000000
  return type deduction: auto fn() { return 42; }
                        → tipo deducido
  make_unique:          make_unique<T>(args)
                        → unique_ptr<T>(new T(args))

C++17 — 16 features:
  structured bindings:  auto [x, y] = pair;
                        → auto tmp=pair; auto x=tmp.first; auto y=tmp.second;
  if constexpr:         if constexpr (cond)
                        → compiletime branch selection
  std::optional<T>:     optional<int> val = 42;
                        → wrapper con has_value()/value()
  std::variant<T...>:   variant<int,string> v = 42;
                        → tagged union type-safe
  std::string_view:     string_view sv = "hello";
                        → non-owning string reference
  std::any:             any a = 42;
                        → type-erased value
  fold expressions:     (args + ...)
                        → expansión de variadic templates
  [[nodiscard]]:        [[nodiscard]] int fn()
                        → warning si resultado ignorado
  [[maybe_unused]]:     [[maybe_unused]] int x
                        → suprime warning de no-uso
  [[fallthrough]]:      [[fallthrough]];
                        → intencional en switch
  nested namespaces:    namespace A::B::C {}
                        → namespace A { namespace B { namespace C {} } }
  inline variables:     inline int global = 42;
                        → ODR-safe global
  constexpr if:         if constexpr(expr)
                        → alias de if constexpr
  type traits:          is_same_v<T,U>, is_integral_v<T>
                        → template metaprogramming helpers
  std::filesystem:      fs::path, fs::exists()
                        → fastos_fs.h wrapper
  class template deduction: vector v = {1,2,3};
                        → CTAD → vector<int> v

Total: 34 features expandidas a C++98 canon puro ✓
```

---

## Los 21 Tipos de UB (ÚNICO EN EL MUNDO)

```rust
pub enum UBKind {
    // ── Memoria ──
    NullPointerDereference,     // ptr usado sin check NULL (+ malloc tracking)
    UseAfterFree,               // ptr usado después de free()
    DoubleFree,                 // free() llamado dos veces
    DanglingPointer,            // ptr a stack variable fuera de scope
    ReturnLocalAddress,         // return &local_var (dangling on return)
    BufferOverflow,             // write past buffer end (memcpy, strcpy)

    // ── Aritmética ──
    ArrayOutOfBounds,           // index >= size (+ negative index)
    IntegerOverflow,            // signed int overflow [C99 §6.5.5]
    IntegerUnderflow,           // signed int underflow
    DivisionByZero,             // division por cero [C99 §6.5.5]
    ShiftOverflow,              // shift >= sizeof(tipo) * 8 [C99 §6.5.7]
    SignedOverflowPromotion,    // char→int promotion causes overflow

    // ── Tipos ──
    UninitializedVariable,      // variable usada sin inicializar
    TypeConfusion,              // cast inválido entre tipos
    InvalidCast,                // downcast sin verificar
    StrictAliasingViolation,    // type punning via pointer cast [C99 §6.5/7]
    AlignmentViolation,         // misaligned pointer cast

    // ── Concurrencia ──
    DataRace,                   // acceso concurrente sin sync
    UnsequencedModification,    // i = i++ (orden no definido) [C99 §6.5/2]
    StackOverflow,              // recursión infinita

    // ── Formato ──
    FormatStringMismatch,       // printf("%d", float_var)
}
```

---

## Modos de Operación

```
MODO ESTRICTO (default):
  adeadc cc archivo.c -o output
  UB encontrado → SE DETIENE
  muestra reporte con línea exacta (token_start_line)
  Ctrl+Click en IDE → línea exacta del UB
  developer arregla ANTES de continuar

MODO ADVERTENCIA:
  adeadc cc archivo.c --warn-ub -o output
  UB encontrado → AVISA Y CONTINUA
  genera reporte completo
  binario generado igualmente
  tu responsabilidad

MODO TODOS LOS TARGETS:
  adeadc cc main.cpp --target all
  genera: output.exe + output.elf + output.po
  mismo código fuente
  tres binarios nativos
  sin linker en ninguno
```

---

## Orden Crítico del Pipeline — Por qué Importa

```
IR → UB_Detector → Optimizer → Reg_Allocator → Encoder
         ↑
   ANTES de optimizar = cobertura total garantizada

POR QUÉ ANTES:
  El optimizer puede eliminar código "muerto"
  que contiene UB → UB desaparece del IR
  → UB_Detector no lo ve
  → UB silencioso llega a producción
  = exactamente lo que hace GCC ❌
  = exactamente lo que hace LLVM ❌

  ADead-BIB: UB_Detector VE el código completo
  antes de cualquier transformación
  = cobertura 100% garantizada ✓
  = ningún UB puede escapar por optimización ✓
  = ÚNICO compilador que hace esto ✓
```

---

## Register Allocator — Dual Mode

```
TempAllocator (v1 — fast path):
  13 registros físicos: RBX,RCX,RDX,RSI,RDI,R8,R9,R10,R11,R12,R13,R14,R15
  5 callee-saved: RBX, R12, R13, R14, R15
  8 caller-saved: RCX, RDX, RSI, RDI, R8, R9, R10, R11
  Windows x64 calling convention args: RCX, RDX, R8, R9
  Linux System V AMD64 args: RDI, RSI, RDX, RCX, R8, R9
  Spill a stack cuando se agotan registros
  Rápido para funciones pequeñas

LinearScanAllocator (v2 — liveness):
  compute_liveness(fn) → LiveIntervals por variable
  Intervalo [def_point, last_use_point]
  allocate_registers(intervals) → RegMap
  spill_furthest(active_set) → spill el que termina más tarde
  Stack alignment 16 bytes automático (x64 ABI)
  Métricas: spill_slots_used, spill_stack_size

StackFrame Calculator:
  Calcula tamaño real de frame (no fijo 128)
  Alignment natural por tipo:
    char:   1 byte
    short:  2 bytes
    int:    4 bytes
    long:   8 bytes
    double: 8 bytes
    __m128: 16 bytes (SSE)
  Total aligned a 16 bytes (x64 ABI obligatorio)
```

---

## Cache fastos.bib v2

```
Header (28 bytes):
  magic:     "ADEAD.BI" (8 bytes)
  version:   u32 (4 bytes) — v2
  timestamp: u64 (8 bytes)
  hash:      u64 (8 bytes) — FNV-1a del source

Body (length-prefixed):
  ast_len:       u32
  ast_data:      [u8; ast_len]       ← serialized AST
  type_count:    u32
  type_entries:  [(kind:u8, size:u32, align:u32); type_count]
  symbol_count:  u32
  symbol_entries:[(kind:u8, name_len:u32, name:[u8]); symbol_count]
  ub_count:      u32
  ub_reports:    [UBReport; ub_count] ← cached UB results

Validación:
  Hit    → hash matches → usa cache (nanosegundos)
  Stale  → hash changed → recompila → nuevo cache
  Miss   → primera vez  → compila → crea cache
  Corrupt→ bad magic    → elimina → recompila

FNV-1a Hash:
  FNV_PRIME  = 1099511628211u64
  FNV_OFFSET = 14695981039346656037u64
  hash = FNV_OFFSET
  for byte in content:
    hash = hash XOR byte
    hash = hash * FNV_PRIME
```

---

## Comandos CLI

```bash
# Compilar C
adeadc cc archivo.c -o output

# Compilar C++
adeadc cxx archivo.cpp -o output

# Modo advertencia UB
adeadc cc archivo.c --warn-ub -o output

# Target específico
adeadc cc archivo.c --target windows -o output.exe
adeadc cc archivo.c --target linux   -o output
adeadc cc archivo.c --target fastos  -o output.po

# Todos los targets simultáneamente
adeadc cc archivo.c --target all -o output

# Build (múltiples archivos — unity build interno)
adeadc build src/*.c -o output

# Run (compila y ejecuta)
adeadc run archivo.c

# Con header_main.h (todo disponible)
# Solo escribe: #include <header_main.h>
# ADead-BIB resuelve TODO internamente
# SIN -lm SIN -lstdc++ SIN NADA
```

---

## Comparación Final

```
                    GCC      LLVM/Clang   MSVC     ADead-BIB v7
─────────────────────────────────────────────────────────────────
Tamaño instalación  200MB    500MB        30GB     2MB ✓
Linker externo      SÍ ❌    SÍ ❌        SÍ ❌    NO ✓
UB detection        NO ❌    parcial ❌   NO ❌    21 tipos ✓
header_main.h       NO ❌    NO ❌        NO ❌    SÍ ✓
Tree shaking auto   NO ❌    parcial ❌   NO ❌    SÍ ✓
Un comando          NO ❌    NO ❌        NO ❌    SÍ ✓
Sin flags           NO ❌    NO ❌        NO ❌    SÍ ✓
UB antes optimizer  NO ❌    NO ❌        NO ❌    SÍ ✓ ÚNICO
C99 completo        SÍ ✓    SÍ ✓        parcial  SÍ ✓
C++98 completo      SÍ ✓    SÍ ✓        SÍ ✓     SÍ ✓
C++17               SÍ ✓    SÍ ✓        SÍ ✓     34 features ✓
Hello World size    50KB     40KB         60KB     2KB ✓
Cross-platform      pain     pain         NO       SÍ ✓
FastOS .po          NO ❌    NO ❌        NO ❌    SÍ ✓ ÚNICO
─────────────────────────────────────────────────────────────────
Filosofía:          ninguna  ninguna      negocio  Grace Hopper ✓
                                                   Dennis Ritchie ✓
                                                   💀🦈 🇵🇪
```

---

*ADead-BIB v7.0 — 2026 — 💀🦈 🇵🇪*
*"la máquina sirve al humano — sin linker — sin UB silencioso — para siempre"*
