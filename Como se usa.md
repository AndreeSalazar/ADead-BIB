# 📖 Cómo usar ADead-BIB

## 🚀 Guía Completa del Lenguaje

ADead-BIB es un lenguaje de programación que combina la sintaxis de **Rust + Python + C++** y compila **directamente a código máquina nativo**. Sin VM, sin intérprete, directo al binario.

---

## 📦 Instalación

```bash
# Clonar el repositorio
git clone https://github.com/tu-usuario/ADead-BIB.git
cd ADead-BIB

# Compilar e instalar globalmente
cargo install --path .

# Verificar instalación
adeadc --help
```

---

## 🎯 Comandos Básicos

### Ejecutar un programa
```bash
adeadc run archivo.adB
```

### Compilar a ejecutable
```bash
adeadc build archivo.adB
adeadc build archivo.adB -o mi_programa.exe
```

### Verificar sintaxis
```bash
adeadc check archivo.adB
```

### Modo Playground (REPL interactivo)
```bash
adeadc play
```

---

## 📝 Sintaxis del Lenguaje

### Hello World

```rust
// Forma más simple - Script directo
print("Hello, ADead-BIB!")

// Forma Rust
fn main() {
    print("Hello desde Rust-style!")
}

// Forma Python
def main():
    print("Hello desde Python-style!")
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

// Strings
let texto = "Hola mundo"

// Booleanos
let verdadero = true
let falso = false
```

### Funciones

```rust
// Estilo Rust
fn sumar(a: i32, b: i32) -> i32 {
    return a + b
}

// Estilo Python
def multiplicar(x, y):
    return x * y

// Llamar funciones
let resultado = sumar(10, 20)
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
}

// Uso
let p = Punto { x: 10, y: 20 }
```

### Clases (Python/C++ style)

```python
class Animal:
    nombre = ""
    edad = 0
    
    def __init__(self, nombre, edad):
        self.nombre = nombre
        self.edad = edad
    
    virtual def hablar(self):
        print("...")

class Perro extends Animal:
    override def hablar(self):
        print("Guau!")
```

### Traits e Interfaces

```rust
// Trait (Rust-style)
trait Dibujable {
    fn dibujar(&self);
    fn obtener_color(&self) -> String;
}

// Interface (Python/Java-style)
interface Movible:
    def mover(self, x: i32, y: i32)

// Implementación
impl Dibujable for MiClase {
    fn dibujar(&self) {
        print("Dibujando...")
    }
}
```

### Herencia y Polimorfismo

```python
class Animal:
    virtual def hablar(self):
        pass

class Perro extends Animal:
    override def hablar(self):
        print("Guau!")

class Gato extends Animal:
    override def hablar(self):
        print("Miau!")

# Polimorfismo
fn hacer_hablar(animal: Animal):
    animal.hablar()  # Llama al método correcto
```

---

## 🎮 Modo Playground (REPL)

```bash
adeadc play
```

### Comandos del Playground

| Comando | Descripción |
|---------|-------------|
| `:help` | Mostrar ayuda |
| `:run` | Ejecutar código en buffer |
| `:clear` | Limpiar buffer |
| `:ast` | Mostrar AST del código |
| `:tokens` | Mostrar tokens |
| `:vars` | Mostrar variables |
| `:example` | Cargar ejemplo |
| `:exit` | Salir |

### Ejemplo de sesión

```
adB[1]> print("Hola!")
▶️  Ejecutando...
   → Hola!
✅ Ejecución completada

adB[2]> let x = 42
   x = 42

adB[3]> :exit
👋 ¡Hasta luego!
```

---

## ⚡ Modos de Compilación

### Standard Build (~1.5 KB)
```bash
adeadc build archivo.adB
```

### Tiny Build (< 500 bytes)
```bash
adeadc tiny archivo.adB
```

### Nano Build (~1 KB, mínimo x64)
```bash
adeadc nano output.exe
```

### Micro Build (< 256 bytes, x86)
```bash
adeadc micro output.exe
```

---

## 🎮 GPU y Vulkan

### Detectar GPU
```bash
adeadc gpu
```

### Generar SPIR-V shader
```bash
adeadc spirv matmul 1024
```

### Inicializar Vulkan runtime
```bash
adeadc vulkan
```

---

## 📁 Estructura de Proyecto

```
mi_proyecto/
├── main.adB          # Archivo principal
├── lib/              # Bibliotecas
│   ├── utils.adB
│   └── math.adB
└── tests/            # Tests
    └── test_main.adB
```

---

## 🔧 Características Únicas

### 1. Directo al Binario
ADead-BIB compila directamente a código máquina x86-64. No hay VM, no hay intérprete, no hay bytecode intermedio.

```
Código .adB → Lexer → Parser → AST → CodeGen → Opcodes x86-64 → PE/ELF
```

### 2. Sintaxis Dual
Puedes mezclar sintaxis de Rust y Python en el mismo archivo:

```rust
fn rust_function() {
    let x = 42
}

def python_function():
    x = 42
```

### 3. Binarios Ultra-Pequeños
- Standard: ~1.5 KB
- Tiny: < 500 bytes
- Nano: ~1 KB
- Micro: < 256 bytes

### 4. OOP Completo
- Clases y Structs
- Herencia (extends)
- Polimorfismo (virtual/override)
- Traits e Interfaces
- Métodos estáticos

### 5. Scripts sin Main
No necesitas función `main()`. Puedes escribir código directamente:

```rust
print("Esto funciona!")
let x = 42
print(x)
```

---

## 📚 Ejemplos

Ver carpeta `/examples` para ejemplos completos:

1. `01_hello_world.adB` - Hello World
2. `02_variables.adB` - Variables y tipos
3. `03_funciones.adB` - Funciones
4. `04_control_flujo.adB` - Control de flujo
5. `05_oop_clases.adB` - OOP básico
6. `06_herencia_polimorfismo.adB` - Herencia
7. `07_traits_interfaces.adB` - Traits
8. `08_game_engine.adB` - Game engine
9. `09_async_concurrencia.adB` - Async
10. `10_directo_binario.adB` - Compilación nativa

---

## 🤝 Contribuir

1. Fork el repositorio
2. Crea una rama: `git checkout -b mi-feature`
3. Commit: `git commit -m 'Añadir feature'`
4. Push: `git push origin mi-feature`
5. Abre un Pull Request

---

## 📄 Licencia

MIT License - Ver archivo LICENSE

---

**ADead-BIB** - El lenguaje que va directo al binario 🚀
