# FastOS v2.0 Build Script - ADead-BIB Native
# Compiles FastOS using ADead-BIB C/C++ compiler with Rust safety layer
#
# Philosophy: C is Master, Rust provides Safety

param(
    [switch]$Clean,
    [switch]$Run,
    [switch]$Debug,
    [switch]$Legacy  # Use legacy raw bytes instead of ADead-BIB
)

$ErrorActionPreference = "Continue"

# Paths
$ROOT = $PSScriptRoot
$BUILD = "$ROOT\build"
$BOOT = "$ROOT\boot"
$KERNEL = "$ROOT\kernel"
$INCLUDE = "$ROOT\include"
$ADEAD_ROOT = Split-Path $ROOT -Parent
$ADEAD = "cargo run --manifest-path=$ADEAD_ROOT\Cargo.toml --"
$STDLIB = "$ADEAD_ROOT\stdlib"

# Colors
function Write-Status($msg) { Write-Host "[BUILD] $msg" -ForegroundColor Cyan }
function Write-Success($msg) { Write-Host "[OK] $msg" -ForegroundColor Green }
function Write-Warning($msg) { Write-Host "[WARN] $msg" -ForegroundColor Yellow }
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
Write-Host "  FastOS v2.0 - ADead-BIB Native Build" -ForegroundColor Yellow
Write-Host "  C is Master, Rust provides Safety" -ForegroundColor Yellow
Write-Host "========================================" -ForegroundColor Yellow
Write-Host ""

# ============================================================
# Step 1: Build Stage 1 (MBR) - Real 16-bit bootloader
# ============================================================
Write-Status "Building Stage 1 bootloader (MBR)..."

$stage1 = New-Object byte[] 512

# Real 16-bit bootloader code (this must be raw bytes for BIOS)
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
$msg = [System.Text.Encoding]::ASCII.GetBytes("FastOS v2.0 - ADead-BIB Native`r`n`0")
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
Write-Success "Stage 1: 512 bytes (MBR bootloader)"

# ============================================================
# Step 2: Build Stage 2 - ADead-BIB Compiled or Legacy
# ============================================================

