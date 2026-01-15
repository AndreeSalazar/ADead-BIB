# ADead-BIB Project Template

> Estructura de proyecto siguiendo la arquitectura binaria dual.
> **OOP Puro + ASM Simbionte = Lenguaje del Futuro**

## Estructura Completa

```
Project/
├── main.adB              # Binario base (entrypoint)
├── call.adB              # Lógica OOP pura (638 líneas)
├── core/                 # Intrínsecos del sistema
│   ├── mod.adB           # Módulo principal core
│   ├── init.adB          # Inicialización CPU/GPU
│   └── shutdown.adB      # Limpieza y shutdown
├── cpu/                  # Módulo CPU
│   └── mod.adB           # Instrucciones x86-64 directas
├── gpu/                  # Módulo GPU
│   └── mod.adB           # Opcodes GPU (0xC0DA...)
├── build.adB             # Configuración de build
└── README.md             # Este archivo
```

## Arquitectura Binaria Dual

### `main.adB` — Binario Base (ROOT)

- Define el **punto de entrada real**
- Controla el **flujo global**
- Inicializa CPU / GPU
- **NO** tiene lógica compleja

### `call.adB` — Binario de Comportamiento (OOP Puro)

- **Traits**: `Drawable`, `Updatable`, `Serializable`
- **Tipos matemáticos**: `Vec2`, `Vec3`, `Matrix4`
- **Entidades**: `Entity`, `Player`, `Enemy`, `Transform`
- **Sistema de juego**: `GameState` completo
- **GPU Compute**: `GpuCompute` para matrices y neural networks
- **CPU Optimizado**: `CpuOptimized` con instrucciones directas

## Módulos Implementados

### `cpu/mod.adB` — Instrucciones x86-64

```rust
// Registros tipados
cpu::rax, cpu::rbx, cpu::rcx, cpu::rdx...

// Instrucciones de movimiento
cpu::mov(dest, value)
cpu::push(reg), cpu::pop(reg)

// Aritméticas
cpu::add(dest, value), cpu::sub(dest, value)
cpu::inc(reg), cpu::dec(reg)

// Lógicas
cpu::and(dest, value), cpu::or(dest, value)
cpu::xor(dest, src), cpu::not(reg)

// Saltos
cpu::jmp(offset), cpu::je(offset), cpu::jne(offset)

// Llamadas
cpu::call(offset), cpu::ret()
```

### `gpu/mod.adB` — Opcodes GPU

```rust
// Registros GPU
gpu::reg0, gpu::reg1, gpu::reg2...

// Control
gpu::init()           // 0xC0DA0001
gpu::shutdown()       // 0xC0DA0002
gpu::sync()           // 0xC0DA00F0

// Memoria
gpu::alloc(size, reg)
gpu::free(reg)
gpu::copy_h2d(src, dest, size)

// Compute
gpu::matmul(a, b, c, m, n, k)
gpu::add(a, b, c, size)
gpu::relu(input, output, size)
gpu::softmax(input, output, size)
```

## Flujo de Ejecución

```
main.adB::_start()
    ↓
core::init()
    ↓
call::run()           ──→  [OOP puro: Game, Player, Enemy]
call::run_gpu_demo()  ──→  [GPU: matmul, neural layers]
call::run_cpu_demo()  ──→  [CPU: instrucciones directas]
    ↓
core::shutdown()
    ↓
exit
```

## Ejemplos de OOP Puro

### Traits y Polimorfismo

```rust
trait Drawable {
    fn draw(self)
}

impl Drawable for Player {
    fn draw(self) {
        print("Drawing player: ")
        println(self.entity.name)
    }
}
```

### Tipos Matemáticos

```rust
let a = Vec3::new(1.0, 2.0, 3.0)
let b = Vec3::new(4.0, 5.0, 6.0)
let c = a.cross(b)
```

### Sistema de Juego Completo

```rust
let game = GameState::new()
game.spawn_enemy("Goblin")
game.start()

for frame in 0..100 {
    game.update()
    game.render()
}

game.stop()
```

### GPU Compute

```rust
let compute = GpuCompute::new()
compute.init()
compute.matrix_multiply(256)
compute.neural_layer(784, 256)
compute.shutdown()
```

### CPU Optimizado

```rust
unsafe {
    cpu::mov(cpu::rcx, 1000000)
    cpu::xor(cpu::rax, cpu::rax)
    
    // Loop ultra-rápido
    cpu::inc(cpu::rax)
    cpu::dec(cpu::rcx)
    cpu::jnz(-10)
}
```

## Compilar y Ejecutar

```bash
# Compilar
adeadc build build.adB

# Ejecutar
adeadc run main.adB
```

## Juego Funcional (COMPILA DE VERDAD)

El directorio `game/` contiene un juego simple que **compila y ejecuta** con el backend actual:

```bash
# Ejecutar el juego
adeadc run game/main.adB

# O desde la raíz del proyecto ADead-BIB:
cargo run --bin adeadc -- run Project/game/main.adB
```

## Crear Nuevo Proyecto

ADead-BIB incluye un comando para crear proyectos nuevos:

```bash
# Crear proyecto nuevo
adeadc new mi_proyecto

# O la forma larga
adeadc create new mi_proyecto

# Luego
cd mi_proyecto
adeadc run main.adB
```

## Filosofía

> **ADead-BIB no abstrae la máquina, la domestica.**

| Nivel | Descripción | Ejemplo |
|-------|-------------|---------|
| **Normal** | OOP puro, sin bytes | `player.move(1, 0)` |
| **Avanzado** | Módulos cpu/gpu | `cpu::inc(rax)` |
| **Peligroso** | Bytes directos | `emit![0x48, 0x31, 0xC0]` |

---

**ADead-BIB v2.0: Código → Bytes → Binario**
**OOP Puro + ASM Simbionte = El Nuevo Lenguaje**
**Sin ASM. Sin LLVM. Sin mentiras.**
