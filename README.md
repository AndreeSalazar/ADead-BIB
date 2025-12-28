# 🔥 ADead-BIB v1.5.0

**Abstract Dead - Binary In Binary**

> **An Assembly-family language** with high-level syntax. Write like Rust/Python, execute as pure ASM.

```
.adB Code → Lexer → Parser → AST → CodeGen → x86-64 Opcodes → PE/ELF Binary
                                                    ↑
                                          NO external assembler
                                          NO linker
                                          DIRECT to .exe/.elf
```

---

## 🧬 ADead-BIB = Modern Assembly

**ADead-BIB belongs to the Assembly language family**, but with a crucial difference:

| Aspect | Traditional Assembly | ADead-BIB |
|--------|---------------------|-----------|
| **Syntax** | `mov rax, 42` | `let x = 42` |
| **Readability** | Low | High (Rust/Python) |
| **Productivity** | Slow | Fast |
| **Control** | Total | Total |
| **Binary** | Direct | **Direct** |
| **Size** | Minimal | **Minimal (~1.5 KB)** |

### Why is it ASM family?

```
┌─────────────────────────────────────────────────────────────┐
│  print("Hello")                                             │
│       ↓                                                     │
│  mov rcx, 0x140003000    ; string address                   │
│  sub rsp, 40             ; shadow space                     │
│  call printf             ; direct syscall                   │
│       ↓                                                     │
│  48 B9 00 30 00 40 01 00 00 00  ; REAL x86-64 opcodes      │
│  48 83 EC 28                                                │
│  FF 15 XX XX XX XX                                          │
└─────────────────────────────────────────────────────────────┘
```

**No intermediate layers.** Your code converts directly to CPU instructions.

---

## 🇵🇪 Made with ❤️ in Peru

**Author:** Eddi Andreé Salazar Matos  
**Email:** eddi.salazar.dev@gmail.com  
**License:** Apache 2.0

---

## 🎯 What is ADead-BIB?

ADead-BIB is an **Assembly-family programming language** that generates native machine code directly. It combines:

- **Assembly efficiency** → Total control, minimal binaries
- **Rust/Python syntax** → Productivity, readability
- **No intermediaries** → No VM, bytecode, or external assembler

### Language Philosophy

- **ASM Family**: Generates x86-64 opcodes directly, like writing ASM
- **Modern syntax**: Write `let x = 42` instead of `mov rax, 42`
- **Ultra-small binaries**: ~1.5 KB (vs 150 KB in Rust, 2 MB in Go)
- **Full OOP**: Classes, inheritance, polymorphism, traits
- **100% Rust**: Compiler written entirely in Rust

---

## 🌍 Advantages by Use Case

### 🖥️ Servers and Backend

| Advantage | Impact |
|-----------|--------|
| **1.5 KB binaries** | Instant deployment, less storage |
| **No runtime** | Lower RAM consumption |
| **Instant startup** | Cold start in microseconds (ideal for serverless) |
| **Memory control** | No garbage collector, predictable latency |

```
Use case: Ultra-light microservices, Lambda functions, high-frequency APIs
```

### 💻 PC and Desktop Applications

| Advantage | Impact |
|-----------|--------|
| **Small executables** | Easy distribution, no heavy installers |
| **Native performance** | C/ASM speed with modern syntax |
| **No dependencies** | No runtime installation needed |
| **Instant launch** | App opens immediately |

```
Use case: CLI tools, system utilities, portable applications
```

### 🔧 Embedded Systems and IoT

| Advantage | Impact |
|-----------|--------|
| **< 2 KB code** | Fits in small microcontrollers |
| **Hardware control** | Direct access to registers and memory |
| **No overhead** | Every byte counts in limited systems |
| **Deterministic** | Predictable execution time |

```
Use case: Firmware, drivers, IoT devices with limited memory
```

### 🎮 Game Development

| Advantage | Impact |
|-----------|--------|
| **GPU Support (Vulkan)** | Native SPIR-V shaders |
| **Low latency** | No GC pauses |
| **Compact binaries** | Games that weigh kilobytes |

```
Use case: Minimalist game engines, demoscene, retro games
```

### 📊 Ecosystem Comparison

