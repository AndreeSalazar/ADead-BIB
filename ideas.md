# ADead-BIB — Documento de Diseño del Lenguaje

> **Versión:** 2.0  
> **Autor:** Eddi Andreé Salazar Matos  
> **Filosofía:** Binary Is Binary — Código → Bytes → Binario

---

## Manifiesto

**ADead-BIB no abstrae la máquina, la domestica.**

El humano piensa en objetos y lógica. El lenguaje traduce eso a bytes sin mentir.

```
┌─────────────────────────────────────────────────────────────┐
│  COMPILADORES TRADICIONALES (7+ capas)                      │
│  Código → Tokens → AST → IR → Optimizer → ASM → Linker → Bin│
├─────────────────────────────────────────────────────────────┤
│  ADead-BIB (2-3 capas)                                      │
│  Código → AST → BYTES DIRECTOS → Binario/HEX                │
└─────────────────────────────────────────────────────────────┘
```

---

## Parte I: Identidad del Lenguaje

### 1.1 Principios Fundamentales

| Principio | Descripción |
|-----------|-------------|
| **Sin ASM intermedio** | Emitimos bytes x86-64 directamente |
| **Sin linker externo** | Generamos PE/ELF completos en memoria |
| **Sin runtime pesado** | El binario es autosuficiente |
| **HEX es ciudadano de primera clase** | Puedes escribir bytes literales |
| **CPU y GPU son iguales** | Misma sintaxis, diferentes targets |

### 1.2 Sensación del Lenguaje

ADead-BIB debe sentirse como:

| Aspecto | Inspiración | Resultado |
|---------|-------------|-----------|
| **Al escribir** | Python | Fluidez, sintaxis limpia |
| **Al estructurar** | Rust | Disciplina, reglas claras |
| **Al ejecutar** | ASM | Control total, bytes directos |
| **Al pensar** | OOP | Objetos como mini-máquinas |

### 1.3 Regla de Oro

> **El 80% del código debe escribirse sin pensar en bytes.**
> **El 20% restante tiene acceso total a la máquina.**

---

## Parte II: Sistema de Tipos

### 2.1 Tipos Primitivos

```rust
// Enteros con signo
i8, i16, i32, i64

// Enteros sin signo  
u8, u16, u32, u64

// Otros
bool        // true/false → 1/0
string      // puntero + longitud, UTF-8, inmutable
ptr<T>      // puntero tipado
```

**Regla:** El tipo define EXACTAMENTE los bytes que se emiten.

### 2.2 Literales Numéricos

```rust
let decimal = 42              // Decimal
let hex = 0xFF                // Hexadecimal
let hex_sep = 0xFF_FF         // Con separadores
let binary = 0b11110000       // Binario
let binary_sep = 0b1111_0000  // Con separadores
let octal = 0o755             // Octal
```

### 2.3 Strings

```rust
let msg = "Hola, ADead-BIB"
```

**Contrato interno:**
- Bytes en `.rodata`
- Longitud conocida en compilación
- UTF-8 por defecto
- Sin null-termination implícita

---

## Parte III: Sintaxis Core

### 3.1 Variables

```rust
let x = 42                    // Inmutable por defecto
let mut counter = 0           // Mutable explícito
const PI = 3                  // Constante de compilación
```

**Ubicación:** El compilador decide (stack/registro). El usuario no elige para lo común.

### 3.2 Funciones

```rust
fn add(a, b) {
    return a + b
}

fn main() {
    let result = add(10, 20)
    println(result)
}
```

**Contrato:**
- Calling convention Windows x64: RCX, RDX, R8, R9
- Retorno en RAX
- Stack alineado a 16 bytes

### 3.3 Control de Flujo

```rust
// Condicionales
if x == 0xFF {
    println("Max byte!")
} else if x == 0 {
    println("Zero")
} else {
    println("Other")
}

// Bucles
for i in 0..10 {
    println(i)
}

while condition {
    // ...
}

loop {
    if done { break }
}
```

