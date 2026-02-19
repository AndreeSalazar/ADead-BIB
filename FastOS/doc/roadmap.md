# FastOS — Roadmap

> Orden exacto de construcción, fase por fase.

---

## Fase 0 — Foundation (COMPLETADA)

```text
✅ stage1.adB — Boot sector con splash screen
✅ stage2.adB — Desktop VGA text mode (login + desktop + shutdown)
✅ kernel/src/main.rs — Kernel entry point
✅ kernel/src/vga.rs — VGA text mode driver
✅ kernel/src/keyboard.rs — Keyboard input
✅ kernel/src/desktop.rs — Desktop manager (text mode)
✅ kernel/src/window.rs — Window manager (text mode)
✅ kernel/src/startmenu.rs — Start menu
✅ kernel/src/apps/ — 6 apps (terminal, files, editor, calc, sysinfo, settings)
✅ desktop/icons/ — 11 SVG icons (Win11 style)
✅ desktop/ folders — Documents, Downloads, Pictures, Music, Videos, Desktop
✅ x86_64-fastos.json — Custom target spec
✅ Disk image build + QEMU test
```

---

## Fase 1 — Boot System Enhancement

**Objetivo:** Stage2 realiza mode switch completo y carga kernel real.

```text
Tareas:
  [ ] Enable A20 line (port 0x92 fast method + keyboard controller fallback)
  [ ] Set up temporary GDT (code32, data32, code64, data64)
  [ ] Switch from Real Mode (16-bit) to Protected Mode (32-bit)
  [ ] Set up identity-mapped page tables (PML4 → PDPT → PD)
  [ ] Enable PAE + Long Mode via MSR
  [ ] Switch to Long Mode (64-bit)
  [ ] Set VBE video mode (1024x768x32 via INT 10h AX=4F02h)
  [ ] Load kernel binary from disk sectors
  [ ] Jump to kernel_main in 64-bit mode

Archivos:
  boot/stage2.adB — Rewrite completo para mode switch
  
Resultado:
  BIOS → Stage1 → Stage2 → 64-bit kernel con framebuffer
```

---

## Fase 2 — Kernel Minimal

**Objetivo:** Kernel funcional con memoria, interrupciones y framebuffer.

```text
Tareas:
  [ ] kernel/src/core/memory.rs — Physical frame allocator (bitmap)
  [ ] kernel/src/core/memory.rs — Virtual memory manager (4-level paging)
  [ ] kernel/src/core/memory.rs — Kernel heap allocator (bump → linked list)
  [ ] kernel/src/core/interrupts.rs — IDT setup (256 entries)
  [ ] kernel/src/core/interrupts.rs — ISR handlers (exceptions 0-31)
  [ ] kernel/src/core/interrupts.rs — IRQ handlers (PIC remapping)
  [ ] kernel/src/arch/x86_64/gdt.rs — GDT with TSS
  [ ] kernel/src/arch/x86_64/idt.rs — IDT implementation
  [ ] kernel/src/arch/x86_64/paging.rs — Page table management
  [ ] kernel/src/drivers/framebuffer.rs — Linear framebuffer driver
  [ ] kernel/src/drivers/framebuffer.rs — Pixel, rect, line, fill primitives
  [ ] kernel/src/drivers/framebuffer.rs — Bitmap font rendering (8x16)

Archivos nuevos:
  kernel/src/core/mod.rs
  kernel/src/core/memory.rs
  kernel/src/core/interrupts.rs
  kernel/src/arch/x86_64/mod.rs
  kernel/src/arch/x86_64/gdt.rs
  kernel/src/arch/x86_64/idt.rs
  kernel/src/arch/x86_64/paging.rs
  kernel/src/arch/x86_64/port.rs
  kernel/src/drivers/framebuffer.rs

Resultado:
  Kernel arranca en 64-bit, maneja memoria, muestra gráficos en framebuffer
```

---

## Fase 3 — Drivers

**Objetivo:** Input completo + timer para multitasking.

```text
Tareas:
  [ ] kernel/src/drivers/keyboard.rs — PS/2 keyboard con scancodes completos
  [ ] kernel/src/drivers/keyboard.rs — Key repeat, modifiers (Shift, Ctrl, Alt)
  [ ] kernel/src/drivers/mouse.rs — PS/2 mouse driver
  [ ] kernel/src/drivers/mouse.rs — Cursor movement + button events
  [ ] kernel/src/drivers/timer.rs — PIT timer (channel 0, ~1000 Hz)
  [ ] kernel/src/drivers/timer.rs — System tick counter
  [ ] kernel/src/drivers/disk.rs — ATA PIO disk read/write

Resultado:
  Keyboard + mouse + timer + disk funcionales
```