| Language | Family | Hello World Binary | Runtime | Ideal for |
|----------|--------|-------------------|---------|-----------|
| **ADead-BIB** | **ASM** | **~1.5 KB** | **None** | **All of the above** |
| Assembly | ASM | ~500 bytes | None | Pure low-level |
| C | Compiled | ~50 KB | libc | Systems, embedded |
| Rust | Compiled | ~150 KB | std | Safe systems |
| Go | Compiled | ~2 MB | Go Runtime | Servers |
| Python | Interpreted | ~5 MB (.exe) | Python VM | Scripts, ML |
| Java | Bytecode | ~5 MB | JVM | Enterprise |

**ADead-BIB combines the best**: ASM size + modern language productivity.

---

## ⚡ Main Features

| Feature | Status | Description |
|---------|--------|-------------|
| **Rust + Python syntax** | ✅ | `fn`/`def`, `let`/direct assignment |
| **Direct compilation** | ✅ | Generates x86-64 opcodes directly |
| **Full OOP** | ✅ | Classes, inheritance, polymorphism |
| **Traits & Interfaces** | ✅ | Behavior abstraction |
| **Scripts without main()** | ✅ | Direct executable code |
| **Escape sequences** | ✅ | `\n`, `\t`, `\r` in strings |
| **Playground mode** | ✅ | Interactive REPL |
| **Ultra-small binaries** | ✅ | < 2 KB typically |
| **GPU Support** | ✅ | Vulkan + SPIR-V |
| **Module System** | ✅ | `import`, `from`, `as` (v1.5.0) |
| **Real input()** | ✅ | Read from stdin (v1.4.0) |
| **Arrays** | ✅ | `[1,2,3]`, `len()`, `for x in arr` (v1.3.0) |
| **100% Rust** | ✅ | No C++ dependencies |

---

## 🚀 Installation

### Requirements
- Rust 1.70+ (rustup)
- Windows 10/11 or Linux

### Quick Installation

```bash
# Clone repository
git clone https://github.com/your-user/ADead-BIB.git
cd ADead-BIB

# Install globally
cargo install --path .

# Verify installation
adeadc --help
```

---

## 📋 Compiler Commands

```bash
# Run program (compile and execute)
adeadc run file.adB

# Compile to executable
adeadc build file.adB
adeadc build file.adB -o my_program.exe

# Check syntax
adeadc check file.adB

# Interactive mode (REPL/Playground)
adeadc play

# Ultra-compact binary modes
adeadc tiny file.adB         # < 500 bytes
adeadc nano output.exe       # ~1 KB
adeadc micro output.exe      # < 256 bytes (x86)

# GPU/Vulkan
adeadc gpu                   # Detect GPU
adeadc spirv matmul 1024     # Generate SPIR-V shader
adeadc vulkan                # Initialize Vulkan
```

---

## 📝 Language Syntax

### Hello World

```rust
// Simplest form - Direct script
print("Hello, ADead-BIB!")

// With Rust-style main function
fn main() {
    print("Hello from Rust-style!")
}

// With Python-style main function
def main():
    print("Hello from Python-style!")
```

### Variables

```rust
// Rust style
let x = 42
let mut counter = 0
const PI = 3

// Python style
x = 42
name = "ADead-BIB"
```

### Data Types

```rust
// Integers
let integer = 42
let negative = -17
let big = 1_000_000    // Thousands separators

// Strings with escape sequences
let text = "Hello\nWorld"   // Line break
let tab = "Col1\tCol2"      // Tab

// Booleans
let is_true = true
let is_false = false
```

### Functions

```rust
// Rust style with types
fn add(a: i32, b: i32) -> i32 {
    return a + b
}

// Python style
def multiply(x, y):
    return x * y

// Call functions
let result = add(10, 20)
print("Result:")
print(result)
```

### Control Flow

```rust
// If-else Rust style
if age >= 18 {
    print("Adult")
} else {
    print("Minor")
}

// If-elif-else Python style
if grade >= 90:
    print("Excellent")
elif grade >= 80:
    print("Very good")
elif grade >= 70:
    print("Good")
else:
    print("Needs improvement")

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

### Arrays (v1.3.0)

```rust
// Array declaration
let numbers = [10, 20, 30, 40, 50]

// Length
let length = len(numbers)

// Iteration
for x in numbers {
    println(x)
}
```

### Modules (v1.5.0)

```rust
// Import from local module
from mymath import double, triple

// Import from standard library
from std::math import abs, max

