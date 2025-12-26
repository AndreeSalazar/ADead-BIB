# ADead-BIB v1.2.0 - Guía en Español

## ¿Qué es ADead-BIB?

ADead-BIB es un lenguaje de programación que **compila directamente a código binario nativo**. Sin máquina virtual, sin intérprete. Tu código se convierte en un ejecutable `.exe` real.

## Instalación

```bash
# Clonar el repositorio
git clone https://github.com/usuario/ADead-BIB.git
cd ADead-BIB

# Instalar (requiere Rust)
cargo install --path .
```

## Comandos Básicos

| Comando | Descripción |
|---------|-------------|
| `adeadc run archivo.adB` | Compilar y ejecutar |
| `adeadc build archivo.adB` | Solo compilar |
| `adeadc check archivo.adB` | Verificar sintaxis |

## Tu Primer Programa

Crea un archivo `hello.adB`:

```rust
fn main() {
    println("Hola, mundo!")
}
```

Ejecuta:
```bash
adeadc run hello.adB
```

## Sintaxis Básica

### Variables

```rust
let x = 42          // Variable
let y = 10          // Otra variable
const PI = 3        // Constante
```

### Operaciones

```rust
let suma = x + y        // Suma
let resta = x - y       // Resta
let producto = x * y    // Multiplicación
let division = x / y    // División
```

### Imprimir

```rust
// print - sin salto de línea
print("Hola ")
print("mundo")

// println - con salto de línea automático
println("Hola mundo!")

// Imprimir números
let x = 42
print("x = ")
println(x)
```

### Secuencias de Escape

```rust
print("Línea 1\n")      // \n = nueva línea
print("Tab:\tvalor")    // \t = tabulación
```

### Funciones

```rust
fn main() {
    // Tu código aquí
}
```

## Ejemplos

Ver la carpeta `examples/`:

| Archivo | Descripción |
|---------|-------------|
| `hello.adB` | Hola mundo |
| `variables.adB` | Variables y constantes |
| `aritmetica.adB` | Operaciones matemáticas |
| `funciones.adB` | Funciones |
| `objetos.adB` | Simulación de objetos |
| `vectores.adB` | Vectores 2D |
| `juego.adB` | Lógica de juego |
| `println.adB` | Uso de println |
| `binario.adB` | Código nativo |

## Control de Flujo (v0.6.0)

### If / Else

```rust
let x = 10
if x > 5 {
    println("x es mayor que 5")
} else {
    println("x es menor o igual a 5")
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

### Comparaciones

| Operador | Significado |
|----------|-------------|
| `==` | Igual |
| `!=` | Diferente |
| `<` | Menor que |
| `>` | Mayor que |
| `<=` | Menor o igual |
| `>=` | Mayor o igual |

## Funciones Propias (v0.7.0)

### Definir Funciones

```rust
fn suma(a, b) {
    return a + b
}

fn cuadrado(x) {
    return x * x
}
```

### Llamar Funciones

```rust
let resultado = suma(10, 5)
println(resultado)  // 15
```

### Recursión

```rust
fn factorial(n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

let f5 = factorial(5)  // 120
```

## Características

- ✅ Sintaxis estilo Rust (`fn`, `let`, `const`)
- ✅ Compilación directa a binario x86-64
- ✅ Binarios pequeños (~2KB)
- ✅ Sin dependencias en runtime
- ✅ `print()` y `println()` para salida
- ✅ Operaciones aritméticas (+, -, *, /)
- ✅ Secuencias de escape (\n, \t, \r)
- ✅ Control de flujo (if/else, while, for)
- ✅ Comparaciones (==, !=, <, >, <=, >=)
- ✅ Funciones propias con parámetros
- ✅ Recursión

## Tamaño de Binarios

| Tipo | Tamaño |
|------|--------|
| Standard | ~2 KB |
| Tiny | < 500 bytes |

## Licencia

MIT License