---

## Fase 4 — Multitasking

**Objetivo:** Procesos y scheduling real.

```text
Tareas:
  [ ] kernel/src/core/process.rs — Process struct (PID, state, context)
  [ ] kernel/src/core/process.rs — Thread struct (stack, registers)
  [ ] kernel/src/core/scheduler.rs — Round-robin scheduler
  [ ] kernel/src/core/scheduler.rs — Timer-driven preemptive switching
  [ ] kernel/src/core/scheduler.rs — Context save/restore
  [ ] ADead-BIB context switch stub (save/restore registers)

Resultado:
  Múltiples procesos ejecutándose concurrentemente
```

---

## Fase 5 — Desktop Engine

**Objetivo:** Desktop gráfico real con compositor.

```text
Tareas:
  [ ] desktop/compositor.rs — Window compositing engine
  [ ] desktop/compositor.rs — Alpha blending
  [ ] desktop/compositor.rs — Damage tracking (dirty rects)
  [ ] desktop/compositor.rs — Z-order management
  [ ] desktop/window_manager.rs — Window create/destroy
  [ ] desktop/window_manager.rs — Window move/resize (mouse drag)
  [ ] desktop/window_manager.rs — Window focus + title bar
  [ ] desktop/window_manager.rs — Window decorations (close, minimize, maximize)
  [ ] desktop/shell.rs — Taskbar rendering
  [ ] desktop/shell.rs — Desktop icons (from SVG rasterized)
  [ ] desktop/shell.rs — Wallpaper rendering
  [ ] desktop/shell.rs — Start menu
  [ ] desktop/cursor.rs — Hardware/software cursor

Resultado:
  Desktop gráfico completo estilo Windows 11
```

---

## Fase 6 — System Services

**Objetivo:** Filesystem, IPC, seguridad.

```text
Tareas:
  [ ] system/fs/fastfs.rs — FastFS filesystem
  [ ] system/fs/vfs.rs — Virtual filesystem layer
  [ ] system/ipc/pipe.rs — Pipe IPC
  [ ] system/ipc/shm.rs — Shared memory
  [ ] system/security/rings.rs — Ring 0/3 separation
  [ ] system/security/rings.rs — Syscall interface

Resultado:
  OS con filesystem, comunicación entre procesos, y seguridad
```

---

## Fase 7 — Apps

**Objetivo:** Aplicaciones nativas completas.

```text
Tareas:
  [ ] apps/terminal/ — Terminal emulator (gráfico)
  [ ] apps/file_manager/ — File explorer (gráfico)
  [ ] apps/settings/ — System settings (gráfico)
  [ ] apps/calculator/ — Calculator (gráfico)
  [ ] apps/editor/ — Text editor (gráfico)
  [ ] apps/sysinfo/ — System info (gráfico)

Resultado:
  Suite completa de aplicaciones nativas
```

---

## Fase 8 — UEFI Boot (Futuro)

```text
  [ ] UEFI boot application (PE format)
  [ ] GOP framebuffer setup
  [ ] UEFI memory map
  [ ] Secure Boot support
```

---

## Fase 9 — Advanced (Futuro)

```text
  [ ] AHCI/NVMe drivers
  [ ] USB stack
  [ ] Network stack (TCP/IP)
  [ ] Audio driver
  [ ] GPU acceleration
  [ ] Package manager
  [ ] Self-hosting compiler
```

---

## Timeline Estimado

| Fase | Nombre              | Duración estimada |
|------|---------------------|-------------------|
| 0    | Foundation          | ✅ Completada     |
| 1    | Boot Enhancement    | 1-2 semanas       |
| 2    | Kernel Minimal      | 2-3 semanas       |
| 3    | Drivers             | 1-2 semanas       |
| 4    | Multitasking        | 2-3 semanas       |
| 5    | Desktop Engine      | 3-4 semanas       |
| 6    | System Services     | 2-3 semanas       |
| 7    | Apps                | 2-3 semanas       |
| 8    | UEFI Boot           | 2-3 semanas       |
| 9    | Advanced            | Ongoing           |
