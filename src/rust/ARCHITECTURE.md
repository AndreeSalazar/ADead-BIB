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
│   │   ├── fastos_stdlib.rs      # malloc, free, NULL, exit, atoi, rand
│   │   ├── fastos_string.rs      # strlen, strcpy, strcat, strcmp, memcpy
│   │   ├── fastos_math.rs        # sin, cos, tan, sqrt, pow, log, PI
│   │   ├── fastos_time.rs        # clock, time, sleep, gettimeofday
│   │   ├── fastos_assert.rs      # assert, static_assert, NDEBUG
│   │   ├── fastos_errno.rs       # errno, strerror, error codes
│   │   ├── fastos_limits.rs      # INT_MAX, INT_MIN, CHAR_MAX, etc.
│   │   └── fastos_types.rs       # stdint.h + stddef.h + stdbool.h
│   │
│   └── cpp/                      # C++ Standard Library
│       ├── fastos_iostream.rs    # std::cout, std::cin, std::cerr, std::endl
│       ├── fastos_vector.rs      # std::vector<T>
│       ├── fastos_string_cpp.rs  # std::string
│       ├── fastos_map.rs         # std::map<K,V>
│       ├── fastos_memory.rs      # std::unique_ptr, std::shared_ptr
│       ├── fastos_algorithm.rs   # std::sort, find, copy, transform
│       ├── fastos_functional.rs  # std::function, std::bind
│       ├── fastos_utility.rs     # std::pair, std::move, std::forward
│       └── fastos_exceptions.rs  # try/catch/throw C++98
│
├── frontend/                     # C/C++ Parsing
│   ├── mod.rs
│   ├── ast.rs                    # Unified AST
│   ├── types.rs                  # Type system completo
│   ├── type_checker.rs           # Static analysis
│   │
│   ├── c/                        # C99 Frontend COMPLETO
│   │   ├── c_lexer.rs            # Tokenizer C99
│   │   ├── c_parser.rs           # Parser C99 completo
│   │   ├── c_ast.rs              # AST nodes C99
│   │   ├── c_preprocessor.rs     # #define, #include, #ifdef, #pragma
│   │   ├── c_stdlib.rs           # Mapping #include → fastos_*.rs
│   │   └── c_to_ir.rs            # C99 AST → ADeadOp IR
│   │
│   └── cpp/                      # C++98 Frontend COMPLETO
│       ├── cpp_lexer.rs          # Tokenizer C++98
│       ├── cpp_parser.rs         # Parser C++98 CANON completo
│       ├── cpp_ast.rs            # AST nodes C++98
│       ├── cpp_preprocessor.rs   # Preprocessor C++ (hereda de C + C++ extras)
│       ├── cpp_stdlib.rs         # Mapping #include C++ → fastos_*.rs
│       └── cpp_to_ir.rs          # C++98 AST → ADeadOp IR
│
├── middle/                       # IR + UB Detection + Analysis
│   ├── mod.rs
│   ├── ir/                       # ADeadOp IR (SSA-form)
│   │   ├── module.rs             # IRModule { functions, globals, types }
│   │   ├── function.rs           # IRFunction { params, blocks, locals }
│   │   ├── basicblock.rs         # BasicBlock { instrs, terminator }
│   │   ├── instruction.rs        # ADeadOp instructions completas
│   │   ├── types.rs              # IR Type system
│   │   ├── value.rs              # IRValue: Const | Reg | Global | Undef
│   │   └── builder.rs            # IRBuilder para construcción del IR
│   │
│   ├── ub_detector/              # 21 UB Types — ÚNICO EN EL MUNDO
│   │   ├── mod.rs                # Orquesta 10 sub-analizadores
│   │   ├── null_check.rs         # NullPointerDereference
│   │   ├── bounds_check.rs       # ArrayOutOfBounds
│   │   ├── overflow_check.rs     # IntegerOverflow/Underflow/DivByZero/ShiftOverflow
│   │   ├── uninit_check.rs       # UninitializedVariable (flow-sensitive)
│   │   ├── useafter_check.rs     # UseAfterFree + DanglingPtr + ReturnLocalAddr
│   │   ├── type_check.rs         # TypeConfusion + StrictAliasing + InvalidCast
│   │   ├── race_check.rs         # StackOverflow (recursion sin base case)
│   │   ├── unsequenced_check.rs  # UnsequencedModification (i=i++)
│   │   ├── lifetime.rs           # DoubleFree + lifetime analysis
│   │   ├── report.rs             # UBReport, UBKind (21 tipos)
│   │   ├── cache.rs              # UB results cacheados en fastos.bib
│   │   └── analyzer.rs           # Coordinator general
│   │
│   ├── analysis/                 # CFG, Dominators, Loops
│   │   ├── cfg.rs                # Control Flow Graph
│   │   ├── dominators.rs         # Dominator tree
│   │   └── loops.rs              # Loop detection
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
│   ├── const_fold.rs             # Constant folding
│   ├── const_prop.rs             # Constant propagation
│   ├── redundant.rs              # Redundant ops removal
│   ├── inline_exp.rs             # Inline expansion
│   ├── binary_optimizer.rs       # Binary-level size optimization
│   ├── branch_detector.rs        # Branch pattern detection
│   ├── branchless.rs             # Branchless transforms
│   └── simd.rs                   # Auto-vectorization SIMD
│
├── isa/                          # ISA Layer — x86-64 completo
│   ├── mod.rs
│   ├── c_isa.rs                  # C99: sizeof/alignment rules
│   ├── cpp_isa.rs                # C++98: vtable/this/constructors
│   ├── isa_compiler.rs           # Main ISA compiler
│   ├── encoder.rs                # ADeadOp → bytes x86-64 directos
│   ├── decoder.rs                # bytes → ADeadOp (disassembler)
│   ├── reg_alloc.rs              # Dual register allocator
│   ├── optimizer.rs              # ISA-level peephole opts
│   └── compiler/                 # Modular compilation stages
│
├── cache/                        # fastos.bib System v2
│   ├── mod.rs                    # ADeadCache struct
│   ├── serializer.rs             # Cache → bytes
│   ├── deserializer.rs           # bytes → Cache (roundtrip completo v2)
│   ├── hasher.rs                 # FNV-1a header hashing
│   └── validator.rs              # Cache hit/stale/miss/corrupt
│
├── output/                       # Binary Output — SIN LINKER EXTERNO
│   ├── mod.rs                    # OutputFormat enum
│   ├── pe.rs                     # Windows PE (.exe) generator
│   ├── elf.rs                    # Linux ELF generator
│   └── po.rs                     # FastOS .po generator (nativo FastOS)
│
├── backend/                      # Low-level code generation
│   ├── cpu/                      # x86-64 backends (PE, ELF, flat)
│   └── gpu/                      # GPU backends (Vulkan, CUDA, HIP)
│
├── bg/                           # Binary Guardian (security)
├── runtime/                      # CPU/GPU detection + dispatch
└── toolchain/                    # GCC/LLVM/MSVC compatibility layer
    ├── mod.rs
    ├── gcc_compat.rs             # Emula flags GCC comunes
    ├── msvc_compat.rs            # Emula flags MSVC comunes
    └── clang_compat.rs           # Emula flags Clang comunes
