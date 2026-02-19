// ============================================================
// FastOS — Stage 2 Bootloader (ADead-BIB)
// ============================================================
// Loaded by stage1. Transitions CPU:
//   Real Mode (16-bit) → Protected Mode (32-bit) → Long Mode (64-bit)
//
// Steps:
//   1. Enable A20 gate
//   2. Load GDT
//   3. Enter Protected Mode (CR0.PE = 1)
//   4. Setup paging (identity map first 2MB)
//   5. Enable Long Mode (EFER.LME, CR0.PG)
//   6. Far jump to 64-bit kernel entry
//
// ADead-BIB generates ALL bytes. Zero ASM.
// ============================================================

// ---- Disable interrupts ----
cli

// ============================================================
// Step 1: Enable A20 Gate (Fast A20 via port 0x92)
// ============================================================
// in al, 0x92
raw { 0xE4, 0x92 }
// or al, 2
raw { 0x0C, 0x02 }
// and al, 0xFE (don't reset)
raw { 0x24, 0xFE }
// out 0x92, al
raw { 0xE6, 0x92 }

// ============================================================
// Step 2: Load GDT
// ============================================================
// lgdt [gdt_ptr] — pointer is at end of this binary
raw { 0x0F, 0x01, 0x16 }
// GDT pointer address (will be patched — use offset 0x80 relative)
raw { 0x80, 0x80 }

// ============================================================
// Step 3: Enter Protected Mode
// ============================================================
// mov eax, cr0
raw { 0x0F, 0x20, 0xC0 }
// or eax, 1 (PE bit)
raw { 0x66, 0x83, 0xC8, 0x01 }
// mov cr0, eax
raw { 0x0F, 0x22, 0xC0 }

// Far jump to 32-bit code (selector 0x08, offset follows)
far_jump(0x08, 0x8060)

// ============================================================
// 32-bit Protected Mode Code (at offset ~0x60)
// ============================================================
// Load data segments with selector 0x10
// mov ax, 0x10
raw { 0x66, 0xB8, 0x10, 0x00 }
// mov ds, ax
raw { 0x8E, 0xD8 }
// mov es, ax
raw { 0x8E, 0xC0 }
// mov fs, ax
raw { 0x8E, 0xE0 }
// mov gs, ax
raw { 0x8E, 0xE8 }
// mov ss, ax
raw { 0x8E, 0xD0 }

// Setup 32-bit stack
// mov esp, 0x90000
raw { 0xBC, 0x00, 0x00, 0x09, 0x00 }

// ============================================================
// Step 4: Setup Paging (Identity map first 2MB)
// ============================================================
// Write "P" to VGA to show progress
raw { 0xC6, 0x05, 0x00, 0x80, 0x0B, 0x00, 0x50 }
raw { 0xC6, 0x05, 0x01, 0x80, 0x0B, 0x00, 0x0A }

// PML4[0] → PDPT at 0x2000
// mov edi, 0x1000 (PML4 base)
raw { 0xBF, 0x00, 0x10, 0x00, 0x00 }
// xor eax, eax
raw { 0x31, 0xC0 }
// mov ecx, 4096 (clear 4 pages = 16KB)
raw { 0xB9, 0x00, 0x10, 0x00, 0x00 }
// rep stosb (clear page tables)
raw { 0xF3, 0xAA }

// PML4[0] = 0x2003 (present + writable + points to PDPT)
raw { 0xC7, 0x05, 0x00, 0x10, 0x00, 0x00, 0x03, 0x20, 0x00, 0x00 }

// PDPT[0] = 0x3003 (present + writable + points to PD)
raw { 0xC7, 0x05, 0x00, 0x20, 0x00, 0x00, 0x03, 0x30, 0x00, 0x00 }

// PD[0] = 0x83 (present + writable + 2MB page, PS bit)
raw { 0xC7, 0x05, 0x00, 0x30, 0x00, 0x00, 0x83, 0x00, 0x00, 0x00 }

// Load CR3 with PML4 base (0x1000)
// mov eax, 0x1000
raw { 0xB8, 0x00, 0x10, 0x00, 0x00 }
// mov cr3, eax
raw { 0x0F, 0x22, 0xD8 }

// ============================================================
// Step 5: Enable Long Mode
// ============================================================
// Enable PAE in CR4
// mov eax, cr4
raw { 0x0F, 0x20, 0xE0 }
// or eax, 0x20 (PAE bit)
raw { 0x83, 0xC8, 0x20 }
// mov cr4, eax
raw { 0x0F, 0x22, 0xE0 }

// Set EFER.LME (Long Mode Enable) via MSR 0xC0000080
// mov ecx, 0xC0000080
raw { 0xB9, 0x80, 0x00, 0x00, 0xC0 }
// rdmsr
raw { 0x0F, 0x32 }
// or eax, 0x100 (LME bit)
raw { 0x0D, 0x00, 0x01, 0x00, 0x00 }
// wrmsr
raw { 0x0F, 0x30 }

// Enable paging (CR0.PG)
// mov eax, cr0
raw { 0x0F, 0x20, 0xC0 }
// or eax, 0x80000000 (PG bit)
raw { 0x0D, 0x00, 0x00, 0x00, 0x80 }
// mov cr0, eax
raw { 0x0F, 0x22, 0xC0 }

// Write "L" to VGA to show Long Mode active
raw { 0xC6, 0x05, 0x02, 0x80, 0x0B, 0x00, 0x4C }
raw { 0xC6, 0x05, 0x03, 0x80, 0x0B, 0x00, 0x0A }

// ============================================================
// Step 6: Far jump to 64-bit code
// ============================================================
far_jump(0x08, 0x100000)

// ---- Halt (safety) ----
cli
hlt
raw { 0xEB, 0xFE }

// ============================================================
// GDT Table (at known offset)
// ============================================================
// Null descriptor (8 bytes)
raw { 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 }
// Kernel Code 64-bit: base=0, limit=0xFFFFF, DPL=0, L=1
raw { 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x9A, 0xAF, 0x00 }
// Kernel Data: base=0, limit=0xFFFFF, DPL=0, W=1
raw { 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x92, 0xCF, 0x00 }
// User Code 64-bit: base=0, limit=0xFFFFF, DPL=3, L=1
raw { 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFA, 0xAF, 0x00 }
// User Data: base=0, limit=0xFFFFF, DPL=3, W=1
raw { 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xF2, 0xCF, 0x00 }

// GDT Pointer (6 bytes: limit u16 + base u32)
// limit = 5*8 - 1 = 39 = 0x27
raw { 0x27, 0x00 }
// base address (will need to be calculated based on load address)
raw { 0x00, 0x80, 0x00, 0x00 }
