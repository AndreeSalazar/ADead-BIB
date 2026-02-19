# FastOS — Security Architecture

> Seguridad por diseño: Rust en kernel + ring separation + memory isolation.

---

## Security Layers

```text
┌─────────────────────────────────────┐
│  Ring 3 — User Mode                │
│  Applications, user processes       │
│  Cannot access hardware directly    │
│  Cannot access kernel memory        │
├─────────────────────────────────────┤
│  Syscall Interface                  │
│  SYSCALL/SYSRET instructions        │
│  Validated parameters               │
├─────────────────────────────────────┤
│  Ring 0 — Kernel Mode              │
│  Kernel, drivers, core services     │
│  Full hardware access               │
│  Memory management                  │
└─────────────────────────────────────┘
```

---

## Memory Safety (Rust Advantage)

FastOS kernel is written in Rust, which prevents at compile time:

```text
✅ No buffer overflows
✅ No use-after-free
✅ No double-free
✅ No null pointer dereference (Option<T>)
✅ No data races (ownership system)
✅ No dangling pointers
✅ No uninitialized memory access
```

Comparison with traditional OS kernels:

| Vulnerability          | C/C++ Kernel | Rust Kernel |
|------------------------|--------------|-------------|
| Buffer overflow        | Common       | Impossible  |
| Use-after-free         | Common       | Impossible  |
| Null dereference       | Common       | Compile error |
| Data race              | Possible     | Compile error |
| Memory leak            | Possible     | Unlikely    |
| Integer overflow       | Silent       | Panic (debug) |

---

## Ring Separation

### Ring 0 — Kernel Mode

```text
Privileges:
  - Execute privileged instructions (CLI, STI, HLT, IN, OUT)
  - Access all memory (physical + virtual)
  - Modify page tables
  - Handle interrupts
  - Access I/O ports

Components running in Ring 0:
  - Kernel core
  - All drivers
  - Interrupt handlers
  - Memory manager
  - Scheduler
```

### Ring 3 — User Mode

```text
Restrictions:
  - Cannot execute privileged instructions
  - Cannot access kernel memory (page fault)
  - Cannot access I/O ports directly
  - Must use syscalls for OS services

Components running in Ring 3:
  - All applications
  - User processes
  - Desktop shell (future)
```

### Transition Mechanism

```text
User → Kernel:
  SYSCALL instruction → MSR-based entry point
  Saves RIP, RFLAGS to RCX, R11
  Switches to kernel stack (from TSS)

Kernel → User:
  SYSRET instruction
  Restores RIP, RFLAGS from RCX, R11
  Switches back to user stack
```

---

## Memory Isolation

### Per-Process Address Space

```text
Each process has its own page table (CR3):
  - Cannot see other processes' memory
  - Cannot see kernel memory (supervisor bit)
  - Shared libraries mapped read-only

Kernel pages:
  - Present in all address spaces (higher half)
  - Supervisor bit set (Ring 3 cannot access)
  - NX bit on data pages (no execute)
```

### Stack Protection

```text
- Guard pages between stacks (unmapped page → page fault on overflow)
- Separate kernel stack per process (in TSS)
- Stack canaries (future)
```

---

## Boot Security

### BIOS Boot (Current)

```text
- No secure boot chain
- Stage1 loaded by BIOS from MBR
- Stage2 loaded by stage1
- Kernel loaded by stage2
- Trust: implicit (physical access = full access)
```

### UEFI Secure Boot (Future)

```text
- Signed boot application
- Certificate chain verification
- Measured boot (TPM)
- Secure Boot database (db/dbx)
```

---

## Future Security Features

```text
Phase 1:
  [ ] Ring 0/3 separation
  [ ] Per-process page tables
  [ ] Syscall interface
  [ ] Guard pages

Phase 2:
  [ ] ASLR (Address Space Layout Randomization)
  [ ] DEP/NX (Data Execution Prevention)
  [ ] Stack canaries
  [ ] Heap hardening

Phase 3:
  [ ] Capabilities-based security
  [ ] Mandatory access control
  [ ] Sandboxing
  [ ] Secure IPC

Phase 4:
  [ ] UEFI Secure Boot
  [ ] TPM integration
  [ ] Encrypted filesystem
  [ ] User authentication (password hash)
```

---

## Threat Model

```text
Threats mitigated by Rust:
  - Kernel exploits via memory corruption → eliminated
  - Privilege escalation via buffer overflow → eliminated
  - RCE via use-after-free → eliminated

Threats mitigated by ring separation:
  - App accessing kernel memory → page fault
  - App executing privileged instructions → #GP fault
  - App accessing other app memory → page fault

Remaining threats (future work):
  - Physical access attacks
  - Side-channel attacks (Spectre/Meltdown)
  - Supply chain attacks
  - Social engineering
```
