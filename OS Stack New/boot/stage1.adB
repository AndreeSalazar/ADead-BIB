// ============================================================
// ADead-OS — Stage 1 Boot Sector (ADead-BIB)
// ============================================================
// 512-byte boot sector. NO ASM. ADead-BIB generates all bytes.
//
// Flow:
//   1. BIOS loads this at 0x7C00
//   2. Initialize segments and stack
//   3. Print "ADead-OS" via BIOS INT 10h
//   4. Load stage2 from disk (sector 2+)
//   5. Jump to stage2
//
// Compile: adB boot stage1.adB -o stage1.bin
// Test:    qemu-system-x86_64 -drive format=raw,file=os.bin
// ============================================================

org 0x7C00

// ---- Step 1: Disable interrupts during init ----
cli

// ---- Step 2: Zero segment registers ----
// xor ax, ax
raw { 0x31, 0xC0 }
// mov ds, ax
raw { 0x8E, 0xD8 }
// mov es, ax
raw { 0x8E, 0xC0 }
// mov ss, ax
raw { 0x8E, 0xD0 }

// ---- Step 3: Setup stack at 0x7C00 (grows down) ----
// mov sp, 0x7C00
raw { 0xBC, 0x00, 0x7C }

// ---- Step 4: Enable interrupts ----
sti

// ---- Step 5: Print "ADead-OS" using BIOS teletype ----
// AH = 0x0E (teletype output)
raw { 0xB4, 0x0E }

// 'A'
raw { 0xB0, 0x41 }
int_call(0x10)
// 'D'
raw { 0xB0, 0x44 }
int_call(0x10)
// 'e'
raw { 0xB0, 0x65 }
int_call(0x10)
// 'a'
raw { 0xB0, 0x61 }
int_call(0x10)
// 'd'
raw { 0xB0, 0x64 }
int_call(0x10)
// '-'
raw { 0xB0, 0x2D }
int_call(0x10)
// 'O'
raw { 0xB0, 0x4F }
int_call(0x10)
// 'S'
raw { 0xB0, 0x53 }
int_call(0x10)

// Print newline (CR + LF)
raw { 0xB0, 0x0D }
int_call(0x10)
raw { 0xB0, 0x0A }
int_call(0x10)

// ---- Step 6: Print "No ASM!" ----
raw { 0xB4, 0x0E }
// 'N'
raw { 0xB0, 0x4E }
int_call(0x10)
// 'o'
raw { 0xB0, 0x6F }
int_call(0x10)
// ' '
raw { 0xB0, 0x20 }
int_call(0x10)
// 'A'
raw { 0xB0, 0x41 }
int_call(0x10)
// 'S'
raw { 0xB0, 0x53 }
int_call(0x10)
// 'M'
raw { 0xB0, 0x4D }
int_call(0x10)
// '!'
raw { 0xB0, 0x21 }
int_call(0x10)

// ---- Step 7: Halt ----
cli
hlt
// Safety: infinite loop
raw { 0xEB, 0xFE }
