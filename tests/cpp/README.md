# tests/cpp — ADead-BIB C++ Test Suite

> Suite de pruebas y fixtures del frontend C++ de ADead-BIB
> 15 fixture files covering C++ basics: classes, namespaces, enums, lambdas, inheritance, virtual, operator overload
> Experimental frontend — maps C++ constructs to C-equivalent IR

---

## Estructura

```text
tests/cpp/
├── README.md
└── fixtures/
    ├── 01_hello_world.cpp             Basic C++ compilation, printf + return 0
    ├── 02_class_basic.cpp             Class definition, fields, methods, constructor
    ├── 03_namespace.cpp               Namespace definition and scoped functions
    ├── 04_enum.cpp                    Enum and C++11 scoped enum class
    ├── 05_control_flow.cpp            if/else, while, for, switch
    ├── 06_assignment.cpp              Assignments & compound assigns (+=, ->field)
    ├── 07_lambda.cpp                  C++11 lambda expression lowering
    ├── 08_inheritance.cpp             Class inheritance, base class embedding
    ├── 09_ub_strict.cpp               UB detection: bit-width, narrowing
    ├── 10_new_delete.cpp              new/delete → malloc/free lowering
    ├── 11_virtual.cpp                 Virtual methods & vtable generation
    ├── 12_operator_overload.cpp       Operator overload → mangled function names
    ├── 13_static_method.cpp           Static methods & initializer list lowering
    ├── 14_extern_c.cpp                extern "C" blocks & forward declarations
    └── 15_destructor.cpp              Destructor emission, delete → dtor + free
```

## Test Categories

| Category | Fixtures | Description |
|---|---|---|
| basics | 01, 05-06 | Hello world, control flow, assignments |
| classes | 02, 08, 13 | Class definition, inheritance, static methods |
| namespaces/enums | 03-04 | Namespace scoping, scoped enums |
| C++11 features | 04, 07 | Enum class, lambda expressions |
| OOP / vtable | 08, 11, 15 | Inheritance, virtual dispatch, destructors |
| operator overload | 12 | Operator overload name mangling |
| memory | 10, 15 | new/delete lowering to malloc/free |
| interop | 14 | extern "C" linkage |
| UB detection | 09 | Strict UB and narrowing checks |

## How to use

```bash
# Compile a fixture
adB cc tests/cpp/fixtures/02_class_basic.cpp -o test.exe

# Compile and run
adB run tests/cpp/fixtures/11_virtual.cpp

# Run all C++ fixtures
for /L %i in (1,1,15) do adB run tests/cpp/fixtures/%02d_*.cpp

# Run Rust unit tests
cargo test -p adeb-frontend-cpp
```

## Note

The C++ frontend is **experimental**. It lowers C++ constructs (classes, namespaces,
virtual dispatch, operator overloading, lambdas, new/delete) into C-equivalent IR
that the existing backend can compile. Not all C++ features are supported yet.
