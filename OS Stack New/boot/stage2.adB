// ============================================================
// ADead-OS — Stage 2: Mode Transition (ADead-BIB)
// ============================================================
// Handles the full CPU mode transition chain:
//   Real Mode (16-bit) → Protected Mode (32-bit) → Long Mode (64-bit)
//
// NO ASM. ADead-BIB generates every byte.
//
// This stage:
//   1. Enables A20 gate
//   2. Loads GDT
//   3. Switches to Protected Mode
//   4. Sets up paging (identity map first 16MB)
//   5. Enables Long Mode via EFER MSR
//   6. Jumps to kernel_main (Rust)
//
// Compile: adB flat stage2.adB -o stage2.bin
// ============================================================

org 0x8000

// ============================================================
// PART 1: Still in Real Mode (16-bit)
// ============================================================

cli

// ---- Enable A20 gate (fast method via port 0x92) ----
// in al, 0x92
raw { 0xE4, 0x92 }
// or al, 2
raw { 0x0C, 0x02 }
// and al, 0xFE (don't reset)
raw { 0x24, 0xFE }
// out 0x92, al
raw { 0xE6, 0x92 }

// ---- Load GDT ----
// lgdt [gdt_pointer] — address will be patched
// For now, use raw bytes for the LGDT instruction
// 0F 01 16 [addr16] = lgdt [addr16]
// GDT pointer is at end of this file (patched by build)
raw { 0x0F, 0x01, 0x16 }
// Address of GDT pointer (relative to segment, will be at known offset)
raw { 0x80, 0x80 }

// ---- Switch to Protected Mode ----
// mov eax, cr0
raw { 0x0F, 0x20, 0xC0 }
// or eax, 1 (set PE bit) — with 0x66 prefix for 32-bit operand in real mode
raw { 0x66, 0x83, 0xC8, 0x01 }
// mov cr0, eax
raw { 0x0F, 0x22, 0xC0 }

// ---- Far jump to 32-bit code (flush pipeline + load CS) ----
// jmp 0x08:protected_mode_entry
// 0x66 0xEA [offset32] [selector16]
far_jump(0x08, 0x8060)

// ============================================================
// PART 2: Protected Mode (32-bit) — offset ~0x60 from start
// ============================================================

// Reload data segment registers with GDT data selector (0x10)
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

// ---- Write 'P' to VGA (Protected Mode indicator) ----
// mov byte [0xB8000], 'P'
raw { 0xC6, 0x05, 0x00, 0x80, 0x0B, 0x00, 0x50 }
// mov byte [0xB8001], 0x0A (green on black)
raw { 0xC6, 0x05, 0x01, 0x80, 0x0B, 0x00, 0x0A }

// ---- Setup paging for Long Mode ----
// Clear page tables area (0x1000 - 0x4000)
// We'll use identity mapping with 2MB pages

// Point PML4[0] → PDPT at 0x2000
// mov eax, 0x2003 (present + writable + address)
raw { 0xB8, 0x03, 0x20, 0x00, 0x00 }
// mov [0x1000], eax
raw { 0xA3, 0x00, 0x10, 0x00, 0x00 }

// Point PDPT[0] → PD at 0x3000
// mov eax, 0x3003
raw { 0xB8, 0x03, 0x30, 0x00, 0x00 }
// mov [0x2000], eax
raw { 0xA3, 0x00, 0x20, 0x00, 0x00 }

// PD[0] = 0x000000 | PRESENT | WRITABLE | HUGE (2MB page)
// mov eax, 0x83
raw { 0xB8, 0x83, 0x00, 0x00, 0x00 }
// mov [0x3000], eax
raw { 0xA3, 0x00, 0x30, 0x00, 0x00 }

// PD[1] = 0x200000 | PRESENT | WRITABLE | HUGE (2MB page)
// mov eax, 0x200083
raw { 0xB8, 0x83, 0x00, 0x20, 0x00 }
// mov [0x3008], eax
raw { 0xA3, 0x08, 0x30, 0x00, 0x00 }

// PD[2] = 0x400000 | flags (4-6MB)
raw { 0xB8, 0x83, 0x00, 0x40, 0x00 }
raw { 0xA3, 0x10, 0x30, 0x00, 0x00 }

// PD[3] = 0x600000 | flags (6-8MB)
raw { 0xB8, 0x83, 0x00, 0x60, 0x00 }
raw { 0xA3, 0x18, 0x30, 0x00, 0x00 }

// ---- Load PML4 into CR3 ----
// mov eax, 0x1000
raw { 0xB8, 0x00, 0x10, 0x00, 0x00 }
// mov cr3, eax
raw { 0x0F, 0x22, 0xD8 }

// ---- Enable PAE in CR4 ----
// mov eax, cr4
raw { 0x0F, 0x20, 0xE0 }
// or eax, (1 << 5) = 0x20 (PAE bit)
raw { 0x83, 0xC8, 0x20 }
// mov cr4, eax
raw { 0x0F, 0x22, 0xE0 }

// ---- Enable Long Mode in EFER MSR ----
// mov ecx, 0xC0000080 (EFER MSR address)
raw { 0xB9, 0x80, 0x00, 0x00, 0xC0 }
// rdmsr
raw { 0x0F, 0x32 }
// or eax, (1 << 8) = 0x100 (LME bit)
raw { 0x0D, 0x00, 0x01, 0x00, 0x00 }
// wrmsr
raw { 0x0F, 0x30 }

// ---- Enable paging (PG bit in CR0) ----
// mov eax, cr0
raw { 0x0F, 0x20, 0xC0 }
// or eax, (1 << 31) = 0x80000000
raw { 0x0D, 0x00, 0x00, 0x00, 0x80 }
// mov cr0, eax
raw { 0x0F, 0x22, 0xC0 }

// ---- Write 'L' to VGA (Long Mode indicator) ----
// mov byte [0xB8002], 'L'
raw { 0xC6, 0x05, 0x02, 0x80, 0x0B, 0x00, 0x4C }
// mov byte [0xB8003], 0x0E (yellow on black)
raw { 0xC6, 0x05, 0x03, 0x80, 0x0B, 0x00, 0x0E }

// ---- Far jump to 64-bit code ----
// This would jump to kernel_main
// For standalone test: halt here
cli
hlt
raw { 0xEB, 0xFE }

// ============================================================
// DATA: GDT (Global Descriptor Table)
// ============================================================
// Offset ~0x80 from start (0x8080 absolute)

// GDT entries (each 8 bytes):
// Entry 0: Null descriptor
raw { 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 }

// Entry 1 (0x08): Code segment — 64-bit, Ring 0
// limit=0xFFFF, base=0, access=0x9A, flags=0xAF (Long mode)
raw { 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x9A, 0xAF, 0x00 }

// Entry 2 (0x10): Data segment — Ring 0
// limit=0xFFFF, base=0, access=0x92, flags=0xCF
raw { 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x92, 0xCF, 0x00 }

// Entry 3 (0x18): Code segment — 32-bit, Ring 0 (for protected mode)
raw { 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x9A, 0xCF, 0x00 }

// Entry 4 (0x20): User Code — 64-bit, Ring 3
raw { 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xFA, 0xAF, 0x00 }

// Entry 5 (0x28): User Data — Ring 3
raw { 0xFF, 0xFF, 0x00, 0x00, 0x00, 0xF2, 0xCF, 0x00 }

// GDT Pointer (6 bytes: 2 limit + 4 base)
// limit = 6 entries * 8 - 1 = 47 = 0x002F
// base = 0x8080 (where GDT starts in memory)
raw { 0x2F, 0x00, 0x80, 0x80, 0x00, 0x00 }
