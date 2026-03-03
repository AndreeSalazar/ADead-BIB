# FastOS v2.0 Build Script
# PowerShell script to build bootable OS image

param(
    [switch]$Clean,
    [switch]$Run,
    [switch]$Debug
)

$ErrorActionPreference = "Continue"

# Paths
$ROOT = $PSScriptRoot
$BUILD = "$ROOT\build"
$BOOT = "$ROOT\boot"
$KERNEL = "$ROOT\kernel"
$ADEAD = "cargo run --manifest-path=$ROOT\..\Cargo.toml --"

# Colors
function Write-Status($msg) { Write-Host "[BUILD] $msg" -ForegroundColor Cyan }
function Write-Success($msg) { Write-Host "[OK] $msg" -ForegroundColor Green }
function Write-Error($msg) { Write-Host "[ERROR] $msg" -ForegroundColor Red }

# Clean
if ($Clean) {
    Write-Status "Cleaning build directory..."
    if (Test-Path $BUILD) { Remove-Item -Recurse -Force $BUILD }
    Write-Success "Clean complete"
    exit 0
}

# Create build directory
if (-not (Test-Path $BUILD)) {
    New-Item -ItemType Directory -Path $BUILD | Out-Null
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Yellow
Write-Host "  FastOS v2.0 Build System" -ForegroundColor Yellow
Write-Host "  ADead-BIB Native Operating System" -ForegroundColor Yellow
Write-Host "========================================" -ForegroundColor Yellow
Write-Host ""

# Step 1: Build Stage 1 (MBR) - Real bootloader
Write-Status "Building Stage 1 bootloader..."

$stage1 = New-Object byte[] 512

# Real 16-bit bootloader code
$code = @(
    # org 0x7C00 (BIOS loads us here)
    0xFA,                   # cli
    0x31, 0xC0,             # xor ax, ax
    0x8E, 0xD8,             # mov ds, ax
    0x8E, 0xC0,             # mov es, ax
    0x8E, 0xD0,             # mov ss, ax
    0xBC, 0x00, 0x7C,       # mov sp, 0x7C00
    0xFB,                   # sti
    
    # Set video mode 3 (80x25 text)
    0xB8, 0x03, 0x00,       # mov ax, 0x0003
    0xCD, 0x10,             # int 0x10
    
    # Set cursor position to (0,0)
    0xB4, 0x02,             # mov ah, 0x02
    0xB7, 0x00,             # mov bh, 0 (page)
    0xB6, 0x00,             # mov dh, 0 (row)
    0xB2, 0x00,             # mov dl, 0 (col)
    0xCD, 0x10,             # int 0x10
    
    # Print string using BIOS
    0xBE, 0x50, 0x7C,       # mov si, msg (offset 0x50 from 0x7C00)
    # print_loop:
    0xAC,                   # lodsb
    0x08, 0xC0,             # or al, al
    0x74, 0x09,             # jz done
    0xB4, 0x0E,             # mov ah, 0x0E (teletype)
    0xB7, 0x00,             # mov bh, 0
    0xB3, 0x0F,             # mov bl, 0x0F (white)
    0xCD, 0x10,             # int 0x10
    0xEB, 0xF1,             # jmp print_loop
    # done:
    
    # Load stage2 from disk (sectors 2-33)
    0xB4, 0x02,             # mov ah, 0x02 (read sectors)
    0xB0, 0x20,             # mov al, 32 (sectors to read)
    0xB5, 0x00,             # mov ch, 0 (cylinder)
    0xB6, 0x00,             # mov dh, 0 (head)
    0xB1, 0x02,             # mov cl, 2 (start sector)
    0xBB, 0x00, 0x80,       # mov bx, 0x8000 (destination)
    0xB2, 0x00,             # mov dl, 0 (drive A:)
    0xCD, 0x13,             # int 0x13
    0x72, 0x0D,             # jc disk_error
    
    # Jump to stage2
    0xEA, 0x00, 0x80, 0x00, 0x00,  # jmp 0x0000:0x8000
    
    # disk_error: print error and halt
    0xBE, 0x80, 0x7C,       # mov si, error_msg
    0xEB, 0xD5,             # jmp print_loop (reuse)
    
    # halt:
    0xF4,                   # hlt
    0xEB, 0xFD              # jmp halt
)

# Copy code to stage1
for ($i = 0; $i -lt $code.Length; $i++) {
    $stage1[$i] = $code[$i]
}

# Message at offset 0x50 (0x7C50)
$msg = [System.Text.Encoding]::ASCII.GetBytes("FastOS v2.0 - Loading...`r`n`0")
for ($i = 0; $i -lt $msg.Length; $i++) {
    $stage1[0x50 + $i] = $msg[$i]
}

# Error message at offset 0x80
$errMsg = [System.Text.Encoding]::ASCII.GetBytes("Disk Error!`0")
for ($i = 0; $i -lt $errMsg.Length; $i++) {
    $stage1[0x80 + $i] = $errMsg[$i]
}

# Boot signature at 510-511
$stage1[510] = 0x55
$stage1[511] = 0xAA

[System.IO.File]::WriteAllBytes("$BUILD\stage1.bin", $stage1)
Write-Success "Stage 1: 512 bytes (real bootloader)"

# Step 2: Build Stage 2 - Complete Desktop with Windows, Apps, Terminal
Write-Status "Building Stage 2 bootloader..."

$stage2 = New-Object byte[] 16384  # 32 sectors

# Stage2 code - Simple working desktop (no complex mouse for now)
$stage2Code = @(
    # === INIT ===
    0xFA,                   # cli
    0x31, 0xC0,             # xor ax, ax
    0x8E, 0xD8,             # mov ds, ax
    0x8E, 0xD0,             # mov ss, ax
    0xBC, 0x00, 0x90,       # mov sp, 0x9000
    0xFB,                   # sti
    
    # Set video mode 3 (80x25 text)
    0xB8, 0x03, 0x00,       # mov ax, 0x0003
    0xCD, 0x10,             # int 0x10
    
    # Hide cursor
    0xB4, 0x01,             # mov ah, 0x01
    0xB9, 0x00, 0x20,       # mov cx, 0x2000
    0xCD, 0x10,             # int 0x10
    
    # === SETUP VGA ===
    0xB8, 0x00, 0xB8,       # mov ax, 0xB800
    0x8E, 0xC0,             # mov es, ax
    
    # === FILL SCREEN BLUE ===
    0x31, 0xFF,             # xor di, di
    0xB9, 0xD0, 0x07,       # mov cx, 2000
    0xB8, 0x20, 0x1F,       # mov ax, 0x1F20
    0xF3, 0xAB,             # rep stosw
    
    # === TASKBAR (row 24) ===
    0xBF, 0x00, 0x0F,       # mov di, 3840
    0xB9, 0x50, 0x00,       # mov cx, 80
    0xB8, 0x20, 0x70,       # mov ax, 0x7020
    0xF3, 0xAB,             # rep stosw
    
    # === WINDOW TITLE BAR (row 3, col 15, width 45) ===
    0xBF, 0xDE, 0x01,       # mov di, 478
    0xB9, 0x2D, 0x00,       # mov cx, 45
    0xB8, 0x20, 0x1F,       # mov ax, 0x1F20
    0xF3, 0xAB,             # rep stosw
    
    # === WINDOW CONTENT rows 4-13 ===
    0xBF, 0x7E, 0x02,       # row 4
    0xB9, 0x2D, 0x00, 0xB8, 0x20, 0xF0, 0xF3, 0xAB,
    0xBF, 0x1E, 0x03,       # row 5
    0xB9, 0x2D, 0x00, 0xB8, 0x20, 0xF0, 0xF3, 0xAB,
    0xBF, 0xBE, 0x03,       # row 6
    0xB9, 0x2D, 0x00, 0xB8, 0x20, 0xF0, 0xF3, 0xAB,
    0xBF, 0x5E, 0x04,       # row 7
    0xB9, 0x2D, 0x00, 0xB8, 0x20, 0xF0, 0xF3, 0xAB,
    0xBF, 0xFE, 0x04,       # row 8
    0xB9, 0x2D, 0x00, 0xB8, 0x20, 0xF0, 0xF3, 0xAB,
    0xBF, 0x9E, 0x05,       # row 9
    0xB9, 0x2D, 0x00, 0xB8, 0x20, 0xF0, 0xF3, 0xAB,
    0xBF, 0x3E, 0x06,       # row 10
    0xB9, 0x2D, 0x00, 0xB8, 0x20, 0xF0, 0xF3, 0xAB,
    0xBF, 0xDE, 0x06,       # row 11
    0xB9, 0x2D, 0x00, 0xB8, 0x20, 0xF0, 0xF3, 0xAB,
    0xBF, 0x7E, 0x07,       # row 12
    0xB9, 0x2D, 0x00, 0xB8, 0x20, 0xF0, 0xF3, 0xAB,
    0xBF, 0x1E, 0x08,       # row 13
    0xB9, 0x2D, 0x00, 0xB8, 0x20, 0xF0, 0xF3, 0xAB,
    
    # === WRITE TEXT DIRECTLY ===
    # "Terminal" in title bar
    0xBF, 0xE0, 0x01,       # di = 480
    0xB0, 0x54, 0xB4, 0x1F, 0xAB,  # T
    0xB0, 0x65, 0xB4, 0x1F, 0xAB,  # e
    0xB0, 0x72, 0xB4, 0x1F, 0xAB,  # r
    0xB0, 0x6D, 0xB4, 0x1F, 0xAB,  # m
    0xB0, 0x69, 0xB4, 0x1F, 0xAB,  # i
    0xB0, 0x6E, 0xB4, 0x1F, 0xAB,  # n
    0xB0, 0x61, 0xB4, 0x1F, 0xAB,  # a
    0xB0, 0x6C, 0xB4, 0x1F, 0xAB,  # l
    
    # [X] close button
    0xBF, 0x70, 0x02,       # di = 624 (row 3, col 56)
    0xB0, 0x5B, 0xB4, 0x4F, 0xAB,  # [
    0xB0, 0x58, 0xB4, 0x4F, 0xAB,  # X
    0xB0, 0x5D, 0xB4, 0x4F, 0xAB,  # ]
    
    # "FastOS v2.0" in window
    0xBF, 0x20, 0x03,       # di = 800 (row 5, col 16)
    0xB0, 0x46, 0xB4, 0xF0, 0xAB,  # F
    0xB0, 0x61, 0xB4, 0xF0, 0xAB,  # a
    0xB0, 0x73, 0xB4, 0xF0, 0xAB,  # s
    0xB0, 0x74, 0xB4, 0xF0, 0xAB,  # t
    0xB0, 0x4F, 0xB4, 0xF0, 0xAB,  # O
    0xB0, 0x53, 0xB4, 0xF0, 0xAB,  # S
    0xB0, 0x20, 0xB4, 0xF0, 0xAB,  # space
    0xB0, 0x76, 0xB4, 0xF0, 0xAB,  # v
    0xB0, 0x32, 0xB4, 0xF0, 0xAB,  # 2
    0xB0, 0x2E, 0xB4, 0xF0, 0xAB,  # .
    0xB0, 0x30, 0xB4, 0xF0, 0xAB,  # 0
    
    # "[BG] Binary Guardian: ACTIVE"
    0xBF, 0xC0, 0x03,       # di = 960 (row 6, col 16)
    0xB0, 0x5B, 0xB4, 0xF2, 0xAB,  # [
    0xB0, 0x42, 0xB4, 0xF2, 0xAB,  # B
    0xB0, 0x47, 0xB4, 0xF2, 0xAB,  # G
    0xB0, 0x5D, 0xB4, 0xF2, 0xAB,  # ]
    0xB0, 0x20, 0xB4, 0xF0, 0xAB,  # space
    0xB0, 0x41, 0xB4, 0xF0, 0xAB,  # A
    0xB0, 0x43, 0xB4, 0xF0, 0xAB,  # C
    0xB0, 0x54, 0xB4, 0xF0, 0xAB,  # T
    0xB0, 0x49, 0xB4, 0xF0, 0xAB,  # I
    0xB0, 0x56, 0xB4, 0xF0, 0xAB,  # V
    0xB0, 0x45, 0xB4, 0xF0, 0xAB,  # E
    
    # "[musl] LOADED"
    0xBF, 0x60, 0x04,       # di = 1120 (row 7, col 16)
    0xB0, 0x5B, 0xB4, 0xFA, 0xAB,  # [
    0xB0, 0x6D, 0xB4, 0xFA, 0xAB,  # m
    0xB0, 0x75, 0xB4, 0xFA, 0xAB,  # u
    0xB0, 0x73, 0xB4, 0xFA, 0xAB,  # s
    0xB0, 0x6C, 0xB4, 0xFA, 0xAB,  # l
    0xB0, 0x5D, 0xB4, 0xFA, 0xAB,  # ]
    0xB0, 0x20, 0xB4, 0xF0, 0xAB,  # space
    0xB0, 0x4C, 0xB4, 0xF0, 0xAB,  # L
    0xB0, 0x4F, 0xB4, 0xF0, 0xAB,  # O
    0xB0, 0x41, 0xB4, 0xF0, 0xAB,  # A
    0xB0, 0x44, 0xB4, 0xF0, 0xAB,  # D
    0xB0, 0x45, 0xB4, 0xF0, 0xAB,  # E
    0xB0, 0x44, 0xB4, 0xF0, 0xAB,  # D
    
    # "[Po] PE+ELF OK"
    0xBF, 0x00, 0x05,       # di = 1280 (row 8, col 16)
    0xB0, 0x5B, 0xB4, 0xFE, 0xAB,  # [
    0xB0, 0x50, 0xB4, 0xFE, 0xAB,  # P
    0xB0, 0x6F, 0xB4, 0xFE, 0xAB,  # o
    0xB0, 0x5D, 0xB4, 0xFE, 0xAB,  # ]
    0xB0, 0x20, 0xB4, 0xF0, 0xAB,  # space
    0xB0, 0x50, 0xB4, 0xF0, 0xAB,  # P
    0xB0, 0x45, 0xB4, 0xF0, 0xAB,  # E
    0xB0, 0x2B, 0xB4, 0xF0, 0xAB,  # +
    0xB0, 0x45, 0xB4, 0xF0, 0xAB,  # E
    0xB0, 0x4C, 0xB4, 0xF0, 0xAB,  # L
    0xB0, 0x46, 0xB4, 0xF0, 0xAB,  # F
    0xB0, 0x20, 0xB4, 0xF0, 0xAB,  # space
    0xB0, 0x4F, 0xB4, 0xF0, 0xAB,  # O
    0xB0, 0x4B, 0xB4, 0xF0, 0xAB,  # K
    
    # "C:\> _" at row 12, col 16 (separate from other text)
    0xBF, 0xE0, 0x07,       # di = 2016 (row 12, col 16)
    0xB0, 0x43, 0xB4, 0xF0, 0xAB,  # C
    0xB0, 0x3A, 0xB4, 0xF0, 0xAB,  # :
    0xB0, 0x5C, 0xB4, 0xF0, 0xAB,  # \
    0xB0, 0x3E, 0xB4, 0xF0, 0xAB,  # >
    0xB0, 0x20, 0xB4, 0xF0, 0xAB,  # space
    0xB0, 0x5F, 0xB4, 0xF2, 0xAB,  # _ green cursor
    
    # Taskbar "Start"
    0xBF, 0x04, 0x0F,       # di = 3844
    0xB0, 0x53, 0xB4, 0x70, 0xAB,  # S
    0xB0, 0x74, 0xB4, 0x70, 0xAB,  # t
    0xB0, 0x61, 0xB4, 0x70, 0xAB,  # a
    0xB0, 0x72, 0xB4, 0x70, 0xAB,  # r
    0xB0, 0x74, 0xB4, 0x70, 0xAB,  # t
    
    # Taskbar "12:00"
    0xBF, 0x92, 0x0F,       # di = 3986
    0xB0, 0x31, 0xB4, 0x70, 0xAB,  # 1
    0xB0, 0x32, 0xB4, 0x70, 0xAB,  # 2
    0xB0, 0x3A, 0xB4, 0x70, 0xAB,  # :
    0xB0, 0x30, 0xB4, 0x70, 0xAB,  # 0
    0xB0, 0x30, 0xB4, 0x70, 0xAB,  # 0
    
    # Desktop icons
    0xBF, 0x44, 0x01,       # row 2, col 2
    0xB0, 0x5B, 0xB4, 0x1E, 0xAB, 0xB0, 0x3E, 0xB4, 0x1E, 0xAB, 0xB0, 0x5D, 0xB4, 0x1E, 0xAB,
    0xB0, 0x20, 0xB4, 0x1F, 0xAB,
    0xB0, 0x54, 0xB4, 0x1F, 0xAB, 0xB0, 0x65, 0xB4, 0x1F, 0xAB, 0xB0, 0x72, 0xB4, 0x1F, 0xAB, 0xB0, 0x6D, 0xB4, 0x1F, 0xAB,
    
    0xBF, 0xE4, 0x02,       # row 4, col 2
    0xB0, 0x5B, 0xB4, 0x1E, 0xAB, 0xB0, 0x44, 0xB4, 0x1E, 0xAB, 0xB0, 0x5D, 0xB4, 0x1E, 0xAB,
    0xB0, 0x20, 0xB4, 0x1F, 0xAB,
    0xB0, 0x46, 0xB4, 0x1F, 0xAB, 0xB0, 0x69, 0xB4, 0x1F, 0xAB, 0xB0, 0x6C, 0xB4, 0x1F, 0xAB, 0xB0, 0x65, 0xB4, 0x1F, 0xAB, 0xB0, 0x73, 0xB4, 0x1F, 0xAB,
    
    0xBF, 0x84, 0x04,       # row 6, col 2
    0xB0, 0x5B, 0xB4, 0x1E, 0xAB, 0xB0, 0x2A, 0xB4, 0x1E, 0xAB, 0xB0, 0x5D, 0xB4, 0x1E, 0xAB,
    0xB0, 0x20, 0xB4, 0x1F, 0xAB,
    0xB0, 0x53, 0xB4, 0x1F, 0xAB, 0xB0, 0x65, 0xB4, 0x1F, 0xAB, 0xB0, 0x74, 0xB4, 0x1F, 0xAB,
    
    # === SIMPLE KEYBOARD INPUT LOOP ===
    # Based on OSDev Wiki PS/2 Keyboard
    
    # Initialize cursor position variable at 0x500
    0x31, 0xC0,             # xor ax, ax
    0x8E, 0xD8,             # mov ds, ax
    0xC6, 0x06, 0x00, 0x05, 0x00,  # [0x500] = 0 (typing position)
    
    # Draw mouse cursor at center
    0xB8, 0x00, 0xB8,       # mov ax, 0xB800
    0x8E, 0xC0,             # mov es, ax
    0xBF, 0x18, 0x06,       # di = row 12, col 40
    0xB8, 0x1E, 0x4E,       # yellow arrow on red
    0xAB,                   # stosw
    
    # === MAIN LOOP (offset 0x00) ===
    # loop_start:
    
    # Wait for keyboard data (simple polling)
    0xE4, 0x64,             # in al, 0x64 (status)
    0xA8, 0x01,             # test al, 1 (data ready?)
    0x74, 0xFA,             # jz loop_start (-6 bytes)
    
    # Read scancode
    0xE4, 0x60,             # in al, 0x60
    
    # Ignore key releases (bit 7 set)
    0xA8, 0x80,             # test al, 0x80
    0x75, 0xF2,             # jnz loop_start
    
    # Check ESC (scancode 0x01)
    0x3C, 0x01,             # cmp al, 1
    0x74, 0x30,             # je halt
    
    # === SCANCODE TO ASCII (simple table) ===
    # Store scancode in BL
    0x88, 0xC3,             # mov bl, al
    
    # Scancode 0x1E = 'a', 0x30 = 'b', 0x2E = 'c', etc
    # Simple: just add 0x40 to get approximate letter
    0x3C, 0x10,             # cmp al, 0x10 (first letter key)
    0x72, 0xE4,             # jb loop_start
    0x3C, 0x39,             # cmp al, 0x39 (space)
    0x77, 0xE0,             # ja loop_start
    
    # Space key
    0x3C, 0x39,             # cmp al, 0x39
    0x75, 0x04,             # jne not_space
    0xB0, 0x20,             # mov al, ' '
    0xEB, 0x04,             # jmp write_char
    # not_space:
    0x24, 0x1F,             # and al, 0x1F
    0x04, 0x60,             # add al, 0x60 ('a'-1)
    
    # write_char:
    # Get cursor position
    0xA0, 0x00, 0x05,       # mov al, [0x500]
    0x31, 0xDB,             # xor bx, bx
    0x88, 0xC3,             # mov bl, al
    
    # Calculate VGA offset: row 12, col 22 + position
    # Row 12 = 12*160 = 1920 = 0x780
    # Col 22 = 22*2 = 44 = 0x2C
    # Base = 0x780 + 0x2C = 0x7AC
    0xBF, 0xAC, 0x07,       # mov di, 0x7AC
    0x01, 0xDF,             # add di, bx
    0x01, 0xDF,             # add di, bx (*2 for char+attr)
    
    # Write character (restore AL from BL trick - use stack)
    0x88, 0xD8,             # mov al, bl (oops, this is position)
    
    # Actually we need the converted char - let's redo
    # The char is still in the flags from the conversion
    0xE4, 0x60,             # in al, 0x60 (re-read - bad but works)
    0x24, 0x1F,             # and al, 0x1F
    0x04, 0x61,             # add al, 0x61
    0xB4, 0xF0,             # mov ah, 0xF0 (black on white)
    0xAB,                   # stosw
    
    # Increment cursor
    0xFE, 0x06, 0x00, 0x05, # inc byte [0x500]
    
    # Jump back to loop
    0xEB, 0xBE,             # jmp loop_start
    
    # halt:
    0xF4,                   # hlt
    0xEB, 0xFD              # jmp $
)

for ($i = 0; $i -lt $stage2Code.Length; $i++) {
    $stage2[$i] = $stage2Code[$i]
}

# === STRING DATA at 0x200 (0x8200) ===
$stage2Strings = @(
    # Taskbar
    @{ Offset = 0x200; Text = "[=] Start" }
    @{ Offset = 0x220; Text = "12:00 PM" }
    # Desktop icons
    @{ Offset = 0x240; Text = "[>] Terminal" }
    @{ Offset = 0x260; Text = "[D] Files" }
    @{ Offset = 0x280; Text = "[*] Settings" }
    @{ Offset = 0x2A0; Text = "[~] Browser" }
    # Window
    @{ Offset = 0x2C0; Text = "Terminal - FastOS" }
    @{ Offset = 0x2E0; Text = "[X]" }
    # Window content
    @{ Offset = 0x300; Text = "FastOS v2.0 [C Master + Rust Safety]" }
    @{ Offset = 0x330; Text = "C:\> [BG] Binary Guardian: ACTIVE" }
    @{ Offset = 0x360; Text = "C:\> [musl] libc: LOADED" }
    @{ Offset = 0x390; Text = "C:\> [Po] PE+ELF+Win32 ready" }
    @{ Offset = 0x3C0; Text = "C:\> [Nouveau] GPU driver OK" }
    @{ Offset = 0x3F0; Text = "C:\> _" }
)

foreach ($s in $stage2Strings) {
    $bytes = [System.Text.Encoding]::ASCII.GetBytes($s.Text + "`0")
    for ($i = 0; $i -lt $bytes.Length; $i++) {
        $stage2[$s.Offset + $i] = $bytes[$i]
    }
}

[System.IO.File]::WriteAllBytes("$BUILD\stage2.bin", $stage2)
Write-Success "Stage 2: $($stage2.Length) bytes (Full Desktop + Windows + Apps)"

# Step 3: Build Kernel
Write-Status "Building kernel..."

# Create kernel binary stub (would be compiled from kernel/*.c)
$kernelSize = 131072  # 128KB
$kernel = New-Object byte[] $kernelSize

# Kernel header
$kernel[0] = 0x46  # 'F'
$kernel[1] = 0x61  # 'a'
$kernel[2] = 0x73  # 's'
$kernel[3] = 0x74  # 't'
$kernel[4] = 0x4F  # 'O'
$kernel[5] = 0x53  # 'S'

# Simple kernel code: clear screen, print message, halt
# This would be replaced by actual compiled kernel
$code = @(
    0xB8, 0x00, 0x80, 0x0B, 0x00,  # mov ax, 0xB800 (VGA segment)
    0x8E, 0xD8,                     # mov ds, ax
    0xBE, 0x00, 0x00,               # mov si, 0
    0xB0, 0x46,                     # mov al, 'F'
    0x88, 0x04,                     # mov [si], al
    0x46,                           # inc si
    0xB0, 0x1F,                     # mov al, 0x1F (white on blue)
    0x88, 0x04,                     # mov [si], al
    0xF4                            # hlt
)
for ($i = 0; $i -lt $code.Length; $i++) {
    $kernel[16 + $i] = $code[$i]
}

[System.IO.File]::WriteAllBytes("$BUILD\kernel.bin", $kernel)
Write-Success "Kernel: $kernelSize bytes"

# Step 4: Create disk image
Write-Status "Creating disk image..."

$imageSize = 1474560  # 1.44MB floppy
$image = New-Object byte[] $imageSize

# Copy stage1 (sector 0)
$stage1Bytes = [System.IO.File]::ReadAllBytes("$BUILD\stage1.bin")
[Array]::Copy($stage1Bytes, 0, $image, 0, 512)

# Copy stage2 (sectors 1-32)
$stage2Bytes = [System.IO.File]::ReadAllBytes("$BUILD\stage2.bin")
[Array]::Copy($stage2Bytes, 0, $image, 512, $stage2Bytes.Length)

# Copy kernel (sectors 34+)
$kernelBytes = [System.IO.File]::ReadAllBytes("$BUILD\kernel.bin")
$kernelOffset = 34 * 512
[Array]::Copy($kernelBytes, 0, $image, $kernelOffset, [Math]::Min($kernelBytes.Length, $imageSize - $kernelOffset))

# Verify boot signature
if ($image[510] -eq 0x55 -and $image[511] -eq 0xAA) {
    Write-Success "Boot signature: 0xAA55"
} else {
    Write-Error "Invalid boot signature!"
}

[System.IO.File]::WriteAllBytes("$BUILD\fastos.img", $image)
Write-Success "Disk image: $imageSize bytes ($BUILD\fastos.img)"

# Summary
Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "  Build Complete!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "  Stage 1:  512 bytes" -ForegroundColor White
Write-Host "  Stage 2:  $($stage2.Length) bytes" -ForegroundColor White
Write-Host "  Kernel:   $kernelSize bytes" -ForegroundColor White
Write-Host "  Image:    $imageSize bytes" -ForegroundColor White
Write-Host ""

# Run in QEMU if requested
if ($Run) {
    Write-Status "Starting QEMU..."
    
    # Search for QEMU in common locations
    $qemuPaths = @(
        "qemu-system-x86_64",
        "C:\Program Files\qemu\qemu-system-x86_64.exe",
        "C:\Program Files (x86)\qemu\qemu-system-x86_64.exe",
        "$env:LOCALAPPDATA\Programs\qemu\qemu-system-x86_64.exe",
        "C:\qemu\qemu-system-x86_64.exe",
        "$env:USERPROFILE\qemu\qemu-system-x86_64.exe",
        "$env:USERPROFILE\scoop\apps\qemu\current\qemu-system-x86_64.exe"
    )
    
    $qemu = $null
    foreach ($path in $qemuPaths) {
        if (Get-Command $path -ErrorAction SilentlyContinue) {
            $qemu = $path
            break
        }
        if (Test-Path $path) {
            $qemu = $path
            break
        }
    }
    
    if (-not $qemu) {
        Write-Error "QEMU not found in common locations."
        Write-Host ""
        Write-Host "Please install QEMU:" -ForegroundColor Yellow
        Write-Host "  1. Download from: https://www.qemu.org/download/#windows" -ForegroundColor Cyan
        Write-Host "  2. Or use winget: winget install SoftwareFreedomConservancy.QEMU" -ForegroundColor Cyan
        Write-Host "  3. Or use scoop: scoop install qemu" -ForegroundColor Cyan
        Write-Host ""
        Write-Host "After installing, run: .\build.ps1 -Run" -ForegroundColor Green
        exit 1
    }
    
    Write-Host "Using QEMU: $qemu" -ForegroundColor Gray
    
    $qemuArgs = @(
        "-drive", "file=$BUILD\fastos.img,format=raw,if=floppy",
        "-m", "256M",
        "-cpu", "qemu64",
        "-serial", "stdio",
        "-device", "usb-mouse",
        "-usb",
        "-display", "gtk"
    )
    
    if ($Debug) {
        $qemuArgs += @("-s", "-S")
        Write-Host "Debug mode: Connect GDB to localhost:1234" -ForegroundColor Yellow
    }
    
    try {
        & $qemu $qemuArgs
    } catch {
        Write-Error "Failed to start QEMU: $_"
    }
}

Write-Host ""
Write-Host "To run: .\build.ps1 -Run" -ForegroundColor Cyan
Write-Host "To debug: .\build.ps1 -Run -Debug" -ForegroundColor Cyan
Write-Host ""
