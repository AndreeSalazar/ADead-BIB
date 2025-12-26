# 🔥 ADead-BIB v1.2.0

**Abstract Dead - Binary In Binary**

> **El lenguaje de la familia Assembly** con sintaxis de alto nivel. Escribe como en Rust/Python, ejecuta como ASM puro.

```
Código .adB → Lexer → Parser → AST → CodeGen → x86-64 Opcodes → PE/ELF Binario
                                                    ↑
                                          SIN ensamblador externo
                                          SIN linker
                                          DIRECTO al .exe/.elf
```

---

## 🧬 ADead-BIB = Assembly Moderno

**ADead-BIB pertenece a la familia de lenguajes Assembly**, pero con una diferencia crucial:

| Aspecto | Assembly Tradicional | ADead-BIB |
|---------|---------------------|-----------|
| **Sintaxis** | `mov rax, 42` | `let x = 42` |
| **Legibilidad** | Baja | Alta (Rust/Python) |
| **Productividad** | Lenta | Rápida |
| **Control** | Total | Total |
| **Binario** | Directo | **Directo** |
| **Tamaño** | Mínimo | **Mínimo (~1.5 KB)** |

### ¿Por qué es familia ASM?

```
┌─────────────────────────────────────────────────────────────┐
│  print("Hola")                                              │
│       ↓                                                     │
│  mov rcx, 0x140003000    ; dirección del string             │
│  sub rsp, 40             ; shadow space                     │
│  call printf             ; syscall directo                  │
│       ↓                                                     │
│  48 B9 00 30 00 40 01 00 00 00  ; opcodes x86-64 REALES    │
│  48 83 EC 28                                                │
│  FF 15 XX XX XX XX                                          │
└─────────────────────────────────────────────────────────────┘
```

**No hay capas intermedias.** Tu código se convierte directamente en instrucciones de CPU.

---

## 🇵🇪 Hecho con ❤️ en Perú

**Autor:** Eddi Andreé Salazar Matos  
**Email:** eddi.salazar.dev@gmail.com  
**Licencia:** Apache 2.0

---

## 🎯 ¿Qué es ADead-BIB?

ADead-BIB es un **lenguaje de programación de la familia Assembly** que genera código máquina nativo directamente. Combina:

- **La eficiencia de Assembly** → Control total, binarios mínimos
- **La sintaxis de Rust/Python** → Productividad, legibilidad
- **Sin intermediarios** → No hay VM, bytecode, ni ensamblador externo

### Filosofía del Lenguaje

- **Familia ASM**: Genera opcodes x86-64 directamente, como escribir ASM
- **Sintaxis moderna**: Escribe `let x = 42` en vez de `mov rax, 42`
- **Binarios ultra-pequeños**: ~1.5 KB (vs 150 KB en Rust, 2 MB en Go)
- **OOP completo**: Clases, herencia, polimorfismo, traits
- **100% Rust**: Compilador escrito completamente en Rust

---

## 🌍 Ventajas por Contexto de Uso

### 🖥️ Servidores y Backend

| Ventaja | Impacto |
|---------|---------|
| **Binarios de 1.5 KB** | Despliegue instantáneo, menos almacenamiento |
| **Sin runtime** | Menor consumo de RAM |
| **Arranque inmediato** | Cold start en microsegundos (ideal para serverless) |
| **Control de memoria** | Sin garbage collector, latencia predecible |

```
Caso de uso: Microservicios ultra-ligeros, funciones Lambda, APIs de alta frecuencia
```

### 💻 PC y Aplicaciones de Escritorio

| Ventaja | Impacto |
|---------|---------|
| **Ejecutables pequeños** | Distribución fácil, sin instaladores pesados |
| **Rendimiento nativo** | Velocidad de C/ASM con sintaxis moderna |
| **Sin dependencias** | No necesita runtime instalado |
| **Inicio instantáneo** | La app abre inmediatamente |