**Internamente:** Se traduce a `cmp` + `jmp`, pero el usuario no ve flags.

---

## Parte IV: OOP Binario

### 4.1 Filosofía

> **Los objetos son mini-máquinas binarias, no abstracciones conceptuales.**

Un objeto en ADead-BIB es:
- Un layout de memoria fijo
- Métodos = funciones con `self` como puntero
- Sin herencia profunda
- Polimorfismo = vtable simple

### 4.2 Structs e Implementaciones

```rust
struct Player {
    x: i32,
    y: i32,
    health: u8
}

impl Player {
    fn new(x, y) {
        return Player { x: x, y: y, health: 100 }
    }
    
    fn move(self, dx, dy) {
        self.x += dx
        self.y += dy
    }
    
    fn is_alive(self) {
        return self.health > 0
    }
}

fn main() {
    let player = Player::new(10, 20)
    player.move(5, -3)
}
```

### 4.3 Traits (Polimorfismo Simple)

```rust
trait Drawable {
    fn draw(self)
}

impl Drawable for Player {
    fn draw(self) {
        // Dibujar jugador
    }
}
```

**Contrato interno:**
- Una tabla de punteros (vtable)
- Un índice por método
- Un call indirecto

**Sin:** lifetimes complejos, inferencia profunda, metaprogramación pesada.

### 4.4 CPU y GPU como Objetos

```rust
// CPU y GPU usan la misma sintaxis
let gpu = GPU::init()
gpu.alloc(4096)
gpu.matmul(a, b, c)
gpu.sync()

// El usuario NO cambia de paradigma
```

---

## Parte V: Niveles de Acceso

### 5.1 Nivel Normal (80% del código)

```rust
fn main() {
    let x = 42
    println("Hello, ADead-BIB!")
    
    for i in 0..10 {
        println(i)
    }
}
```

- Sin bytes visibles
- Sin registros
- Sin preocupaciones

### 5.2 Nivel Avanzado (módulos especiales)

```rust
fn optimized() {
    cpu::mov(rcx, 1000000)
    cpu::xor(rax, rax)
    
    gpu::init()
    gpu::matmul(a, b, c)
}
```

- Acceso a instrucciones directas
- Registros como constantes tipadas
- Validación en compilación

### 5.3 Nivel Peligroso (explícito)

```rust
unsafe {
    emit![0x48, 0x31, 0xC0]  // xor rax, rax
    emit![0xC3]              // ret
}
```

- Bytes crudos
- Sin validación
- Poder total

---

## Parte VI: Módulos Core

### 6.1 Módulos Estándar Mínimos

```rust
core::io      // println, print, input
core::mem     // alloc, free, copy
core::cpu     // Instrucciones x86-64 directas
core::gpu     // Opcodes GPU (0xC0DA...)
core::panic   // Manejo de errores fatales
core::sys     // Syscalls del sistema
```

### 6.2 Intrínsecos del Compilador

| Función | Descripción | Emite |
|---------|-------------|-------|
| `println(x)` | Imprimir con newline | Syscall write |
| `print(x)` | Imprimir sin newline | Syscall write |
| `input()` | Leer entrada | Syscall read |
| `len(s)` | Longitud de string | Valor inmediato |
| `panic(msg)` | Error fatal | Exit con código |

**Regla:** Los intrínsecos son funciones que el compilador conoce y emite bytes específicos. No hay runtime oculto.

---

## Parte VII: Manejo de Errores

### 7.1 Filosofía

> **Errores como contratos, no excepciones.**

ADead-BIB no tiene:
- Excepciones
- Stack unwinding complejo
- Try/catch oculto

### 7.2 Errores de Compilación

```
error[E001]: variable 'x' not defined
  --> main.adB:5:10
   |
 5 |     println(x)
   |             ^ not found in this scope
```

### 7.3 Errores de Runtime

```rust
// Opción 1: panic explícito
file.open() or panic("No se pudo abrir")

// Opción 2: Result simple
let result = file.open()
if result.is_err() {
    println("Error al abrir")
    return
}
```

### 7.4 Panic

