# FastOS — Architecture Document

> Arquitectura completa de un OS real: ADead-BIB + Rust + C

---

## System Layers

```text
┌─────────────────────────────────────────────────┐
│                  Apps Layer                      │
│         ADead-BIB + C + Rust applications        │
├─────────────────────────────────────────────────┤
│                Desktop Layer                     │
│     Compositor │ Window Manager │ Shell          │
├─────────────────────────────────────────────────┤
│                System Layer                      │
│   Filesystem │ IPC │ Security │ Services         │
├─────────────────────────────────────────────────┤
│                Kernel Core                       │
│  Memory │ Scheduler │ Interrupts │ Processes     │
├─────────────────────────────────────────────────┤
│                Driver Layer                      │
│  Framebuffer │ Keyboard │ Mouse │ Disk │ Timer   │
├─────────────────────────────────────────────────┤
│                Boot System                       │
│         Stage1 │ Stage2 (ADead-BIB)              │
├─────────────────────────────────────────────────┤
│                  Hardware                        │
│        CPU │ RAM │ GPU │ Storage │ I/O           │
└─────────────────────────────────────────────────┘
```

---

## Directory Structure

```text
FastOS/
├── boot/                          # Boot system (ADead-BIB)
│   ├── stage1.adB                 #   BIOS boot sector (512 bytes)
│   └── stage2.adB                 #   Stage2 loader + mode switch
│
├── kernel/                        # Kernel (Rust)
│   ├── Cargo.toml
│   ├── x86_64-fastos.json         #   Custom target spec
│   └── src/
│       ├── main.rs                #   Kernel entry point
│       ├── panic.rs               #   Panic handler
│       │
│       ├── core/                  #   Core kernel subsystems
│       │   ├── mod.rs
│       │   ├── memory.rs          #     Physical + virtual memory manager
│       │   ├── scheduler.rs       #     Process scheduler (round-robin → CFS)
│       │   ├── process.rs         #     Process/thread management
│       │   └── interrupts.rs      #     IDT, ISR, IRQ handling
│       │
│       ├── arch/                  #   Architecture-specific code
│       │   └── x86_64/
│       │       ├── mod.rs
│       │       ├── gdt.rs         #     Global Descriptor Table
│       │       ├── idt.rs         #     Interrupt Descriptor Table
│       │       ├── paging.rs      #     Page tables (4-level)
│       │       ├── port.rs        #     I/O port access
│       │       └── cpu.rs         #     CPU feature detection
│       │
│       └── drivers/               #   Hardware drivers
│           ├── mod.rs
│           ├── framebuffer.rs     #     BIOS VBE / UEFI GOP framebuffer
│           ├── vga.rs             #     VGA text mode (legacy)
│           ├── keyboard.rs        #     PS/2 keyboard
│           ├── mouse.rs           #     PS/2 mouse
│           ├── disk.rs            #     ATA/AHCI disk
│           └── timer.rs           #     PIT / APIC timer
│
├── system/                        # System services (Rust + C)
│   ├── fs/                        #   Filesystem
│   │   ├── mod.rs
│   │   ├── fastfs.rs             #     FastFS (native filesystem)
│   │   └── vfs.rs                #     Virtual Filesystem layer
│   ├── ipc/                       #   Inter-Process Communication
│   │   ├── mod.rs
│   │   ├── pipe.rs
│   │   └── shm.rs                #     Shared memory
│   └── security/                  #   Security
│       ├── mod.rs
│       └── rings.rs              #     Ring 0/3 separation
│
├── desktop/                       # Desktop environment (Rust)
│   ├── compositor.rs              #   Window compositor (alpha, z-order)
│   ├── window_manager.rs          #   Window create/move/resize/close
│   ├── shell.rs                   #   Desktop shell (taskbar, icons, bg)
│   ├── cursor.rs                  #   Mouse cursor rendering
│   └── assets/
│       ├── icons/                 #   SVG icons (rasterized at boot)
│       └── wallpaper/             #   Desktop wallpapers
│
├── apps/                          # Applications
│   ├── terminal/                  #   Terminal emulator
│   ├── file_manager/              #   File explorer
│   ├── settings/                  #   System settings
│   ├── calculator/                #   Calculator
│   ├── editor/                    #   Text editor
│   └── sysinfo/                   #   System information
│
├── sdk/                           # Development SDK
│   └── libraries/                 #   Shared libraries
│
├── doc/                           # Documentation
│   ├── ideas.md
│   ├── architecture.md            #   This file
│   ├── roadmap.md
│   ├── boot.md
│   ├── kernel.md
│   ├── desktop.md
│   ├── drivers.md
│   ├── apps.md
│   └── security.md
│
├── build/                         # Build output
│   ├── stage1.bin
│   ├── stage2.bin
│   └── fastos.bin                 #   Complete disk image
│
├── link/                          # Linker scripts
│   └── kernel.ld
│
├── include/                       # C headers
│   └── fastos.h
│
├── build.ps1                      # Build script
└── README.md
```

