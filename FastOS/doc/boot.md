# FastOS — Boot System

> ADead-BIB bootloader: BIOS first, UEFI later.

---

## Boot Flow Overview

```text
Power On → BIOS POST → MBR (Stage1) → Stage2 → Kernel
```

---

## Stage1 — Boot Sector (ADead-BIB)

**File:** `boot/stage1.adB`
**Size:** Exactly 512 bytes (MBR)
**Signature:** `0x55AA` at bytes 510-511
**Load address:** `0x7C00` (BIOS standard)

### Responsibilities

```text
1. Disable interrupts (CLI)
2. Initialize segment registers (DS=ES=SS=0)
3. Set up stack (SP=0x7C00, grows down)
4. Set VGA text mode 3 (80x25, 16 colors)
5. Display splash screen (FastOS logo, version, "Booting...")
6. Load stage2 from disk sectors 2-N into 0x8000
7. Jump to stage2 at 0x8000
```

### Disk Loading

```text
INT 13h, AH=02h — Read sectors
  AL = number of sectors (currently 4, max 16)
  CH = cylinder 0
  CL = sector 2 (stage2 starts at sector 2)
  DH = head 0
  DL = drive number (preserved from BIOS)
  ES:BX = 0x0000:0x8000 (load address)
```

### Current Implementation

```text
- Auto-boot (no Y/N prompt)
- Dark blue splash with FastOS branding
- Loads 4 sectors (2048 bytes) for stage2
- Far jump: EA 00 80 00 00 → 0x0000:0x8000
```

---

## Stage2 — Loader (ADead-BIB)

**File:** `boot/stage2.adB`
**Load address:** `0x8000`
**Max size:** ~8KB (16 sectors)

### Current Implementation (VGA Text Mode)

```text
1. Set VGA mode 3 (80x25)
2. Hide cursor
3. Draw login screen (Win11 style)
4. Wait for keypress → draw desktop
5. Desktop with 6 icons, taskbar, system tray
6. Q/q → shutdown screen → HLT
```

### Future Implementation (Mode Switch)

Stage2 will be enhanced to perform the full mode switch sequence:

```text
Phase A — A20 Line
  1. Fast A20 via port 0x92
  2. Fallback: keyboard controller method
  3. Verify A20 is enabled

Phase B — Protected Mode
  1. Load temporary GDT (null, code32, data32, code64, data64)
  2. Set CR0.PE = 1
  3. Far jump to 32-bit code segment

Phase C — Paging Setup
  1. Create PML4 table (identity map first 4GB)
  2. Create PDPT entries
  3. Create PD entries (2MB pages)
  4. Set CR3 = PML4 address

Phase D — Long Mode
  1. Enable PAE (CR4.PAE = 1)
  2. Enable Long Mode (MSR 0xC0000080, bit 8)
  3. Enable Paging (CR0.PG = 1)
  4. Far jump to 64-bit code segment

Phase E — Video Mode
  1. Call INT 10h, AX=4F02h (VBE set mode)
  2. Mode 0x118 = 1024x768x32bpp
  3. Save framebuffer address from VBE info block
  4. Pass framebuffer info to kernel

Phase F — Load Kernel
  1. Read kernel from disk (starting sector 20+)
  2. Load to physical address 0x100000 (1MB)
  3. Jump to kernel_main
```

---

## GDT Layout

```text
Entry 0: Null descriptor
Entry 1: Code32 — base=0, limit=4GB, 32-bit, execute/read
Entry 2: Data32 — base=0, limit=4GB, 32-bit, read/write
Entry 3: Code64 — base=0, limit=4GB, 64-bit, execute/read
Entry 4: Data64 — base=0, limit=4GB, 64-bit, read/write
```

---

## Page Table Layout (Identity Map)

```text
PML4[0] → PDPT
  PDPT[0] → PD
    PD[0]   → 0x00000000 (2MB page)
    PD[1]   → 0x00200000 (2MB page)
    PD[2]   → 0x00400000 (2MB page)
    ...
    PD[511] → 0x3FE00000 (2MB page)
  PDPT[1] → PD
    ... (maps 1GB-2GB)
  PDPT[2] → PD
    ... (maps 2GB-3GB)
  PDPT[3] → PD
    ... (maps 3GB-4GB)
```

---

## VBE Mode Info

```text
INT 10h, AX=4F01h — Get VBE mode info
  CX = mode number (0x118 = 1024x768x32)
  ES:DI = buffer for mode info block

Mode Info Block (relevant fields):
  Offset 0x28: PhysBasePtr (framebuffer physical address)
  Offset 0x12: XResolution
  Offset 0x14: YResolution
  Offset 0x19: BitsPerPixel
  Offset 0x10: BytesPerScanLine
```

---

## Boot Info Structure

Passed from stage2 to kernel via a known memory address:

```text
Address: 0x9000 (boot info block)

struct BootInfo {
    magic: u32,              // 0x464F5321 ("FOS!")
    framebuffer_addr: u64,   // Physical address of framebuffer
    framebuffer_width: u32,  // Pixels
    framebuffer_height: u32, // Pixels
    framebuffer_bpp: u32,    // Bits per pixel (32)
    framebuffer_pitch: u32,  // Bytes per scanline
    memory_map_addr: u64,    // E820 memory map address
    memory_map_count: u32,   // Number of E820 entries
}
```

---

## UEFI Boot (Future)

```text
UEFI boot will use:
  - PE format boot application
  - GOP (Graphics Output Protocol) for framebuffer
  - UEFI memory map (replaces E820)
  - Secure Boot chain
  - No real mode, no VBE, no INT 13h

File: boot/uefi_boot.adB (or .efi)
```

---

## Memory Map (E820)

```text
INT 15h, AX=E820h — Get memory map

Typical entries:
  0x00000000 - 0x0009FFFF  → Usable (640KB conventional)
  0x000A0000 - 0x000FFFFF  → Reserved (VGA + ROM)
  0x00100000 - 0x????????  → Usable (extended memory)
  0xFEC00000 - 0xFECFFFFF  → Reserved (APIC)
  0xFEE00000 - 0xFEEFFFFF  → Reserved (Local APIC)
```