```rust
panic("División por cero")
```

**Contrato:**
- Imprime mensaje
- Termina con código de error
- Sin cleanup complejo

---

## Parte VIII: CPU Backend

### 8.1 Arquitectura

```
Código ADead-BIB
      ↓
┌─────────────────────────────────┐
│       BINARY EMITTER            │
│                                 │
│  codegen_v2.rs → bytes x86-64   │
│  pe.rs         → Windows PE     │
│  elf.rs        → Linux ELF      │
│  pe_tiny.rs    → PE <500 bytes  │
└─────────────────────────────────┘
      ↓
.exe / .elf (Binario Nativo)
```

### 8.2 Tabla de Opcodes x86-64

| Instrucción | Bytes | Descripción |
|-------------|-------|-------------|
| `push rbp` | `0x55` | Guardar base pointer |
| `mov rbp, rsp` | `0x48 0x89 0xE5` | Setup stack frame |
| `pop rbp` | `0x5D` | Restaurar base pointer |
| `ret` | `0xC3` | Retornar |
| `xor rax, rax` | `0x48 0x31 0xC0` | Limpiar rax |
| `call rel32` | `0xE8 [4 bytes]` | Llamar función |

### 8.3 Calling Convention

```
Parámetros: RCX, RDX, R8, R9 (primeros 4)
            Stack (resto)
Retorno:    RAX
Preservar:  RBX, RBP, RDI, RSI, R12-R15
Alineación: Stack a 16 bytes antes de call
```

---

## Parte IX: GPU Backend

### 9.1 Arquitectura de Dos Niveles

```
Nivel 1: Opcodes ADead-BIB (0xC0DA...)
  - Formato propio
  - Portable
  - Documentado

Nivel 2: Backend por target
  ├── SPIR-V  → Todas las GPUs Vulkan
  ├── CUDA    → NVIDIA (PTX)
  └── Vulkan  → Runtime directo
```

### 9.2 Tabla de Opcodes GPU

| Opcode | HEX | Descripción |
|--------|-----|-------------|
| `GPU_INIT` | `0xC0DA0001` | Inicializar contexto |
| `GPU_SHUTDOWN` | `0xC0DA0002` | Cerrar contexto |
| `GPU_ALLOC` | `0xC0DA0010` | Reservar memoria |
| `GPU_FREE` | `0xC0DA0011` | Liberar memoria |
| `GPU_COPY_H2D` | `0xC0DA0012` | Host → Device |
| `GPU_COPY_D2H` | `0xC0DA0013` | Device → Host |
| `GPU_MATMUL` | `0xC0DA0020` | Multiplicación matrices |
| `GPU_SYNC` | `0xC0DA00F0` | Sincronizar |
| `GPU_END` | `0xC0DAFFFF` | Fin programa |

### 9.3 Relación CPU ↔ GPU

```
CPU prepara → GPU ejecuta → CPU recibe

CPU:
  1. Escribe datos en memoria
  2. Escribe comandos GPU
  3. Dispara ejecución
  4. Se aparta

GPU:
  1. Lee comandos
  2. Ejecuta kernels
  3. Escribe resultados
  4. Sin volver a preguntar
```

**La CPU NO mira cada iteración. La GPU NO pide permiso.**

---

## Parte X: Formatos de Salida

### 10.1 Formatos Soportados

| Formato | Extensión | Descripción |
|---------|-----------|-------------|
| **PE Standard** | `.exe` | Windows ejecutable |
| **PE Tiny** | `.exe` | Windows <500 bytes |
| **ELF** | (sin ext) | Linux ejecutable |
| **Raw Binary** | `.bin` | Bytes puros sin header |
| **Intel HEX** | `.hex` | Formato HEX estándar |
| **AHYB** | `.ahyb` | Híbrido CPU+GPU |

### 10.2 Modo Raw

```rust
#![mode(raw)]
#![base(0x1000)]

fn _start() {
    // Solo bytes de código
}
```

### 10.3 Formato AHYB

