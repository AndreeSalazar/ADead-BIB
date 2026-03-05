# Test-Canon — Herencia Total del Canon C99 y C++98

> **Propósito:** Validar que ADead-BIB compila auténticamente el canon completo
> de C99 (ISO/IEC 9899:1999) y C++98 (ISO/IEC 14882:1998).
>
> Cada archivo testea UNA categoría del estándar con instrucciones claras
> sobre qué intención del lenguaje se verifica.

---

## Estructura

```
Test-Canon/
├── CANON.md                          # Este documento
│
├── C99/                              # Canon C99 completo
│   ├── 01_tipos_fundamentales.c      # int, char, short, long, float, double
│   ├── 02_punteros_autenticos.c      # *, &, ->, aritmética de punteros
│   ├── 03_arrays_memoria.c           # arrays fijos, multidimensionales, stride
│   ├── 04_structs_layout.c           # struct layout, campos, nested structs
│   ├── 05_unions_memoria.c           # unions, tipo-punning, sizeof
│   ├── 06_enums_constantes.c         # enum, constantes enteras
│   ├── 07_typedef_alias.c            # typedef para tipos y function pointers
│   ├── 08_control_flujo.c            # if/else, for, while, do-while, switch
│   ├── 09_funciones_calling.c        # declaración, definición, recursión, prototipos
│   ├── 10_punteros_funcion.c         # function pointers, callbacks
│   ├── 11_preprocesador.c            # #define, #ifdef, #ifndef, macros
│   ├── 12_bitwise_operadores.c       # &, |, ^, ~, <<, >>
│   ├── 13_casting_tipos.c            # cast explícito, promoción, truncamiento
│   ├── 14_scope_lifetime.c           # scope de variables, static, auto
│   ├── 15_string_operaciones.c       # char*, strlen, strcmp, strcpy (conceptual)
│   ├── 16_malloc_free.c              # malloc, free, realloc, linked list
│   ├── 17_sizeof_alineacion.c        # sizeof, offsetof, alignment
│   └── 18_expresiones_complejas.c    # ternario, comma, precedencia completa
│
└── Cpp98/                            # Canon C++98 completo
    ├── 01_clases_basicas.cpp         # class, public/private, constructor, destructor
    ├── 02_herencia.cpp               # single inheritance, base/derived
    ├── 03_virtual_polimorfismo.cpp   # virtual, override, vtable
    ├── 04_templates_funcion.cpp      # function templates, monomorphización
    ├── 05_templates_clase.cpp        # class templates, instanciación
    ├── 06_namespaces.cpp             # namespace, using, anidados
    ├── 07_operator_overload.cpp      # operator+, operator==, operator<<
    ├── 08_referencias.cpp            # T&, pass-by-reference, const ref
    ├── 09_const_correctness.cpp      # const, const methods, constexpr
    ├── 10_encapsulamiento.cpp        # private data + public interface
    ├── 11_constructores_avanzados.cpp # copy constructor, default, member init
    ├── 12_static_members.cpp         # static data, static methods
    ├── 13_punteros_objetos.cpp       # punteros a objetos, -> con clases
    ├── 14_enum_class.cpp             # enum class (scoped)
    └── 15_stl_basico.cpp             # uso básico de STL reconocido
```

---

## Reglas del Canon

### C99 — Intenciones Absolutas

1. **Cada tipo tiene tamaño exacto**: `char`=1, `short`=2, `int`=4, `long long`=8
2. **Punteros son direcciones reales**: `int *p = &x` → `p` contiene la dirección de `x`
3. **Aritmética de punteros**: `p + 1` avanza `sizeof(*p)` bytes, no 1 byte
4. **Structs tienen layout en memoria**: los campos están en orden, con alignment
5. **Arrays son memoria contigua**: `arr[i]` = `*(arr + i)`
6. **`malloc/free` = control manual**: sin garbage collector, sin RAII
7. **Casting es responsabilidad del programador**: el compilador obedece
8. **Bitwise opera sobre los bits reales**: `<<` es shift de bits, no multiplicación

### C++98 — Zero Overhead Principle

1. **Classes = structs con métodos**: cero overhead si no hay virtual
2. **Virtual = vtable**: solo cuando el programador lo pide explícitamente
3. **Templates = monomorphización**: solo se genera código para instancias usadas
4. **Namespaces = organización**: cero costo runtime, solo prefijo de nombre
5. **Referencias = punteros seguros**: el compilador las implementa como punteros
6. **Constructores/Destructores = RAII**: orden determinístico
7. **Lo que no usas, no pagas**: ninguna feature tiene costo si no se usa

---

## Cómo Compilar

```powershell
# C99
.\target\release\adeadc.exe cc Test-Canon\C99\01_tipos_fundamentales.c -o test_tipos.exe

# C++98
.\target\release\adeadc.exe cxx Test-Canon\Cpp98\01_clases_basicas.cpp -o test_clases.exe

# Compilar TODOS los C99
foreach ($f in Get-ChildItem Test-Canon\C99\*.c) {
    $out = $f.BaseName + ".exe"
    .\target\release\adeadc.exe cc $f.FullName -o $out
    Write-Host "$($f.Name) → $out"
}

# Compilar TODOS los C++98
foreach ($f in Get-ChildItem Test-Canon\Cpp98\*.cpp) {
    $out = $f.BaseName + ".exe"
    .\target\release\adeadc.exe cxx $f.FullName -o $out
    Write-Host "$($f.Name) → $out"
}
```

---

**Autor:** Eddi Andreé Salazar Matos
**Versión:** Canon v1.0 — C99 + C++98
**Compilador:** ADead-BIB (adeadc) — Sin GCC, Sin LLVM, Sin Clang
