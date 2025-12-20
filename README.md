# 🔥 ADead-BIB

**Abstract Dead - Binary In Binary**

Compilador que genera **binarios ejecutables puros** escribiendo opcodes directamente a la CPU, sin pasar por ensamblador. **Binario + HEX = ADead-BIB**.

## ✅ Estado: LENGUAJE COMPLETO + IA

| Característica | Estado |
|----------------|--------|
| **60+ funciones built-in** | ✅ |
| **OOP completo** | ✅ |
| **Sistema de imports** | ✅ |
| **FFI Python** | ✅ |
| **IA integrada (0.19 MB RAM)** | ✅ |

## 📁 Estructura del Proyecto

```
ADead-BIB/
├── src/rust/          # Compilador (Lexer, Parser, Codegen, PE)
├── examples/          # Ejemplos .adB
├── stdlib/            # Librería estándar (math, io, string)
├── python/            # FFI Python + IA
│   ├── adead_ffi.py   # Wrapper FFI
│   └── ai_complete.py # IA completa (0.19 MB RAM)
├── build/             # Binarios compilados (.exe)
├── docs/              # Documentación
├── ideas-2.md         # Roadmap del lenguaje
└── ideas-3.md         # Arquitectura IA
```

## 🚀 Uso Rápido

```powershell
cargo run --release examples/hello_world.adB
.\hello_world.exe
# Output: Hello, World!
```

| Métrica | Valor |
|---------|-------|
| **Binario mínimo** | 1.5 KB |
| **Binario con juego** | 2 KB |
| **Dependencias runtime** | 0 |
| **OOP** | ✅ Clases, Herencia, Polimorfismo |

---

## 🎯 ¿Qué es ADead-BIB?

Un compilador que transforma código con sintaxis estilo Python directamente en **opcodes x86-64** y genera **binarios PE ejecutables** sin usar ensamblador.

```
hello_world.adB → Lexer → Parser → AST → Opcodes x86-64 → PE → CPU ejecuta
```

**La CPU ejecuta exactamente lo que escribes** - sin capas intermedias, sin runtime, sin overhead.

---

## 🔥 ¿Por qué es Diferente?

### Comparación con Otros Enfoques

| Enfoque | Flujo | Overhead |
|---------|-------|----------|
| **C/C++** | Código → Compilador → ASM → Objeto → Linker → Binario | Medio |
| **ASM** | ASM → Assembler → Objeto → Linker → Binario | Bajo |
| **ADead-BIB** | Código → **Opcodes directos** → Binario | **Mínimo** |

### Ventajas Clave

- ✅ **Sin ASM** - Escribimos bytes directamente, no texto ensamblador
- ✅ **Sin Linker** - Generamos PE completo en un paso
- ✅ **Sin Runtime** - Binarios standalone, sin dependencias
- ✅ **Control Total** - Cada byte del ejecutable es tuyo
- ✅ **Binarios Mínimos** - Solo lo necesario, nada más

---

## 🚀 Quick Start

### 1. Compilar ADead-BIB

```powershell
cargo build --release
```

### 2. Escribir un Programa

```python
# examples/hello_world.adB
def main():
    print("Hello, World!")
```

### 3. Compilar y Ejecutar

```powershell
cargo run --release examples/hello_world.adB
.\hello_world.exe
```

**Output:** `Hello, World!`

---

## 📝 Sintaxis

ADead-BIB usa sintaxis estilo Python con OOP:

```python
# Función principal
def main():
    print("Hello, World!")
    x = 10
    y = 20
    print(x + y)

# Clases con herencia
class Entity:
    x = 0
    y = 0
    
    virtual def update(self):
        pass

class Player extends Entity:
    health = 100
    
    override def update(self):
        print("Player update")
```

Ver `docs/SINTAXIS.md` para documentación completa.

---

## 🏗️ Arquitectura

