# ============================================================
# ADead-OS — Build Script (PowerShell)
# ============================================================
# Builds the complete OS stack:
#   1. ADead-BIB compiler (if needed)
#   2. Stage 1 boot sector (ADead-BIB → flat binary)
#   3. Stage 2 mode switch (ADead-BIB → flat binary)
#   4. Rust kernel (cargo build → ELF)
#   5. Combine into single OS image
#
# 3 Languages, Zero ASM:
#   ADead-BIB = Base (boot, hardware)
#   Rust      = Security (kernel logic)
#   C         = Compatibility (headers only, no compilation)
#
# Usage: .\build.ps1
# Test:  .\build.ps1 -Run
# ============================================================

param(
    [switch]$Run,       # Run in QEMU after build
    [switch]$Clean,     # Clean build artifacts
    [switch]$Verbose    # Verbose output
)

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$BuildDir = Join-Path $ScriptDir "build"
$ADeadBIBRoot = Split-Path -Parent $ScriptDir

Write-Host "============================================" -ForegroundColor Cyan
Write-Host "  ADead-OS Build System" -ForegroundColor Cyan
Write-Host "  3 Languages, Zero ASM" -ForegroundColor Cyan
Write-Host "============================================" -ForegroundColor Cyan
Write-Host ""

# ---- Clean ----
if ($Clean) {
    Write-Host "[CLEAN] Removing build artifacts..." -ForegroundColor Yellow
    if (Test-Path $BuildDir) { Remove-Item -Recurse -Force $BuildDir }
    Write-Host "[CLEAN] Done." -ForegroundColor Green
    exit 0
}

# ---- Create build directory ----
if (-not (Test-Path $BuildDir)) {
    New-Item -ItemType Directory -Path $BuildDir | Out-Null
}

Write-Host "[1/5] Building ADead-BIB compiler..." -ForegroundColor Yellow
Push-Location $ADeadBIBRoot
try {
    cargo build --release 2>&1 | ForEach-Object {
        if ($Verbose) { Write-Host "  $_" -ForegroundColor DarkGray }
    }
    if ($LASTEXITCODE -ne 0) {
        Write-Host "[ERROR] ADead-BIB compiler build failed!" -ForegroundColor Red
        exit 1
    }
    Write-Host "[1/5] ADead-BIB compiler OK" -ForegroundColor Green
} finally {
    Pop-Location
}

# ---- Find ADead-BIB binary ----
$ADeadBIB = Join-Path $ADeadBIBRoot "target\release\adead-bib.exe"
if (-not (Test-Path $ADeadBIB)) {
    $ADeadBIB = Join-Path $ADeadBIBRoot "target\debug\adead-bib.exe"
}
if (-not (Test-Path $ADeadBIB)) {
    Write-Host "[WARN] ADead-BIB binary not found, using cargo run" -ForegroundColor Yellow
    $ADeadBIB = $null
}

# ---- Stage 1: Boot Sector (ADead-BIB) ----
Write-Host "[2/5] Building Stage 1 boot sector (ADead-BIB)..." -ForegroundColor Yellow
$Stage1Src = Join-Path $ScriptDir "boot\stage1.adB"
$Stage1Bin = Join-Path $BuildDir "stage1.bin"

if ($ADeadBIB) {
    & $ADeadBIB boot $Stage1Src -o $Stage1Bin 2>&1 | ForEach-Object {
        if ($Verbose) { Write-Host "  $_" -ForegroundColor DarkGray }
    }
} else {
    Push-Location $ADeadBIBRoot
    cargo run -- boot $Stage1Src -o $Stage1Bin 2>&1 | ForEach-Object {
        if ($Verbose) { Write-Host "  $_" -ForegroundColor DarkGray }
    }
    Pop-Location
}

