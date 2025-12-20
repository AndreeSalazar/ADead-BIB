# Build script para ADead-BIB (Windows PowerShell)

Write-Host "🔨 Building ADead-BIB..." -ForegroundColor Cyan

# 1. Compilar C++ con CMake
Write-Host "`n[1/2] Compilando C++ emitter..." -ForegroundColor Yellow
if (!(Test-Path "build")) {
    New-Item -ItemType Directory -Path "build" | Out-Null
}

Push-Location build
cmake .. -DCMAKE_BUILD_TYPE=Release
cmake --build . --config Release
Pop-Location

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Error compilando C++" -ForegroundColor Red
    exit 1
}

Write-Host "✓ C++ compilado" -ForegroundColor Green

# 2. Compilar Rust
Write-Host "`n[2/2] Compilando Rust..." -ForegroundColor Yellow
cargo build --release

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Error compilando Rust" -ForegroundColor Red
    exit 1
}

Write-Host "✓ Rust compilado" -ForegroundColor Green

Write-Host "`n✅ Build completo!" -ForegroundColor Green
Write-Host "Ejecutable: target/release/adeadc.exe" -ForegroundColor Cyan

