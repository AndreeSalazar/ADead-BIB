# ADead-BIB — Test & Benchmark Suite

**Versión:** 1.0  
**Fecha:** 2026-02-22

---

## Estructura del Directorio

```
Test/
├── Testeo general.md            # Guía completa de testing
├── README.md                    # Este archivo
├── run_benchmarks.ps1           # Script principal de ejecución
├── check_pe.ps1                 # Verificador de formato PE
├── bench_comparison_rust.rs     # Benchmark de comparación con Rust (15 tests)
├── bench_comparison_c.c         # Benchmark de comparación con C (15 tests)
│
├── bench_01_cpu_integers.adB    # ✅ CPU Integer ALU (10 tests)
├── bench_02_cpu_floats.adB      # ✅ CPU Float/FixedPoint (7 tests)
├── bench_03_optimizer.adB       # ✅ Compiler Optimizations (8 tests)
├── bench_04_memory.adB          # ✅ Memory & Registers (8 tests)
├── bench_05_branch.adB          # ✅ Branch Prediction (8 tests)
├── bench_06_bitwise.adB         # ✅ Arithmetic Intensity (10 tests)
├── bench_07_real_algorithms.adB # ✅ Real Algorithms (10 tests)
├── bench_08_math_intense.adB    # ✅ Intensive Math (10 tests)
├── bench_09_stress.adB          # ✅ Stress Tests (8 tests)
├── bench_10_data_structures.adB # ✅ Data Structures (10 tests)
├── bench_11_sorting.adB         # ✅ Sorting Algorithms (8 tests)
├── bench_12_crypto.adB          # ✅ Crypto & Hashing (10 tests)
├── bench_13_pathfinding.adB     # ✅ Pathfinding & Graphs (8 tests)
└── bench_14_oop_patterns.adB    # ✅ OOP-Light Patterns (10 tests)
```

---

## Estado de los Benchmarks

| # | Archivo | Estado | Tests | Estilo |
|---|---------|--------|-------|--------|
| 1 | `bench_01_cpu_integers.adB` | ✅ | 10 | Procedural |
| 2 | `bench_02_cpu_floats.adB` | ✅ | 7 | Procedural |
| 3 | `bench_03_optimizer.adB` | ✅ | 8 | Procedural |
| 4 | `bench_04_memory.adB` | ✅ | 8 | Procedural |
| 5 | `bench_05_branch.adB` | ✅ | 8 | Procedural |
| 6 | `bench_06_bitwise.adB` | ✅ | 10 | Procedural |
| 7 | `bench_07_real_algorithms.adB` | ✅ | 10 | Procedural |
| 8 | `bench_08_math_intense.adB` | ✅ | 10 | Procedural |
| 9 | `bench_09_stress.adB` | ✅ | 8 | Procedural |
| 10 | `bench_10_data_structures.adB` | ✅ | 10 | **Procedural + OOP-light** |
| 11 | `bench_11_sorting.adB` | ✅ | 8 | **Procedural + OOP-light** |
| 12 | `bench_12_crypto.adB` | ✅ | 10 | **Procedural + Rust-style** |
| 13 | `bench_13_pathfinding.adB` | ✅ | 8 | **Procedural + OOP-light** |
| 14 | `bench_14_oop_patterns.adB` | ✅ | 10 | **OOP-light (C struct + Rust impl)** |

**Total:** 14 benchmarks, 125+ tests individuales

### Comparaciones externas

| Archivo | Lenguaje | Tests | Estilo |
|---------|----------|-------|--------|
| `bench_comparison_rust.rs` | Rust | 15 | `struct` + `impl` + generics |
| `bench_comparison_c.c` | C (gcc -O3) | 15 | `struct` + funciones + macros |

---

## Uso

### Ejecutar Todos los Benchmarks

```powershell
cd Test
powershell -ExecutionPolicy Bypass -File run_benchmarks.ps1
```

### Ejecutar un Benchmark Individual

```powershell
# Compilar
..\target\release\adeadc.exe bench_01_cpu_integers.adB -o test01.exe

# Ejecutar
.\test01.exe
```

### Verificar Formato PE

```powershell
powershell -File check_pe.ps1 test01.exe
```

---

## Categorías de Tests

### 1️⃣ CPU Integer ALU (bench_01)
- Sum masivo (10M iteraciones)
- Multiplicación con módulo
- División repetida
- Bitwise XOR chain
- Factorial iterativo
- Fibonacci iterativo
- GCD (Euclides)
- Primalidad (trial division)
- Búsqueda binaria
- Hash FNV-1a