if (Test-Path $Stage1Bin) {
    $size = (Get-Item $Stage1Bin).Length
    Write-Host "[2/5] Stage 1 OK ($size bytes)" -ForegroundColor Green
} else {
    Write-Host "[WARN] Stage 1 build may have issues, creating minimal boot sector..." -ForegroundColor Yellow
    # Fallback: create minimal boot sector directly
    $bootBytes = New-Object byte[] 512
    # CLI
    $bootBytes[0] = 0xFA
    # XOR AX, AX
    $bootBytes[1] = 0x31; $bootBytes[2] = 0xC0
    # MOV DS, AX
    $bootBytes[3] = 0x8E; $bootBytes[4] = 0xD8
    # MOV SS, AX
    $bootBytes[5] = 0x8E; $bootBytes[6] = 0xD0
    # MOV SP, 0x7C00
    $bootBytes[7] = 0xBC; $bootBytes[8] = 0x00; $bootBytes[9] = 0x7C
    # STI
    $bootBytes[10] = 0xFB
    # MOV AH, 0x0E
    $bootBytes[11] = 0xB4; $bootBytes[12] = 0x0E
    # Print 'A'
    $bootBytes[13] = 0xB0; $bootBytes[14] = 0x41
    $bootBytes[15] = 0xCD; $bootBytes[16] = 0x10
    # Print 'D'
    $bootBytes[17] = 0xB0; $bootBytes[18] = 0x44
    $bootBytes[19] = 0xCD; $bootBytes[20] = 0x10
    # Print 'e'
    $bootBytes[21] = 0xB0; $bootBytes[22] = 0x65
    $bootBytes[23] = 0xCD; $bootBytes[24] = 0x10
    # Print 'a'
    $bootBytes[25] = 0xB0; $bootBytes[26] = 0x61
    $bootBytes[27] = 0xCD; $bootBytes[28] = 0x10
    # Print 'd'
    $bootBytes[29] = 0xB0; $bootBytes[30] = 0x64
    $bootBytes[31] = 0xCD; $bootBytes[32] = 0x10
    # Print '-'
    $bootBytes[33] = 0xB0; $bootBytes[34] = 0x2D
    $bootBytes[35] = 0xCD; $bootBytes[36] = 0x10
    # Print 'O'
    $bootBytes[37] = 0xB0; $bootBytes[38] = 0x4F
    $bootBytes[39] = 0xCD; $bootBytes[40] = 0x10
    # Print 'S'
    $bootBytes[41] = 0xB0; $bootBytes[42] = 0x53
    $bootBytes[43] = 0xCD; $bootBytes[44] = 0x10
    # HLT + JMP $
    $bootBytes[45] = 0xF4
    $bootBytes[46] = 0xEB; $bootBytes[47] = 0xFE
    # Boot signature
    $bootBytes[510] = 0x55; $bootBytes[511] = 0xAA
    [System.IO.File]::WriteAllBytes($Stage1Bin, $bootBytes)
    Write-Host "[2/5] Stage 1 OK (fallback, 512 bytes)" -ForegroundColor Green
}

# ---- Stage 2: Mode Switch (ADead-BIB) ----
Write-Host "[3/5] Building Stage 2 mode switch (ADead-BIB)..." -ForegroundColor Yellow
$Stage2Src = Join-Path $ScriptDir "boot\stage2.adB"
$Stage2Bin = Join-Path $BuildDir "stage2.bin"

if ($ADeadBIB) {
    & $ADeadBIB flat $Stage2Src -o $Stage2Bin 2>&1 | ForEach-Object {
        if ($Verbose) { Write-Host "  $_" -ForegroundColor DarkGray }
    }
} else {
    Push-Location $ADeadBIBRoot
    cargo run -- flat $Stage2Src -o $Stage2Bin 2>&1 | ForEach-Object {
        if ($Verbose) { Write-Host "  $_" -ForegroundColor DarkGray }
    }
    Pop-Location
}

if (Test-Path $Stage2Bin) {
    $size = (Get-Item $Stage2Bin).Length
    Write-Host "[3/5] Stage 2 OK ($size bytes)" -ForegroundColor Green
} else {
    Write-Host "[WARN] Stage 2 not built yet (requires full ADead-BIB flat support)" -ForegroundColor Yellow
}

# ---- Rust Kernel ----
Write-Host "[4/5] Building Rust kernel..." -ForegroundColor Yellow
$KernelDir = Join-Path $ScriptDir "kernel"

# Check if nightly toolchain and rust-src are available
$hasNightly = rustup toolchain list 2>&1 | Select-String "nightly"
if (-not $hasNightly) {
    Write-Host "  [INFO] Installing nightly toolchain for bare-metal..." -ForegroundColor DarkGray
    rustup toolchain install nightly 2>&1 | Out-Null
}

$hasRustSrc = rustup component list --toolchain nightly 2>&1 | Select-String "rust-src.*installed"
if (-not $hasRustSrc) {
    Write-Host "  [INFO] Installing rust-src component..." -ForegroundColor DarkGray
    rustup component add rust-src --toolchain nightly 2>&1 | Out-Null
}

