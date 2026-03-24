# ============================================================
# FastOS v2.0 - CSM Legacy Build (Ryzen 5 5600X)
# Builds: MBR + Stage2 + Kernel (NASM + GCC cross-compiler)
# ============================================================

param(
    [switch]$Clean,
    [switch]$KernelOnly,
    [string]$UsbDisk
)

$ErrorActionPreference = "Stop"
$ROOT   = $PSScriptRoot
$BUILD  = "$ROOT\build"
$LEGACY = "$ROOT\legacy"
$KDIR   = "$ROOT\kernel"

# Tool paths — adjust these for your system
$NASM    = "C:\Users\andre\AppData\Local\bin\NASM\nasm.exe"
$CC      = "x86_64-elf-gcc"
$LD      = "x86_64-elf-ld"
$OBJCOPY = "x86_64-elf-objcopy"

# Compiler flags
$CFLAGS = @(
    "-ffreestanding", "-nostdlib", "-nostdinc",
    "-mno-red-zone", "-mcmodel=kernel",
    "-Wall", "-Wextra", "-O2",
    "-fno-exceptions", "-fno-stack-protector",
    "-fno-pic", "-fno-pie",
    "-mno-sse", "-mgeneral-regs-only",
    "-I$KDIR\include"
)

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  FastOS v2.0 - Full Kernel Build"       -ForegroundColor Cyan
Write-Host "  Target: Ryzen 5 5600X (256-bit)"       -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

if ($Clean) {
    Remove-Item -Path "$BUILD\*" -Recurse -Force -ErrorAction SilentlyContinue
    Write-Host "[OK] Clean" -ForegroundColor Green
    exit 0
}

New-Item -ItemType Directory -Path $BUILD -Force | Out-Null
New-Item -ItemType Directory -Path "$BUILD\kernel\lib" -Force | Out-Null

# ── Step 1: MBR ──
if (-not $KernelOnly) {
    Write-Host "[1/6] MBR..." -NoNewline
    & $NASM -f bin "$LEGACY\boot.asm" -o "$BUILD\mbr.bin" 2>&1
    if (-not (Test-Path "$BUILD\mbr.bin")) {
        Write-Host " FAILED" -ForegroundColor Red; exit 1
    }
    Write-Host " OK ($((Get-Item "$BUILD\mbr.bin").Length) bytes)" -ForegroundColor Green

    # ── Step 2: Stage2 ──
    Write-Host "[2/6] Stage2..." -NoNewline
    & $NASM -f bin "$LEGACY\stage2.asm" -o "$BUILD\stage2.bin" 2>&1
    if (-not (Test-Path "$BUILD\stage2.bin")) {
        Write-Host " FAILED" -ForegroundColor Red; exit 1
    }
    Write-Host " OK ($((Get-Item "$BUILD\stage2.bin").Length) bytes)" -ForegroundColor Green
}

# ── Step 3: Assemble kernel ASM files ──
Write-Host "[3/6] Kernel ASM..." -NoNewline
$asmFiles = @(
    @{ src = "$KDIR\kernel_entry.asm"; obj = "$BUILD\kernel\kernel_entry.o" },
    @{ src = "$KDIR\isr.asm";         obj = "$BUILD\kernel\isr.o" },
    @{ src = "$KDIR\syscall.asm";      obj = "$BUILD\kernel\syscall.o" },
    @{ src = "$KDIR\hal.asm";          obj = "$BUILD\kernel\hal.o" }
)

foreach ($f in $asmFiles) {
    & $NASM -f elf64 $f.src -o $f.obj 2>&1
    if ($LASTEXITCODE -ne 0) {
        Write-Host " FAILED ($($f.src))" -ForegroundColor Red; exit 1
    }
}
Write-Host " OK" -ForegroundColor Green

