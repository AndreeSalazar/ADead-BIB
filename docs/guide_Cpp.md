# ADead-BIB — Guía de Compilación C++

## Requisitos

- **ADead-BIB** compilado (`cargo build --release`)
- Windows x86-64 (target nativo)
- No requiere GCC, Clang, ni LLVM

## Comandos Básicos

### Compilar un archivo C++

```bash
adB cxx app.cpp                   # Genera app.exe
adB cxx app.cpp -o output.exe     # Nombre de salida personalizado
adB cpp app.cpp                   # Alias: cpp también funciona
adB c++ app.cpp                   # Alias: c++ también funciona
adB app.cpp                       # Auto-detecta C++ por extensión
```

### Compilar y ejecutar

```bash
adB run app.cpp                   # Compila + ejecuta automáticamente
adB run main.cc                   # También funciona con .cc
adB run lib.cxx                   # También funciona con .cxx
```

### Modo paso a paso

```bash
adB cxx app.cpp -step             # Muestra cada fase del compilador
adB step app.cpp                  # Atajo para step mode
```

## Modo Estricto (Siempre Activo)

**C++ en ADead-BIB es SIEMPRE estricto.** No existe un modo relajado.

- Todo UB detectado es un **error** (no warning)
- Bit-width enforcement: `char c = 256` es un error
- Narrowing conversions detectadas en casts
- Signed integer overflow en aritmética de literales
- No necesitas pasar `-Wstrict` — ya es implícito

## Estándares Soportados

| Estándar | Soporte | Notas |
|----------|---------|-------|
| C++98    | ~87%    | Classes, inheritance, virtual, templates (parse) |
| C++11    | ~71%    | Lambda, range-for, auto, nullptr, enum class |
| C++14    | ~25%    | Heredado de C++11 |
| C++17    | ~18%    | if constexpr, structured bindings (parse) |
| C++20    | Parcial | Spaceship operator (parse + mangle) |

## Características del Lenguaje C++

### Clases y Objetos

```cpp
class Counter {
public:
    int count;

    Counter(int initial) : count(initial) {}

    void increment() { count++; }
    int get() { return count; }
};

int main() {
    Counter c(0);
    c.increment();
    return c.get();
}
```

ADead-BIB genera:
- `Counter` → struct con campo `count`
- `Counter::__init` → constructor (con initializer list)
- `Counter::increment` → método con `this` implícito
- `Counter::get` → método con `this` implícito

### Herencia

```cpp
class Shape {
public:
    int sides;
    virtual int area() { return 0; }
};

class Rectangle : public Shape {
public:
    int width;
    int height;
    int area() { return width * height; }
};
```

ADead-BIB genera:
- `Shape` → struct con `__vptr` + `sides`
- `Rectangle` → struct con `__base_Shape` + `width` + `height`
- `__vtable_Shape` → struct con slot `area`

### Métodos Virtuales y vtable

```cpp
class Animal {
public:
    virtual int speak() { return 0; }
    virtual int move() { return 1; }
};
```

Se emite:
- Campo `__vptr` en `Animal`
- Struct `__vtable_Animal` con campos `speak` y `move` (function pointers)

### Operator Overloading

```cpp
class Vec2 {
public:
    int x, y;
    int operator+(int rhs) { return x + y + rhs; }
    int operator==(int rhs) { return x == rhs; }
    int operator[](int i) { return i; }
};
```

Los operadores se manglan a nombres de función:

| Operador | Nombre IR |
|----------|-----------|
| `operator+` | `Vec2::operator_add` |
| `operator-` | `Vec2::operator_sub` |
| `operator*` | `Vec2::operator_mul` |
| `operator==` | `Vec2::operator_eq` |
| `operator!=` | `Vec2::operator_ne` |
| `operator<` | `Vec2::operator_lt` |
| `operator[]` | `Vec2::operator_index` |
| `operator()` | `Vec2::operator_call` |
| `operator<<` | `Vec2::operator_shl` |
| `operator>>` | `Vec2::operator_shr` |
| `operator<=>` | `Vec2::operator_spaceship` |

### Métodos Estáticos

