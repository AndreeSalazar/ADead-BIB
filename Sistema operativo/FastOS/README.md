# FastOS — GPU-First / Binary-First Operating System

> **FastOS** = El primer **GPU-First / Binary-First Operating System**
> 
> Stack: **ADead-BIB + Rust + wgpu**
> 
> Virgen. Directo. Sin legacy. GPU desde el día uno.

---

## 🎯 Nueva Categoría: GPU-First / Binary-First OS

FastOS introduce una nueva categoría de sistemas operativos:

| Característica | OS Tradicional | FastOS |
|----------------|----------------|--------|
| **Prioridad** | CPU-first | **GPU-first** |
| **Compilación** | ASM → Linker → Binary | **Binary-first (directo)** |
| **Graphics** | Driver separado | **GPU integrado en kernel** |
| **Rendering** | Software fallback | **Hardware acelerado siempre** |
| **Boot** | BIOS → DOS → Windows | **UEFI → GPU → FastOS** |

---

## 🎯 Filosofía

```
┌─────────────────────────────────────────────────────────────────┐
│                    FastOS — Arquitectura                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                   APLICACIONES                             │  │
│  │   • Juegos (ADead-BIB)                                     │  │
│  │   • Utilidades                                             │  │
│  │   • Shell                                                  │  │
│  └───────────────────────────────────────────────────────────┘  │
│                              │                                   │
│                              ▼                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                   FastOS API (Syscalls)                    │  │
│  └───────────────────────────────────────────────────────────┘  │
│                              │                                   │
│                              ▼                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                   FastOS KERNEL                            │  │
│  │                                                            │  │
│  │   ┌────────────┐  ┌────────────┐  ┌────────────┐          │  │
│  │   │  Scheduler │  │   Memory   │  │  Drivers   │          │  │
│  │   │   (Rust)   │  │   (Rust)   │  │(Rust/wgpu) │          │  │
│  │   └────────────┘  └────────────┘  └────────────┘          │  │
│  │                                                            │  │
│  └───────────────────────────────────────────────────────────┘  │
│                              │                                   │
│                              ▼                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                   HARDWARE (x86-64)                        │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 🚀 Stack Tecnológico

| Componente | Tecnología | Descripción |
|------------|------------|-------------|
| **Bootloader** | Rust + ADead-BIB | UEFI/BIOS boot |
| **Kernel** | Rust (no_std) | Kernel mínimo |
| **Drivers** | Rust + wgpu | GPU, teclado, disco |
| **Filesystem** | Rust | FAT32 + FastFS |
| **Graphics** | wgpu/Vulkan | GPU directo |
| **Apps** | ADead-BIB | Aplicaciones nativas |

---

## 📁 Estructura del Proyecto

```
FastOS/
├── boot/
│   └── bootloader.rs       # Bootloader UEFI
│
├── kernel/
│   ├── main.rs             # Entry point
│   ├── memory.rs           # Gestión de memoria
│   ├── scheduler.rs        # Planificador
│   ├── syscalls.rs         # Llamadas al sistema
│   └── interrupts.rs       # IDT
│
├── drivers/
│   ├── keyboard.rs         # Driver teclado
│   ├── display.rs          # Framebuffer
│   ├── gpu.rs              # GPU (wgpu)
│   └── disk.rs             # Disco
│
├── fs/
│   ├── vfs.rs              # Virtual File System
│   └── fat32.rs            # FAT32
│
├── userspace/
│   ├── shell.rs            # Shell
│   └── apps/               # Aplicaciones
│
└── src/
    └── lib.rs              # Librería común
```

---

## 🛠️ Compilar y Ejecutar

```bash
# Compilar kernel
cargo build --release

# Crear imagen booteable
cargo run --bin mkimage

# Ejecutar en QEMU
qemu-system-x86_64 -drive format=raw,file=fastos.img
```

---

## 📋 Requisitos

- Rust nightly (para `#![no_std]`)
- QEMU (para testing)
- wgpu (para GPU)

---

## 🎯 Objetivos v1.0

- [x] Estructura del proyecto
- [ ] Bootloader UEFI básico
- [ ] Kernel mínimo (print)
- [ ] Gestión de memoria
- [ ] Driver de teclado
- [ ] Framebuffer básico
- [ ] Shell simple

---

## 👤 Autor

**Eddi Andreé Salazar Matos**  
📧 eddi.salazar.dev@gmail.com  
🇵🇪 Hecho con ❤️ en Perú

---

**FastOS: Rápido. Directo. Sin mentiras.**