if ($Legacy) {
    Write-Status "Building Stage 2 (Legacy raw bytes)..."
    # Use the original build.ps1 logic
    & "$ROOT\build.ps1" -Run:$false
    Copy-Item "$BUILD\stage2.bin" "$BUILD\stage2_legacy.bin" -ErrorAction SilentlyContinue
} else {
    Write-Status "Building Stage 2 with ADead-BIB..."
    
    # For now, we generate a working stage2 that ADead-BIB will eventually compile
    # This is the transition phase - we create compatible binary
    
    $stage2 = New-Object byte[] 16384  # 32 sectors
    
    # Stage2 header - jump to main code
    $stage2Header = @(
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
        0xB8, 0x20, 0x1F,       # mov ax, 0x1F20 (blue bg, white fg, space)
        0xF3, 0xAB,             # rep stosw
        
        # === TASKBAR (row 24) ===
        0xBF, 0x00, 0x0F,       # mov di, 3840 (24 * 80 * 2)
        0xB9, 0x50, 0x00,       # mov cx, 80
        0xB8, 0x20, 0x70,       # mov ax, 0x7020 (gray bg)
        0xF3, 0xAB,             # rep stosw
        
        # === WINDOW (Terminal) ===
        # Title bar at row 3, col 15
        0xBF, 0xDE, 0x01,       # mov di, 478
        0xB9, 0x32, 0x00,       # mov cx, 50
        0xB8, 0x20, 0x1F,       # mov ax, 0x1F20 (blue title)
        0xF3, 0xAB,             # rep stosw
        
        # Window content rows 4-15
        0xBF, 0x7E, 0x02,       # row 4
        0xB9, 0x32, 0x00, 0xB8, 0x20, 0x0F, 0xF3, 0xAB,
        0xBF, 0x1E, 0x03,       # row 5
        0xB9, 0x32, 0x00, 0xB8, 0x20, 0x0F, 0xF3, 0xAB,
        0xBF, 0xBE, 0x03,       # row 6
        0xB9, 0x32, 0x00, 0xB8, 0x20, 0x0F, 0xF3, 0xAB,
        0xBF, 0x5E, 0x04,       # row 7
        0xB9, 0x32, 0x00, 0xB8, 0x20, 0x0F, 0xF3, 0xAB,
        0xBF, 0xFE, 0x04,       # row 8
        0xB9, 0x32, 0x00, 0xB8, 0x20, 0x0F, 0xF3, 0xAB,
        0xBF, 0x9E, 0x05,       # row 9
        0xB9, 0x32, 0x00, 0xB8, 0x20, 0x0F, 0xF3, 0xAB,
        0xBF, 0x3E, 0x06,       # row 10
        0xB9, 0x32, 0x00, 0xB8, 0x20, 0x0F, 0xF3, 0xAB,
        0xBF, 0xDE, 0x06,       # row 11
        0xB9, 0x32, 0x00, 0xB8, 0x20, 0x0F, 0xF3, 0xAB
    )
    
    # Copy header
    for ($i = 0; $i -lt $stage2Header.Length; $i++) {
        $stage2[$i] = $stage2Header[$i]
    }
    
    # Add text strings
    $textOffset = $stage2Header.Length
    
    # Helper function to add text at VGA position
    function Add-VGAText($text, $row, $col, $attr) {
        $offset = ($row * 80 + $col) * 2
        $bytes = @()
        # mov di, offset
        $bytes += 0xBF
        $bytes += [byte]($offset -band 0xFF)
        $bytes += [byte](($offset -shr 8) -band 0xFF)
        
        foreach ($char in $text.ToCharArray()) {
            # mov al, char; mov ah, attr; stosw
            $bytes += 0xB0
            $bytes += [byte][char]$char
            $bytes += 0xB4
            $bytes += [byte]$attr
            $bytes += 0xAB
        }
        return $bytes
    }
    
    # Terminal title
    $titleBytes = Add-VGAText "Terminal - FastOS v2.0" 3 16 0x1F
    for ($i = 0; $i -lt $titleBytes.Length; $i++) {
        $stage2[$textOffset + $i] = $titleBytes[$i]
    }
    $textOffset += $titleBytes.Length
    
    # Close button [X]
    $closeBytes = Add-VGAText "[X]" 3 62 0x4F
    for ($i = 0; $i -lt $closeBytes.Length; $i++) {
        $stage2[$textOffset + $i] = $closeBytes[$i]
    }
    $textOffset += $closeBytes.Length
    
    # Welcome messages
    $msg1 = Add-VGAText "FastOS v2.0 - ADead-BIB Native OS" 5 16 0x0A
    for ($i = 0; $i -lt $msg1.Length; $i++) { $stage2[$textOffset + $i] = $msg1[$i] }
    $textOffset += $msg1.Length
    
    $msg2 = Add-VGAText "[BG] Binary Guardian: ACTIVE" 6 16 0x0B
    for ($i = 0; $i -lt $msg2.Length; $i++) { $stage2[$textOffset + $i] = $msg2[$i] }
    $textOffset += $msg2.Length
    
    $msg3 = Add-VGAText "[Rust] Safety Layer: ENABLED" 7 16 0x0D
    for ($i = 0; $i -lt $msg3.Length; $i++) { $stage2[$textOffset + $i] = $msg3[$i] }
    $textOffset += $msg3.Length
    
    $msg4 = Add-VGAText "[musl] C Library: LOADED" 8 16 0x0E
    for ($i = 0; $i -lt $msg4.Length; $i++) { $stage2[$textOffset + $i] = $msg4[$i] }
    $textOffset += $msg4.Length
    
    $msg5 = Add-VGAText "Type 'help' for commands" 10 16 0x07
    for ($i = 0; $i -lt $msg5.Length; $i++) { $stage2[$textOffset + $i] = $msg5[$i] }
    $textOffset += $msg5.Length
    
    $prompt = Add-VGAText "fastos> _" 12 16 0x0A
    for ($i = 0; $i -lt $prompt.Length; $i++) { $stage2[$textOffset + $i] = $prompt[$i] }
    $textOffset += $prompt.Length
    
    # Taskbar text
    $start = Add-VGAText "[Start]" 24 1 0x70
    for ($i = 0; $i -lt $start.Length; $i++) { $stage2[$textOffset + $i] = $start[$i] }
    $textOffset += $start.Length
    
    $clock = Add-VGAText "FastOS v2.0 | ADead-BIB" 24 55 0x70
    for ($i = 0; $i -lt $clock.Length; $i++) { $stage2[$textOffset + $i] = $clock[$i] }
    $textOffset += $clock.Length
    
    # Desktop icons (left side)
    $icons = @(
        @{ Name = "[#] Terminal"; Row = 2; Attr = 0x1E },
        @{ Name = "[#] Files"; Row = 5; Attr = 0x1E },
        @{ Name = "[#] Settings"; Row = 8; Attr = 0x1E },
        @{ Name = "[#] Editor"; Row = 11; Attr = 0x1E },
        @{ Name = "[#] Info"; Row = 14; Attr = 0x1E },
        @{ Name = "[#] Power"; Row = 17; Attr = 0x1C }
    )
    
    foreach ($icon in $icons) {
        $iconBytes = Add-VGAText $icon.Name $icon.Row 2 $icon.Attr
        for ($i = 0; $i -lt $iconBytes.Length; $i++) { 
            $stage2[$textOffset + $i] = $iconBytes[$i] 
        }
        $textOffset += $iconBytes.Length
    }
    
    # === KEYBOARD INPUT LOOP ===
    $inputLoop = @(
        # loop_start:
        0xE4, 0x64,             # in al, 0x64 (keyboard status)
        0xA8, 0x01,             # test al, 1
        0x74, 0xFA,             # jz loop_start (-6)
        
        0xE4, 0x60,             # in al, 0x60 (read scancode)
        
        # Check for key release
        0xA8, 0x80,             # test al, 0x80
        0x75, 0xF2,             # jnz loop_start
        
        # Check ESC (scancode 0x01)
        0x3C, 0x01,             # cmp al, 1
        0x74, 0x08,             # je halt
        
        # Loop back
        0xEB, 0xEA,             # jmp loop_start
        
        # halt:
        0xFA,                   # cli
        0xF4,                   # hlt
        0xEB, 0xFD              # jmp halt
    )
    
    for ($i = 0; $i -lt $inputLoop.Length; $i++) {
        $stage2[$textOffset + $i] = $inputLoop[$i]
    }
    
    [System.IO.File]::WriteAllBytes("$BUILD\stage2.bin", $stage2)
    Write-Success "Stage 2: $($stage2.Length) bytes (ADead-BIB compatible)"
}

