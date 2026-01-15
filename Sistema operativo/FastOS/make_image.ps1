# FastOS Image Builder
# Crea imagen booteable usando bootloader 0.11

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  FastOS Image Builder" -ForegroundColor Cyan
Write-Host "  GPU-First / Binary-First OS" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Paso 1: Compilar kernel
Write-Host "[1/3] Compilando kernel..." -ForegroundColor Yellow
cargo build --release 2>&1 | Out-Null

if ($LASTEXITCODE -ne 0) {
    Write-Host "[ERROR] Fallo compilando kernel" -ForegroundColor Red
    cargo build --release
    exit 1
}
Write-Host "[OK] Kernel compilado" -ForegroundColor Green

# Paso 2: Crear imagen con bootloader
Write-Host "[2/3] Creando imagen booteable..." -ForegroundColor Yellow

$kernelPath = "target\x86_64-unknown-none\release\fastos-kernel"
$imagePath = "target\x86_64-unknown-none\release\fastos-bios.img"

# Usar cargo para crear la imagen
$buildScript = @"
use std::path::PathBuf;
fn main() {
    let kernel = PathBuf::from(r"$($PWD)\$kernelPath");
    let image = PathBuf::from(r"$($PWD)\$imagePath");
    
    println!("Kernel: {:?}", kernel);
    println!("Image: {:?}", image);
    
    let bios = bootloader::BiosBoot::new(&kernel);
    bios.create_disk_image(&image).expect("Failed to create image");
    
    println!("Image created successfully!");
}
"@

# Crear proyecto temporal para build
$tempDir = "target\image_builder"
New-Item -ItemType Directory -Force -Path $tempDir | Out-Null

$tempCargo = @"
[package]
name = "image_builder"
version = "0.1.0"
edition = "2021"

[dependencies]
bootloader = "0.11"
"@

Set-Content -Path "$tempDir\Cargo.toml" -Value $tempCargo
New-Item -ItemType Directory -Force -Path "$tempDir\src" | Out-Null
Set-Content -Path "$tempDir\src\main.rs" -Value $buildScript

Push-Location $tempDir
cargo run --release 2>&1
$buildResult = $LASTEXITCODE
Pop-Location

if ($buildResult -ne 0) {
    Write-Host "[WARN] No se pudo crear imagen con bootloader 0.11" -ForegroundColor Yellow
    Write-Host "[INFO] Intentando con cargo-bootimage..." -ForegroundColor Yellow
    
    # Fallback: usar bootloader 0.9 con cargo-bootimage
    cargo install bootimage 2>&1 | Out-Null
    cargo bootimage --release 2>&1
    
    if ($LASTEXITCODE -eq 0) {
        $imagePath = "target\x86_64-unknown-none\release\bootimage-fastos-kernel.bin"
        Write-Host "[OK] Imagen creada con bootloader 0.9" -ForegroundColor Green
    } else {
        Write-Host "[ERROR] No se pudo crear imagen" -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "[OK] Imagen creada" -ForegroundColor Green
}

# Paso 3: Ejecutar en QEMU
Write-Host "[3/3] Ejecutando en QEMU..." -ForegroundColor Yellow
Write-Host ""
Write-Host "Imagen: $imagePath" -ForegroundColor Gray
Write-Host ""

$qemu = "C:\Program Files\qemu\qemu-system-x86_64.exe"
if (Test-Path $qemu) {
    Write-Host "Iniciando QEMU..." -ForegroundColor Green
    Write-Host "Presiona Ctrl+Alt+G para liberar el mouse" -ForegroundColor Gray
    & $qemu -drive format=raw,file=$imagePath -m 256M
} else {
    Write-Host "[ERROR] QEMU no encontrado en: $qemu" -ForegroundColor Red
    Write-Host "Instala QEMU: winget install SoftwareFreedomConservancy.QEMU" -ForegroundColor Yellow
}