```
┌─────────────────────────────────┐
│ Header AHYB (8 bytes)           │
│   Magic: "AHYB"                 │
│   Version: u8                   │
│   CPU_size: u16                 │
│   GPU_size: u16                 │
├─────────────────────────────────┤
│ CPU Section (bytes x86-64)      │
├─────────────────────────────────┤
│ GPU Section (opcodes HEX)       │
└─────────────────────────────────┘
```

---

## Parte XI: Ejemplos Completos

### 11.1 Hello World

```rust
fn main() {
    println("Hello, ADead-BIB!")
}
```

### 11.2 Operaciones con Literales

```rust
fn main() {
    let hex = 0xFF
    let bin = 0b11110000
    let sum = hex + bin
    
    println("Sum: ")
    println(sum)  // 495
}
```

### 11.3 OOP Completo

```rust
struct Vector2 {
    x: i32,
    y: i32
}

impl Vector2 {
    fn new(x, y) {
        return Vector2 { x: x, y: y }
    }
    
    fn add(self, other) {
        return Vector2 {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
    
    fn magnitude_squared(self) {
        return self.x * self.x + self.y * self.y
    }
}

fn main() {
    let a = Vector2::new(3, 4)
    let b = Vector2::new(1, 2)
    let c = a.add(b)
    
    println(c.x)  // 4
    println(c.y)  // 6
}
```

### 11.4 GPU Compute

```rust
fn main() {
    let gpu = GPU::init()
    
    // Reservar memoria
    let a = gpu.alloc(4096)
    let b = gpu.alloc(4096)
    let c = gpu.alloc(4096)
    
    // Multiplicación de matrices
    gpu.matmul(a, b, c)
    gpu.sync()
    
    // Copiar resultado a CPU
    gpu.copy_d2h(c, result)
    
    gpu.shutdown()
}
```

### 11.5 Acceso Directo a CPU

```rust
fn fast_counter() {
    unsafe {
        cpu::mov(rcx, 1000000)
        cpu::xor(rax, rax)
        
        loop {
            cpu::inc(rax)
            cpu::dec(rcx)
            if rcx == 0 { break }
        }
    }
    
    return rax
}
```

---

## Parte XII: Comparación con Otros Lenguajes

| Característica | C | Rust | Python | ADead-BIB |
|----------------|---|------|--------|-----------|
| **Compilación** | ASM → Linker | LLVM | Interpretado | Bytes directos |
| **Runtime** | libc | std | CPython | Ninguno |
| **Tamaño Hello World** | ~50 KB | ~150 KB | N/A | ~1.5 KB |
| **Acceso a hardware** | Inline ASM | unsafe | No | Nativo |
| **GPU** | CUDA/OpenCL | wgpu | No | Integrado |
| **OOP** | Manual | Traits | Classes | Híbrido |

---

## Parte XIII: Lógicas de Uso

### 13.1 Lo Común Debe Ser Corto

> El 80% del código debe escribirse sin pensar en bytes.

```rust
user.login()
user.logout()
player.move(1, 0)
```

### 13.2 OOP Como Modelo Mental

- Los objetos son layouts de memoria
- Los métodos son funciones con `self` explícito
- Sin herencia profunda
- Polimorfismo = vtable simple

### 13.3 Métodos > Funciones Sueltas

> Todo lo que tenga estado debe ser método.

```rust
gpu.init()
gpu.alloc(4096)
gpu.matmul(a, b, c)
```

### 13.4 Explicitud Silenciosa

- Explícito internamente
- Silencioso externamente
- El humano no escribe ruido

### 13.5 ASM Como Modo, No Como Lenguaje

```rust
// Código normal
player.move(1, 0)

// Modo ASM (explícito)
unsafe {
    cpu::inc(rax)
}
```

---

## Parte XIV: Resumen de Diseño

### Lo Normal es Fácil

```rust
println("Hello")
for i in 0..10 { }
let x = 42
```

### Lo Avanzado es Visible

```rust
cpu::mov(rax, 42)
gpu::matmul(a, b, c)
```

