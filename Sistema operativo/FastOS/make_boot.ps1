# FastOS Boot Image Creator
# Usa bootloader 0.11 para crear imagen sin bug de paging

Write-Host "FastOS Boot Image Creator" -ForegroundColor Cyan
Write-Host "=========================" -ForegroundColor Cyan

# Paso 1: Compilar kernel
Write-Host "`n[1/2] Compilando kernel..." -ForegroundColor Yellow
cargo build --release 2>&1 | Out-Null
if ($LASTEXITCODE -ne 0) {
    Write-Host "[ERROR] Fallo compilando kernel" -ForegroundColor Red
    cargo build --release
    exit 1
}
Write-Host "[OK] Kernel compilado" -ForegroundColor Green

# Paso 2: Crear imagen con script Rust temporal
Write-Host "`n[2/2] Creando imagen booteable..." -ForegroundColor Yellow

$tempDir = "$env:TEMP\fastos_boot_$(Get-Random)"
New-Item -ItemType Directory -Force -Path "$tempDir\src" | Out-Null

# Cargo.toml para el builder
@"
[package]
name = "boot_builder"
version = "0.1.0"
edition = "2021"

[dependencies]
bootloader = "0.11"
"@ | Set-Content "$tempDir\Cargo.toml"

# main.rs para el builder
$kernelPath = (Resolve-Path "target\x86_64-unknown-none\release\fastos-kernel").Path -replace '\\', '/'
$imagePath = (Join-Path $PWD "target\fastos-bios.img") -replace '\\', '/'

@"
fn main() {
    let kernel = std::path::PathBuf::from(r"$kernelPath");
    let image = std::path::PathBuf::from(r"$imagePath");
    
    println!("Kernel: {:?}", kernel);
    println!("Image: {:?}", image);
    
    if !kernel.exists() {
        eprintln!("ERROR: Kernel not found!");
        std::process::exit(1);
    }
    
    let bios = bootloader::BiosBoot::new(&kernel);
    bios.create_disk_image(&image).expect("Failed to create image");
    
    println!("Image created successfully!");
}
"@ | Set-Content "$tempDir\src\main.rs"

# Ejecutar builder con nightly
Push-Location $tempDir
cargo +nightly run --release 2>&1
$result = $LASTEXITCODE
Pop-Location

# Limpiar
Remove-Item -Path $tempDir -Recurse -Force -ErrorAction SilentlyContinue

if ($result -ne 0) {
    Write-Host "[ERROR] Fallo creando imagen" -ForegroundColor Red
    exit 1
}

Write-Host "`n[OK] Imagen creada: target\fastos-bios.img" -ForegroundColor Green
Write-Host "`nPara ejecutar:" -ForegroundColor White
Write-Host '  & "C:\Program Files\qemu\qemu-system-x86_64.exe" -drive format=raw,file=target\fastos-bios.img -m 128M' -ForegroundColor Gray
