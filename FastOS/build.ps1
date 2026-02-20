# ============================================================
# FastOS — Build Script (PowerShell)
# ============================================================
# Builds the complete FastOS image:
#   1. ADead-BIB compiler (if needed)
#   2. Stage1 boot sector (512 bytes)
#   3. Stage2 bootloader (flat binary)
#   4. Rust kernel (bare-metal x86_64)
#   5. Combine into fastos.bin disk image
#
# Usage:
#   .\build.ps1          # Build only
#   .\build.ps1 -Run     # Build + launch QEMU
# ============================================================

param(
    [switch]$Run,
    [switch]$Clean
)

$ErrorActionPreference = "Continue"
$ProjectRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$ADeadRoot = Split-Path -Parent $ProjectRoot
$BuildDir = Join-Path $ProjectRoot "build"
$QEMU = "C:\Program Files\qemu\qemu-system-x86_64.exe"

Write-Host "============================================" -ForegroundColor Green
Write-Host "  FastOS Build System" -ForegroundColor Green
Write-Host "  ADead-BIB + Rust + C = FastOS" -ForegroundColor Green
Write-Host "  Format: FsOS (not PE, not ELF)" -ForegroundColor Green
Write-Host "============================================" -ForegroundColor Green
Write-Host ""

# Clean
if ($Clean) {
    Write-Host "[CLEAN] Removing build directory..." -ForegroundColor Yellow
    if (Test-Path $BuildDir) { Remove-Item -Recurse -Force $BuildDir }
}

# Create build dir
if (-not (Test-Path $BuildDir)) {
    New-Item -ItemType Directory -Path $BuildDir -Force | Out-Null
}

# ============================================================
# Step 1: Build ADead-BIB compiler
# ============================================================
Write-Host "[1/4] Building ADead-BIB compiler..." -ForegroundColor Cyan
Push-Location $ADeadRoot
try {
    $buildOutput = cargo build --release 2>&1
    if ($LASTEXITCODE -ne 0) {
        # cargo writes to stderr even on success (warnings), check for actual errors
        $errors = $buildOutput | Select-String "^error\[" 
        if ($errors) {
            Write-Host "  FAILED: ADead-BIB compiler build error" -ForegroundColor Red
            $errors | ForEach-Object { Write-Host "  $_" -ForegroundColor Red }
            exit 1
        }
    }
    Write-Host "  OK" -ForegroundColor Green
} finally {
    Pop-Location
}

$ADeadCompiler = Join-Path $ADeadRoot "target\release\adeadc.exe"
if (-not (Test-Path $ADeadCompiler)) {
    $ADeadCompiler = Join-Path $ADeadRoot "target\debug\adeadc.exe"
}

# ============================================================
# Step 2: Compile Stage1 boot sector
# ============================================================
Write-Host "[2/4] Compiling Stage1 boot sector..." -ForegroundColor Cyan
$Stage1Src = Join-Path $ProjectRoot "boot\stage1.adB"
$Stage1Bin = Join-Path $BuildDir "stage1.bin"

Push-Location $ADeadRoot
try {
    & cargo run -- boot $Stage1Src -o $Stage1Bin 2>&1 | Out-Null
    if (-not (Test-Path $Stage1Bin)) {
        Write-Host "  FAILED: stage1.bin not created" -ForegroundColor Red
        exit 1
    }
    $size = (Get-Item $Stage1Bin).Length
    Write-Host "  OK ($size bytes)" -ForegroundColor Green
} finally {
    Pop-Location
}

# ============================================================
# Step 3: Compile Stage2 bootloader (v2 with labels)
# ============================================================
Write-Host "[3/4] Compiling Stage2 bootloader (v2 with labels)..." -ForegroundColor Cyan
$Stage2Src = Join-Path $ProjectRoot "boot\stage2_v2.adB"
$Stage2Bin = Join-Path $BuildDir "stage2.bin"

Push-Location $ADeadRoot
try {
    & cargo run -- flat $Stage2Src -o $Stage2Bin 2>&1 | Out-Null
    if (-not (Test-Path $Stage2Bin)) {
        Write-Host "  FAILED: stage2.bin not created" -ForegroundColor Red
        exit 1
    }
    $size = (Get-Item $Stage2Bin).Length
    Write-Host "  OK ($size bytes)" -ForegroundColor Green
} finally {
    Pop-Location
}

# ============================================================
# Step 4: Build Rust kernel
# ============================================================
Write-Host "[4/5] Building Rust kernel..." -ForegroundColor Cyan
$KernelDir = Join-Path $ProjectRoot "kernel"
$KernelBin = Join-Path $BuildDir "kernel.bin"

