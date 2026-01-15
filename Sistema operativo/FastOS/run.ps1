# ============================================================================
# FastOS Run Script
# ============================================================================
# GPU-First / Binary-First Operating System (64-bit)
# Stack: ADead-BIB + Rust + wgpu
#
# Author: Eddi Andreé Salazar Matos
# ============================================================================

param(
    [string]$Action = "run"
)

$BOOTIMAGE = "target\x86_64-unknown-none\release\bootimage-fastos-kernel.bin"
# QEMU 8.2 no tiene el bug PageAlreadyMapped de QEMU 10.x
$QEMU_ARGS = @('-drive', "format=raw,file=$BOOTIMAGE", '-m', '128M')

Write-Host ""
Write-Host "========================================================" -ForegroundColor Cyan
Write-Host "     FastOS - GPU-First / Binary-First OS" -ForegroundColor Cyan
Write-Host "     Stack: ADead-BIB + Rust + wgpu" -ForegroundColor Cyan
Write-Host "========================================================" -ForegroundColor Cyan
Write-Host ""

switch ($Action.ToLower()) {
    "build" {
        Write-Host "[BUILD] Compilando FastOS kernel..." -ForegroundColor Yellow
        cargo bootimage --release
        if ($LASTEXITCODE -eq 0) {
            Write-Host "[OK] Kernel compilado!" -ForegroundColor Green
            Write-Host "     Imagen: $BOOTIMAGE" -ForegroundColor Gray
        }
    }
    "run" {
        Write-Host "[BUILD] Compilando FastOS kernel..." -ForegroundColor Yellow
        cargo bootimage --release
        
        if ($LASTEXITCODE -ne 0) {
            Write-Host "[ERROR] Fallo en compilacion" -ForegroundColor Red
            exit 1
        }
        
        Write-Host "[OK] Kernel compilado!" -ForegroundColor Green
        Write-Host ""
        Write-Host "[RUN] Buscando QEMU..." -ForegroundColor Yellow
        
        # Buscar QEMU en ubicaciones comunes (QEMU 8.2 primero para evitar bug)
        $qemuPaths = @(
            "C:\qemu8\qemu-system-x86_64.exe",
            "C:\Program Files\qemu\qemu-system-x86_64.exe",
            "C:\qemu\qemu-system-x86_64.exe",
            "$env:USERPROFILE\qemu\qemu-system-x86_64.exe",
            "qemu-system-x86_64"
        )
        
        $qemu = $null
        foreach ($path in $qemuPaths) {
            if (Test-Path $path -ErrorAction SilentlyContinue) {
                $qemu = $path
                break
            }
            $cmd = Get-Command $path -ErrorAction SilentlyContinue
            if ($cmd) {
                $qemu = $cmd.Source
                break
            }
        }
        
        if ($qemu) {
            Write-Host "[OK] QEMU encontrado: $qemu" -ForegroundColor Green
            Write-Host ""
            Write-Host "[RUN] Iniciando FastOS en QEMU..." -ForegroundColor Yellow
            Write-Host "      Presiona Ctrl+Alt+G para liberar el mouse" -ForegroundColor Gray
            Write-Host ""
            & $qemu @QEMU_ARGS
        } else {
            Write-Host "[WARN] QEMU no encontrado!" -ForegroundColor Yellow
            Write-Host ""
            Write-Host "Para instalar QEMU:" -ForegroundColor White
            Write-Host "  winget install SoftwareFreedomConservancy.QEMU" -ForegroundColor Gray
            Write-Host ""
            Write-Host "Imagen booteable creada en:" -ForegroundColor White
            Write-Host "  $BOOTIMAGE" -ForegroundColor Green
            Write-Host ""
            Write-Host "Ejecutar manualmente:" -ForegroundColor White
            Write-Host "  qemu-system-x86_64 -drive format=raw,file=$BOOTIMAGE -m 8M -vga std" -ForegroundColor Gray
        }
    }
    "clean" {
        Write-Host "[CLEAN] Limpiando..." -ForegroundColor Yellow
        cargo clean
        Write-Host "[OK] Limpieza completada" -ForegroundColor Green
    }
    default {
        Write-Host "Uso: .\run.ps1 [build|run|clean]" -ForegroundColor White
        Write-Host ""
        Write-Host "  build  - Solo compilar kernel" -ForegroundColor Gray
        Write-Host "  run    - Compilar y ejecutar en QEMU (default)" -ForegroundColor Gray
        Write-Host "  clean  - Limpiar archivos de build" -ForegroundColor Gray
    }
}
