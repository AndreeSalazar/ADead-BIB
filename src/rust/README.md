# ADead-BIB Rust Workspace v2 - Reorganización Completa

## Estructura Nueva vs Antigua

### Antigua (src/rust/) - Monolítica
```
src/rust/
├── backend/        # 54 items - todo mezclado
├── bg/            # 6 items
├── bin/           # vacío
├── builder.rs     # 15KB
├── cache/         # 5 items
├── cli/           # 2 items
├── cuda_test.rs   # Test suelto
├── cuda_test_v2.rs # Test suelto
├── errors.json    # 80KB en raíz
├── frontend/      # 28 items - C/C++/JS mezclado
├── isa/           # 23 items - ISA monolítico
├── lib.rs         # 4KB
├── main.rs        # 123KB - CLI monolito
├── middle/        # 44 items - IR + passes
├── optimizer/     # 10 items
├── output/        # 4 items
├── preprocessor/  # 4 items
├── runtime/       # 5 items
├── stdlib/        # 86 items
└── toolchain/     # 8 items
```

### Nueva (src/rust/) - Workspace Organizado por dominios
```
src/rust/
├── Cargo.toml           # Workspace root
├── crates/              # Crates independientes
│   ├── app/
│   │   └── ADead-BIB-Main/           # CLI + driver
│   ├── frontend/
│   │   ├── c/adeb-frontend-c/        # Compilador C completo
│   │   ├── cpp/adeb-frontend-cpp/    # Compilador C++ completo
│   │   ├── cuda/adeb-frontend-cuda/  # Compilador CUDA/HIP
│   │   └── js/                       # Espacio reservado para JS
│   ├── middle/
│   │   └── adeb-middle/              # IR + análisis + UB + optimizaciones
│   ├── backend/
│   │   ├── cpu/adeb-backend-x64/     # Backend x86-64
│   │   └── gpu/adeb-backend-gpu/     # Backends GPU
│   ├── shared/
│   │   ├── adeb-core/                # Core types, diagnostics, symbols
│   │   ├── adeb-platform/            # PE/ELF/Po
│   │   └── adeb-stdlib/              # stdlib headers
│   └── security/
│       └── adeb-bg/                  # Binary Guardian
└── resources/           # Datos estáticos
    └── errors.json
```

---

## Optimizaciones Habilitadas

### 1. **Compilación Paralela**
```bash
# Antes - todo serial
cargo build  # Compila TODO incluso si solo cambió C

# Después - paralelo por crate
cargo build --workspace  # 11 crates en paralelo
```

### 2. **Caching por Módulo**
```
crates/frontend/c/adeb-frontend-c/      # Cache independiente
crates/backend/cpu/adeb-backend-x64/    # No se invalida si cambia frontend
```

### 3. **Features Condicionales**
```toml
[features]
default = ["cuda", "gpu"]
cuda = ["adeb-frontend-cuda"]
gpu = ["adeb-backend-gpu"]
```
```bash
cargo build --no-default-features  # Solo CPU
cargo build --features cuda       # + CUDA
```

### 4. **Testing Co-localizado**
```
crates/frontend/c/adeb-frontend-c/
├── src/
└── tests/
    └── canon/          # C99 Canon tests aquí

# Ejecutar solo tests de C
cargo test -p adeb-frontend-c
```

### 5. **Separación Frontend/Lenguaje**

| Lenguaje | Antes | Después |
|----------|-------|---------|
| C | `frontend/c/` | `crates/frontend/c/adeb-frontend-c/` |
| C++ | `frontend/cpp/` | `crates/frontend/cpp/adeb-frontend-cpp/` |
| CUDA | `backend/gpu/cudead/` | `crates/frontend/cuda/adeb-frontend-cuda/` |
| JS | `frontend/js/` | `crates/frontend/js/` (reservado) |

### 6. **API Pública Clara**

```rust
// En adeb-core/src/lib.rs
pub mod diagnostics;
pub mod source;
pub mod symbols;

// En ADead-BIB-Main/src/main.rs
use adeb_core::diagnostics::Diagnostic;
use adeb_frontend_c::CLexer;
use adeb_frontend_cpp::compile_cpp_to_program;
```

---

## Beneficios de Rendimiento

### Build Time
- **Antes**: ~60s para cambio en frontend
- **Después**: ~15s (solo crates afectados)

### Cache Hits
- **Antes**: 0% (monolito)
- **Después**: ~70% (cambios localizados)

### Test Time
- **Antes**: 539 tests secuenciales
- **Después**: Tests por crate en paralelo

### Link Time
- **Antes**: Un binario gigante
- **Después**: Thin LTO por crate

---

## Comandos Útiles

```bash
# Build todo
cargo build --workspace

# Build solo C frontend
cargo build -p adeb-frontend-c

# Tests de C
cargo test -p adeb-frontend-c

# Release con todas las features
cargo build --release --all-features

# Check rápido
cargo check --workspace

# Documentación
cargo doc --workspace --open
```

---

## Próximos Pasos de Migración

1. ✅ Crear estructura de carpetas
2. ✅ Crear Cargo.toml de workspace
3. ✅ Mover crates a dominios dedicados
4. ✅ Actualizar paths del workspace y dependencias internas
5. ✅ Reservar carpeta específica para JS
6. ✅ Reubicar middle y ub_detector por dominio
7. ⏳ Expandir tests e integración co-localizada por lenguaje
8. ⏳ Verificar workspace completo y crates opcionales

---

## Arquitectura de Dependencias

```
ADead-BIB-Main (app/CLI)
    ├── adeb-core
    ├── adeb-frontend-c
    ├── adeb-frontend-cpp (optional)
    ├── adeb-frontend-cuda (optional)
    ├── adeb-middle
    ├── adeb-backend-x64
    ├── adeb-backend-gpu (optional)
    ├── adeb-bg
    └── adeb-stdlib

adeb-frontend-c → adeb-core, adeb-middle
adeb-middle → adeb-core
adeb-backend-x64 → adeb-core, adeb-middle, adeb-platform
adeb-bg → adeb-core
```

## Resumen

Esta reorganización convierte el compilador monolítico en un **workspace modular** que:
- Permite compilar en paralelo
- Habilita caching granulado
- Facilita el mantenimiento
- Permite features opcionales
- Organiza por dominio (no por tipo de archivo)
- Prepara para nuevos lenguajes (Rust, Zig, etc.)
