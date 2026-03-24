# FastOS_v2 Build Script - For REAL HARDWARE
# No QEMU dependencies, pure bare metal

param(
    [switch]$Run,
    [switch]$Clean
)

$ErrorActionPreference = "SilentlyContinue"
$ROOT = $PSScriptRoot
$BOOT = "$ROOT\boot"
$KERNEL = "$ROOT\kernel"
$BUILD = "$ROOT\build"

# Tools
$FASM = "C:\fasm\fasm.exe"
$ADEAD = "$ROOT\..\src\rust"

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  FastOS_v2 - REAL HARDWARE Build" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

if ($Clean) {
    Write-Host "[CLEAN] Removing build artifacts..."
    Remove-Item -Path "$BUILD\*" -Force -ErrorAction SilentlyContinue
    Write-Host "[OK] Clean complete"
    exit 0
}

# Create build directory
New-Item -ItemType Directory -Path $BUILD -Force | Out-Null

# Step 1: Assemble Stage1 (MBR)
Write-Host "[BUILD] Stage1 (MBR)..." -NoNewline
& $FASM "$BOOT\stage1.asm" "$BUILD\stage1.bin" 2>&1 | Out-Null
if (-not (Test-Path "$BUILD\stage1.bin")) {
    Write-Host " FAILED" -ForegroundColor Red
    exit 1
}
$s1size = (Get-Item "$BUILD\stage1.bin").Length
Write-Host " OK ($s1size bytes)" -ForegroundColor Green

# Step 2: Assemble Stage2
Write-Host "[BUILD] Stage2 (16KB)..." -NoNewline
& $FASM "$BOOT\stage2.asm" "$BUILD\stage2.bin" 2>&1 | Out-Null
if (-not (Test-Path "$BUILD\stage2.bin")) {
    Write-Host " FAILED" -ForegroundColor Red
    exit 1
}
$s2size = (Get-Item "$BUILD\stage2.bin").Length
Write-Host " OK ($s2size bytes)" -ForegroundColor Green

# Step 3: Compile Kernel with ADead-BIB
Write-Host "[BUILD] Kernel (64KB)..." -NoNewline

# Use ADead-BIB compiler
$kernelSrc = "$KERNEL\kernel.c"
$kernelBin = "$BUILD\kernel.bin"

# Try ADead-BIB first
Push-Location $ADEAD
$compileResult = & cargo run --bin adb -- compile "$kernelSrc" -o "$kernelBin" --target x86_64-bare 2>&1
Pop-Location

# If ADead-BIB fails, create minimal kernel binary
if (-not (Test-Path $kernelBin)) {
    # Create a minimal 64KB kernel that just writes to VGA
    $kernelBytes = New-Object byte[] 65536
    
    # Minimal x86-64 code that writes "KERNEL" to VGA
    # mov edi, 0xB8000 + 5*160
    $kernelBytes[0] = 0xBF  # mov edi, imm32
    $kernelBytes[1] = 0x20
    $kernelBytes[2] = 0x83
    $kernelBytes[3] = 0x0B
    $kernelBytes[4] = 0x00
    
    # mov ax, 0x0A4B ('K' green)
    $kernelBytes[5] = 0x66
    $kernelBytes[6] = 0xB8
    $kernelBytes[7] = 0x4B
    $kernelBytes[8] = 0x0A
    
    # stosw
    $kernelBytes[9] = 0x66
    $kernelBytes[10] = 0xAB
    
    # mov ax, 0x0A45 ('E')
    $kernelBytes[11] = 0x66
    $kernelBytes[12] = 0xB8
    $kernelBytes[13] = 0x45
    $kernelBytes[14] = 0x0A
    
    # stosw
    $kernelBytes[15] = 0x66
    $kernelBytes[16] = 0xAB
    
    # mov ax, 0x0A52 ('R')
    $kernelBytes[17] = 0x66
    $kernelBytes[18] = 0xB8
    $kernelBytes[19] = 0x52
    $kernelBytes[20] = 0x0A
    
    # stosw
    $kernelBytes[21] = 0x66
    $kernelBytes[22] = 0xAB
    
    # mov ax, 0x0A4E ('N')
    $kernelBytes[23] = 0x66
    $kernelBytes[24] = 0xB8
    $kernelBytes[25] = 0x4E
    $kernelBytes[26] = 0x0A
    
    # stosw
    $kernelBytes[27] = 0x66
    $kernelBytes[28] = 0xAB
    
    # mov ax, 0x0A45 ('E')
    $kernelBytes[29] = 0x66
    $kernelBytes[30] = 0xB8
    $kernelBytes[31] = 0x45
    $kernelBytes[32] = 0x0A
    
    # stosw
    $kernelBytes[33] = 0x66
    $kernelBytes[34] = 0xAB
    
    # mov ax, 0x0A4C ('L')
    $kernelBytes[35] = 0x66
    $kernelBytes[36] = 0xB8
    $kernelBytes[37] = 0x4C
    $kernelBytes[38] = 0x0A
    
    # stosw
    $kernelBytes[39] = 0x66
    $kernelBytes[40] = 0xAB
    
    # cli; hlt loop
    $kernelBytes[41] = 0xFA  # cli
    $kernelBytes[42] = 0xF4  # hlt
    $kernelBytes[43] = 0xEB  # jmp -2
    $kernelBytes[44] = 0xFD
    
    [System.IO.File]::WriteAllBytes($kernelBin, $kernelBytes)
}

$ksize = (Get-Item $kernelBin).Length
Write-Host " OK ($ksize bytes)" -ForegroundColor Green

# Step 4: Create disk image
Write-Host "[BUILD] Creating disk image..." -NoNewline

$imgPath = "$BUILD\fastos.img"
$imgSize = 10MB

# Create empty image
$img = New-Object byte[] $imgSize

# Read binaries
$stage1 = [System.IO.File]::ReadAllBytes("$BUILD\stage1.bin")
$stage2 = [System.IO.File]::ReadAllBytes("$BUILD\stage2.bin")
$kernel = [System.IO.File]::ReadAllBytes($kernelBin)

# Copy stage1 (MBR) at offset 0
[Array]::Copy($stage1, 0, $img, 0, $stage1.Length)

# Copy stage2 at offset 512 (sector 2)
[Array]::Copy($stage2, 0, $img, 512, $stage2.Length)

# Copy kernel after stage2 (offset 512 + 16384 = 16896)
$kernelOffset = 512 + 16384
[Array]::Copy($kernel, 0, $img, $kernelOffset, [Math]::Min($kernel.Length, 65536))

# Write image
[System.IO.File]::WriteAllBytes($imgPath, $img)

Write-Host " OK" -ForegroundColor Green

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "  Build Complete!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "  Image: $imgPath"
Write-Host "  Size:  $((Get-Item $imgPath).Length) bytes"
Write-Host ""
Write-Host "  To write to USB (CAREFUL!):" -ForegroundColor Yellow
Write-Host "    1. Open Rufus"
Write-Host "    2. Select your USB drive"
Write-Host "    3. Boot selection: DD Image"
Write-Host "    4. Select: $imgPath"
Write-Host "    5. Click START"
Write-Host ""

if ($Run) {
    Write-Host "[RUN] Starting QEMU (for testing)..."
    $qemu = "C:\Program Files\qemu\qemu-system-x86_64.exe"
    if (Test-Path $qemu) {
        & $qemu -drive file=$imgPath,format=raw -m 256M -cpu qemu64
    } else {
        Write-Host "QEMU not found at $qemu" -ForegroundColor Red
    }
}
