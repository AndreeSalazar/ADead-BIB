# FastOS — Custom Operating System

**Format: FsOS** (not PE, not ELF — our own binary format)

## Architecture: 3 Languages, Zero ASM

| Language  | Role                    | Files                                    |
|-----------|-------------------------|------------------------------------------|
| ADead-BIB | Base / Hardware         | `boot/stage1.adB`, `stage2.adB`          |
| Rust      | Security / Kernel Logic | `kernel/src/*.rs`, `kernel/src/apps/*.rs` |
| C         | Compatibility / ABI     | `include/*.h`                             |

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
│   ├── stage1.adB              # Boot sector (512 bytes, interactive Y/N)
│   └── stage2.adB              # Mode switch: Real → Protected → Long Mode
├── kernel/
│   ├── Cargo.toml              # Rust bare-metal kernel
│   ├── x86_64-fastos.json      # Custom target (no_std, no SSE)
│   └── src/
│       ├── main.rs             # Kernel entry → installer → login → desktop
│       ├── vga.rs              # VGA 80x25 driver (16 colors, rects, lines)
│       ├── keyboard.rs         # PS/2 keyboard (scancode set 1, shift)
│       ├── installer.rs        # Interactive installer (Y=install / N=live)
│       ├── login.rs            # Windows-style login (user/password)
│       ├── desktop.rs          # Desktop manager (taskbar, icons, navigation)
│       ├── window.rs           # Window manager (title bar, borders, [X])
│       ├── startmenu.rs        # Start menu popup (app list + shutdown)
│       ├── shell.rs            # CLI shell (fastos> prompt, 8 commands)
│       ├── panic.rs            # Red screen panic handler
│       └── apps/
│           ├── mod.rs          # Apps module
│           ├── terminal.rs     # Terminal emulator (inside window)
│           ├── files.rs        # File Manager (virtual filesystem)
│           ├── editor.rs       # Text Editor (line numbers, cursor)
│           ├── calc.rs         # Calculator (+, -, *, /)
│           ├── sysinfo.rs      # System Information panel
│           └── settings.rs     # Settings panel
├── include/
│   ├── fastos_types.h          # C type definitions (shared ABI)
│   └── fastos_kernel.h         # C kernel interface (FFI contract)
├── link/
│   └── kernel.ld               # Linker script (kernel at 0x100000)
├── build/                      # Build output
├── build.ps1                   # PowerShell build script
└── README.md
```

## Boot Flow

```text
BIOS → Stage1 (16-bit, ADead-BIB)
         ↓ "=== FastOS v1.0 ==="
         ↓ "Install FastOS? (Y/N)"
       Stage2 (ADead-BIB: A20 → GDT → Protected → Long Mode)
         ↓
       Kernel (64-bit Rust)
         ↓
       Installer → Login Screen → Desktop
```

## Desktop (Windows-like)

```text
┌──────────────────────────────────────────────────────────────────────────────┐
│ FastOS Desktop                                        ADead-BIB+Rust+C v1.0│  ← Title bar
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐      │
│  │> Terminal │  │░ Files   │  │≡ Editor  │  │# Calc    │  │i SysInfo │      │  ← Desktop icons
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘  └──────────┘      │
│  ┌──────────┐                                                                │
│  │■ Settings│                                                                │
│  └──────────┘                                                                │
│                                                                              │
│  WASD:Move  Enter:Open  1-6:Quick  M:Menu  Q:Shutdown                       │
├──────────────────────────────────────────────────────────────────────────────┤
│[M]Start │ Terminal │                                    │ FsOS 64bit        │  ← Taskbar
└──────────────────────────────────────────────────────────────────────────────┘
```

## Desktop Apps

- **Terminal** — Full shell inside a window (help, ver, info, echo, clear)
- **File Manager** — Virtual filesystem browser (directories + files)
- **Text Editor** — Line-numbered editor with cursor (Ln/Col status bar)
- **Calculator** — Integer calculator (+, -, *, /, =, C, backspace)
- **System Info** — OS, CPU, memory, format, technology stack
- **Settings** — Username, hostname, theme, keyboard, language, boot mode

## Login Screen

- **Default credentials:** `admin` / `fastos`
- Blue Windows-style background with bordered login box
- Password field masked with `*`
- 3 attempts before reboot

## Start Menu

Press **M** on the desktop to open the Start menu:
- App launcher (1-6 hotkeys)
- About FastOS
- Shutdown

## Features

- **Interactive installer** — Always asks before installing
- **Windows-style login** — User/password before desktop
- **Desktop with taskbar** — Icons, Start menu, system tray
- **Window manager** — Title bars, borders, close buttons
- **6 desktop apps** — Terminal, Files, Editor, Calc, SysInfo, Settings
- **FsOS format** — Own binary format, not PE or ELF
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

Eddi Andreé Salazar Matos
