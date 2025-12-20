# 📁 Estructura del Proyecto ADead-BIB

## Organización de Carpetas

```
ADead-BIB/
│
├── src/                          # Código fuente principal
│   ├── rust/
│   │   ├── frontend/            # Frontend en Rust
│   │   │   ├── lexer.rs         # Tokenización
│   │   │   ├── parser.rs        # Parser con nom
│   │   │   └── ast.rs           # Estructuras AST
│   │   │
│   │   └── backend/              # Backend en Rust
│   │       ├── pe.rs            # Generador PE (Windows)
│   │       ├── elf.rs           # Generador ELF (Linux)
│   │       └── binary.rs        # Utilidades binarias
│   │
│   └── cpp/
│       └── emitter/              # Emisor de opcodes en C++
│           ├── emitter.h        # Header principal
│           ├── emitter.cpp      # Implementación
│           ├── opcodes.h        # Definiciones de opcodes
│           └── x86_64.cpp      # Opcodes x86-64
│
├── docs/                         # Documentación
│   ├── ESTRUCTURA.md            # Este archivo
│   ├── OPCODES.md               # Referencia de opcodes
│   ├── PE_FORMAT.md             # Formato PE
│   └── ELF_FORMAT.md            # Formato ELF
│
├── examples/                     # Ejemplos de código
│   ├── hello_world.adB          # Ejemplo básico (.adB = ADead-BIB)
│   ├── arithmetic.adB           # Operaciones aritméticas
│   └── functions.adB            # Funciones
│
├── tests/                        # Tests
│   ├── unit/                    # Tests unitarios
│   ├── integration/              # Tests de integración
│   └── binary/                  # Tests de binarios generados
│
├── build/                        # Scripts de build
│   ├── build.ps1                # Build script (Windows)
│   ├── build.sh                 # Build script (Linux)
│   └── CMakeLists.txt           # CMake para C++
│
├── scripts/                      # Utilidades
│   ├── analyze_binary.ps1       # Analizar binarios
│   └── test_opcodes.ps1         # Probar opcodes
│
├── ideas.md                      # Ideas y arquitectura completa
├── Rutas.md                      # Todas las rutas de implementación
├── README.md                     # Documentación principal
└── .gitignore                    # Git ignore
```

## Descripción de Componentes

### `src/rust/frontend/`
**Responsabilidad**: Parsing del código fuente
- **Lexer**: Tokeniza el código fuente
- **Parser**: Construye AST usando `nom`
- **AST**: Representación intermedia del código

### `src/rust/backend/`
**Responsabilidad**: Generación de binarios PE/ELF
- **PE**: Generador de ejecutables Windows
- **ELF**: Generador de ejecutables Linux
- **Binary**: Utilidades para manipular bytes

### `src/cpp/emitter/`
**Responsabilidad**: Emisión de opcodes directamente
- **Emitter**: Clase principal para emitir opcodes
- **Opcodes**: Definiciones de opcodes x86-64
- **x86_64**: Implementación específica de arquitectura

### `docs/`
**Responsabilidad**: Documentación técnica
- Referencias de opcodes
- Formatos de binarios
- Guías de implementación

### `examples/`
**Responsabilidad**: Ejemplos de código fuente
- Programas de ejemplo en el lenguaje ADead
- Casos de uso comunes

### `tests/`
**Responsabilidad**: Testing
- Tests unitarios de cada componente
- Tests de integración
- Validación de binarios generados

### `build/`
**Responsabilidad**: Compilación
- Scripts para compilar Rust + C++
- Configuración de CMake
- Build automation

### `scripts/`
**Responsabilidad**: Utilidades
- Herramientas para analizar binarios
- Scripts de desarrollo
- Utilidades de debugging

## Flujo de Datos

```
examples/hello_world.adB
    ↓
src/rust/frontend/parser.rs
    ↓ (AST)
src/cpp/emitter/emitter.cpp
    ↓ (Opcodes en bytes)
src/rust/backend/pe.rs o elf.rs
    ↓
build/output.exe o output
    ↓
CPU ejecuta bytes directamente
```

## Convenciones

- **Rust**: snake_case para funciones, PascalCase para tipos
- **C++**: camelCase para funciones, PascalCase para clases
- **Archivos**: snake_case para Rust, camelCase para C++
- **Tests**: `*_test.rs` para Rust, `*_test.cpp` para C++

## Próximos Pasos

1. Implementar lexer básico en `src/rust/frontend/lexer.rs`
2. Implementar parser con nom en `src/rust/frontend/parser.rs`
3. Implementar emisor de opcodes en `src/cpp/emitter/emitter.cpp`
4. Implementar generador PE en `src/rust/backend/pe.rs`
5. Crear primer ejemplo funcional

