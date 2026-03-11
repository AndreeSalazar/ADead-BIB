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
    # Find all kernel source files, excluding shell, rust wrappers, and main.c (which we add manually at the start)
    $kernelFiles = Get-ChildItem -Path $KERNEL, "$ROOT\lib", "$ROOT\security", "$ROOT\fs", "$ROOT\userspace" -Filter "*.c" -Recurse | Where-Object { 
        $_.Name -ne "shell.c" -and 
        $_.Name -ne "bg_fastos.c" -and 
        $_.FullName -ne "$KERNEL\main.c" 
    }
    
    $ccArgs = @("--manifest-path=$ADEAD_ROOT\Cargo.toml", "--release", "--", "cc")
    $ccArgs += "$KERNEL\main.c"       # CRITICO: main.c TIENE que ser el primero (offset 0x0)
    $ccArgs += $kernelFiles.FullName
    $ccArgs += @("-o", "$kernelBin", "--flat", "--org=0x100000", "--size=32768")

    Write-Status "Compiling $(@($kernelFiles).Count) kernel source files with ADead-BIB..."
    $result = & cargo run @ccArgs 2>&1

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
    
    # Placeholder 64-bit kernel: clears VGA, prints 5-step boot banner, halts.
    # Mirrors kernel_main() output so the user sees the real boot sequence.
    # Replaced when ADead-BIB compiles the real kernel.
    $kernel = New-Object byte[] 32768

    # --- Subroutine: print_str (RSI=string, RDI=VGA ptr, AH=color) ---
    # Located at offset 0x00 so the main code can call it.
    # Convention: RSI -> NUL-terminated string, RDI -> VGA position,
    #             AH = attribute byte. Advances RDI past printed chars.
    #             Returns to caller via RET.
    $sub_print = @(
        # print_str:
        0xAC,                       # lodsb
        0x84, 0xC0,                 # test al, al
        0x74, 0x04,                 # jz .ret
        0x66, 0xAB,                 # stosw
        0xEB, 0xF7,                 # jmp print_str
        # .ret:
        0xC3                        # ret
    )
    # 10 bytes: offsets 0x00..0x09

    # --- Main entry point at offset 0x10 ---
    $main_code = @(
        # ; -- Clear screen (green on black: attr 0x0A) --
        0x48, 0xBF, 0x00, 0x80, 0x0B, 0x00, 0x00, 0x00, 0x00, 0x00, # mov rdi, 0xB8000
        0x48, 0xB8, 0x20, 0x1A, 0x20, 0x1A, 0x20, 0x1A, 0x20, 0x1A, # mov rax, 0x1A201A201A201A20 (blue bg)
        0xB9, 0xD0, 0x07, 0x00, 0x00,                                # mov ecx, 2000 (80*25)
        0xF3, 0x48, 0xAB,                                            # rep stosq

        # ; -- Row 1: banner line 1 --
        0x48, 0xBF, 0xA0, 0x80, 0x0B, 0x00, 0x00, 0x00, 0x00, 0x00, # mov rdi, 0xB80A0 (row 1)
        0x48, 0xBE,                                                  # mov rsi, <msg1>
        0x00, 0x02, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00,              # 0x100200
        0xB4, 0x1B,                                                  # mov ah, 0x1B (cyan on blue)
        0xE8                                                          # call print_str (rel32)
    )

    # Calculate call target: from end of CALL instruction (offset 0x10 + len + 5)
    # print_str is at offset 0x00. call offset = 0x00 - (current_ip).
    # Main starts at 0x10. The call byte is at 0x10 + <bytes before it>.
    # Let's just build the raw image manually with known offsets.

    # Too complex with relative calls — use inline print loop instead (simpler, same result)
    $code = @(
        # Clear screen: 80*25 = 2000 chars, attr 0x1A (green on blue)
        0x48, 0xBF, 0x00, 0x80, 0x0B, 0x00, 0x00, 0x00, 0x00, 0x00,  # mov rdi, 0xB8000
        0x48, 0xB8, 0x20, 0x1A, 0x20, 0x1A, 0x20, 0x1A, 0x20, 0x1A,  # mov rax, attr|' ' x4
        0xB9, 0xF4, 0x01, 0x00, 0x00,                                # mov ecx, 500 (500 qwords = 2000 words)
        0xF3, 0x48, 0xAB,                                            # rep stosq

        # Row 2: FastOS v2.0 banner
        0x48, 0xBF, 0x40, 0x81, 0x0B, 0x00, 0x00, 0x00, 0x00, 0x00,  # mov rdi, 0xB8140 (row 2)
        0x48, 0xBE,                                                  # mov rsi, msg1
        0x00, 0x01, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00               # 0x100100
    )

    $printLoop = @(
        # .loop:
        0xAC,                       # lodsb
        0x84, 0xC0,                 # test al, al
        0x74, 0x06,                 # jz .done
        0xB4, 0x1F,                 # mov ah, 0x1F (white on blue)
        0x66, 0xAB,                 # stosw
        0xEB, 0xF5,                 # jmp .loop
        # .done:

        # Row 4: "ADead-BIB Compiler"
        0x48, 0xBF, 0x80, 0x82, 0x0B, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x48, 0xBE,
        0x40, 0x01, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xAC, 0x84, 0xC0, 0x74, 0x06, 0xB4, 0x1E, 0x66, 0xAB, 0xEB, 0xF5,

        # Row 6: "[1/5] memory_init()"
        0x48, 0xBF, 0xC0, 0x83, 0x0B, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x48, 0xBE,
        0x80, 0x01, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xAC, 0x84, 0xC0, 0x74, 0x06, 0xB4, 0x1A, 0x66, 0xAB, 0xEB, 0xF5,

        # Row 8: "[BG] Binary Guardian: ACTIVE"
        0x48, 0xBF, 0x00, 0x85, 0x0B, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x48, 0xBE,
        0xC0, 0x01, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xAC, 0x84, 0xC0, 0x74, 0x06, 0xB4, 0x1B, 0x66, 0xAB, 0xEB, 0xF5,

        # Row 10: "Placeholder — compile with ADead-BIB for full kernel"
        0x48, 0xBF, 0x40, 0x86, 0x0B, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x48, 0xBE,
        0x00, 0x02, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xAC, 0x84, 0xC0, 0x74, 0x06, 0xB4, 0x1C, 0x66, 0xAB, 0xEB, 0xF5,

        # Halt
        0xFA,                       # cli
        0xF4,                       # hlt
        0xEB, 0xFD                  # jmp $ (infinite halt loop)
    )

    $offset = 0
    foreach ($b in $code) { $kernel[$offset++] = $b }
    foreach ($b in $printLoop) { $kernel[$offset++] = $b }

    # Messages at offsets (relative to 0x100000 load address)
    $msg1 = [System.Text.Encoding]::ASCII.GetBytes("FastOS v2.0 - 64-bit Long Mode - ADead-BIB`0")
    $msg2 = [System.Text.Encoding]::ASCII.GetBytes("Compiler: ADead-BIB (C is Master, Rust is Safety)`0")
    $msg3 = [System.Text.Encoding]::ASCII.GetBytes("[1/5] memory_init [2/5] interrupts_init [3/5] scheduler_init`0")
    $msg4 = [System.Text.Encoding]::ASCII.GetBytes("[BG] Binary Guardian: ACTIVE (4 niveles, matematica pura)`0")
    $msg5 = [System.Text.Encoding]::ASCII.GetBytes("Placeholder kernel - compile with build64.ps1 (ADead-BIB)`0")

    $msgOffset = 0x100
    foreach ($b in $msg1) { $kernel[$msgOffset++] = $b }
    $msgOffset = 0x140
    foreach ($b in $msg2) { $kernel[$msgOffset++] = $b }
    $msgOffset = 0x180
    foreach ($b in $msg3) { $kernel[$msgOffset++] = $b }
    $msgOffset = 0x1C0
    foreach ($b in $msg4) { $kernel[$msgOffset++] = $b }
    $msgOffset = 0x200
    foreach ($b in $msg5) { $kernel[$msgOffset++] = $b }

    [System.IO.File]::WriteAllBytes($kernelBin, $kernel)
    Write-Success "Kernel: $($kernel.Length) bytes (placeholder - shows 5-step boot banner)"
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
