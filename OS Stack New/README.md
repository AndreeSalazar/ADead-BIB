# ADead-OS Stack — 3 Languages, Zero ASM

## Architecture

```
┌─────────────────────────────────────────────────┐
│                  ADead-OS Stack                 │
├─────────────────────────────────────────────────┤
│                                                 │
│   ┌───────────┐  ┌──────────┐  ┌────────────┐   │
│   │ ADead-BIB │  │   Rust   │  │     C      │   │
│   │   BASE    │  │ SECURITY │  │ COMPAT     │   │
│   ├───────────┤  ├──────────┤  ├────────────┤   │
│   │ Boot      │  │ Kernel   │  │ ABI Headers│   │
│   │ GDT/IDT   │  │ Scheduler│  │ Types      │   │
│   │ ISR Wrap  │  │ Memory   │  │ Interfaces │   │
│   │ Hardware  │  │ VGA      │  │ libc compat│   │
│   │ Mode Sw.  │  │ Panic    │  │            │   │
│   └───────────┘  └──────────┘  └────────────┘   │
│                                                 │
│              NO ASM. EVER.                      │
│     ADead-BIB generates all machine code.       │
└─────────────────────────────────────────────────┘
```

## The 3 Languages

| Language | Role | Handles |
|----------|------|---------|
| **ADead-BIB** | Base | Boot sector, GDT/IDT setup, ISR wrappers, hardware init, mode transitions, port I/O — replaces ASM 100% |
| **Rust** | Security | Kernel main logic, memory manager, VGA driver, panic handler — `#![no_std]` bare-metal |
| **C** | Compatibility | ABI headers (`stdint.h` style), type definitions, interface contracts between ADead-BIB and Rust |

## Directory Structure

```
OS Stack New/
├── boot/
│   ├── stage1.adB          # Boot sector (512 bytes) — ADead-BIB
│   └── stage2.adB          # Mode switch Real→Protected→Long — ADead-BIB
├── kernel/
│   ├── Cargo.toml          # Rust kernel crate
│   └── src/
│       ├── main.rs         # kernel_main entry point — Rust
│       ├── vga.rs          # VGA text mode driver — Rust
│       └── panic.rs        # Panic handler — Rust
├── include/
│   ├── adead_kernel.h      # C header: ADead-BIB ↔ Rust interface
│   └── adead_types.h       # C header: shared type definitions
├── link/
│   └── kernel.ld           # Linker script combining all 3
├── build.ps1               # Build script (Windows PowerShell)
└── README.md               # This file
```

## Build & Test

```powershell
# Build everything
.\build.ps1

# Test with QEMU
qemu-system-x86_64 -drive format=raw,file=build/os.bin -no-reboot -no-shutdown
```

## How It Works

1. **BIOS loads `stage1`** (ADead-BIB boot sector) at 0x7C00
2. **stage1** initializes segments, prints "ADead-OS", loads stage2
3. **stage2** (ADead-BIB) sets up GDT, enables A20, transitions to Protected Mode, then Long Mode
4. **stage2** jumps to `kernel_main` (Rust)
5. **Rust kernel** initializes VGA, prints welcome message, enters idle loop
6. **C headers** define the ABI contract between ADead-BIB and Rust

## Philosophy

> **ADead-BIB IS the assembler.** It generates x86 machine code directly — no NASM, no GAS, no LLVM asm. The CPU sees bytes, and ADead-BIB writes those bytes. Rust provides memory safety for kernel logic. C provides the universal ABI glue.

---

**Author:** Eddi Andreé Salazar Matos
**Version:** 1.0
