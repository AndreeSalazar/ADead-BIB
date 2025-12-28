# ADead-BIB - Roadmap de Mejoras Futuras

> **ADead-BIB**: Lenguaje de programación que compila directo a binario nativo x86-64.
> Parte de la familia ASM pero con sintaxis moderna estilo Rust/Python.
> 100% escrito en Rust, sin dependencias de C/C++.

---

## 📊 Estado del Proyecto

| Componente | Estado | Tests |
|------------|--------|-------|
| Lexer | ✅ Completo | 8 tests |
| Parser | ✅ Funcional | Rust + Python style |
| Type Checker | ⚠️ Básico | Inferencia limitada |
| CodeGen x86-64 | ✅ Funcional | Windows PE + Linux ELF |
| GPU Backend | ✅ Vulkan + CUDA | SPIR-V generation |
| **Total Tests** | **61 pasando** | ✅ |

---

## ✅ Versiones Completadas

### v0.5.0 ✅ - Fundamentos
- [x] Sintaxis estilo Rust (`fn`, `let`, `const`)
- [x] `print()` sin salto de línea automático
- [x] `println()` con salto de línea automático
- [x] Secuencias de escape (`\n`, `\t`, `\r`)
- [x] Operaciones aritméticas (+, -, *, /, %)
- [x] Compilación directa a binario x86-64
- [x] Soporte para Windows PE
- [x] Ejemplos organizados y simplificados
- [x] Guías en español e inglés

### v0.6.0 ✅ - Control de Flujo
- [x] `if` / `else` condicionales
- [x] `while` loops
- [x] `for` loops (for i in 0..10)
- [x] `break` y `continue`
- [x] Operadores de comparación: `==`, `!=`, `<`, `>`, `<=`, `>=`

### v0.7.0 ✅ - Funciones
- [x] Definir funciones propias con `fn`
- [x] Llamar funciones
- [x] Parámetros de funciones
- [x] Valores de retorno (`return`)
- [x] Recursión

### v0.8.0 ✅ - Tipos de Datos
- [x] Booleanos (`true`, `false`)
- [x] Números enteros (i64)
- [x] Números flotantes con decimales reales (%.2f)
- [x] Strings básicos

### v0.9.0 ✅ - Entrada de Usuario
- [x] `input()` para leer del teclado (placeholder: retorna 42)

### v1.0.0 ✅ - Estabilidad
- [x] Manejo de errores mejorado
- [x] Mensajes de error claros con línea y columna
- [x] Tracking de líneas en lexer
- [x] Tests automatizados (61 tests)
- [x] Documentación completa

### v1.1.0 ✅ - Flotantes Reales
- [x] Números flotantes con decimales (%.2f)
- [x] Constantes matemáticas: PI = 3.14, E = 2.72

### v1.2.0 ✅ (Actual) - OOP Básico
- [x] `struct` con campos tipados
- [x] `impl` para métodos
- [x] Sintaxis `Struct::method()`
- [x] GPU Backend con Vulkan/CUDA
- [x] SPIR-V shader generation
- [x] Pipeline unificado CPU↔GPU

---

## 🚧 v1.3.0 - Arrays y Strings (En Desarrollo)

### Arrays/Listas ✅
- [x] Declaración: `let arr = [1, 2, 3]` ✅
- [x] Indexación: `arr[0]` ✅
- [x] Longitud: `len(arr)` ✅ **FUNCIONA**
- [x] Iteración: `for x in arr { }` ✅ **FUNCIONA**
- [ ] Push/Pop: `arr.push(4)`, `arr.pop()` (futuro)
- [ ] Slicing: `arr[1..3]` (futuro)

### Operaciones de String
- [ ] Concatenación: `"Hello" + " World"` (futuro)
- [ ] Longitud: `len(str)` (futuro)
- [ ] Indexación: `str[0]` (futuro)
- [ ] Métodos: `str.upper()`, `str.lower()`, `str.trim()` (futuro)
- [ ] Interpolación: `f"Valor: {x}"` (futuro)
- [ ] Split/Join: `str.split(",")`, `arr.join("-")` (futuro)

### Conversión de Tipos ✅
- [x] `int(valor)` - Convertir a entero ✅
- [x] `float(valor)` - Convertir a flotante ✅
- [ ] `str(valor)` - Convertir a string (futuro)
- [x] `bool(valor)` - Convertir a booleano ✅ **FUNCIONA**

### Carpeta TESTEO ✅
- [x] Estructura de tests creada
- [x] `TESTEO/arrays/` - Tests de arrays
- [x] `TESTEO/arrays/test_foreach.adB` - ✅ for x in arr funciona
- [x] `TESTEO/conversiones/` - Tests de conversión de tipos
- [x] `TESTEO/len/test_len_array.adB` - ✅ len() funciona
- [x] `TESTEO/integrados/test_v1_3_0_completo.adB` - ✅ Test completo

---

## ✅ v1.4.0 - Input Real y I/O (COMPLETADO)

