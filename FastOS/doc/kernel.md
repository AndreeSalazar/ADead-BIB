# FastOS — Kernel Architecture

> Hybrid kernel escrito en Rust. Seguridad de memoria garantizada en compile-time.

---

## Kernel Type

**Hybrid Kernel** — Comparable a Windows NT y macOS XNU.

- Drivers ejecutan en kernel space (rendimiento)
- Módulos aislados (seguridad)
- Path futuro a microkernel (drivers en userspace)

---

## Kernel Modules

```text
kernel/src/
├── main.rs              # Entry point (kernel_main)
├── panic.rs             # Panic handler
│
├── core/                # Core subsystems
│   ├── mod.rs
│   ├── memory.rs        # Memory manager
│   ├── scheduler.rs     # Process scheduler
│   ├── process.rs       # Process management
│   └── interrupts.rs    # Interrupt handling
│
├── arch/x86_64/         # Architecture-specific
│   ├── mod.rs
│   ├── gdt.rs           # Global Descriptor Table
│   ├── idt.rs           # Interrupt Descriptor Table
│   ├── paging.rs        # Page table management
│   ├── port.rs          # I/O port access
│   └── cpu.rs           # CPU features
│
└── drivers/             # Hardware drivers
    ├── mod.rs
    ├── framebuffer.rs   # Framebuffer (VBE/GOP)
    ├── vga.rs           # VGA text mode (legacy)
    ├── keyboard.rs      # PS/2 keyboard
    ├── mouse.rs         # PS/2 mouse
    ├── disk.rs          # ATA/AHCI
    └── timer.rs         # PIT/APIC timer
```

---

## Memory Manager

### Physical Memory

```text
Frame Allocator (bitmap-based):
  - Each bit represents one 4KB frame
  - Bitmap stored after kernel in memory
  - Initialized from E820 memory map
  - alloc_frame() → PhysAddr
  - free_frame(PhysAddr)
```

### Virtual Memory

```text
4-Level Paging (x86-64):
  PML4 → PDPT → PD → PT → Physical Frame

  Page sizes:
    4KB  (PT entry)     — default
    2MB  (PD entry)     — large pages
    1GB  (PDPT entry)   — huge pages

  Recursive mapping at PML4[511] for self-referencing
```

### Kernel Heap

```text
Allocator progression:
  1. Bump allocator (initial, simple)
  2. Linked-list allocator (general purpose)
  3. Slab allocator (future, for fixed-size objects)

Interface:
  alloc(size, align) → *mut u8
  dealloc(ptr, size, align)
```

---

## Interrupt System

### IDT (Interrupt Descriptor Table)

```text
256 entries:
  0-31:   CPU exceptions (divide error, page fault, etc.)
  32-47:  Hardware IRQs (remapped PIC)
  48-255: Software interrupts / syscalls

Key exceptions:
  #0  — Divide Error
  #6  — Invalid Opcode
  #8  — Double Fault (uses IST)
  #13 — General Protection Fault
  #14 — Page Fault
```

### PIC (Programmable Interrupt Controller)

```text
Remapping:
  Master PIC: IRQ 0-7  → INT 32-39
  Slave PIC:  IRQ 8-15 → INT 40-47

Key IRQs:
  IRQ 0  (INT 32) — Timer (PIT)
  IRQ 1  (INT 33) — Keyboard
  IRQ 12 (INT 44) — Mouse
  IRQ 14 (INT 46) — Primary ATA
```

---

## Scheduler

### Round-Robin (Initial)

```text
- Fixed time quantum (10ms)
- Timer IRQ triggers context switch
- Process states: Ready, Running, Blocked, Terminated
- Run queue: circular linked list
```

### CFS-like (Future)

```text
- Virtual runtime tracking
- Red-black tree for process ordering
- Fair CPU distribution
- Priority support (nice values)
```

### Context Switch

```text
Save current process:
  - Push all general registers (RAX-R15)
  - Save RSP, RIP, RFLAGS
  - Save CR3 (page table)

Restore next process:
  - Load CR3 (switch address space)
  - Restore RSP, RIP, RFLAGS
  - Pop all general registers
  - IRETQ
```

---

## Process Management

### Process Structure

```rust
struct Process {
    pid: u64,
    name: [u8; 64],
    state: ProcessState,
    page_table: PhysAddr,      // CR3
    kernel_stack: VirtAddr,
    user_stack: VirtAddr,
    context: CpuContext,
    priority: i8,
    parent_pid: u64,
}

enum ProcessState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

struct CpuContext {
    rax: u64, rbx: u64, rcx: u64, rdx: u64,
    rsi: u64, rdi: u64, rbp: u64, rsp: u64,
    r8: u64,  r9: u64,  r10: u64, r11: u64,
    r12: u64, r13: u64, r14: u64, r15: u64,
    rip: u64, rflags: u64, cr3: u64,
}
```

---

## Syscall Interface (Future)

```text
Syscall convention (SYSCALL/SYSRET):
  RAX = syscall number
  RDI = arg1
  RSI = arg2
  RDX = arg3
  R10 = arg4
  R8  = arg5
  R9  = arg6
  RAX = return value

Syscalls:
  0  — sys_exit(code)
  1  — sys_write(fd, buf, len)
  2  — sys_read(fd, buf, len)
  3  — sys_open(path, flags)
  4  — sys_close(fd)
  5  — sys_fork()
  6  — sys_exec(path, argv)
  7  — sys_wait(pid)
  8  — sys_mmap(addr, len, prot)
  9  — sys_munmap(addr, len)
  10 — sys_getpid()
```

---

## Kernel Boot Sequence

```text
kernel_main():
  1. Parse boot info (framebuffer, memory map)
  2. Initialize GDT + TSS
  3. Initialize IDT
  4. Remap PIC
  5. Initialize physical frame allocator
  6. Initialize page tables
  7. Initialize kernel heap
  8. Initialize framebuffer driver
  9. Initialize keyboard driver
  10. Initialize mouse driver
  11. Initialize timer (PIT at 1000 Hz)
  12. Enable interrupts (STI)
  13. Initialize scheduler
  14. Launch desktop process
  15. Enter idle loop
```

---

## extern "C" Interface (ADead-BIB)

Functions provided by ADead-BIB for hardware access:

```rust
extern "C" {
    fn fastos_cli();              // Disable interrupts
    fn fastos_sti();              // Enable interrupts
    fn fastos_hlt();              // Halt CPU
    fn fastos_outb(port: u16, value: u8);  // Write byte to I/O port
    fn fastos_inb(port: u16) -> u8;        // Read byte from I/O port
    fn fastos_lgdt(gdtr: *const u8);       // Load GDT
    fn fastos_lidt(idtr: *const u8);       // Load IDT
    fn fastos_ltr(selector: u16);          // Load Task Register
    fn fastos_read_cr3() -> u64;           // Read CR3
    fn fastos_write_cr3(val: u64);         // Write CR3
    fn fastos_read_msr(msr: u32) -> u64;   // Read MSR
    fn fastos_write_msr(msr: u32, val: u64); // Write MSR
    fn fastos_context_switch(old: *mut CpuContext, new: *const CpuContext);
}
```
