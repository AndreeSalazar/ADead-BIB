# ============================================================
# FastOS v2.0 - CSM Legacy Build (Ryzen 5 5600X)
# ============================================================

param(
    [switch]$Clean,
    [string]$UsbDisk
)

$ErrorActionPreference = "Stop"
$ROOT   = $PSScriptRoot
$BUILD  = "$ROOT\build"
$LEGACY = "$ROOT\legacy"
$NASM   = "C:\Users\andre\AppData\Local\bin\NASM\nasm.exe"

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  FastOS v2.0 - CSM Legacy Build"        -ForegroundColor Cyan
Write-Host "  Target: Ryzen 5 5600X (256-bit)"       -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

if ($Clean) {
    Remove-Item -Path "$BUILD\*" -Recurse -Force -ErrorAction SilentlyContinue
    Write-Host "[OK] Clean" -ForegroundColor Green
    exit 0
}

if (-not (Test-Path $NASM)) {
    Write-Host "[ERROR] NASM not found: $NASM" -ForegroundColor Red
    exit 1
}

New-Item -ItemType Directory -Path $BUILD -Force | Out-Null

# Step 1: Assemble MBR
Write-Host "[1/4] MBR..." -NoNewline
& $NASM -f bin "$LEGACY\boot.asm" -o "$BUILD\mbr.bin" 2>&1
if (-not (Test-Path "$BUILD\mbr.bin")) {
    Write-Host " FAILED" -ForegroundColor Red
    exit 1
}
$mbrSize = (Get-Item "$BUILD\mbr.bin").Length
Write-Host " OK ($mbrSize bytes)" -ForegroundColor Green

# Step 2: Assemble Stage2
Write-Host "[2/4] Stage2..." -NoNewline
& $NASM -f bin "$LEGACY\stage2.asm" -o "$BUILD\stage2.bin" 2>&1
if (-not (Test-Path "$BUILD\stage2.bin")) {
    Write-Host " FAILED" -ForegroundColor Red
    exit 1
}
$s2Size = (Get-Item "$BUILD\stage2.bin").Length
Write-Host " OK ($s2Size bytes)" -ForegroundColor Green

# Step 3: Concatenate MBR + Stage2
Write-Host "[3/4] Image..." -NoNewline
$mbr = [System.IO.File]::ReadAllBytes("$BUILD\mbr.bin")
$s2  = [System.IO.File]::ReadAllBytes("$BUILD\stage2.bin")
$img = New-Object byte[] ($mbr.Length + $s2.Length)
[Array]::Copy($mbr, 0, $img, 0, $mbr.Length)
[Array]::Copy($s2, 0, $img, $mbr.Length, $s2.Length)
[System.IO.File]::WriteAllBytes("$BUILD\fastos.img", $img)
$imgSize = $img.Length
Write-Host " OK ($imgSize bytes)" -ForegroundColor Green

# Step 4: Verify
Write-Host "[4/4] Verify..." -NoNewline
if ($img[510] -ne 0x55 -or $img[511] -ne 0xAA) {
    Write-Host " FAILED (no MBR sig)" -ForegroundColor Red
    exit 1
}
Write-Host " OK" -ForegroundColor Green

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "  BUILD COMPLETE" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "  Image: $BUILD\fastos.img ($imgSize bytes)"
Write-Host ""
Write-Host "  Boot Sequence:" -ForegroundColor Cyan
Write-Host "    16-bit  : MBR -> Stage2 -> A20 -> E820"
Write-Host "    32-bit  : Protected Mode -> Paging 4GB"
Write-Host "    64-bit  : Long Mode"
Write-Host "    128-bit : SSE (XMM0-15)"
Write-Host "    256-bit : AVX/AVX2 (YMM0-15)"
Write-Host ""
Write-Host "  QEMU test:" -ForegroundColor Yellow
Write-Host "    qemu-system-x86_64 -drive file=$BUILD\fastos.img,format=raw -m 512"
Write-Host ""
Write-Host "  USB (Admin PowerShell):" -ForegroundColor Yellow
Write-Host '    .\build.ps1 -UsbDisk "\\.\PhysicalDrive#"'
Write-Host ""

if ($UsbDisk) {
    Write-Host "WRITING TO: $UsbDisk" -ForegroundColor Red
    $confirm = Read-Host "Type YES"
    if ($confirm -ne "YES") { exit 0 }
    try {
        $stream = [System.IO.File]::OpenWrite($UsbDisk)
        $stream.Write($img, 0, $img.Length)
        $stream.Flush()
        $stream.Close()
        Write-Host "USB bootable!" -ForegroundColor Green
    } catch {
        Write-Host "FAILED: $_" -ForegroundColor Red
    }
}
