# TESTEO - Tests de ADead-BIB

> **Binary Is Binary** - Tests organizados por categoría
> 
> CPU = Contratos Binarios | GPU = Contratos HEX

Esta carpeta contiene tests para todas las funcionalidades de ADead-BIB.

---

## 🎯 Filosofía: Contratos Directos

```
CPU (Binario):
  Código → Bytes x86-64 → Ejecutable
  Sin ASM intermedio. Sin reinterpretación.

GPU (HEX):
  Código → Opcodes HEX → GPU
  Sin GLSL/HLSL. Sin shaders textuales.
```

---

## 📁 Estructura Principal

```
TESTEO/
│
├── CPU/                    # 🔵 CONTRATOS BINARIOS x86-64
│   ├── binario/            # Literales 0b...
│   ├── opcodes/            # Opcodes x86-64
│   ├── contratos/          # Calling conventions
│   └── README.md           # Guía CPU
│
├── GPU/                    # 🟢 CONTRATOS HEX DIRECTOS
│   ├── hex/                # Literales 0x...
│   ├── opcodes/            # Opcodes GPU (0xC0DA...)
│   ├── contratos/          # Command buffers
│   └── README.md           # Guía GPU
│
├── v1/                     # Tests legacy v1.x
│   ├── arrays/             # Arrays y colecciones
│   ├── conversiones/       # int(), float(), bool()
│   ├── input/              # input() real
│   ├── len/                # len() function
│   ├── modules/            # Sistema de módulos
│   └── traits/             # Traits e interfaces
│
├── v2/                     # Tests v2.0.0 HEX-First
│   ├── hex/                # Literales HEX
│   ├── raw/                # Modo raw binary
│   ├── cpu/                # CPU opcodes
│   ├── gpu/                # GPU opcodes
│   ├── clean/              # Post-procesamiento
│   └── integrados/         # Tests completos
│
└── README.md               # Esta guía
```

---

## 🔵 Tests CPU (Binario)

```bash
# Literales binarios (0b...)
cargo run --bin adeadc -- run TESTEO/CPU/binario/test_binary_literals.adB

# Opcodes x86-64
cargo run --bin adeadc -- run TESTEO/CPU/opcodes/test_x86_opcodes.adB

# Calling convention
cargo run --bin adeadc -- run TESTEO/CPU/contratos/test_calling_convention.adB
```

| Test | Archivo | Estado |
|------|---------|--------|
| Literales Binarios | test_binary_literals.adB | ✅ PASA |
| Opcodes x86-64 | test_x86_opcodes.adB | ✅ PASA |
| Calling Convention | test_calling_convention.adB | ✅ PASA |

---

## 🟢 Tests GPU (HEX)

```bash
# Literales HEX (0x...)
cargo run --bin adeadc -- run TESTEO/GPU/hex/test_hex_literals.adB

# Opcodes GPU (0xC0DA...)
cargo run --bin adeadc -- run TESTEO/GPU/opcodes/test_gpu_opcodes.adB

# Command buffer
cargo run --bin adeadc -- run TESTEO/GPU/contratos/test_command_buffer.adB
```

| Test | Archivo | Estado |
|------|---------|--------|
| Literales HEX | test_hex_literals.adB | ✅ PASA |
| Opcodes GPU | test_gpu_opcodes.adB | ✅ PASA |
| Command Buffer | test_command_buffer.adB | ✅ PASA |

---

## 🔗 Relación CPU ↔ GPU

```
CPU prepara → GPU ejecuta → CPU recibe

CPU:
  1. Escribe datos en memoria
  2. Escribe comandos GPU
  3. Dispara ejecución
  4. Se aparta

GPU:
  1. Lee comandos
  2. Ejecuta kernels
  3. Escribe resultados
  4. Sin volver a preguntar
```

**La CPU NO mira cada iteración.**
**La GPU NO pide permiso.**

---

## 📋 Tests Legacy (v1.x)

```bash
# Test de arrays
cargo run --bin adeadc -- run TESTEO/arrays/test_foreach.adB

# Test de len()
cargo run --bin adeadc -- run TESTEO/len/test_len_array.adB

# Test completo v1.3.0
cargo run --bin adeadc -- run TESTEO/integrados/test_v1_3_0_completo.adB
```

| Feature | Test | Estado |
|---------|------|--------|
| Arrays | test_array_basico.adB | ✅ OK |
| for x in arr | test_foreach.adB | ✅ OK |
| len(arr) | test_len_array.adB | ✅ OK |
| int() | test_int.adB | ✅ OK |
| float() | test_float.adB | ✅ OK |
| bool() | test_bool.adB | ✅ OK |
| input() | test_v1_4_0_input.adB | ✅ OK |

---

*ADead-BIB: CPU (Binario) + GPU (HEX) = Contratos Directos*
