# ADead-BIB v1.2.0 - English Guide

## What is ADead-BIB?

ADead-BIB is a programming language that **compiles directly to native binary code**. No virtual machine, no interpreter. Your code becomes a real `.exe` executable.

## Installation

```bash
# Clone the repository
git clone https://github.com/user/ADead-BIB.git
cd ADead-BIB

# Install (requires Rust)
cargo install --path .
```

## Basic Commands

| Command | Description |
|---------|-------------|
| `adeadc run file.adB` | Compile and run |
| `adeadc build file.adB` | Compile only |
| `adeadc check file.adB` | Check syntax |

## Your First Program

Create a file `hello.adB`:

```rust
fn main() {
    println("Hello, world!")
}
```

Run:
```bash
adeadc run hello.adB
```

## Basic Syntax

### Variables

```rust
let x = 42          // Variable
let y = 10          // Another variable
const PI = 3        // Constant
```

### Operations

```rust
let sum = x + y         // Addition
let diff = x - y        // Subtraction
let product = x * y     // Multiplication
let quotient = x / y    // Division
```

### Printing

```rust
// print - no newline
print("Hello ")
print("world")

// println - automatic newline
println("Hello world!")

// Print numbers
let x = 42
print("x = ")
println(x)
```

### Escape Sequences

```rust
print("Line 1\n")       // \n = newline
print("Tab:\tvalue")    // \t = tab
```

### Functions

```rust
fn main() {
    // Your code here
}
```

## Examples

See the `examples/` folder:

| File | Description |
|------|-------------|
| `hello.adB` | Hello world |
| `variables.adB` | Variables and constants |
| `aritmetica.adB` | Math operations |
| `funciones.adB` | Functions |
| `objetos.adB` | Object simulation |
| `vectores.adB` | 2D vectors |
| `juego.adB` | Game logic |
| `println.adB` | Using println |
| `binario.adB` | Native code |

## Control Flow (v0.6.0)

### If / Else

```rust
let x = 10
if x > 5 {
    println("x is greater than 5")
} else {
    println("x is less or equal to 5")
}
```

### While

```rust
let i = 0
while i < 5 {
    println(i)
    i = i + 1
}
```

### For

```rust
for i in 0..5 {
    println(i)
}
```

### Comparisons

| Operator | Meaning |
|----------|---------|
| `==` | Equal |
| `!=` | Not equal |
| `<` | Less than |
| `>` | Greater than |
| `<=` | Less or equal |
| `>=` | Greater or equal |

## Custom Functions (v0.7.0)

### Define Functions

```rust
fn add(a, b) {
    return a + b
}

fn square(x) {
    return x * x
}
```

### Call Functions

```rust
let result = add(10, 5)
println(result)  // 15
```

### Recursion

```rust
fn factorial(n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

let f5 = factorial(5)  // 120
```

## Features

- ✅ Rust-style syntax (`fn`, `let`, `const`)
- ✅ Direct compilation to x86-64 binary
- ✅ Small binaries (~2KB)
- ✅ No runtime dependencies
- ✅ `print()` and `println()` for output
- ✅ Arithmetic operations (+, -, *, /)
- ✅ Escape sequences (\n, \t, \r)
- ✅ Control flow (if/else, while, for)
- ✅ Comparisons (==, !=, <, >, <=, >=)
- ✅ Custom functions with parameters
- ✅ Recursion

## Binary Sizes

| Type | Size |
|------|------|
| Standard | ~2 KB |
| Tiny | < 500 bytes |

## License

MIT License