```cpp
class Math {
public:
    static int square(int x) { return x * x; }
};
```

Los métodos estáticos **no** reciben parámetro `this`.

### Namespaces

```cpp
namespace math {
    int square(int x) { return x * x; }

    namespace detail {
        int helper() { return 0; }
    }
}

int main() {
    return math::square(5);
}
```

Funciones emitidas: `math::square`, `math::detail::helper`

### Enums

```cpp
enum Color { Red, Green, Blue };         // C-style
enum class Direction { Up, Down };       // C++11 scoped
```

### Lambda

```cpp
auto add = [](int a, int b) { return a + b; };
auto capture = [x](int y) { return x + y; };
```

### Control de Flujo

```cpp
// if constexpr (C++17) — compile-time branch elimination
if constexpr (sizeof(int) == 4) {
    // Solo este branch se emite
}

// Range-for (C++11)
for (auto& item : collection) {
    process(item);
}

// Switch, while, do-while, for — igual que C
```

### cout/cin (I/O)

```cpp
#include <iostream>

int main() {
    int x;
    std::cout << "Hello " << 42 << std::endl;
    std::cin >> x;
    return 0;
}
```

ADead-BIB convierte:
- `cout << "Hello"` → `printf("Hello")`
- `cout << 42` → `printf("%d", 42)`
- `cout << endl` → `printf("\n")`
- `cin >> x` → `scanf("%d", &x)`

### new/delete

```cpp
int* p = new int(42);    // → malloc(8)
delete p;                 // → destructor + free(p)

int* arr = new int[10];   // → malloc(80)
delete[] arr;              // → free(arr)
```

### extern "C"

```cpp
extern "C" {
    int abs(int x);
    int atoi(const char* str);
}
```

Las funciones dentro de `extern "C"` se emiten sin mangling.

## Detección de UB

ADead-BIB detecta estos problemas en C++:

| Categoría | Ejemplo | Severidad |
|-----------|---------|-----------|
| División por cero | `int x = 5 / 0;` | Error |
| Shift overflow | `int x = 1 << 64;` | Error |
| Null pointer deref | `*nullptr` | Error |
| Virtual en constructor | `this->method()` en ctor | Error |
| Throw en destructor | `throw` en dtor | Error |
| Self-assignment | `x = x` | Error |
| Bit-width violation | `char c = 256` | Error |
| Narrowing conversion | `(char)300` | Error |
| Signed overflow | `INT_MAX + 1` | Error |

**Todos** son errores (no warnings) porque C++ es implícitamente estricto.

## Pipeline de Compilación

```
.cpp → Preprocessor → Lexer → Parser → UB Detector → CppToIR → Program IR → ISA → PE
```

### Fases en Step Mode

```
Phase 0: Preprocessor    — #include, #define, #ifdef
Phase 1: Lexer           — Tokenización C++
Phase 2: Parser          — AST C++ (classes, templates, lambdas)
Phase 3: UB Detection    — Análisis estático (strict)
Phase 4: IR Generation   — CppToIR (3 passes: enums→decls→vtables)
```

### 3 Passes del Lowering

1. **Pass 1**: Recolecta enums, type aliases, template definitions
2. **Pass 2**: Procesa todas las declaraciones top-level
3. **Pass 3**: Emite vtable structs

## Ejemplo Completo

```cpp
#include <iostream>

class Animal {
public:
    int legs;

    Animal(int l) : legs(l) {}

    virtual int speak() { return 0; }
    int get_legs() { return legs; }
};

class Dog : public Animal {
public:
    Dog() : Animal(4) {}
    int speak() { return 1; }
};

int main() {
    std::cout << "ADead-BIB C++ Compiler" << std::endl;
    std::cout << "Dog has " << 4 << " legs" << std::endl;
    return 0;
}
```

Compilar:

```bash
adB run example.cpp
```

## Extensiones Reconocidas

| Extensión | Detectada como |
|-----------|---------------|
| `.cpp`    | C++ |
| `.cxx`    | C++ |
| `.cc`     | C++ |
| `.hpp`    | C++ |

---

*ADead-BIB v9.0 — C++ Compiler Complete*
*No LLVM. No GCC. Pure bits.*