Push-Location $KernelDir
try {
    # Build kernel with custom target
    $kernelOutput = cargo build --release --target x86_64-fastos.json "-Zbuild-std=core,compiler_builtins" "-Zbuild-std-features=compiler-builtins-mem" 2>&1
    if ($LASTEXITCODE -ne 0) {
        $errors = $kernelOutput | Select-String "^error"
        if ($errors) {
            Write-Host "  FAILED: Kernel build error" -ForegroundColor Red
            $errors | ForEach-Object { Write-Host "  $_" -ForegroundColor Red }
            # Continue without kernel for now (VGA text mode fallback)
            Write-Host "  WARNING: Continuing without kernel binary" -ForegroundColor Yellow
            $KernelBin = $null
        }
    }

    if ($KernelBin) {
        # Find the built ELF and convert to flat binary
        $KernelElf = Join-Path $KernelDir "target\x86_64-fastos\release\fastos-kernel"
        if (Test-Path $KernelElf) {
            # Use objcopy to create flat binary (if available)
            $objcopy = Get-Command "rust-objcopy" -ErrorAction SilentlyContinue
            if (-not $objcopy) {
                $objcopy = Get-Command "llvm-objcopy" -ErrorAction SilentlyContinue
            }
            if ($objcopy) {
                & $objcopy.Source -O binary $KernelElf $KernelBin
                $size = (Get-Item $KernelBin).Length
                Write-Host "  OK ($size bytes)" -ForegroundColor Green
            } else {
                Write-Host "  WARNING: objcopy not found, copying ELF directly" -ForegroundColor Yellow
                Copy-Item $KernelElf $KernelBin
                $size = (Get-Item $KernelBin).Length
                Write-Host "  OK ($size bytes, ELF format)" -ForegroundColor Green
            }
        } else {
            Write-Host "  WARNING: Kernel ELF not found at $KernelElf" -ForegroundColor Yellow
            $KernelBin = $null
        }
    }
} finally {
    Pop-Location
}

# ============================================================
# Step 5: Combine into disk image
# ============================================================
Write-Host "[5/5] Creating FastOS disk image..." -ForegroundColor Cyan
$FastOSBin = Join-Path $BuildDir "fastos.bin"

# Read stage1 (512 bytes boot sector)
$stage1Bytes = [System.IO.File]::ReadAllBytes($Stage1Bin)

# Read stage2
$stage2Bytes = [System.IO.File]::ReadAllBytes($Stage2Bin)

# Pad stage2 to sector boundary (512 bytes)
$stage2Padded = New-Object byte[] ([Math]::Ceiling($stage2Bytes.Length / 512) * 512)
[Array]::Copy($stage2Bytes, $stage2Padded, $stage2Bytes.Length)

# Read kernel (if available)
$kernelBytes = $null
$kernelPadded = @()
if ($KernelBin -and (Test-Path $KernelBin)) {
    $kernelBytes = [System.IO.File]::ReadAllBytes($KernelBin)
    $kernelPadded = New-Object byte[] ([Math]::Ceiling($kernelBytes.Length / 512) * 512)
    [Array]::Copy($kernelBytes, $kernelPadded, $kernelBytes.Length)
}

# Combine: stage1 + stage2 + kernel
$totalLen = $stage1Bytes.Length + $stage2Padded.Length + $kernelPadded.Length
# Minimum 1.44MB floppy image for QEMU compatibility
$imageSize = [Math]::Max($totalLen, 1474560)
$image = New-Object byte[] $imageSize
[Array]::Copy($stage1Bytes, 0, $image, 0, $stage1Bytes.Length)
[Array]::Copy($stage2Padded, 0, $image, $stage1Bytes.Length, $stage2Padded.Length)
if ($kernelPadded.Length -gt 0) {
    [Array]::Copy($kernelPadded, 0, $image, $stage1Bytes.Length + $stage2Padded.Length, $kernelPadded.Length)
}

[System.IO.File]::WriteAllBytes($FastOSBin, $image)
$totalSize = $image.Length
Write-Host "  OK ($totalSize bytes)" -ForegroundColor Green

# ============================================================
# Summary
# ============================================================
Write-Host ""
Write-Host "============================================" -ForegroundColor Green
Write-Host "  FastOS Build Complete!" -ForegroundColor Green
Write-Host "============================================" -ForegroundColor Green
Write-Host "  Stage1:  $($stage1Bytes.Length) bytes (boot sector)" -ForegroundColor White
Write-Host "  Stage2:  $($stage2Bytes.Length) bytes (mode switch)" -ForegroundColor White
if ($kernelBytes) {
    Write-Host "  Kernel:  $($kernelBytes.Length) bytes (Rust x86_64)" -ForegroundColor White
} else {
    Write-Host "  Kernel:  (not included - VGA text mode only)" -ForegroundColor Yellow
}
Write-Host "  Image:   $totalSize bytes total" -ForegroundColor White
Write-Host "  Output:  $FastOSBin" -ForegroundColor White
Write-Host "  Sig:     0x$('{0:X2}' -f $stage1Bytes[510])$('{0:X2}' -f $stage1Bytes[511])" -ForegroundColor White
Write-Host ""

# ============================================================
# Run in QEMU (optional)
# ============================================================
if ($Run) {
    if (Test-Path $QEMU) {
        Write-Host "Launching QEMU..." -ForegroundColor Cyan
        & $QEMU -drive "format=raw,file=$FastOSBin" `
                -m 256M `
                -vga std `
                -serial stdio `
                -no-reboot -no-shutdown
    } else {
        Write-Host "QEMU not found at: $QEMU" -ForegroundColor Red
        Write-Host "Install QEMU or update the path in build.ps1" -ForegroundColor Yellow
    }
} else {
    Write-Host "To test: .\build.ps1 -Run" -ForegroundColor Yellow
    Write-Host "Or:      & `"$QEMU`" -drive format=raw,file=$FastOSBin -m 256M" -ForegroundColor Yellow
}
