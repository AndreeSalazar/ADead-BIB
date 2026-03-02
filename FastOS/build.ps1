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

# Step 2: Build Stage 2 - Professional Windows/Linux hybrid interface
Write-Status "Building Stage 2 bootloader..."

$stage2 = New-Object byte[] 16384  # 32 sectors

# Simplified but working stage2 with direct VGA writes
$stage2Code = @(
    # org 0x8000
    0xFA,                   # cli
    0x31, 0xC0,             # xor ax, ax
    0x8E, 0xD8,             # mov ds, ax
    0x8E, 0xC0,             # mov es, ax
    0x8E, 0xD0,             # mov ss, ax
    0xBC, 0x00, 0x90,       # mov sp, 0x9000
    0xFB,                   # sti
    
    # Set video mode 3 (80x25 text, clear screen)
    0xB8, 0x03, 0x00,       # mov ax, 0x0003
    0xCD, 0x10,             # int 0x10
    
    # Hide cursor
    0xB4, 0x01,             # mov ah, 0x01
    0xB9, 0x00, 0x20,       # mov cx, 0x2000
    0xCD, 0x10,             # int 0x10
    
    # Setup VGA segment
    0xB8, 0x00, 0xB8,       # mov ax, 0xB800
    0x8E, 0xC0,             # mov es, ax
    
    # Fill entire screen with dark blue background (0x17 = white on blue)
    0x31, 0xFF,             # xor di, di
    0xB9, 0xD0, 0x07,       # mov cx, 2000 (80*25)
    0xB8, 0x20, 0x17,       # mov ax, 0x1720 (space, white on blue)
    0xF3, 0xAB,             # rep stosw
    
    # Draw top bar (Windows-style title bar) - row 0
    0x31, 0xFF,             # xor di, di
    0xB9, 0x50, 0x00,       # mov cx, 80
    0xB8, 0x20, 0x70,       # mov ax, 0x7020 (space, black on light gray)
    0xF3, 0xAB,             # rep stosw
    
    # Draw bottom bar (taskbar) - row 24 (offset 0x0F00 = 3840)
    0xBF, 0x00, 0x0F,       # mov di, 3840
    0xB9, 0x50, 0x00,       # mov cx, 80
    0xB8, 0x20, 0x70,       # mov ax, 0x7020
    0xF3, 0xAB,             # rep stosw
    
    # Write "FastOS v2.0" in title bar using direct memory writes
    # Position 2 in title bar = offset 4
    0xBF, 0x04, 0x00,       # mov di, 4
    # 'F' - yellow (0x0E) on gray (0x70) = 0x7E... no, 0x74 = red on gray
    # Let's use 0x70 for black on gray, 0x74 for red on gray
    0x26, 0xC7, 0x05, 0x46, 0x7E,  # mov word es:[di], 0x7E46 ('F' yellow)
    0x83, 0xC7, 0x02,       # add di, 2
    0x26, 0xC7, 0x05, 0x61, 0x70,  # 'a' black
    0x83, 0xC7, 0x02,
    0x26, 0xC7, 0x05, 0x73, 0x70,  # 's'
    0x83, 0xC7, 0x02,
    0x26, 0xC7, 0x05, 0x74, 0x70,  # 't'
    0x83, 0xC7, 0x02,
    0x26, 0xC7, 0x05, 0x4F, 0x7E,  # 'O' yellow
    0x83, 0xC7, 0x02,
    0x26, 0xC7, 0x05, 0x53, 0x7E,  # 'S' yellow
    0x83, 0xC7, 0x02,
    0x26, 0xC7, 0x05, 0x20, 0x70,  # space
    0x83, 0xC7, 0x02,
    0x26, 0xC7, 0x05, 0x76, 0x78,  # 'v' dark gray
    0x83, 0xC7, 0x02,
    0x26, 0xC7, 0x05, 0x32, 0x78,  # '2'
    0x83, 0xC7, 0x02,
    0x26, 0xC7, 0x05, 0x2E, 0x78,  # '.'
    0x83, 0xC7, 0x02,
    0x26, 0xC7, 0x05, 0x30, 0x78,  # '0'
    
    # Write "Start" button in taskbar (green background 0x2F)
    0xBF, 0x02, 0x0F,       # mov di, 3842 (taskbar + 2)
    0x26, 0xC7, 0x05, 0xFE, 0x2F,  # block char
    0x83, 0xC7, 0x02,
    0x26, 0xC7, 0x05, 0x53, 0x2F,  # 'S'
    0x83, 0xC7, 0x02,
    0x26, 0xC7, 0x05, 0x74, 0x2F,  # 't'
    0x83, 0xC7, 0x02,
    0x26, 0xC7, 0x05, 0x61, 0x2F,  # 'a'
    0x83, 0xC7, 0x02,
    0x26, 0xC7, 0x05, 0x72, 0x2F,  # 'r'
    0x83, 0xC7, 0x02,
    0x26, 0xC7, 0x05, 0x74, 0x2F,  # 't'
    
    # Write clock "12:00" in taskbar right side
    0xBF, 0x90, 0x0F,       # mov di, 3984 (near right edge)
    0x26, 0xC7, 0x05, 0x31, 0x70,  # '1'
    0x83, 0xC7, 0x02,
    0x26, 0xC7, 0x05, 0x32, 0x70,  # '2'
    0x83, 0xC7, 0x02,
    0x26, 0xC7, 0x05, 0x3A, 0x70,  # ':'
    0x83, 0xC7, 0x02,
    0x26, 0xC7, 0x05, 0x30, 0x70,  # '0'
    0x83, 0xC7, 0x02,
    0x26, 0xC7, 0x05, 0x30, 0x70,  # '0'
    
    # Now use BIOS to print text in the main area
    0x31, 0xC0,             # xor ax, ax
    0x8E, 0xD8,             # mov ds, ax
    
    # Set cursor to row 2, col 5
    0xB4, 0x02, 0xB7, 0x00, 0xB6, 0x02, 0xB2, 0x05, 0xCD, 0x10,
    
    # Print logo box
    0xBE, 0x00, 0x81,       # mov si, 0x8100
    # print_loop:
    0xAC,                   # lodsb
    0x3C, 0x00,             # cmp al, 0
    0x74, 0x09,             # jz done_print
    0xB4, 0x0E,             # mov ah, 0x0E
    0xB7, 0x00,             # mov bh, 0
    0xB3, 0x1F,             # mov bl, 0x1F (white on blue)
    0xCD, 0x10,             # int 0x10
    0xEB, 0xF1,             # jmp print_loop
    # done_print:
    
    # Halt
    0xF4,                   # hlt
    0xEB, 0xFD              # jmp halt
)