fn main() {
    println(double(5))  // 10
}
```

---

## 🏗️ Object-Oriented Programming

### Structs (Rust-style)

```rust
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        return Point { x: x, y: y }
    }
    
    fn distance(&self) -> i32 {
        return self.x + self.y
    }
    
    fn move_by(&mut self, dx: i32, dy: i32) {
        self.x = self.x + dx
        self.y = self.y + dy
    }
}

// Usage
let p = Point { x: 10, y: 20 }
let d = p.distance()
```

### Classes with Inheritance (Python/C++ style)

```python
class Animal:
    name = ""
    age = 0
    
    def __init__(self, name, age):
        self.name = name
        self.age = age
    
    virtual def speak(self):
        print("...")
    
    def info(self):
        print("Name:")
        print(self.name)

class Dog extends Animal:
    breed = ""
    
    override def speak(self):
        print("Woof woof!")
    
    def bark(self):
        print("WOOF!")

class Cat extends Animal:
    override def speak(self):
        print("Meow!")
```

### Traits and Interfaces

```rust
// Rust-style trait
trait Drawable {
    fn draw(&self);
    fn get_color(&self) -> String;
}

// Python/Java-style interface
interface Movable:
    def move(self, x: i32, y: i32)
    def get_position(self)

// Trait implementation
impl Drawable for Sprite {
    fn draw(&self) {
        print("Drawing sprite...")
    }
    
    fn get_color(&self) -> String {
        return "red"
    }
}

// Class with multiple interfaces
class Sprite implements Drawable, Movable:
    x = 0
    y = 0
    
    def draw(self):
        print("Sprite on screen")
    
    def move(self, dx, dy):
        self.x = self.x + dx
        self.y = self.y + dy
```

---

## 🎮 Playground Mode (REPL)

The playground mode allows you to write and execute code interactively:

```bash
adeadc play
```

```
╔══════════════════════════════════════════════════════════════╗
║        🎮 ADead-BIB Playground v0.2.0 🎮                     ║
║     Interactive mode - Write code and press Enter            ║
╚══════════════════════════════════════════════════════════════╝

adB[1]> print("Hello!")
▶️  Executing...
   → Hello!
✅ Execution completed

adB[2]> let x = 42
   x = 42

adB[3]> :help
🎮 ADead-BIB Playground - Help
...

adB[4]> :exit
👋 Goodbye!
```

### Playground Commands

| Command | Shortcut | Description |
|---------|----------|-------------|
| `:help` | `:h` | Show help |
| `:run` | `:r` | Execute code in buffer |
| `:clear` | `:c` | Clear buffer |
| `:ast` | `:a` | Show code AST |
| `:tokens` | `:t` | Show tokens |
| `:vars` | `:v` | Show variables |
| `:example` | `:e` | Load example |
| `:exit` | `:q` | Exit |

---

## ⚡ Binary Sizes

ADead-BIB generates **extremely small** binaries because it writes opcodes directly:

| Mode | Size | Command | Description |
|------|------|---------|-------------|
| Standard | ~1.5 KB | `adeadc build` | Complete binary |
| Tiny | < 500 bytes | `adeadc tiny` | Ultra-compact PE |
| Nano | ~1 KB | `adeadc nano` | Minimum valid x64 |
| Micro | < 256 bytes | `adeadc micro` | Sub-256 bytes PE32 |

### Comparison with other languages

| Language | Hello World |
|----------|-------------|
| **ADead-BIB** | **~1.5 KB** |
| C (MinGW) | ~50 KB |
| Rust | ~150 KB |
| Go | ~2 MB |
| Python (.exe) | ~5 MB |

---

## 📁 Project Structure

```
ADead-BIB/
├── src/rust/                    # Compiler (100% Rust)
│   ├── frontend/                # Compiler frontend
│   │   ├── lexer.rs            # Tokenizer (Rust + Python syntax)
│   │   ├── parser.rs           # Parser (dual syntax)
│   │   ├── ast.rs              # Abstract Syntax Tree
│   │   └── type_checker.rs     # Type checking
│   ├── backend/                 # Code generation backend
│   │   ├── cpu/                # Direct x86-64
│   │   │   ├── codegen_v2.rs   # Main generator
│   │   │   ├── pe.rs           # Windows binaries (PE)
│   │   │   ├── elf.rs          # Linux binaries (ELF)
│   │   │   └── syscalls.rs     # Direct syscalls
│   │   └── gpu/                # GPU/Vulkan
│   │       ├── vulkan_runtime.rs
│   │       └── bytecode_spirv.rs
│   ├── optimizer/              # Optimizations
│   ├── runtime/                # Minimal runtime
│   ├── builder.rs              # Build system
│   └── main.rs                 # Main CLI
├── std/                         # Standard library (v1.5.0)
│   ├── math.adB                # Math functions
│   ├── io.adB                  # I/O functions
│   └── string.adB              # String utilities
├── examples/                    # Language examples
│   ├── 01_hello_world.adB
│   ├── 02_variables.adB
│   └── ...
├── TESTEO/                      # Test suite
│   ├── arrays/                 # Array tests
│   ├── modules/                # Module tests
│   └── integrados/             # Integration tests
├── Cargo.toml                  # Rust configuration
└── README.md                   # This file
```

---

## 📚 Examples

The `/examples` folder contains complete examples:

| File | Description |
|------|-------------|
| `01_hello_world.adB` | Basic Hello World |
| `02_variables.adB` | Variables and types |
| `03_funciones.adB` | Functions with types |
| `04_control_flujo.adB` | if/while/for |
| `05_oop_clases.adB` | Classes and structs |
| `06_herencia_polimorfismo.adB` | Inheritance and override |
| `07_traits_interfaces.adB` | Traits and interfaces |
| `08_game_engine.adB` | Game engine demo |

### Run an example

```bash
adeadc run examples/01_hello_world.adB
```

Output:
```
🚀 Running examples/01_hello_world.adB...