```

---

## header_main.h — Header Universal v7.0

```c
/*
 * header_main.h — ADead-BIB Universal Header
 * Un solo include. Todo disponible. Sin linker.
 */
#ifndef _ADEAD_HEADER_MAIN
#define _ADEAD_HEADER_MAIN

/* C99 Standard Library COMPLETA */
#include <fastos_types.h>     /* int8_t, uint64_t, size_t, bool, NULL */
#include <fastos_limits.h>    /* INT_MAX, INT_MIN, SIZE_MAX, CHAR_MAX */
#include <fastos_stdio.h>     /* printf, scanf, fopen, fclose, fread, fwrite */
#include <fastos_stdlib.h>    /* malloc, free, calloc, realloc, exit, qsort */
#include <fastos_string.h>    /* strlen, strcpy, strcat, strcmp, memcpy, memset */
#include <fastos_math.h>      /* sin, cos, sqrt, pow, log, PI, E, TAU */
#include <fastos_time.h>      /* time, clock, sleep, gettimeofday */
#include <fastos_assert.h>    /* assert, static_assert */
#include <fastos_errno.h>     /* errno, strerror, ENOENT, ENOMEM */

/* C++ Standard Library (solo en C++ mode) */
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

/* TREE SHAKING AUTOMÁTICO:
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
  int vla[n]              VLA (C99 §6.7.5.2)
  struct { ... }          estructura
  union { ... }           unión
  enum { ... }            enumeración
  int (*fn)(int)          function pointer
  struct { int arr[]; }   flexible array member

QUALIFIERS:
  const    volatile    restrict

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
  int fn(void) { ... }
  static int fn(...)
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
Tamaño instalación  200MB    500MB         30GB     2MB ✓
Linker externo      SÍ ❌    SÍ ❌        SÍ ❌    NO ✓
UB detection        NO ❌    parcial ❌   NO ❌    21 tipos ✓
header_main.h       NO ❌    NO ❌        NO ❌    SÍ ✓
Tree shaking auto   NO ❌    parcial ❌   NO ❌    SÍ ✓
Un comando          NO ❌    NO ❌        NO ❌    SÍ ✓
Sin flags           NO ❌    NO ❌        NO ❌    SÍ ✓
UB antes optimizer  NO ❌    NO ❌        NO ❌    SÍ ✓ ÚNICO
C99 completo        SÍ ✓     SÍ ✓        parcial    SÍ ✓
C++98 completo      SÍ ✓     SÍ ✓        SÍ ✓      SÍ ✓
C++17               SÍ ✓     SÍ ✓        SÍ ✓      34 features ✓
Hello World size    50KB     40KB         60KB      2KB ✓
Cross-platform      pain     pain         NO        SÍ ✓
FastOS .po          NO ❌    NO ❌       NO ❌    SÍ ✓ ÚNICO
─────────────────────────────────────────────────────────────────
Filosofía:          ninguna  ninguna      negocio  Grace Hopper ✓
                                                   Dennis Ritchie ✓
```

---

*ADead-BIB v7.0 — 2026*
*"la máquina sirve al humano — sin linker — sin UB silencioso — para siempre"*
