# FastOS — Custom Operating System

**Format: FsOS** (not PE, not ELF — our own binary format)

## Architecture: 3 Languages, Zero ASM

| Language   | Role                    | Files                          |
| ---------- | ----------------------- | ------------------------------ |
| ADead-BIB  | Base / Hardware         | `boot/stage1.adB`, `stage2.adB` |
| Rust       | Security / Kernel Logic | `kernel/src/*.rs`              |
| C          | Compatibility / ABI     | `include/*.h`                  |

## FsOS Binary Format

```text
Magic:   "FsOS" (0x46 0x73 0x4F 0x53)
Header:  64 bytes (compact, like ELF but simpler)
Modes:   16-bit / 32-bit / 64-bit (natural scaling)
Default: 64-bit Long Mode
```

**Not PE. Not ELF. FastOS has its own format.**

## Directory Structure

```text
FastOS/
├── boot/
│   ├── stage1.adB          # Boot sector (512 bytes, interactive Y/N prompt)
│   └── stage2.adB          # Mode switch: Real → Protected → Long Mode
├── kernel/
│   ├── Cargo.toml           # Rust bare-metal kernel
│   ├── x86_64-fastos.json   # Custom target (no_std, no SSE)
│   └── src/
│       ├── main.rs          # Kernel entry (kernel_main)
│       ├── vga.rs           # VGA text mode driver (80x25, 16 colors)
│       ├── keyboard.rs      # PS/2 keyboard driver
│       ├── installer.rs     # Interactive installer (asks Y/N on first boot)
│       ├── shell.rs         # Command-line shell (fastos> prompt)
│       └── panic.rs         # Red screen panic handler
├── include/
│   ├── fastos_types.h       # C type definitions (shared ABI)
│   └── fastos_kernel.h      # C kernel interface (FFI contract)
├── link/
│   └── kernel.ld            # Linker script (kernel at 0x100000)
├── build/                   # Build output directory
├── build.ps1                # PowerShell build script
└── README.md                # This file
```

## Boot Flow

```text
BIOS → Stage1 (16-bit, ADead-BIB)
         ↓ "Install FastOS? (Y/N)"
       Stage2 (ADead-BIB: A20 → GDT → Protected → Long Mode)
         ↓
       Kernel (64-bit Rust: VGA + Keyboard + Installer + Shell)
         ↓
       fastos> prompt (interactive shell)
```

## Features

- **Interactive installer** — Always asks before installing
- **FsOS format** — Own binary format, not PE or ELF
- **Green terminal theme** — Unique personality
- **Shell commands** — help, info, ver, about, clear, echo, halt, reboot
- **Red screen panic** — Rust panic handler with file/line info
- **64-bit native** — Scales from 16-bit boot to 64-bit kernel

## Build & Test

```powershell
# Build everything
.\build.ps1

# Build and run in QEMU
.\build.ps1 -Run

# Or manually:
adB fastos boot\stage1.adB -o build\fastos.bin --run
```

## Author

Eddi Andreé Salazar Matos — eddi.salazar.dev@gmail.com