---

## Language Roles

### ADead-BIB → Hardware Control

```text
Responsibilities:
  - Boot sector (stage1)
  - Stage2 loader
  - CPU initialization
  - Real mode → Protected mode → Long mode switch
  - Interrupt entry stubs (ISR wrappers)
  - Context switch ASM sequences
  - Low-level hardware entry points

Mode: Machine Code (raw bytes, no NASM, no LLVM)
```

### Rust → System Core

```text
Responsibilities:
  - Kernel core (memory, scheduler, processes)
  - Interrupt handling logic
  - All drivers (framebuffer, keyboard, mouse, disk, timer)
  - Filesystem implementation
  - Desktop compositor + window manager
  - System services
  - Application framework

Benefits:
  - No memory corruption
  - No use-after-free
  - No buffer overflows in kernel
  - Compile-time safety guarantees
```

### C → Compatibility Layer

```text
Responsibilities:
  - Legacy hardware drivers
  - Portable library interfaces
  - Third-party software compatibility
  - ABI-stable interfaces
  - FFI bridge between ADead-BIB and Rust
```

---

## Kernel Type: Hybrid

FastOS uses a **hybrid kernel** architecture:

- **Monolithic performance** — drivers run in kernel space for speed
- **Modular design** — components are isolated modules
- **Future microkernel path** — drivers can be moved to userspace later

Comparable to:

- Windows NT (hybrid)
- macOS XNU (hybrid)

---

## Memory Model

```text
Virtual Address Space (48-bit, 4-level paging):

0x0000_0000_0000_0000 ─ 0x0000_7FFF_FFFF_FFFF  → User space (128 TB)
0xFFFF_8000_0000_0000 ─ 0xFFFF_FFFF_FFFF_FFFF  → Kernel space (128 TB)

Kernel layout:
  0xFFFF_8000_0000_0000  → Kernel code + data
  0xFFFF_8000_0100_0000  → Kernel heap
  0xFFFF_8000_1000_0000  → Framebuffer mapping
  0xFFFF_FFFF_8000_0000  → Recursive page tables
```

---

## Boot Flow

```text
Power On
  ↓
BIOS POST
  ↓
Stage1 (ADead-BIB, 512 bytes)
  - Init CPU segments + stack
  - Splash screen
  - Load stage2 from disk
  - Jump to stage2
  ↓
Stage2 (ADead-BIB, ~8KB max)
  - Enable A20 line
  - Set up GDT
  - Switch to Protected Mode (32-bit)
  - Set up page tables
  - Switch to Long Mode (64-bit)
  - Set VBE framebuffer mode
  - Load kernel from disk
  - Jump to kernel_main
  ↓
Kernel (Rust)
  - Initialize memory manager
  - Set up IDT + interrupts
  - Initialize drivers
  - Start scheduler
  - Launch desktop
  ↓
Desktop (Rust)
  - Login screen
  - Desktop compositor
  - Window manager
  - Shell (taskbar, icons)
  ↓
Apps (ADead-BIB + C + Rust)
  - Terminal, Files, Settings, etc.
```

---

## Graphics Stack

```text
Hardware (GPU/VGA)
     ↓
Framebuffer Driver (Rust)
  - BIOS VBE mode setting
  - Linear framebuffer access
  - Pixel/rect/line primitives
     ↓
Graphics Engine (Rust)
  - Font rendering (bitmap fonts)
  - SVG rasterizer (icons)
  - Image decoding
     ↓
Compositor (Rust)
  - Window compositing
  - Alpha blending
  - Z-order management
  - Damage tracking
     ↓
Desktop Shell (Rust)
  - Taskbar
  - Desktop icons
  - Wallpaper
  - Start menu
     ↓
Screen
```

---

## Comparable Systems

| Aspect          | Windows NT        | Linux              | FastOS             |
|-----------------|-------------------|--------------------|--------------------|
| Kernel          | Hybrid (C/C++)    | Monolithic (C)     | Hybrid (Rust)      |
| Boot            | NTLDR/bootmgr     | GRUB               | ADead-BIB          |
| Binary format   | PE                | ELF                | FsOS               |
| Desktop         | DWM               | X11/Wayland        | FastOS Compositor  |
| Filesystem      | NTFS              | ext4/btrfs         | FastFS             |
| Memory safety   | No                | No                 | Yes (Rust)         |
| ASM layer       | MASM              | GAS/NASM           | ADead-BIB          |