### 2️⃣ CPU Float/FixedPoint (bench_02)
- Suma de enteros (simulando float)
- Multiplicación escalada
- División escalada
- Raíz cuadrada (Newton)
- Producto punto
- Normalización de vectores
- Matriz 4x4
- Interpolación lineal
- Distancia euclidiana
- Trigonometría aproximada

### 3️⃣ Compiler Optimizations (bench_03)
- Dead code elimination
- Constant folding
- Loop unrolling
- Inlining agresivo
- Strength reduction
- Common subexpression
- Loop invariant code motion
- Tail call optimization

### 4️⃣ Memory & Registers (bench_04)
- Acceso secuencial
- Acceso aleatorio
- Registro-intensivo
- Spill pressure
- Locality test
- Cache thrashing
- Register reuse
- Memory bandwidth

### 5️⃣ Branch Prediction (bench_05)
- Siempre verdadero
- Siempre falso
- Aleatorio (50/50)
- Switch grande
- Árbol de decisiones
- Máquina de estados
- Nested conditions
- Pattern prediction

### 6️⃣ Bitwise Operations (bench_06)
- AND masivo
- OR masivo
- XOR masivo
- NOT masivo
- Shift left
- Shift right
- Rotate simulation
- Popcount
- Parity check
- Bit reversal

### 7️⃣ Real Algorithms (bench_07)
- Quicksort
- Binary search
- String hash
- CRC32
- LCG random
- Sieve of Eratosthenes
- Dijkstra pathfinding
- Levenshtein distance

### 8️⃣ Intensive Math (bench_08)
- Power iterativo
- Logaritmo binario
- Raíz n-ésima
- Combinaciones
- Permutaciones
- Series de Taylor
- Integración numérica
- Método de Newton

### 9️⃣ Stress Tests (bench_09)
- Deep recursion (50,000 depth)
- Many function calls (20-chain x 1M)
- Long computation chain (20 ops x 5M)
- Many local variables (32 locals x 5M)
- Nested loops (100x100x100x10)
- Recursive Fibonacci(40)
- Mixed operations (10M)
- Many parameters (6-8 params x 5M)

### 🔟 Data Structures (bench_10) — **Procedural + OOP-light**
- Stack simulation (push/pop 100K)
- Queue / circular buffer (100K)
- Hash table simulation (100K inserts)
- Priority queue / min-heap
- Linked list via recursion
- Binary tree traversal (depth 15)
- Ring buffer (100K ops)
- LRU cache (4 slots, 100K lookups)
- Accumulator pattern (multiple objects)
- State machine with history (1M transitions)

### 1️⃣1️⃣ Sorting Algorithms (bench_11) — **Procedural + OOP-light**
- Bubble sort (16 elements x 10K)
- Selection sort (8 elements x 10K)
- Sorting network (8 elements x 100K)
- Insertion sort (8 elements x 10K)
- Cocktail shaker sort (8 elements x 10K)
- Gnome sort (8 elements x 10K)
- Comb sort (8 elements x 10K)
- Sort + verify (correctness check)

### 1️⃣2️⃣ Crypto & Hashing (bench_12) — **Procedural + Rust-style**
- FNV-1a hash simulation (1M)
- DJB2 hash simulation (1M)
- CRC32 simulation (100K)
- SHA-like compression (100K)
- Caesar cipher roundtrip (1M)
- XOR cipher simulation (100K)
- LFSR pseudo-random (1M)
- Merkle-Damgård chain (100K)
- Key derivation / PBKDF2-like
- Checksum validator (100K)

### 1️⃣3️⃣ Pathfinding & Graphs (bench_13) — **Procedural + OOP-light**
- Manhattan distance (10M pairs)
- Flood fill 4x4 grid (10K)
- Dijkstra 4-node graph (100K)
- A* heuristic computation (10M)
- BFS 4-node graph (100K)
- TSP brute force 5 cities (1K)
- Union-Find 6 nodes (100K)
- Chebyshev distance (10M)

### 1️⃣4️⃣ OOP-Light Patterns (bench_14) — **C struct + Rust impl style**
- Point2D "class" (distance, translate, scale)
- Vector3D "class" (add, dot, cross, normalize)
- Counter "object" (increment, decrement, reset)
- Builder pattern (config builder)
- Strategy pattern (dispatch by strategy_id)
- Observer pattern (event fire → N observers)
- State pattern (traffic light, 3 states)
- Iterator pattern (sequence iteration)
- Polymorphism via dispatch (shape_area)
- RAII resource pattern (acquire/use/release)