### Lo Peligroso es Explícito

```rust
unsafe {
    emit![0x48, 0x31, 0xC0]
}
```

---

## Parte XV: Próximos Pasos de Diseño

1. **OOP Core Spec v1.0** — Definir formalmente structs, impl, traits
2. **Vtables Simples** — Implementación de polimorfismo
3. **Módulo cpu::** — Funciones de instrucciones directas
4. **Módulo gpu::** — Funciones de opcodes GPU
5. **emit![] Macro** — Inserción de bytes directos
6. **Formato AHYB** — Binarios híbridos CPU+GPU
7. **Arquitectura main.adB + call.adB** — Separación binaria

---

## Parte XVI: Arquitectura Binaria Dual (main.adB + call.adB)

### 16.1 Concepto Fundamental

No es "dos archivos por comodidad". Es **separación de responsabilidades binaria**.

> **Separar BINARIO FUNDAMENTAL de BINARIO DE COMPORTAMIENTO**

Esto es una **decisión de arquitectura**, no de estilo.

### 16.2 Roles de Cada Archivo

#### `main.adB` — BINARIO BASE (ROOT)

Este archivo:
- Define el **punto de entrada real**
- Crea el **layout inicial**
- Controla el **flujo global**
- Inicializa CPU / GPU
- **NO** tiene lógica compleja

Es el **esqueleto binario**.

```rust
// main.adB - Binario Base
fn _start() {
    core::init()
    call::run()
    core::shutdown()
}
```

Piensa en `main.adB` como:
- Bootloader lógico
- Entrypoint
- Controlador

#### `call.adB` — BINARIO DE COMPORTAMIENTO (OOP PURO)

Este archivo:
- Define **objetos**
- Define **métodos**
- Define **lógica de alto nivel**
- NO conoce el entrypoint
- NO emite bytes peligrosos

```rust
// call.adB - Lógica OOP
struct Engine {
    running: bool
}

impl Engine {
    fn new() {
        return Engine { running: false }
    }
    
    fn run(self) {
        self.running = true
        println("Engine running")
    }
    
    fn shutdown(self) {
        self.running = false
        println("Engine stopped")
    }
}

// Función exportada
pub fn run() {
    let engine = Engine::new()
    engine.run()
    engine.shutdown()
}
```

### 16.3 Por Qué Esta Separación es Necesaria

#### Problema Clásico del ASM

Todo vive en el mismo archivo:
- Entrypoint
- Llamadas
- Datos
- Lógica

**Resultado:** Caos, difícil mantenimiento, imposible escalar.

#### Solución ADead-BIB

| Archivo | Rol | Cambios |
|---------|-----|---------|
| `main.adB` | Binario **estable** | Rara vez |
| `call.adB` | Binario **evolutivo** | Frecuente |

> Cambias lógica **sin tocar el core binario**.

### 16.4 Cooperación Sin Linker Externo

No usamos linker externo. La cooperación es **explícita y binaria**.

#### Modelo de Linking Interno

`call.adB` expone una tabla clara de métodos:

```
CALL_TABLE:
  0x00 → Engine::new
  0x01 → Engine::run
  0x02 → Engine::shutdown
  0x10 → run (función pública)
```

`main.adB` conoce estos símbolos:

```rust
// main.adB
fn _start() {
    core::init()
    call::run()   // Salta a offset conocido en CALL_TABLE
    core::shutdown()
}
```

### 16.5 VTable Binaria Simple

```
┌─────────────────────────────────────┐
│ CALL_TABLE Header                   │
│   Magic: "CALL"                     │
│   Version: u8                       │
│   Entry_count: u16                  │
├─────────────────────────────────────┤
│ Entry 0: offset → Engine::new       │
│ Entry 1: offset → Engine::run       │
│ Entry 2: offset → Engine::shutdown  │
│ Entry 16: offset → run              │
├─────────────────────────────────────┤
│ Code Section                        │
│   [bytes de Engine::new]            │
│   [bytes de Engine::run]            │
│   [bytes de Engine::shutdown]       │
│   [bytes de run]                    │
└─────────────────────────────────────┘
```