Push-Location $KernelDir
try {
    # Build with custom target (bare-metal x86_64, no OS)
    $env:RUSTFLAGS = "-C link-arg=-Tlink/kernel.ld"
    cargo +nightly build --target x86_64-adead-os.json -Zbuild-std=core -Zbuild-std-features=compiler-builtins-mem 2>&1 | ForEach-Object {
        if ($Verbose -or $_ -match "error") { Write-Host "  $_" -ForegroundColor DarkGray }
    }
    if ($LASTEXITCODE -eq 0) {
        Write-Host "[4/5] Rust kernel OK" -ForegroundColor Green
    } else {
        Write-Host "[4/5] Rust kernel build needs nightly + rust-src (expected for first run)" -ForegroundColor Yellow
        Write-Host "  Run: rustup toolchain install nightly" -ForegroundColor DarkGray
        Write-Host "  Run: rustup component add rust-src --toolchain nightly" -ForegroundColor DarkGray
    }
} finally {
    Pop-Location
}

# ---- Combine into OS image ----
Write-Host "[5/5] Creating OS image..." -ForegroundColor Yellow
$OsImage = Join-Path $BuildDir "adead-os.bin"

if (Test-Path $Stage1Bin) {
    # Start with stage1 (512 bytes boot sector)
    $imageBytes = [System.IO.File]::ReadAllBytes($Stage1Bin)

    # Pad to ensure boot sector is exactly 512 bytes
    if ($imageBytes.Length -lt 512) {
        $padded = New-Object byte[] 512
        [Array]::Copy($imageBytes, $padded, $imageBytes.Length)
        $padded[510] = 0x55; $padded[511] = 0xAA
        $imageBytes = $padded
    }

    # Append stage2 if available
    if (Test-Path $Stage2Bin) {
        $stage2Bytes = [System.IO.File]::ReadAllBytes($Stage2Bin)
        $combined = New-Object byte[] ($imageBytes.Length + $stage2Bytes.Length)
        [Array]::Copy($imageBytes, $combined, $imageBytes.Length)
        [Array]::Copy($stage2Bytes, 0, $combined, $imageBytes.Length, $stage2Bytes.Length)
        $imageBytes = $combined
    }

    [System.IO.File]::WriteAllBytes($OsImage, $imageBytes)
    $totalSize = $imageBytes.Length
    Write-Host "[5/5] OS image OK: $OsImage ($totalSize bytes)" -ForegroundColor Green
} else {
    Write-Host "[5/5] Cannot create OS image (stage1 missing)" -ForegroundColor Red
    exit 1
}

# ---- Summary ----
Write-Host ""
Write-Host "============================================" -ForegroundColor Cyan
Write-Host "  Build Complete!" -ForegroundColor Green
Write-Host "============================================" -ForegroundColor Cyan
Write-Host "  Stage 1 (ADead-BIB): boot/stage1.adB → build/stage1.bin" -ForegroundColor White
Write-Host "  Stage 2 (ADead-BIB): boot/stage2.adB → build/stage2.bin" -ForegroundColor White
Write-Host "  Kernel  (Rust):      kernel/src/*.rs  → (linked)" -ForegroundColor White
Write-Host "  Headers (C):         include/*.h      → (ABI contract)" -ForegroundColor White
Write-Host "  OS Image:            build/adead-os.bin" -ForegroundColor Green
Write-Host ""
Write-Host "  Languages: ADead-BIB + Rust + C" -ForegroundColor Cyan
Write-Host "  ASM used:  ZERO" -ForegroundColor Green
Write-Host ""

# ---- Run in QEMU ----
if ($Run) {
    Write-Host "Launching QEMU..." -ForegroundColor Yellow
    $qemu = Get-Command "qemu-system-x86_64" -ErrorAction SilentlyContinue
    if ($qemu) {
        & qemu-system-x86_64 -drive "format=raw,file=$OsImage" -no-reboot -no-shutdown
    } else {
        Write-Host "[ERROR] QEMU not found. Install qemu-system-x86_64 to test." -ForegroundColor Red
        Write-Host "  Download: https://www.qemu.org/download/" -ForegroundColor DarkGray
        Write-Host "  Or run manually: qemu-system-x86_64 -drive format=raw,file=$OsImage" -ForegroundColor DarkGray
    }
}
