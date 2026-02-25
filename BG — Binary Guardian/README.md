# BG — Binary Guardian

**Deterministic ISA-Level Capability Guardian**

> No antivirus. No sandbox clásico. No heurísticas.
> Arquitectura de control estructural.

---

## ¿Qué es BG?

BG analiza binarios **antes de ejecutarlos**, clasificando cada instrucción
por lo que **ES**, no por lo que **parece**. Genera un **Architecture Map**
compacto y lo evalúa contra una **Security Policy** determinista.

```
Binary → ISA Decoder → ABIB IR → Capability Mapper
    → Architecture Map → Policy Engine → APPROVE / DENY
```

### Principios

| Principio | Descripción |
|-----------|-------------|
| **Pre-execution** | Analiza una vez, genera mapa compacto |
| **Determinista** | Mismo binario + misma policy = mismo resultado |
| **O(n) build** | Single-pass sobre el stream de instrucciones |
| **O(1) query** | El mapa ya tiene toda la información |
| **Directo al ISA** | No depende de lenguaje, formato, ni alto nivel |
| **No heurístico** | Clasificación matemática, no probabilística |

---

## Arquitectura

```
              ┌───────────────────┐
              │ External Binary   │  (.exe / .elf / .bin)
              └─────────┬─────────┘
                        ↓
              ┌───────────────────┐
              │ Binary Loader     │  PE/ELF/Raw parsing
              └─────────┬─────────┘
                        ↓
              ┌───────────────────┐
              │ ISA Decoder       │  bytes → ADeadOp (ABIB IR)
              └─────────┬─────────┘
                        ↓
              ┌───────────────────┐
              │ Capability Mapper │  ADeadOp → ArchitectureMap
              └─────────┬─────────┘
                        ↓
              ┌───────────────────┐
              │ Architecture Map  │  Perfil estructural completo
              └─────────┬─────────┘
                        ↓
              ┌───────────────────┐
              │ Policy Engine     │  ArchMap × Policy → Verdict
              └─────────┬─────────┘
                        ↓
                 APPROVE / DENY
```

---

## Architecture Map

El Architecture Map contiene 5 sub-mapas:

### Instruction Map
Clasifica cada instrucción:
- **SAFE** — mov, add, sub, xor, push, pop, ret, nop...
- **RESTRICTED** — syscall, int N, far jmp
- **PRIVILEGED** — cli, sti, hlt, lgdt, mov cr, rdmsr, in/out...

### Memory Map
- Regiones de código, datos, read-only
- Detección de regiones RWX (sospechosas)
- Detección de código auto-modificante

### Syscall Map
- Conteo de syscalls
- Vectores de interrupción usados (INT 0x80, etc.)
- Uso de instrucción SYSCALL

### IO Map
- Accesos a puertos (estáticos y dinámicos)
- Puertos únicos usados

### Control Flow Map
- Saltos directos/indirectos
- Calls directos/indirectos
- Far jumps
- Sitios de control flow indirecto (potenciales gadgets)

---

## Security Levels (x86-64 Rings)

| Level | Ring | Permite |
|-------|------|---------|
| **Kernel** | 0 | Todo — CR, MSR, GDT/IDT, IO, interrupts |
| **Driver** | 1 | IO + instrucciones restringidas, sin CR/MSR/GDT |
| **Service** | 2 | Syscalls solamente, sin hardware directo |
| **User** | 3 | Solo instrucciones safe + syscalls |
| **Sandbox** | 3 | Casi nada — sin syscalls, sin indirectos |

---

## Uso — CLI

```bash
# Analizar un binario con policy de usuario (default)
bg analyze program.exe

# Analizar con policy de kernel
bg analyze kernel.bin --policy kernel

# Quick check: ¿puede ejecutarse como driver?
bg check driver.sys --level driver

# Inspeccionar sin policy (solo muestra el map)
bg inspect firmware.bin

# Info del binario (formato, secciones)
bg info program.exe
```

---

## Uso — Como Library

```rust
use bg::{BinaryGuardian, SecurityPolicy, SecurityLevel};
use std::path::Path;

// Analizar archivo
let result = BinaryGuardian::analyze_file(
    Path::new("program.exe"),
    &SecurityPolicy::user(),
).unwrap();

println!("{}", result);

// Quick check
if BinaryGuardian::can_execute(&code_bytes, SecurityLevel::User) {
    // Safe to execute
}

// Integración con compilador ADead-BIB (zero-cost)
let map = BinaryGuardian::inspect_ir(&compiler_ops);
```

---

## Diferencia con Antivirus

| | Antivirus | BG |
|---|---|---|
| Método | Firmas + heurísticas | Clasificación ISA |
| Base | `if hash == known_bad` | `if instruction ∈ privileged_set` |
| Actualizaciones | Necesita DB de firmas | No necesita — es matemático |
| False positives | Frecuentes | Imposibles (determinista) |
| Performance | Scan pesado | O(n) una vez, O(1) después |
| Scope | Solo malware conocido | TODO binario |

---

## Integración con FastOS

BG está diseñado para integrarse en el loader de FastOS:

```rust
// En el kernel loader de FastOS
fn load_program(binary: &[u8]) -> Result<(), &'static str> {
    if !BinaryGuardian::can_execute(binary, SecurityLevel::User) {
        return Err("Binary denied by BG policy");
    }
    // Proceed with loading...
    Ok(())
}
```

---

## Build

```bash
cd "BG — Binary Guardian"
cargo build
cargo test
```

---

## Autor

**Eddi Andreé Salazar Matos**
`eddi.salazar.dev@gmail.com`

Parte del proyecto **ADead-BIB** — ASM Dead, Binary Is Binary.