### 16.6 Ventaja: OOP Sin Runtime

| Tradicional | ADead-BIB |
|-------------|-----------|
| OOP necesita runtime | OOP es binario puro |
| VTables dinámicas | VTable = tabla binaria |
| RTTI pesado | Métodos = offsets |
| Objetos complejos | Objetos = memoria plana |

> **OOP sin runtime. ASM organizado.**

### 16.7 Estructura de Proyecto Recomendada

```
project/
├── main.adB          # Binario base (entrypoint)
├── call.adB          # Lógica OOP pura
├── core/             # Intrínsecos del sistema
│   ├── init.adB
│   └── shutdown.adB
├── cpu/              # Módulos CPU
├── gpu/              # Módulos GPU
└── build.adB         # Configuración de build
```

### 16.8 Reglas de Visibilidad

```rust
// call.adB

// Público - exportado a CALL_TABLE
pub fn run() { ... }

// Privado - solo interno
fn helper() { ... }

// Público - struct exportado
pub struct Engine { ... }
```

### 16.9 Contrato de Exportación (CALL CONTRACT v1.0)

```rust
// call.adB debe declarar explícitamente qué exporta

#![exports(run, Engine)]

pub fn run() { ... }

pub struct Engine { ... }
```

```rust
// main.adB importa explícitamente

#![imports(call: run)]

fn _start() {
    call::run()
}
```

### 16.10 Beneficios del Modelo

| Beneficio | Descripción |
|-----------|-------------|
| **Código limpio** | Separación clara de responsabilidades |
| **Menos ASM mezclado** | OOP vive en call.adB |
| **Binarios estables** | main.adB cambia poco |
| **Evolución segura** | Cambias lógica sin tocar core |
| **Menos bugs** | Aislamiento de complejidad |
| **Mentalidad clara** | "main = no toco, call = programo" |

### 16.11 Ejemplo Completo

#### `main.adB`

```rust
#![imports(call: run)]

fn _start() {
    // Inicialización del sistema
    core::init()
    
    // Ejecutar lógica principal
    call::run()
    
    // Limpieza
    core::shutdown()
}
```

#### `call.adB`

```rust
#![exports(run)]

struct Player {
    x: i32,
    y: i32,
    health: u8
}

impl Player {
    fn new() {
        return Player { x: 0, y: 0, health: 100 }
    }
    
    fn move(self, dx, dy) {
        self.x += dx
        self.y += dy
    }
    
    fn is_alive(self) {
        return self.health > 0
    }
}

struct Game {
    player: Player,
    running: bool
}

impl Game {
    fn new() {
        return Game {
            player: Player::new(),
            running: false
        }
    }
    
    fn start(self) {
        self.running = true
        println("Game started!")
    }
    
    fn update(self) {
        if self.player.is_alive() {
            self.player.move(1, 0)
        }
    }
    
    fn stop(self) {
        self.running = false
        println("Game stopped!")
    }
}

// Función pública exportada
pub fn run() {
    let game = Game::new()
    game.start()
    
    for i in 0..100 {
        game.update()
    }
    
    game.stop()
}
```

---

## Parte XVII: Resumen de Arquitectura

### División por Verdad Binaria

| Archivo | Contenido | Frecuencia de Cambio |
|---------|-----------|---------------------|
| `main.adB` | Entrypoint, init, shutdown | Rara vez |
| `call.adB` | OOP, lógica, comportamiento | Frecuente |

### Flujo de Ejecución

```
main.adB::_start()
    ↓
core::init()
    ↓
call::run()  ──→  [OOP puro en call.adB]
    ↓
core::shutdown()
    ↓
exit
```

### Principio Clave

> **No estás dividiendo por comodidad,
> estás dividiendo por VERDAD BINARIA.**

---

**ADead-BIB: El lenguaje que domestica la máquina sin mentir.**

**Código → Bytes → Binario**
**Sin ASM. Sin LLVM. Sin mentiras.**
