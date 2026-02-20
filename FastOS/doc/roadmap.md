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

## Fase 1 — Boot System Enhancement (COMPLETADA)

**Objetivo:** Stage2 realiza mode switch completo y carga kernel real.

```text
Tareas:
  ✅ Enable A20 line (port 0x92 fast method + keyboard controller fallback)
  ✅ Set up temporary GDT (code32, data32, code64, data64)
  ✅ Switch from Real Mode (16-bit) to Protected Mode (32-bit)
  ✅ Set up identity-mapped page tables (PML4 → PDPT → PD)
  ✅ Enable PAE + Long Mode via MSR
  ✅ Switch to Long Mode (64-bit)
  ✅ Set VBE video mode (1024x768x32 via INT 10h AX=4F02h)
  ✅ Load kernel binary from disk sectors
  ✅ Jump to kernel_main in 64-bit mode
  ✅ kernel/linker.ld — Kernel binary layout at 0x100000
  ✅ kernel/rust-toolchain.toml — Nightly + rust-src + llvm-tools
  ✅ kernel/x86_64-fastos.json — Updated with linker script reference
  ✅ build.ps1 — Updated with Rust kernel compilation step (5 steps)
  ✅ kernel/src/main.rs — Wired all new modules (arch, boot, kernel_core, drivers)
  ✅ boot/stage2.adB — Full rewrite: A20→GDT→PM→Paging→LM→VBE→kernel→jump
  ✅ kernel/src/main.rs — Replaced extern C with inline asm (hlt, inb, outb, cli, sti)
  ✅ kernel/x86_64-fastos.json — Fixed target-pointer-width, SSE2 ABI, features

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
  ✅ kernel/src/kernel_core/memory.rs — Virtual memory manager (4-level paging)
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
  ✅ Context switch stub — inline asm register save/restore in scheduler.rs

Resultado:
  Process table, scheduler, virtual memory, y context switch funcionales.
```

---

## Fase 5 — Desktop Engine (COMPLETADA)

**Objetivo:** Desktop gráfico real con compositor.

```text
Tareas:
  ✅ desktop_engine/compositor.rs — Window compositing engine
  ✅ desktop_engine/compositor.rs — Alpha blending
  ✅ desktop_engine/compositor.rs — Damage tracking (dirty rects)
  ✅ desktop_engine/compositor.rs — Z-order management (layer sort)
  ✅ desktop_engine/wm.rs — Window create/destroy
  ✅ desktop_engine/wm.rs — Window move/resize
  ✅ desktop_engine/wm.rs — Window focus + title bar + decorations
  ✅ desktop_engine/wm.rs — Close button hit detection
  ✅ desktop_engine/shell.rs — Taskbar rendering (Win11 style)
  ✅ desktop_engine/shell.rs — Desktop icons (6 default apps)
  ✅ desktop_engine/shell.rs — Wallpaper (vertical gradient)
  ✅ desktop_engine/shell.rs — Start menu (pinned apps grid)
  ✅ desktop_engine/cursor.rs — Software cursor (sprite + bg save/restore)

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

## Fase 7 — Apps (COMPLETADA)

**Objetivo:** Aplicaciones nativas completas.

```text
Tareas:
  ✅ gfx_apps/terminal.rs — Terminal emulator (framebuffer, command execution)
  ✅ gfx_apps/file_manager.rs — File explorer (toolbar, path bar, file list)
  ✅ gfx_apps/settings.rs — System settings (display, theme, brightness)
  ✅ gfx_apps/calculator.rs — Calculator (4-function, button grid)
  ✅ gfx_apps/editor.rs — Text editor (line numbers, cursor, scrolling)
  ✅ gfx_apps/sysinfo.rs — System info (CPU, memory, heap, progress bar)

Resultado:
  Suite completa de 6 aplicaciones gráficas nativas
```

---

## Fase 8 — UEFI Boot (COMPLETADA)

```text
  ✅ boot/uefi/types.rs — UEFI type definitions (SystemTable, BootServices, GUIDs)
  ✅ boot/uefi/gop.rs — GOP framebuffer (locate protocol, mode selection, 1024x768)
  ✅ boot/uefi/memory_map.rs — UEFI memory map retrieval + E820 conversion
  ✅ boot/uefi/boot.rs — efi_main entry point (banner, GOP, mmap, BootInfo, ExitBootServices, jump)
  ✅ Secure Boot compatible (PE32+ DLL subsystem, standard UEFI calling convention)
```

---

## Fase 9 — Advanced (COMPLETADA)

```text
  ✅ drivers/ahci.rs — AHCI SATA driver (PCI scan, port probe, device detection)
  ✅ drivers/usb.rs — xHCI USB 3.x driver (PCI scan, controller init, port enumeration)
  ✅ drivers/network.rs — Intel E1000 Ethernet driver (PCI scan, MMIO init, MAC, link status)
  ✅ drivers/audio.rs — Intel HD Audio driver (PCI scan, codec discovery, HDA init)
  [ ] GPU acceleration (future)
  [ ] Package manager (future)
  [ ] Self-hosting compiler (future)
```

---

## Timeline Estimado

| Fase | Nombre              | Duración estimada |
|------|---------------------|-------------------|
| 0    | Foundation          | ✅ Completada     |
| 1    | Boot Enhancement    | ✅ Completada     |
| 2    | Kernel Minimal      | ✅ Completada     |
| 3    | Drivers             | ✅ Completada     |
| 4    | Multitasking        | ✅ Completada     |
| 5    | Desktop Engine      | ✅ Completada     |
| 6    | System Services     | ✅ Completada     |
| 7    | Apps                | ✅ Completada     |
| 8    | UEFI Boot           | ✅ Completada     |
| 9    | Advanced            | ✅ Completada     |
