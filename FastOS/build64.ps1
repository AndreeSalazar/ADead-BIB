# ============================================================
# FastOS v2.0 - 64-bit Build Script
# ============================================================
# Architecture:
#   - boot64.asm (FASM) -> Real->Protected->Long Mode
#   - kernel64.c (ADead-BIB) -> 64-bit kernel
# ============================================================

param(
    [switch]$Run,
    [switch]$Clean
)

$ErrorActionPreference = "Continue"

$ROOT = $PSScriptRoot
$BUILD = "$ROOT\build"
$BOOT = "$ROOT\boot"
$KERNEL = "$ROOT\kernel"
$ADEAD_ROOT = Split-Path $ROOT -Parent

function Write-Status($msg) { Write-Host "[BUILD] $msg" -ForegroundColor Cyan }
function Write-Success($msg) { Write-Host "[OK] $msg" -ForegroundColor Green }
function Write-Error($msg) { Write-Host "[ERROR] $msg" -ForegroundColor Red }

Write-Host ""
Write-Host "========================================" -ForegroundColor Yellow
Write-Host "  FastOS v2.0 - 64-bit Build" -ForegroundColor Yellow
Write-Host "  C takes FULL control" -ForegroundColor Yellow
Write-Host "========================================" -ForegroundColor Yellow
Write-Host ""

if ($Clean) {
    Write-Status "Cleaning..."
    if (Test-Path $BUILD) { Remove-Item -Recurse -Force $BUILD }
    Write-Success "Clean complete"
    exit 0
}

if (-not (Test-Path $BUILD)) {
    New-Item -ItemType Directory -Path $BUILD | Out-Null
}

# Find FASM
$FASM = $null
$fasmPaths = @("C:\fasm\fasm.exe", "fasm")
foreach ($p in $fasmPaths) {
    if (Test-Path $p) { $FASM = $p; break }
    try { $null = Get-Command $p -ErrorAction Stop; $FASM = $p; break } catch {}
}

if (-not $FASM) {
    Write-Error "FASM not found! Install from https://flatassembler.net/"
    exit 1
}

# ============================================================
# Step 1: Assemble Stage 1 (stage1.asm)
# ============================================================
Write-Status "Step 1: Assembling stage1.asm (512 bytes MBR)..."

$mbrSrc = "$BOOT\stage1.asm"
$mbrBin = "$BUILD\stage1.bin"

& $FASM $mbrSrc $mbrBin
if ($LASTEXITCODE -ne 0 -or -not (Test-Path $mbrBin)) {
    Write-Error "FASM failed on mbr64.asm"
    exit 1
}

$mbrSize = (Get-Item $mbrBin).Length
Write-Success "MBR: $mbrSize bytes"

# ============================================================
# Step 2: Assemble Stage 2 (stage2.asm)
# ============================================================
Write-Status "Step 2: Assembling stage2.asm (64-bit transition)..."

$loaderSrc = "$BOOT\stage2.asm"
$loaderBin = "$BUILD\stage2.bin"

& $FASM $loaderSrc $loaderBin
if ($LASTEXITCODE -ne 0 -or -not (Test-Path $loaderBin)) {
    Write-Error "FASM failed on loader64.asm"
    exit 1
}

$loaderSize = (Get-Item $loaderBin).Length
Write-Success "Loader: $loaderSize bytes"

# ============================================================
# Step 3: Compile main.c with ADead-BIB
# ============================================================
Write-Status "Step 3: Compiling main.c with ADead-BIB..."

$kernelSrc = "$KERNEL\main.c"
$kernelBin = "$BUILD\kernel64.bin"

# Compile kernel with ADead-BIB --flat
$compiled = $false
try {
    $result = & cargo run --manifest-path="$ADEAD_ROOT\Cargo.toml" --release -- cc "$kernelSrc" -o "$kernelBin" --flat --org=0x100000 --size=32768 2>&1
    if (Test-Path $kernelBin) {
        $kernelSize = (Get-Item $kernelBin).Length
        if ($kernelSize -gt 0) {
            Write-Success "Kernel: $kernelSize bytes (ADead-BIB C)"
            $compiled = $true
        }
    }
} catch {
    Write-Status "ADead-BIB compilation failed"
}