Hello, ADead-BIB!
Welcome to the language that goes direct to binary
This is a basic example
```

---

## 🔧 Why ADead-BIB?

### 1. **Direct to Binary (like ASM)**
ADead-BIB writes x86-64 opcodes directly to the executable file. No intermediate assembler, no external linker.

```
print("Hello")  →  mov rcx, addr  →  48 B9 XX XX XX XX XX XX XX XX
                   call printf    →  FF 15 XX XX XX XX
```

### 2. **Familiar Syntax**
You can use the syntax you prefer - Rust or Python:

```rust
// This is valid
fn main() {
    let x = 42
}

// And this too
def main():
    x = 42
```

### 3. **Small Binaries**
Executables are extremely small because there's no heavy runtime.

### 4. **Full OOP**
Supports everything you expect from a modern language:
- Classes and Structs
- Inheritance (`extends`)
- Polymorphism (`virtual`/`override`)
- Traits and Interfaces
- Static methods

### 5. **Scripts without Main**
You don't need a `main()` function. Write code directly:

```rust
print("This works!")
let x = 42
print(x)
```

---

## 🎮 GPU and Vulkan

ADead-BIB supports GPU computing:

```bash
# Detect available GPU
adeadc gpu

# Generate SPIR-V shader for matrix multiplication
adeadc spirv matmul 1024

# Initialize Vulkan runtime
adeadc vulkan
```

---

## 📖 Complete Documentation

For a complete language guide, see:
- **[ROADMAP.md](ROADMAP.md)** - Future improvements and roadmap

---

## 🤝 Contributing

1. Fork the repository
2. Create a branch: `git checkout -b my-feature`
3. Commit: `git commit -m 'Add feature'`
4. Push: `git push origin my-feature`
5. Open a Pull Request

---

## 📄 License

Apache 2.0 - See [LICENSE](LICENSE) file

---

<div align="center">

## 🔥 ADead-BIB

**Modern Assembly: ASM power with Rust/Python productivity**

```
┌────────────────────────────────────────────────────────────┐
│  ASM Family → 1.5 KB Binaries → No Runtime                │
│  Modern Syntax → Full OOP → GPU Vulkan                    │
│  Servers ✓ PC ✓ Embedded ✓ Games ✓                        │
└────────────────────────────────────────────────────────────┘
```

*The only language that combines: ASM size + Rust syntax + Python flexibility*

[![Made in Peru](https://img.shields.io/badge/Made%20in-Peru-red)](https://github.com/your-user/ADead-BIB)
[![Family ASM](https://img.shields.io/badge/Family-ASM-blue)](https://github.com/your-user/ADead-BIB)
[![100% Rust](https://img.shields.io/badge/Compiler-100%25%20Rust-orange)](https://www.rust-lang.org/)
[![Binary Size](https://img.shields.io/badge/Binary-~1.5KB-green)](https://github.com/your-user/ADead-BIB)
[![No Runtime](https://img.shields.io/badge/Runtime-None-purple)](https://github.com/your-user/ADead-BIB)

</div>