### Entrada de Usuario Real ✅
- [x] `input()` lee de stdin usando scanf - **FUNCIONA**
- [ ] `input("prompt")` con mensaje (futuro)
- [x] Parsing automático de números enteros

**Implementación técnica (completada)**:
1. ✅ Agregado `scanf` a las importaciones del PE (IAT en 0x2048)
2. ✅ Modificada la estructura de la Import Directory Table
3. ✅ Actualizado codegen con nuevas direcciones (printf@0x2040, scanf@0x2048)
4. ✅ data_rva actualizado a 0x2078

### Test de input() ✅
```
echo 25 | test_input.exe
Ingresa un numero: Ingresaste: 25
El doble es: 50
```

### Archivos (Futuro v1.6.0)
- [ ] `open(path, mode)` - Abrir archivo
- [ ] `file.read()` - Leer contenido
- [ ] `file.write(data)` - Escribir contenido
- [ ] `file.close()` - Cerrar archivo

### Salida Formateada (Futuro)
- [ ] `printf(format, args...)` - Formato estilo C
- [ ] `format!()` - Formato estilo Rust

---

## 🔮 v1.5.0 - Sistema de Módulos

### Imports
- [ ] `import modulo` - Importar módulo completo
- [ ] `from modulo import func` - Importar específico
- [ ] `import modulo as alias` - Alias
- [ ] Resolución de paths relativos
- [ ] Biblioteca estándar básica

### Organización
- [ ] Un archivo = un módulo
- [ ] Carpetas como paquetes
- [ ] `mod.adB` como índice de paquete
- [ ] Visibilidad: `pub` para exportar

### Biblioteca Estándar (std)
- [ ] `std::io` - Entrada/Salida
- [ ] `std::math` - Funciones matemáticas
- [ ] `std::string` - Operaciones de string
- [ ] `std::collections` - Estructuras de datos
- [ ] `std::fs` - Sistema de archivos

---

## 🔮 v1.6.0 - Traits e Interfaces

### Traits
- [ ] `trait Nombre { fn metodo(&self); }`
- [ ] `impl Trait for Struct { }`
- [ ] Traits como bounds: `fn foo<T: Trait>(x: T)`
- [ ] Default implementations
- [ ] Traits derivables: `#[derive(Debug, Clone)]`

### Polimorfismo
- [ ] Dispatch dinámico con `dyn Trait`
- [ ] Dispatch estático con generics
- [ ] Trait objects

---

## 🔮 v1.7.0 - Manejo de Errores

### Option y Result
- [ ] `Option<T>` - Some(valor) | None
- [ ] `Result<T, E>` - Ok(valor) | Err(error)
- [ ] Operador `?` para propagación
- [ ] `unwrap()`, `expect()`, `unwrap_or()`
- [ ] Pattern matching con `match`

### Excepciones (Opcional)
- [ ] `try { } catch { }` estilo tradicional
- [ ] `panic!()` para errores irrecuperables

---

## 🔮 v1.8.0 - Generics y Tipos Avanzados

### Generics
- [ ] Funciones genéricas: `fn foo<T>(x: T)`
- [ ] Structs genéricos: `struct Vec<T>`
- [ ] Traits bounds: `<T: Clone + Debug>`
- [ ] Where clauses

### Tipos Avanzados
- [ ] Enums con datos: `enum Result<T, E> { Ok(T), Err(E) }`
- [ ] Type aliases: `type Punto = (i32, i32)`
- [ ] Tuples: `let t = (1, "hello", 3.14)`
- [ ] Destructuring: `let (x, y) = punto`

---

## 🔮 v1.9.0 - Optimizaciones

### Compilador
- [ ] Constant folding: `2 + 3` → `5`
- [ ] Dead code elimination
- [ ] Inlining de funciones pequeñas
- [ ] Loop unrolling
- [ ] Tail call optimization

### Binarios
- [ ] Binarios más pequeños (< 1KB para hello world)
- [ ] Strip de símbolos
- [ ] Compresión de secciones
- [ ] Link-time optimization (LTO)

### SIMD Automático
- [ ] Vectorización automática de loops
- [ ] Detección de patrones SIMD
- [ ] SSE/AVX/AVX-512 según CPU

---

## 🔮 v2.0.0 - Características Avanzadas

### Async/Await
- [ ] `async fn` - Funciones asíncronas
- [ ] `await` - Esperar resultado
- [ ] Runtime async básico
- [ ] Futures y Promises

### Manejo de Memoria
- [ ] Ownership básico (sin borrow checker completo)
- [ ] `Box<T>` - Heap allocation
- [ ] `Rc<T>` - Reference counting
- [ ] `Arc<T>` - Atomic reference counting
- [ ] Drop automático

### FFI (Foreign Function Interface)
- [ ] `extern "C"` - Llamar funciones C
- [ ] Cargar DLLs/SOs dinámicamente
- [ ] Exportar funciones para C
- [ ] Bindings automáticos