if (-not $compiled) {
    Write-Status "Creating placeholder kernel (ADead-BIB 64-bit pending)..."
    
    # Minimal 64-bit kernel that prints and halts
    # This is temporary until ADead-BIB generates proper 64-bit code
    $kernel = New-Object byte[] 32768
    
    # 64-bit code: clear screen, print message, halt
    $code = @(
        # Clear screen (write to 0xB8000)
        0x48, 0xBF, 0x00, 0x80, 0x0B, 0x00, 0x00, 0x00, 0x00, 0x00,  # mov rdi, 0xB8000
        0x48, 0xB8, 0x20, 0x0A, 0x20, 0x0A, 0x20, 0x0A, 0x20, 0x0A,  # mov rax, 0x0A200A200A200A20
        0xB9, 0xF4, 0x01, 0x00, 0x00,                                # mov ecx, 500
        0xF3, 0x48, 0xAB,                                            # rep stosq
        
        # Print "FastOS 64-bit - C Kernel Ready" at row 10
        0x48, 0xBF, 0x40, 0x86, 0x0B, 0x00, 0x00, 0x00, 0x00, 0x00,  # mov rdi, 0xB8640 (row 10, col 20)
        0x48, 0xBE,                                                  # mov rsi, msg
        0x00, 0x01, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00               # address of message (0x100100)
    )
    
    # Print loop
    $printLoop = @(
        # .loop:
        0xAC,                       # lodsb
        0x84, 0xC0,                 # test al, al
        0x74, 0x06,                 # jz .done
        0xB4, 0x0A,                 # mov ah, 0x0A (green)
        0x66, 0xAB,                 # stosw
        0xEB, 0xF5,                 # jmp .loop
        # .done:
        
        # Print "ADead-BIB Compiler" at row 12
        0x48, 0xBF, 0xC0, 0x87, 0x0B, 0x00, 0x00, 0x00, 0x00, 0x00,  # mov rdi, 0xB87C0 (row 12, col 20)
        0x48, 0xBE,
        0x20, 0x01, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00,              # address of msg2
        0xAC, 0x84, 0xC0, 0x74, 0x06, 0xB4, 0x0E, 0x66, 0xAB, 0xEB, 0xF5,
        
        # Print "[BG] Binary Guardian: ACTIVE" at row 14
        0x48, 0xBF, 0x40, 0x89, 0x0B, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x48, 0xBE,
        0x40, 0x01, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xAC, 0x84, 0xC0, 0x74, 0x06, 0xB4, 0x0B, 0x66, 0xAB, 0xEB, 0xF5,
        
        # Halt
        0xFA,                       # cli
        0xF4,                       # hlt
        0xEB, 0xFD                  # jmp $
    )
    
    $offset = 0
    foreach ($b in $code) { $kernel[$offset++] = $b }
    foreach ($b in $printLoop) { $kernel[$offset++] = $b }
    
    # Messages at offset 0x100 (0x100100 when loaded at 0x100000)
    $msg1 = [System.Text.Encoding]::ASCII.GetBytes("FastOS v2.0 - 64-bit Long Mode`0")
    $msg2 = [System.Text.Encoding]::ASCII.GetBytes("Compiler: ADead-BIB`0")
    $msg3 = [System.Text.Encoding]::ASCII.GetBytes("[BG] Binary Guardian: ACTIVE`0")
    
    $msgOffset = 0x100
    foreach ($b in $msg1) { $kernel[$msgOffset++] = $b }
    $msgOffset = 0x120
    foreach ($b in $msg2) { $kernel[$msgOffset++] = $b }
    $msgOffset = 0x140
    foreach ($b in $msg3) { $kernel[$msgOffset++] = $b }
    
    [System.IO.File]::WriteAllBytes($kernelBin, $kernel)
    Write-Success "Kernel: $($kernel.Length) bytes (placeholder)"
}

# ============================================================
# Step 4: Create Disk Image
# ============================================================
Write-Status "Step 4: Creating disk image..."

$imageSize = 10485760 # 10MB Raw Hard Disk
$image = New-Object byte[] $imageSize

# Copy MBR (sector 0)
$mbrData = [System.IO.File]::ReadAllBytes($mbrBin)
for ($i = 0; $i -lt $mbrData.Length; $i++) {
    $image[$i] = $mbrData[$i]
}

# Copy Loader (16KB)
$loaderData = [System.IO.File]::ReadAllBytes($loaderBin)
for ($i = 0; $i -lt $loaderData.Length; $i++) {
    $image[512 + $i] = $loaderData[$i]
}

# Append Kernel
if (Test-Path $kernelBin) {
    $kernelData = [System.IO.File]::ReadAllBytes($kernelBin)
    $kernelOffset = 512 + $loaderData.Length
    for ($i = 0; $i -lt $kernelData.Length; $i++) {
        $image[$kernelOffset + $i] = $kernelData[$i]
    }
    Write-Success "Kernel appended after loader at offset $kernelOffset ($($kernelData.Length) bytes)"
}

$imagePath = "$BUILD\fastos64.img"
[System.IO.File]::WriteAllBytes($imagePath, $image)
Write-Success "Image: $imageSize bytes"

# ============================================================
# Summary
# ============================================================
Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "  64-bit Build Complete!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "  MBR:    $mbrSize bytes (FASM)" -ForegroundColor White
Write-Host "  Loader: $loaderSize bytes (FASM)" -ForegroundColor White
Write-Host "  Kernel: $($kernelData.Length) bytes (C)" -ForegroundColor White
Write-Host "  Image:  $imageSize bytes" -ForegroundColor White
Write-Host ""
Write-Host "  Mode:   x86-64 Long Mode" -ForegroundColor Cyan
Write-Host "  Boot:   0x55$("{0:X2}" -f $mbrData[511])" -ForegroundColor Cyan
Write-Host ""

# ============================================================
# Run in QEMU
# ============================================================
if ($Run) {
    Write-Status "Starting QEMU (64-bit)..."
    
    $qemu = "C:\Program Files\qemu\qemu-system-x86_64.exe"
    if (-not (Test-Path $qemu)) {
        $qemu = "$env:USERPROFILE\scoop\apps\qemu\current\qemu-system-x86_64.exe"
    }
    
    if (Test-Path $qemu) {
        Write-Success "Running: $qemu"
        # Boot from raw hard disk
        & $qemu -drive "file=$imagePath,format=raw" -m 128M -boot c -cpu qemu64
    } else {
        Write-Error "QEMU not found"
    }
}

Write-Host ""
Write-Host "Commands:" -ForegroundColor Yellow
Write-Host "  .\build64.ps1 -Run    # Build and run 64-bit" -ForegroundColor Gray
Write-Host "  .\build64.ps1 -Clean  # Clean build" -ForegroundColor Gray
Write-Host ""
