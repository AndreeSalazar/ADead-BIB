# 🔥 ADead-BIB

**Abstract Dead - Binary In Binary**

Sistema para generar **binarios ejecutables puros** escribiendo opcodes directamente, sin pasar por assembly.

## 🎯 Objetivo

Generar binarios que la CPU ejecuta directamente, controlando cada byte del ejecutable.

```
Lenguaje (.adB) → AST → Opcodes (bytes) → Binario Ejecutable Puro
```

**⚠️ NO usamos ASM** - Escribimos opcodes directamente en bytes.

## 🏗️ Arquitectura

- **Rust**: Parser manual con `nom` + Generación de PE/ELF
- **C++**: Emisión de opcodes directamente
- **Parser Manual**: Control total sobre el proceso

## 📝 Sintaxis (Estilo Python)

```adB
def main():
    print("Hello, World!")
```

Ver `docs/SINTAXIS.md` para documentación completa.

## 📁 Estructura del Proyecto

```
ADead-BIB/
├── src/
│   ├── rust/
│   │   ├── frontend/     # Parser (nom)
│   │   └── backend/      # Generación PE/ELF
│   └── cpp/
│       └── emitter/      # Emisión de opcodes
├── docs/                 # Documentación
├── examples/             # Ejemplos de código
├── tests/                # Tests
├── build/                # Scripts de build
├── ideas.md              # Ideas y arquitectura
└── Rutas.md             # Todas las rutas posibles
```

## 🚀 Quick Start

### Compilar el Proyecto

**Windows:**
```powershell
.\build.ps1
```

**Linux/Mac:**
```bash
mkdir build && cd build
cmake .. && make
cd ..
cargo build --release
```

### Compilar un Programa

```bash
cargo run --release examples/hello_world.adB
```

## 📚 Documentación

- `docs/SINTAXIS.md`: Sintaxis del lenguaje
- `docs/RECOMENDACION.md`: Por qué Ruta 2
- `ideas.md`: Arquitectura y diseño completo
- `Rutas.md`: Todas las rutas para generar binarios puros

## 🎓 Aprendizaje

Este proyecto te enseñará:
- Cómo funcionan los binarios a nivel de bytes
- Cómo la CPU ejecuta código directamente
- Formatos PE/ELF en detalle
- Opcodes x86-64
- Generación de código sin assembler

## 🔥 Características

- ✅ Parser manual (control total)
- ✅ Emisión directa de opcodes (NO ASM)
- ✅ Generación de PE/ELF
- ✅ Binarios puros sin dependencias
- ✅ Sintaxis estilo Python

## 📖 Estado Actual

🚧 **En Desarrollo** - Implementando Ruta 2 (Directo AST → Opcodes)

- ✅ Lexer básico
- ✅ Parser básico
- ✅ Estructura AST
- 🚧 Emisión de opcodes (C++)
- 🚧 Generación PE completa
- 🚧 Integración FFI Rust ↔ C++

## 📖 Licencia

[Tu licencia aquí]

---

**¡Construyendo binarios puros desde cero! 🚀**