```
Caso de uso: Herramientas CLI, utilidades del sistema, aplicaciones portables
```

### 🔧 Sistemas Embebidos e IoT

| Ventaja | Impacto |
|---------|---------|
| **< 2 KB de código** | Cabe en microcontroladores pequeños |
| **Control de hardware** | Acceso directo a registros y memoria |
| **Sin overhead** | Cada byte cuenta en sistemas limitados |
| **Determinístico** | Tiempo de ejecución predecible |

```
Caso de uso: Firmware, controladores, dispositivos IoT con memoria limitada
```

### 🎮 Desarrollo de Juegos

| Ventaja | Impacto |
|---------|---------|
| **GPU Support (Vulkan)** | Shaders SPIR-V nativos |
| **Baja latencia** | Sin pausas de GC |
| **Binarios compactos** | Juegos que pesan kilobytes |

```
Caso de uso: Game engines minimalistas, demoscene, juegos retro
```

### 📊 Comparación de Ecosistemas

| Lenguaje | Familia | Binario Hello World | Runtime | Ideal para |
|----------|---------|---------------------|---------|------------|
| **ADead-BIB** | **ASM** | **~1.5 KB** | **Ninguno** | **Todo lo anterior** |
| Assembly | ASM | ~500 bytes | Ninguno | Bajo nivel puro |
| C | Compilado | ~50 KB | libc | Sistemas, embebidos |
| Rust | Compilado | ~150 KB | std | Sistemas seguros |
| Go | Compilado | ~2 MB | Runtime Go | Servidores |
| Python | Interpretado | ~5 MB (.exe) | Python VM | Scripts, ML |
| Java | Bytecode | ~5 MB | JVM | Enterprise |

**ADead-BIB combina lo mejor**: tamaño de ASM + productividad de lenguajes modernos.

---

## ⚡ Características Principales

| Característica | Estado | Descripción |
|----------------|--------|-------------|
| **Sintaxis Rust + Python** | ✅ | `fn`/`def`, `let`/asignación directa |
| **Compilación directa** | ✅ | Genera opcodes x86-64 directamente |
| **OOP completo** | ✅ | Clases, herencia, polimorfismo |
| **Traits e Interfaces** | ✅ | Abstracción de comportamiento |
| **Scripts sin main()** | ✅ | Código ejecutable directo |
| **Secuencias de escape** | ✅ | `\n`, `\t`, `\r` en strings |
| **Modo Playground** | ✅ | REPL interactivo |
| **Binarios ultra-pequeños** | ✅ | < 2 KB típicamente |
| **GPU Support** | ✅ | Vulkan + SPIR-V |
| **100% Rust** | ✅ | Sin dependencias C++ |

---

## 🚀 Instalación

### Requisitos
- Rust 1.70+ (rustup)
- Windows 10/11 o Linux

### Instalación Rápida

```bash
# Clonar repositorio
git clone https://github.com/tu-usuario/ADead-BIB.git
cd ADead-BIB

# Instalar globalmente
cargo install --path .

# Verificar instalación
adeadc --help
```

---

## 📋 Comandos del Compilador

```bash
# Ejecutar programa (compila y ejecuta)
adeadc run archivo.adB

# Compilar a ejecutable
adeadc build archivo.adB
adeadc build archivo.adB -o mi_programa.exe

# Verificar sintaxis
adeadc check archivo.adB

# Modo interactivo (REPL/Playground)
adeadc play

# Modos de binario ultra-compacto
adeadc tiny archivo.adB      # < 500 bytes
adeadc nano output.exe       # ~1 KB
adeadc micro output.exe      # < 256 bytes (x86)

# GPU/Vulkan
adeadc gpu                   # Detectar GPU
adeadc spirv matmul 1024     # Generar shader SPIR-V
adeadc vulkan                # Inicializar Vulkan
```

---

## 📝 Sintaxis del Lenguaje

### Hello World