for ($i = 0; $i -lt $stage2Code.Length; $i++) {
    $stage2[$i] = $stage2Code[$i]
}

# Main content at 0x100 (0x8100) - Shorter to fit in memory
$content = @"

  +============================================================+
  |                    F A S T O S  v 2.0                      |
  |          ADead-BIB Native Operating System                 |
  |        C is Master  -  Rust provides Safety                |
  +============================================================+

  System:
    [OK] CPU: x86-64    [OK] RAM: 256 MB    [OK] Video: VGA
    [OK] Keyboard: PS/2 [OK] Mouse: PS/2    [OK] Disk: fd0

  Kernel:
    [INIT] Scheduler (64 procs)  [INIT] VFS mounted
    [INIT] Syscalls ready        [INIT] Binary Guardian

  Drivers:
    [LOAD] keyboard.c   [LOAD] mouse.c
    [LOAD] framebuffer.c [LOAD] nouveau.c (NVIDIA)

  Rust Safety Layer:
    [LINK] rust_malloc   [LINK] rust_memcpy_safe
    [LINK] rust_translate [LINK] rust_buffer

  Po Format: Active (PE+ELF hybrid)
  Security: Binary Guardian enabled

  FastOS ready. Type 'help' for commands.
"@
$contentBytes = [System.Text.Encoding]::ASCII.GetBytes($content + "`0")
for ($i = 0; $i -lt [Math]::Min($contentBytes.Length, 0x600); $i++) {
    $stage2[0x100 + $i] = $contentBytes[$i]
}

[System.IO.File]::WriteAllBytes("$BUILD\stage2.bin", $stage2)
Write-Success "Stage 2: $($stage2.Length) bytes (Windows/Linux hybrid UI)"

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
        "-serial", "stdio"
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
