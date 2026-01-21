# Hello - ADead-BIB Example

> Ejemplo simple de "Hola Mundo" en ADead-BIB.
> 
> **Código → Bytes → Binario. Sin ASM. Sin LLVM.**

---

## 🚀 Ejecutar (Windows)

```powershell
# Desde la raíz del proyecto ADead-BIB:
cd C:\Users\andre\OneDrive\Documentos\ADead-BIB

# Ejecutar el ejemplo
cargo run --bin adeadc -- run examples/hello/main.adB

# O si tienes adeadc instalado globalmente:
adeadc run examples/hello/main.adB
```

## 🐧 Ejecutar (Linux / macOS)

```bash
# Desde la raíz del proyecto ADead-BIB:
cd ~/ADead-BIB

# Ejecutar el ejemplo
cargo run --bin adeadc -- run examples/hello/main.adB

# O si tienes adeadc instalado globalmente:
adeadc run examples/hello/main.adB
```

---

## 📝 Código

```rust
// main.adB - Punto de entrada
fn main() {
    println("Hola, ADead-BIB!")
    println("")
    println("Este es un ejemplo simple.")
    println("ADead-BIB compila DIRECTO a binario.")
    println("")
    
    // Variables
    let x = 10
    let y = 20
    let suma = x + y
    
    print("10 + 20 = ")
    println(suma)
    
    println("")
    println("Listo!")
}
```

---

## 📦 Crear tu Propio Proyecto

### Comparación con Rust

| Rust | ADead-BIB |
|------|-----------|
| `cargo new hola` | `adB new hola` |
| `cargo run` | `adB run main.adB` |
| `cargo build` | `adB build main.adB` |
| `cargo check` | `adB check main.adB` |

### Windows (PowerShell)

```powershell
# Crear nuevo proyecto (como cargo new)
cargo run --bin adeadc -- new mi_proyecto

# Entrar al proyecto
cd mi_proyecto

# Ejecutar
cargo run --bin adeadc -- run main.adB
```

### Linux / macOS

```bash
# Crear nuevo proyecto (como cargo new)
cargo run --bin adeadc -- new mi_proyecto

# Entrar al proyecto
cd mi_proyecto

# Ejecutar
cargo run --bin adeadc -- run main.adB
```

---

## 📁 Estructura del Proyecto Generado

```
mi_proyecto/
├── main.adB          # 🎯 Punto de entrada
├── call.adB          # 📦 Lógica OOP (structs, traits)
├── build.adB         # ⚙️ Configuración
├── README.md         # 📖 Documentación
├── core/
│   └── mod.adB       # 🔧 init(), shutdown()
├── cpu/
│   └── mod.adB       # 💻 Instrucciones x86-64
└── gpu/
    └── mod.adB       # 🎮 Opcodes GPU
```

---

## 🎮 Ejemplo con OOP

Edita `call.adB` para agregar clases:

```rust
struct Player {
    name: string,
    health: i32
}

impl Player {
    fn new(name: string) -> Player {
        return Player { name: name, health: 100 }
    }
    
    fn info(self) {
        print("Player: ")
        println(self.name)
        print("Health: ")
        println(self.health)
    }
}

pub fn run() {
    let player = Player::new("Hero")
    player.info()
}
```

---

## 📋 Comandos Disponibles

| Comando | Descripción |
|---------|-------------|
| `adB new <nombre>` | Crear proyecto nuevo |
| `adB run <archivo>` | Compilar y ejecutar |
| `adB build <archivo>` | Solo compilar |
| `adB check <archivo>` | Verificar sintaxis |
| `adB help` | Mostrar ayuda |

---

**ADead-BIB v2.5 — Código → Bytes → Binario**