```rust
// La forma más simple - Script directo
print("Hello, ADead-BIB!")

// Con función main estilo Rust
fn main() {
    print("Hola desde Rust-style!")
}

// Con función main estilo Python
def main():
    print("Hola desde Python-style!")
```

### Variables

```rust
// Estilo Rust
let x = 42
let mut contador = 0
const PI = 3

// Estilo Python
x = 42
nombre = "ADead-BIB"
```

### Tipos de Datos

```rust
// Enteros
let entero = 42
let negativo = -17
let grande = 1_000_000    // Separadores de miles

// Strings con secuencias de escape
let texto = "Hola\nMundo"   // Salto de línea
let tab = "Col1\tCol2"      // Tabulación

// Booleanos
let verdadero = true
let falso = false
```

### Funciones

```rust
// Estilo Rust con tipos
fn sumar(a: i32, b: i32) -> i32 {
    return a + b
}

// Estilo Python
def multiplicar(x, y):
    return x * y

// Llamar funciones
let resultado = sumar(10, 20)
print("Resultado:")
print(resultado)
```

### Control de Flujo

```rust
// If-else estilo Rust
if edad >= 18 {
    print("Mayor de edad")
} else {
    print("Menor de edad")
}

// If-elif-else estilo Python
if nota >= 90:
    print("Excelente")
elif nota >= 80:
    print("Muy bien")
elif nota >= 70:
    print("Bien")
else:
    print("Necesita mejorar")

// While loop
let i = 0
while i < 10 {
    print(i)
    i = i + 1
}

// For loop (Python-style)
for i in range(10):
    print(i)
```

---

## 🏗️ Programación Orientada a Objetos

### Structs (Rust-style)

```rust
struct Punto {
    x: i32,
    y: i32,
}

impl Punto {
    fn new(x: i32, y: i32) -> Punto {
        return Punto { x: x, y: y }
    }
    
    fn distancia(&self) -> i32 {
        return self.x + self.y
    }
    
    fn mover(&mut self, dx: i32, dy: i32) {
        self.x = self.x + dx
        self.y = self.y + dy
    }
}

// Uso
let p = Punto { x: 10, y: 20 }
let d = p.distancia()
```

### Clases con Herencia (Python/C++ style)

```python
class Animal:
    nombre = ""
    edad = 0
    
    def __init__(self, nombre, edad):
        self.nombre = nombre
        self.edad = edad
    
    virtual def hablar(self):
        print("...")
    
    def info(self):
        print("Nombre:")
        print(self.nombre)

class Perro extends Animal:
    raza = ""
    
    override def hablar(self):
        print("Guau guau!")
    
    def ladrar(self):
        print("GUAU!")

class Gato extends Animal:
    override def hablar(self):
        print("Miau!")
```

### Traits e Interfaces

```rust
// Trait estilo Rust
trait Dibujable {
    fn dibujar(&self);
    fn obtener_color(&self) -> String;
}

// Interface estilo Python/Java
interface Movible:
    def mover(self, x: i32, y: i32)
    def obtener_posicion(self)

// Implementación de trait
impl Dibujable for Sprite {
    fn dibujar(&self) {
        print("Dibujando sprite...")
    }
    
    fn obtener_color(&self) -> String {
        return "rojo"
    }
}

// Clase con múltiples interfaces
class Sprite implements Dibujable, Movible:
    x = 0
    y = 0
    
    def dibujar(self):
        print("Sprite en pantalla")
    
    def mover(self, dx, dy):
        self.x = self.x + dx
        self.y = self.y + dy
```

---

## 🎮 Modo Playground (REPL)

El modo playground permite escribir y ejecutar código de forma interactiva:

```bash
adeadc play
```