# ── Step 4: Compile kernel C files ──
Write-Host "[4/6] Kernel C..." -NoNewline
$cFiles = @(
    @{ src = "$KDIR\kernel.c";     obj = "$BUILD\kernel\kernel.o" },
    @{ src = "$KDIR\vga.c";        obj = "$BUILD\kernel\vga.o" },
    @{ src = "$KDIR\gdt.c";        obj = "$BUILD\kernel\gdt.o" },
    @{ src = "$KDIR\idt.c";        obj = "$BUILD\kernel\idt.o" },
    @{ src = "$KDIR\pic.c";        obj = "$BUILD\kernel\pic.o" },
    @{ src = "$KDIR\timer.c";      obj = "$BUILD\kernel\timer.o" },
    @{ src = "$KDIR\keyboard.c";   obj = "$BUILD\kernel\keyboard.o" },
    @{ src = "$KDIR\pmm.c";        obj = "$BUILD\kernel\pmm.o" },
    @{ src = "$KDIR\vmm.c";        obj = "$BUILD\kernel\vmm.o" },
    @{ src = "$KDIR\heap.c";       obj = "$BUILD\kernel\heap.o" },
    @{ src = "$KDIR\scheduler.c";  obj = "$BUILD\kernel\scheduler.o" },
    @{ src = "$KDIR\shell.c";      obj = "$BUILD\kernel\shell.o" },
    @{ src = "$KDIR\lib\string.c"; obj = "$BUILD\kernel\lib\string.o" },
    @{ src = "$KDIR\lib\printf.c"; obj = "$BUILD\kernel\lib\printf.o" }
)

foreach ($f in $cFiles) {
    $args = $CFLAGS + @("-c", $f.src, "-o", $f.obj)
    & $CC @args 2>&1
    if ($LASTEXITCODE -ne 0) {
        Write-Host " FAILED ($($f.src))" -ForegroundColor Red; exit 1
    }
}
Write-Host " OK" -ForegroundColor Green

# ── Step 5: Link kernel ──
Write-Host "[5/6] Link kernel..." -NoNewline
$allObjs = ($asmFiles + $cFiles) | ForEach-Object { $_.obj }
& $LD -T "$ROOT\kernel.ld" -nostdlib -z max-page-size=4096 -o "$BUILD\kernel.elf" @allObjs 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Host " FAILED" -ForegroundColor Red; exit 1
}
& $OBJCOPY -O binary "$BUILD\kernel.elf" "$BUILD\kernel.bin" 2>&1
if (-not (Test-Path "$BUILD\kernel.bin")) {
    Write-Host " FAILED (objcopy)" -ForegroundColor Red; exit 1
}
$kSize = (Get-Item "$BUILD\kernel.bin").Length
Write-Host " OK ($kSize bytes)" -ForegroundColor Green

# ── Step 6: Create disk image ──
Write-Host "[6/6] Disk image..." -NoNewline
# Layout: MBR(512) + Stage2(sectors 1-32) + Kernel(sectors 33+)
$mbr   = [System.IO.File]::ReadAllBytes("$BUILD\mbr.bin")
$s2    = [System.IO.File]::ReadAllBytes("$BUILD\stage2.bin")
$kern  = [System.IO.File]::ReadAllBytes("$BUILD\kernel.bin")

# Create 1.44MB floppy image (2880 sectors * 512 = 1474560 bytes)
$imgSize = 2880 * 512
$img = New-Object byte[] $imgSize

# MBR at sector 0
[Array]::Copy($mbr, 0, $img, 0, [Math]::Min($mbr.Length, 512))

# Stage2 at sector 1 (offset 512)
[Array]::Copy($s2, 0, $img, 512, $s2.Length)

# Kernel at sector 33 (offset 512 + 32*512 = 512 + 16384 = 16896)
$kernOffset = 512 + 16384
[Array]::Copy($kern, 0, $img, $kernOffset, [Math]::Min($kern.Length, $imgSize - $kernOffset))

[System.IO.File]::WriteAllBytes("$BUILD\fastos.img", $img)
Write-Host " OK ($imgSize bytes)" -ForegroundColor Green

# ── Summary ──
Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "  BUILD COMPLETE" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "  MBR:    $($mbr.Length) bytes"
Write-Host "  Stage2: $($s2.Length) bytes"
Write-Host "  Kernel: $kSize bytes"
Write-Host "  Image:  $imgSize bytes"
Write-Host ""
Write-Host "  Boot Chain:" -ForegroundColor Cyan
Write-Host "    MBR -> Stage2 (16→32→64→SSE→AVX2) -> Kernel @ 0x100000"
Write-Host "    Kernel: GDT→IDT→PIC→PMM→VMM→Heap→Timer→Keyboard→Shell"
Write-Host ""
Write-Host "  QEMU:" -ForegroundColor Yellow
Write-Host "    qemu-system-x86_64 -drive file=$BUILD\fastos.img,format=raw -m 512M -serial stdio"
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
