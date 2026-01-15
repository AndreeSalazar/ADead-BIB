# ============================================================================
# FastOS Build Script (Windows PowerShell)
# ============================================================================
# GPU-First / Binary-First Operating System
# Stack: ADead-BIB + Rust + wgpu
#
# Author: Eddi Andreé Salazar Matos
# ============================================================================

param(
    [string]$Action = "build"
)

$QEMU_PATH = "C:\Program Files\qemu\qemu-system-x86_64.exe"
$BOOTIMAGE = "target\x86_64-unknown-none\release\bootimage-fastos-kernel.bin"

$ErrorActionPreference = "Stop"

# Directorios
$BuildDir = "build"
$BootloaderAsm = "boot\bootloader.asm"
$BootloaderBin = "$BuildDir\bootloader.bin"
$KernelBin = "$BuildDir\kernel.bin"
$Image = "$BuildDir\fastos.img"

function Write-Header {
    Write-Host ""
    Write-Host "╔════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
    Write-Host "║           FastOS Build System v0.1                         ║" -ForegroundColor Cyan
    Write-Host "║           ADead-BIB + Rust + wgpu                          ║" -ForegroundColor Cyan
    Write-Host "╚════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
    Write-Host ""
}

function Build-Bootloader {
    Write-Host "[BUILD] Compilando bootloader..." -ForegroundColor Yellow
    
    # Crear directorio de build
    if (-not (Test-Path $BuildDir)) {
        New-Item -ItemType Directory -Path $BuildDir | Out-Null
    }
    
    # Verificar si NASM está instalado
    $nasm = Get-Command nasm -ErrorAction SilentlyContinue
    if ($nasm) {
        & nasm -f bin $BootloaderAsm -o $BootloaderBin
        Write-Host "[OK] Bootloader compilado: $BootloaderBin" -ForegroundColor Green
    } else {
        Write-Host "[SKIP] NASM no encontrado, usando bootloader pre-compilado" -ForegroundColor Yellow
        # Crear bootloader mínimo de prueba (512 bytes con firma)
        $bootBytes = [byte[]]::new(512)
        # Código mínimo: jmp $, firma 0xAA55
        $bootBytes[0] = 0xEB  # jmp short
        $bootBytes[1] = 0xFE  # -2 (loop infinito)
        $bootBytes[510] = 0x55
        $bootBytes[511] = 0xAA
        [System.IO.File]::WriteAllBytes($BootloaderBin, $bootBytes)
        Write-Host "[OK] Bootloader placeholder creado" -ForegroundColor Green
    }
}

function Build-Kernel {
    Write-Host "[BUILD] Compilando kernel..." -ForegroundColor Yellow
    
    # Compilar con Cargo
    $env:RUSTFLAGS = "-C link-arg=-Tlinker.ld"
    
    try {
        & cargo build --release 2>&1
        
        $kernelPath = "target\x86_64-fastos\release\fastos-kernel"
        if (Test-Path $kernelPath) {
            Copy-Item $kernelPath $KernelBin -Force
            Write-Host "[OK] Kernel compilado: $KernelBin" -ForegroundColor Green
        } else {
            Write-Host "[INFO] Kernel no encontrado en target, creando placeholder" -ForegroundColor Yellow
            # Crear kernel placeholder
            $kernelBytes = [byte[]]::new(4096)
            [System.IO.File]::WriteAllBytes($KernelBin, $kernelBytes)
        }
    } catch {
        Write-Host "[WARN] Error compilando kernel: $_" -ForegroundColor Yellow
        Write-Host "[INFO] Creando kernel placeholder para testing" -ForegroundColor Yellow
        $kernelBytes = [byte[]]::new(4096)
        [System.IO.File]::WriteAllBytes($KernelBin, $kernelBytes)
    }
}

function Build-Image {
    Write-Host "[BUILD] Creando imagen de disco..." -ForegroundColor Yellow
    
    # Tamaño: 1.44MB floppy
    $imageSize = 1474560
    $imageBytes = [byte[]]::new($imageSize)
    
    # Leer bootloader
    if (Test-Path $BootloaderBin) {
        $bootBytes = [System.IO.File]::ReadAllBytes($BootloaderBin)
        [Array]::Copy($bootBytes, 0, $imageBytes, 0, [Math]::Min($bootBytes.Length, 512))
    }
    
    # Leer kernel (a partir del sector 1)
    if (Test-Path $KernelBin) {
        $kernelBytes = [System.IO.File]::ReadAllBytes($KernelBin)
        [Array]::Copy($kernelBytes, 0, $imageBytes, 512, [Math]::Min($kernelBytes.Length, $imageSize - 512))
    }
    
    # Escribir imagen
    [System.IO.File]::WriteAllBytes($Image, $imageBytes)
    
    $sizeKB = [Math]::Round((Get-Item $Image).Length / 1024, 2)
    Write-Host "[OK] Imagen creada: $Image ($sizeKB KB)" -ForegroundColor Green
}

function Run-Qemu {
    Write-Host "[RUN] Ejecutando en QEMU..." -ForegroundColor Yellow
    
    $qemu = Get-Command qemu-system-x86_64 -ErrorAction SilentlyContinue
    if ($qemu) {
        & qemu-system-x86_64 -drive format=raw,file=$Image -m 128M
    } else {
        Write-Host "[ERROR] QEMU no encontrado. Instala QEMU para ejecutar FastOS." -ForegroundColor Red
        Write-Host "        Descarga: https://www.qemu.org/download/" -ForegroundColor Yellow
    }
}

function Show-Help {
    Write-Host "Uso: .\build.ps1 [accion]" -ForegroundColor White
    Write-Host ""
    Write-Host "Acciones:" -ForegroundColor Yellow
    Write-Host "  build     - Compilar todo (default)"
    Write-Host "  bootloader - Solo compilar bootloader"
    Write-Host "  kernel    - Solo compilar kernel"
    Write-Host "  image     - Solo crear imagen"
    Write-Host "  run       - Compilar y ejecutar en QEMU"
    Write-Host "  clean     - Limpiar archivos de build"
    Write-Host "  help      - Mostrar esta ayuda"
}

function Clean-Build {
    Write-Host "[CLEAN] Limpiando..." -ForegroundColor Yellow
    
    if (Test-Path $BuildDir) {
        Remove-Item -Recurse -Force $BuildDir
    }
    
    if (Test-Path "target") {
        & cargo clean 2>$null
    }
    
    Write-Host "[OK] Limpieza completada" -ForegroundColor Green
}

# Main
Write-Header

switch ($Action.ToLower()) {
    "build" {
        Build-Bootloader
        Build-Kernel
        Build-Image
        Write-Host ""
        Write-Host "Build completado! Ejecuta '.\build.ps1 run' para probar." -ForegroundColor Green
    }
    "bootloader" {
        Build-Bootloader
    }
    "kernel" {
        Build-Kernel
    }
    "image" {
        Build-Image
    }
    "run" {
        Build-Bootloader
        Build-Kernel
        Build-Image
        Run-Qemu
    }
    "clean" {
        Clean-Build
    }
    "help" {
        Show-Help
    }
    default {
        Write-Host "Accion desconocida: $Action" -ForegroundColor Red
        Show-Help
    }
}