---

## Métricas Medidas

| Métrica | Descripción |
|---------|-------------|
| **Compile Time** | Tiempo de compilación (ms) |
| **Run Time** | Tiempo de ejecución (ms) |
| **Binary Size** | Tamaño del ejecutable (bytes) |
| **Throughput** | Operaciones por segundo |
| **Ratio vs Rust** | Comparación con LLVM (si disponible) |

---

## Comparación con Rust y C

El script `run_benchmarks.ps1` automáticamente compila y ejecuta ambos benchmarks de comparación si están disponibles.

```powershell
# Rust (LLVM backend)
rustc -O bench_comparison_rust.rs -o bench_rust.exe
.\bench_rust.exe

# C (gcc backend)
gcc -O3 bench_comparison_c.c -o bench_c.exe
.\bench_c.exe
```

Los benchmarks de comparación incluyen 15 tests cada uno:
- Tests 1-10: Algoritmos clásicos (sum, fibonacci, factorial, GCD, primes, etc.)
- Tests 11-15: **Patrones OOP-light** (sorting network, Point2D, Vec3, Dijkstra, mod_pow)

Tanto el archivo Rust como el C usan patrones OOP:
- **Rust**: `struct Point2D` + `impl Point2D { fn distance_sq(...) }`
- **C**: `typedef struct { ... } Point2D;` + `int64_t point_distance_sq(Point2D a, Point2D b)`
- **ADead-BIB**: Funciones con prefijo `point_` que reciben campos como parámetros

---

## Interpretación de Resultados

### ✅ Bueno
- Compile time < 100ms por benchmark
- Run time comparable a Rust (1x - 3x)
- Binary size < 20KB
- Sin crashes ni errores

### ⚠️ Aceptable
- Compile time 100-500ms
- Run time 3x - 10x vs Rust
- Binary size 20-50KB
- Warnings menores

### ❌ Necesita Mejora
- Compile time > 500ms
- Run time > 10x vs Rust
- Binary size > 50KB
- Crashes o errores de ejecución

---

## Notas Importantes

1. **Formato PE:** Los ejecutables generados son PE64 para Windows. Verificar con `check_pe.ps1`.

2. **Sintaxis ADead-BIB:**
   - Sin punto y coma
   - `if condition { }` sin paréntesis
   - `while condition { }` sin paréntesis
   - `int`, `void` para tipos
   - `printf()` para output

3. **Limitaciones Actuales:**
   - No soporta `&&` ni `||` (usar if anidados)
   - No soporta `for` loops (usar while)
   - No soporta arrays dinámicos
   - No soporta float/double nativos (usar fixed-point)

4. **Warnings Esperados:**
   - `Return type mismatch. Expected I32, found I64` - Normal en x86-64

---

## Próximos Pasos

### Tests Completados (v2.0)

- [x] Data Structures (bench_10) — Stack, Queue, Hash, Heap, Tree, LRU
- [x] Sorting Algorithms (bench_11) — Bubble, Selection, Network, Insertion, Comb
- [x] Crypto & Hashing (bench_12) — FNV, DJB2, CRC32, SHA-like, Caesar, LFSR
- [x] Pathfinding & Graphs (bench_13) — Dijkstra, BFS, TSP, Flood Fill, Union-Find
- [x] OOP-Light Patterns (bench_14) — Point2D, Vec3, Builder, Strategy, Observer
- [x] Comparación con C (gcc -O3)
- [x] Patrones OOP-light en comparaciones Rust y C

### Tests Pendientes (requieren soporte del OS/runtime)

- [ ] Concurrencia (threads, mutex, atomic) — requiere soporte de threads
- [ ] Syscalls (write, open, fork) — requiere syscall API
- [ ] I/O real (archivos, sockets) — requiere I/O layer
- [ ] Latencia del sistema (boot time, load time)

### Mejoras Sugeridas

- [ ] Agregar timer de alta precisión (RDTSC)
- [ ] Profiling con perf/VTune
- [ ] Comparación con Zig
- [ ] Tests de regresión automatizados
- [ ] CI/CD integration

---

## Contacto

**Autor:** Eddi Andreé Salazar Matos  
**Proyecto:** ADead-BIB v3.2  
**Licencia:** MIT