### Multi-plataforma
- [ ] Windows PE (x86-64) ✅
- [ ] Linux ELF (x86-64)
- [ ] macOS Mach-O (x86-64 + ARM64)
- [ ] WebAssembly (WASM)
- [ ] ARM64 nativo

---

## 🎮 GPU Computing (Ya Implementado)

### Vulkan Backend ✅
- [x] Detección de GPU
- [x] SPIR-V shader generation
- [x] Compute shaders
- [x] MatMul optimizado

### CUDA Backend ✅
- [x] Generación de código CUDA (.cu)
- [x] VectorAdd, MatMul kernels
- [x] Benchmarks CPU vs GPU

### Pipeline Unificado ✅
- [x] Decisión automática CPU↔GPU
- [x] Threshold basado en tamaño de datos
- [x] HEX optimization layer

---

## 🐛 Bugs Conocidos y Fixes Pendientes

### Alta Prioridad
- [ ] Type Checker no infiere tipos de retorno de funciones
- [ ] `input()` es placeholder (siempre retorna 42)
- [ ] Parser Python-style no soporta indentación real

### Media Prioridad
- [ ] Warnings de variables no usadas en código interno
- [ ] Algunos campos de structs internos no se usan

### Baja Prioridad
- [ ] Mensajes de error podrían ser más descriptivos
- [ ] Documentación de API interna incompleta

---

## 📋 Prioridades de Desarrollo

| Prioridad | Feature | Versión Target |
|-----------|---------|----------------|
| 🔴 Alta | Arrays y Strings | v1.3.0 |
| 🔴 Alta | Input() real | v1.4.0 |
| 🔴 Alta | Type Checker mejorado | v1.3.0 |
| 🟡 Media | Sistema de módulos | v1.5.0 |
| 🟡 Media | Traits | v1.6.0 |
| 🟡 Media | Manejo de errores | v1.7.0 |
| 🟢 Baja | Generics | v1.8.0 |
| 🟢 Baja | Async/await | v2.0.0 |
| 🟢 Baja | FFI | v2.0.0 |

---

## 🛠️ Arquitectura del Compilador

```
┌─────────────────────────────────────────────────────────────┐
│                    ADead-BIB Compiler                       │
├─────────────────────────────────────────────────────────────┤
│  Source (.adB)                                              │
│       ↓                                                     │
│  ┌─────────┐  ┌────────┐  ┌─────────────┐  ┌─────────────┐ │
│  │  Lexer  │→ │ Parser │→ │ Type Checker│→ │  Optimizer  │ │
│  └─────────┘  └────────┘  └─────────────┘  └─────────────┘ │
│       ↓                                                     │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                    CodeGen                           │   │
│  │  ┌───────────┐  ┌───────────┐  ┌───────────────┐    │   │
│  │  │ CPU x86-64│  │ GPU Vulkan│  │ GPU CUDA      │    │   │
│  │  └───────────┘  └───────────┘  └───────────────┘    │   │
│  └─────────────────────────────────────────────────────┘   │
│       ↓                                                     │
│  ┌─────────────────────────────────────────────────────┐   │
│  │                  Binary Output                       │   │
│  │  ┌────────┐  ┌────────┐  ┌────────┐  ┌──────────┐   │   │
│  │  │ PE/EXE │  │  ELF   │  │ SPIR-V │  │ CUDA .cu │   │   │
│  │  └────────┘  └────────┘  └────────┘  └──────────┘   │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

---

## 🤝 Cómo Contribuir

1. Fork el repositorio
2. Crea una rama para tu feature: `git checkout -b feature/nueva-funcionalidad`
3. Haz tus cambios
4. Ejecuta los tests: `cargo test`
5. Envía un Pull Request

### Áreas donde se necesita ayuda
- Implementación de arrays y strings
- Mejoras al type checker
- Documentación y ejemplos
- Testing en Linux/macOS
- Optimizaciones de código generado

---

## 📚 Recursos

- **Documentación**: `GUIA_ES.md`, `GUIDE_EN.md`
- **Ejemplos**: `/examples/*.adB`
- **Tests**: `cargo test`
- **Issues**: GitHub Issues

---

## 📜 Historial de Cambios

| Versión | Fecha | Cambios Principales |
|---------|-------|---------------------|
| v1.2.0 | 2024-12 | Structs, impl, GPU backends |
| v1.1.0 | 2024-12 | Flotantes reales |
| v1.0.0 | 2024-12 | Estabilidad, 50+ tests |
| v0.9.0 | 2024-12 | Input placeholder |
| v0.8.0 | 2024-12 | Booleanos, flotantes |
| v0.7.0 | 2024-12 | Funciones propias |
| v0.6.0 | 2024-12 | Control de flujo |
| v0.5.0 | 2024-12 | Fundamentos |

---

*Este roadmap se actualiza conforme avanza el desarrollo del proyecto.*
*Última actualización: Diciembre 2024*
