# FastOS v2.0 — ADead-BIB Native Operating System

**Un OS moderno escrito en C puro, compilado con ADead-BIB.**

Inspirado en:
- **Linux** — Arquitectura de kernel modular, syscalls POSIX
- **Windows NT** — Diseño de drivers, formato ejecutable
- **Pop!_OS** — Sistema de drivers GPU (System76-power style)
- **Nouveau** — Driver open-source para NVIDIA GPUs

## Características Únicas

- **Formato Po** — Ejecutable nativo híbrido (PE + ELF inspirado)
- **BG Integration** — Binary Guardian verifica binarios antes de ejecutar
- **Syscalls Híbridos** — Compatible Linux + extensiones FastOS
- **Seguridad por diseño** — Niveles de seguridad integrados (Kernel/Driver/User/Sandbox)

## Arquitectura

```
FastOS/
├── boot/                    # Bootloader (BIOS + UEFI)
│   ├── stage1.c            # MBR boot sector (512 bytes)
│   ├── stage2.c            # Second stage loader
│   └── uefi/               # UEFI boot support
├── kernel/                  # Kernel core
│   ├── main.c              # Kernel entry point
│   ├── memory.c            # Memory management (paging, heap)
│   ├── interrupts.c        # IDT, IRQ handlers
│   ├── scheduler.c         # Process scheduler
│   ├── syscall.c           # System call interface
│   └── panic.c             # Kernel panic handler
├── drivers/                 # Device drivers
│   ├── video/              # Video drivers
│   │   ├── vga.c          # VGA text mode
│   │   ├── vesa.c         # VESA framebuffer
│   │   └── nouveau/       # NVIDIA GPU (Nouveau-inspired)
│   ├── storage/            # Storage drivers
│   │   ├── ata.c          # ATA/IDE
│   │   └── ahci.c         # SATA AHCI
│   ├── input/              # Input drivers
│   │   ├── keyboard.c     # PS/2 keyboard
│   │   └── mouse.c        # PS/2 mouse
│   └── pci/                # PCI bus
│       └── pci.c          # PCI enumeration
├── fs/                      # File systems
│   ├── vfs.c               # Virtual File System
│   ├── fat32.c             # FAT32 support
│   └── ext2.c              # EXT2 support
├── lib/                     # C runtime library
│   ├── string.c            # String functions
│   ├── memory.c            # memcpy, memset, etc.
│   └── printf.c            # printf implementation
├── include/                 # Header files
│   ├── kernel.h            # Kernel definitions
│   ├── types.h             # Type definitions
│   └── drivers/            # Driver headers
└── userspace/               # User applications
    ├── shell.c             # Command shell
    └── init.c              # Init process
```

## Compilación

```bash
# Compilar bootloader
adB cc boot/stage1.c -o boot/stage1.bin --flat --org=0x7C00

# Compilar kernel
adB cc kernel/main.c -o kernel/kernel.bin --flat --org=0x100000

# Crear imagen de disco
./build/mkimage.sh
```

## Características

### Kernel
- **Monolítico modular** — Core pequeño, drivers cargables
- **Paginación x86-64** — 4-level page tables
- **Multitarea preemptiva** — Round-robin scheduler
- **System calls** — Interfaz POSIX-like

### Drivers GPU (Nouveau-inspired)
- **Detección PCI** — Enumera dispositivos NVIDIA
- **Framebuffer** — Modo gráfico básico
- **Reclocking** — Control de frecuencia (GPUs soportadas)

### Compatibilidad
- **BIOS Legacy** — Boot desde MBR
- **UEFI** — Boot moderno con GOP
- **x86-64** — Solo 64-bit

## Filosofía

> **C es el lenguaje del kernel. ADead-BIB es el compilador.**

- Sin dependencias externas (no GCC, no LLVM)
- Código limpio y legible
- Documentación inline
- Compatible con hardware real

## Licencia

MIT License — Eddi Andreé Salazar Matos