```
╔══════════════════════════════════════════════════════════════╗
║        🎮 ADead-BIB Playground v0.2.0 🎮                     ║
║     Modo interactivo - Escribe código y presiona Enter       ║
╚══════════════════════════════════════════════════════════════╝

adB[1]> print("Hola!")
▶️  Ejecutando...
   → Hola!
✅ Ejecución completada

adB[2]> let x = 42
   x = 42

adB[3]> :help
🎮 ADead-BIB Playground - Ayuda
...

adB[4]> :exit
👋 ¡Hasta luego!
```

### Comandos del Playground

| Comando | Atajo | Descripción |
|---------|-------|-------------|
| `:help` | `:h` | Mostrar ayuda |
| `:run` | `:r` | Ejecutar código en buffer |
| `:clear` | `:c` | Limpiar buffer |
| `:ast` | `:a` | Mostrar AST del código |
| `:tokens` | `:t` | Mostrar tokens |
| `:vars` | `:v` | Mostrar variables |
| `:example` | `:e` | Cargar ejemplo |
| `:exit` | `:q` | Salir |

---

## ⚡ Tamaños de Binario

ADead-BIB genera binarios **extremadamente pequeños** porque escribe opcodes directamente:

| Modo | Tamaño | Comando | Descripción |
|------|--------|---------|-------------|
| Standard | ~1.5 KB | `adeadc build` | Binario completo |
| Tiny | < 500 bytes | `adeadc tiny` | PE ultra-compacto |
| Nano | ~1 KB | `adeadc nano` | Mínimo válido x64 |
| Micro | < 256 bytes | `adeadc micro` | PE32 sub-256 bytes |

### Comparación con otros lenguajes

| Lenguaje | Hello World |
|----------|-------------|
| **ADead-BIB** | **~1.5 KB** |
| C (MinGW) | ~50 KB |
| Rust | ~150 KB |
| Go | ~2 MB |
| Python (.exe) | ~5 MB |

---

## 📁 Estructura del Proyecto

```
ADead-BIB/
├── src/rust/                    # Compilador (100% Rust)
│   ├── frontend/                # Frontend del compilador
│   │   ├── lexer.rs            # Tokenizador (Rust + Python syntax)
│   │   ├── parser.rs           # Parser (dual syntax)
│   │   ├── ast.rs              # Abstract Syntax Tree
│   │   └── type_checker.rs     # Verificación de tipos
│   ├── backend/                 # Backend de generación de código
│   │   ├── cpu/                # x86-64 directo
│   │   │   ├── codegen_v2.rs   # Generador principal
│   │   │   ├── pe.rs           # Binarios Windows (PE)
│   │   │   ├── elf.rs          # Binarios Linux (ELF)
│   │   │   └── syscalls.rs     # Syscalls directos
│   │   └── gpu/                # GPU/Vulkan
│   │       ├── vulkan_runtime.rs
│   │       └── bytecode_spirv.rs
│   ├── optimizer/              # Optimizaciones
│   ├── runtime/                # Runtime mínimo
│   ├── builder.rs              # Sistema de build
│   └── main.rs                 # CLI principal
├── examples/                    # Ejemplos del lenguaje
│   ├── 01_hello_world.adB
│   ├── 02_variables.adB
│   ├── 03_funciones.adB
│   ├── 04_control_flujo.adB
│   ├── 05_oop_clases.adB
│   ├── 06_herencia_polimorfismo.adB
│   ├── 07_traits_interfaces.adB
│   ├── 08_game_engine.adB
│   └── ...
├── Como se usa.md              # Guía completa del lenguaje
├── Cargo.toml                  # Configuración Rust
└── README.md                   # Este archivo
```

---

## 📚 Ejemplos

La carpeta `/examples` contiene ejemplos completos:

| Archivo | Descripción |
|---------|-------------|
| `01_hello_world.adB` | Hello World básico |
| `02_variables.adB` | Variables y tipos |
| `03_funciones.adB` | Funciones con tipos |
| `04_control_flujo.adB` | if/while/for |
| `05_oop_clases.adB` | Clases y structs |
| `06_herencia_polimorfismo.adB` | Herencia y override |
| `07_traits_interfaces.adB` | Traits e interfaces |
| `08_game_engine.adB` | Game engine demo |
| `09_async_concurrencia.adB` | Async (futuro) |
| `10_directo_binario.adB` | Compilación nativa |