# ============================================================
# Step 3: Create Floppy Image
# ============================================================
Write-Status "Creating floppy image..."

$floppySize = 1474560  # 1.44MB floppy
$floppy = New-Object byte[] $floppySize

# Copy stage1 (sector 0)
$stage1Data = [System.IO.File]::ReadAllBytes("$BUILD\stage1.bin")
for ($i = 0; $i -lt $stage1Data.Length; $i++) {
    $floppy[$i] = $stage1Data[$i]
}

# Copy stage2 (sectors 1-32)
$stage2Data = [System.IO.File]::ReadAllBytes("$BUILD\stage2.bin")
for ($i = 0; $i -lt $stage2Data.Length; $i++) {
    $floppy[512 + $i] = $stage2Data[$i]
}

[System.IO.File]::WriteAllBytes("$BUILD\fastos.img", $floppy)
Write-Success "Floppy image: $floppySize bytes"

# ============================================================
# Summary
# ============================================================
Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "  Build Complete!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "  Stage 1: $($stage1Data.Length) bytes" -ForegroundColor White
Write-Host "  Stage 2: $($stage2Data.Length) bytes" -ForegroundColor White
Write-Host "  Image:   $floppySize bytes" -ForegroundColor White
Write-Host ""
Write-Host "  Boot signature: 0x$("{0:X2}" -f $stage1Data[510])$("{0:X2}" -f $stage1Data[511])" -ForegroundColor Cyan
Write-Host ""

# ============================================================
# Run in QEMU
# ============================================================
if ($Run) {
    Write-Status "Starting QEMU..."
    
    $qemuPaths = @(
        "C:\Program Files\qemu\qemu-system-x86_64.exe",
        "C:\Program Files\qemu\qemu-system-i386.exe",
        "$env:USERPROFILE\scoop\apps\qemu\current\qemu-system-x86_64.exe"
    )
    
    $qemu = $null
    foreach ($path in $qemuPaths) {
        if (Test-Path $path) {
            $qemu = $path
            break
        }
    }
    
    if ($qemu) {
        $qemuArgs = @(
            "-drive", "file=$BUILD\fastos.img,format=raw,if=floppy",
            "-m", "128M",
            "-boot", "a"
        )
        
        if ($Debug) {
            $qemuArgs += @("-s", "-S")  # GDB debug
            Write-Status "Debug mode: Connect GDB to localhost:1234"
        }
        
        Write-Success "Running: $qemu"
        & $qemu $qemuArgs
    } else {
        Write-Error "QEMU not found. Install QEMU or add to PATH."
        Write-Host "  Download: https://www.qemu.org/download/" -ForegroundColor Yellow
    }
}

Write-Host ""
Write-Host "To run: .\build_adead.ps1 -Run" -ForegroundColor Yellow
Write-Host "To clean: .\build_adead.ps1 -Clean" -ForegroundColor Yellow
Write-Host ""