```
ADead-BIB/
├── src/rust/
│   ├── frontend/
│   │   ├── lexer.rs      # Tokenizador
│   │   ├── parser.rs     # Parser → AST
│   │   └── ast.rs        # Estructuras AST
│   └── backend/
│       ├── codegen.rs    # AST → Opcodes x86-64
│       └── pe.rs         # Generador PE Windows
├── examples/
│   └── hello_world.adB   # Ejemplo funcional
└── docs/                 # Documentación
```

### Flujo Interno

```
1. Lexer    → Tokeniza código fuente
2. Parser   → Construye AST
3. Codegen  → Emite opcodes x86-64 directamente
4. PE Gen   → Genera binario Windows ejecutable
```

---

## 💡 Casos de Uso

### 🎯 Uso General
- Compilador para aplicaciones de sistema
- Herramientas de línea de comandos
- Binarios pequeños y rápidos

### 🔧 Trabajos Pesados
- Procesamiento de datos de alto rendimiento
- Aplicaciones donde cada ciclo de CPU importa
- Sistemas embebidos con recursos limitados

### 🎓 Aprendizaje
- Entender cómo funcionan los binarios
- Aprender opcodes x86-64
- Comprender formato PE de Windows

### 🚀 Potencial Futuro
- **JIT Compiler** - Compilación en tiempo de ejecución
- **Cross-compilation** - Generar binarios para múltiples plataformas
- **Optimizador** - Optimizaciones a nivel de opcode
- **VM Engine** - Runtime interpretado si se necesita

---

## 📊 Características Implementadas

| Componente | Estado | Descripción |
|------------|--------|-------------|
| **Lexer** | ✅ | Tokeniza código .adB |
| **Parser** | ✅ | Genera AST desde tokens |
| **Codegen** | ✅ | Emite opcodes x86-64 |
| **PE Generator** | ✅ | Genera binarios Windows |
| **Variables** | ✅ | Variables locales en stack |
| **Operaciones** | ✅ | +, -, *, /, % |
| **Comparaciones** | ✅ | ==, !=, <, <=, >, >= |
| **Condicionales** | ✅ | if/elif/else |
| **Bucles** | ✅ | while, for |
| **Funciones** | ✅ | Con parámetros |
| **OOP** | ✅ | Clases, herencia, polimorfismo |
| **print()** | ✅ | Strings y números |

### Próximas Características

- 🚧 Arrays y listas
- 🚧 Strings avanzados
- 🚧 Generación ELF (Linux)
- 🚧 Optimizaciones

---

## 🔬 Detalles Técnicos

### Layout del PE Generado

```
0x0000 - Headers (DOS, PE, COFF, Optional, Sections)
0x1000 - .text  (código ejecutable - opcodes)
0x2000 - .rdata (imports + datos)
```

### Ejemplo de Opcodes Generados

Para `print("Hello, World!")`:

```asm
48 83 EC 28          ; sub rsp, 40 (shadow space)
48 B9 60 20 40 00... ; mov rcx, string_address
FF 15 xx xx xx xx    ; call [rip+offset] (printf)
31 C0                ; xor eax, eax (return 0)
48 83 C4 28          ; add rsp, 40
C3                   ; ret
```

**27 bytes de código máquina** - directo a la CPU.

---

## 📚 Documentación

| Documento | Descripción |
|-----------|-------------|
| `docs/SINTAXIS.md` | Sintaxis completa del lenguaje |
| `docs/ESTRUCTURA.md` | Estructura del proyecto |
| `docs/POTENCIAL.md` | Potencial y evoluciones posibles |
| `Rutas.md` | Todas las rutas para generar binarios |

---

## 🎯 Filosofía

> **"Código → Opcodes → Binario"**

ADead-BIB elimina las capas innecesarias entre tu código y la CPU. No hay ensamblador, no hay linker, no hay runtime. Solo bytes que la CPU ejecuta directamente.

**Menos pasos = Menos errores = Más control = Mejor performance**

---

## 📖 Licencia

MIT License

---

**ADead-BIB: Binarios puros, control total, directo a la CPU. 🚀**