### Ejecutar un ejemplo

```bash
adeadc run examples/01_hello_world.adB
```

Salida:
```
🚀 Running examples/01_hello_world.adB...

Hello, ADead-BIB!
Bienvenido al lenguaje que va directo al binario
Este es un ejemplo basico
```

---

## 🔧 ¿Por qué ADead-BIB?

### 1. **Directo al Binario (como ASM)**
ADead-BIB escribe opcodes x86-64 directamente al archivo ejecutable. No hay ensamblador intermedio, no hay linker externo.

```
print("Hola")  →  mov rcx, addr  →  48 B9 XX XX XX XX XX XX XX XX
                  call printf    →  FF 15 XX XX XX XX
```

### 2. **Sintaxis Familiar**
Puedes usar la sintaxis que prefieras - Rust o Python:

```rust
// Esto es válido
fn main() {
    let x = 42
}

// Y esto también
def main():
    x = 42
```

### 3. **Binarios Pequeños**
Los ejecutables son extremadamente pequeños porque no hay runtime pesado.

### 4. **OOP Completo**
Soporta todo lo que esperas de un lenguaje moderno:
- Clases y Structs
- Herencia (`extends`)
- Polimorfismo (`virtual`/`override`)
- Traits e Interfaces
- Métodos estáticos

### 5. **Scripts sin Main**
No necesitas función `main()`. Escribe código directamente:

```rust
print("Esto funciona!")
let x = 42
print(x)
```

---

## 🎮 GPU y Vulkan

ADead-BIB soporta computación en GPU:

```bash
# Detectar GPU disponible
adeadc gpu

# Generar shader SPIR-V para multiplicación de matrices
adeadc spirv matmul 1024

# Inicializar runtime Vulkan
adeadc vulkan
```

---

## 📖 Documentación Completa

Para una guía completa del lenguaje, ver:
- **[Como se usa.md](Como%20se%20usa.md)** - Guía detallada con ejemplos

---

## 🤝 Contribuir

1. Fork el repositorio
2. Crea una rama: `git checkout -b mi-feature`
3. Commit: `git commit -m 'Añadir feature'`
4. Push: `git push origin mi-feature`
5. Abre un Pull Request

---

## 📄 Licencia

Apache 2.0 - Ver archivo [LICENSE](LICENSE)

---

<div align="center">

## 🔥 ADead-BIB

**Assembly Moderno: La potencia de ASM con la productividad de Rust/Python**

```
┌────────────────────────────────────────────────────────────┐
│  Familia ASM → Binarios de 1.5 KB → Sin Runtime           │
│  Sintaxis Moderna → OOP Completo → GPU Vulkan             │
│  Servidores ✓ PC ✓ Embebidos ✓ Juegos ✓                   │
└────────────────────────────────────────────────────────────┘
```

*El único lenguaje que combina: tamaño de ASM + sintaxis de Rust + flexibilidad de Python*

[![Made in Peru](https://img.shields.io/badge/Made%20in-Peru-red)](https://github.com/tu-usuario/ADead-BIB)
[![Family ASM](https://img.shields.io/badge/Family-ASM-blue)](https://github.com/tu-usuario/ADead-BIB)
[![100% Rust](https://img.shields.io/badge/Compiler-100%25%20Rust-orange)](https://www.rust-lang.org/)
[![Binary Size](https://img.shields.io/badge/Binary-~1.5KB-green)](https://github.com/tu-usuario/ADead-BIB)
[![No Runtime](https://img.shields.io/badge/Runtime-None-purple)](https://github.com/tu-usuario/ADead-BIB)

</div>
