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
  ✅ kernel/linker.ld — Kernel binary layout at 0x100000
  ✅ kernel/rust-toolchain.toml — Nightly + rust-src + llvm-tools
  ✅ kernel/x86_64-fastos.json — Updated with linker script reference
  ✅ build.ps1 — Updated with Rust kernel compilation step (5 steps)
  ✅ kernel/src/main.rs — Wired all new modules (arch, boot, kernel_core, drivers)

Archivos:
  boot/stage2.adB — Rewrite completo para mode switch (PENDIENTE)
  
Resultado:
  BIOS → Stage1 → Stage2 → 64-bit kernel con framebuffer
```

---

## Fase 2 — Kernel Minimal (COMPLETADA)

**Objetivo:** Kernel funcional con memoria, interrupciones y framebuffer.

```text
Tareas:
  ✅ kernel/src/kernel_core/memory.rs — Physical frame allocator (bitmap, 4GB max)
  ✅ kernel/src/kernel_core/memory.rs — Kernel heap allocator (bump, 1MB)
  [ ] kernel/src/kernel_core/memory.rs — Virtual memory manager (4-level paging)
  ✅ kernel/src/kernel_core/interrupts.rs — High-level interrupt management
  ✅ kernel/src/arch/x86_64/gdt.rs — GDT with TSS (5 segments)
  ✅ kernel/src/arch/x86_64/idt.rs — IDT (256 entries) + PIC remap (32-47)
  ✅ kernel/src/arch/x86_64/paging.rs — 4-level page table structures
  ✅ kernel/src/arch/x86_64/port.rs — I/O port wrappers (inb/outb/inw/outw/inl/outl)
  ✅ kernel/src/arch/x86_64/cpu.rs — CPU control (CR0-4, MSR, CPUID, TLB)
  ✅ kernel/src/drivers/framebuffer.rs — Linear framebuffer (VBE 1024x768x32)
  ✅ kernel/src/drivers/framebuffer.rs — Pixel, rect, line, circle, rounded rect
  ✅ kernel/src/drivers/framebuffer.rs — Alpha blending, bitmap font (8x16, A-Z/a-z/0-9)
  ✅ kernel/src/boot.rs — BootInfo struct at 0x9000 + E820 memory map

Archivos creados:
  kernel/src/kernel_core/mod.rs
  kernel/src/kernel_core/memory.rs
  kernel/src/kernel_core/interrupts.rs
  kernel/src/arch/mod.rs
  kernel/src/arch/x86_64/mod.rs
  kernel/src/arch/x86_64/gdt.rs
  kernel/src/arch/x86_64/idt.rs
  kernel/src/arch/x86_64/paging.rs
  kernel/src/arch/x86_64/port.rs
  kernel/src/arch/x86_64/cpu.rs
  kernel/src/drivers/mod.rs
  kernel/src/drivers/framebuffer.rs
  kernel/src/drivers/timer.rs
  kernel/src/boot.rs

Resultado:
  Kernel arranca en 64-bit, maneja memoria, muestra gráficos en framebuffer
```

---

## Fase 3 — Drivers (COMPLETADA)

**Objetivo:** Input completo + timer para multitasking.

```text
Tareas:
  ✅ kernel/src/drivers/keyboard.rs — PS/2 keyboard (IRQ + polling, scancode set 1)
  ✅ kernel/src/drivers/keyboard.rs — Modifiers (Shift, Ctrl, Alt, CapsLock)
  ✅ kernel/src/drivers/keyboard.rs — Circular key buffer (64 keys)
  ✅ kernel/src/drivers/mouse.rs — PS/2 mouse driver (3-byte protocol)
  ✅ kernel/src/drivers/mouse.rs — Cursor position + 3 buttons + screen clamping
  ✅ kernel/src/drivers/timer.rs — PIT timer (channel 0, 1000 Hz)
  ✅ kernel/src/drivers/timer.rs — System tick counter + uptime + sleep
  ✅ kernel/src/drivers/disk.rs — ATA PIO disk driver (LBA28 read/write, IDENTIFY)

Resultado:
  Keyboard + mouse + timer + disk funcionales.
```

---

## Fase 4 — Multitasking (COMPLETADA)

**Objetivo:** Procesos y scheduling real.

```text
Tareas:
  ✅ kernel/src/kernel_core/process.rs — Process struct (PID, state, priority, exit_code)
  ✅ kernel/src/kernel_core/process.rs — Thread struct (stack, CpuContext with all regs)
  ✅ kernel/src/kernel_core/process.rs — Process table (64 slots), create/kill/reap
  ✅ kernel/src/kernel_core/process.rs — Kernel + user thread contexts (Ring 0/3)
  ✅ kernel/src/kernel_core/scheduler.rs — Round-robin scheduler (10ms time slice)
  ✅ kernel/src/kernel_core/scheduler.rs — Timer-driven preemptive switching
  ✅ kernel/src/kernel_core/scheduler.rs — yield, block, unblock, sleep
  ✅ kernel/src/kernel_core/memory.rs — Virtual memory (map/unmap/virt_to_phys)
  ✅ kernel/src/kernel_core/memory.rs — User page table creation (kernel half shared)
  [ ] ADead-BIB context switch stub (actual register save/restore in asm)

Resultado:
  Process table, scheduler, y virtual memory funcionales.
  Context switch real pendiente de stub ASM.
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

## Fase 6 — System Services (COMPLETADA)

**Objetivo:** Filesystem, IPC, seguridad.

```text
Tareas:
  ✅ system/fs/vfs.rs — VFS trait + FileType + DirEntry + FsError
  ✅ system/fs/fastfs.rs — FastFS con Filesystem trait (RAM-backed, 256 inodes)
  ✅ system/fs/fastfs.rs — read/write/lookup/create/delete/stat implementados
  ✅ system/ipc/pipe.rs — Pipe IPC (ring buffer 4KB, read/write)
  ✅ system/ipc/shm.rs — Shared memory (16 regions, open/close/get, ref counting)
  ✅ system/security/rings.rs — Ring 0/3 + is_kernel_mode()
  ✅ system/security/rings.rs — SYSCALL/SYSRET MSR setup (STAR, LSTAR, FMASK)
  ✅ system/security/rings.rs — Naked syscall_entry + dispatcher (13 syscalls)
  ✅ system/security/rings.rs — sys_exit, sys_write, sys_read, sys_getpid, sys_yield, sys_sleep

Resultado:
  Filesystem, IPC, y syscall interface funcionales.
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
| 1    | Boot Enhancement    | 🔧 En progreso    |
| 2    | Kernel Minimal      | ✅ Completada     |
| 3    | Drivers             | ✅ Completada     |
| 4    | Multitasking        | ✅ Completada     |
| 5    | Desktop Engine      | 3-4 semanas       |
| 6    | System Services     | ✅ Completada     |
| 7    | Apps                | 2-3 semanas       |
| 8    | UEFI Boot           | 2-3 semanas       |
| 9    | Advanced            | Ongoing           |
