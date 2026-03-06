# ============================================================
# ADead-BIB — Test-Canon: Compilar TODOS los tests
# ============================================================
# Uso: .\Test-Canon\compile_canon.ps1
# ============================================================

$ErrorActionPreference = "Continue"
$compiler = ".\target\release\adeadc.exe"

# Verificar que el compilador existe
if (-not (Test-Path $compiler)) {
    $compiler = ".\target\debug\adeadc.exe"
    if (-not (Test-Path $compiler)) {
        Write-Host "ERROR: No se encuentra adeadc.exe" -ForegroundColor Red
        Write-Host "  Ejecutar: cargo build --release"
        exit 1
    }
}

Write-Host "============================================" -ForegroundColor Cyan
Write-Host "  ADead-BIB Test-Canon: Compilacion Total" -ForegroundColor Cyan
Write-Host "  Compiler: $compiler" -ForegroundColor Cyan
Write-Host "============================================" -ForegroundColor Cyan
Write-Host ""

$total = 0
$passed = 0
$failed = 0
$errors = @()

# --- C99 ---
Write-Host "=== C99 Canon ===" -ForegroundColor Yellow
$c_files = Get-ChildItem "Test-Canon\C99\*.c" | Sort-Object Name
foreach ($f in $c_files) {
    $total++
    $out = "Test-Canon\C99\" + $f.BaseName + ".exe"
    $result = & $compiler cc $f.FullName -o $out --warn-ub 2>&1
    if ($LASTEXITCODE -eq 0) {
        $size = (Get-Item $out -ErrorAction SilentlyContinue).Length
        Write-Host "  OK  $($f.Name) -> $size bytes" -ForegroundColor Green
        $passed++
    } else {
        Write-Host "  FAIL  $($f.Name)" -ForegroundColor Red
        $failed++
        $errors += $f.Name
    }
}

Write-Host ""

# --- C++98 ---
Write-Host "=== C++98 Canon ===" -ForegroundColor Yellow
$cpp_files = Get-ChildItem "Test-Canon\Cpp98\*.cpp" | Sort-Object Name
foreach ($f in $cpp_files) {
    $total++
    $out = "Test-Canon\Cpp98\" + $f.BaseName + ".exe"
    $result = & $compiler cxx $f.FullName -o $out --warn-ub 2>&1
    if ($LASTEXITCODE -eq 0) {
        $size = (Get-Item $out -ErrorAction SilentlyContinue).Length
        Write-Host "  OK  $($f.Name) -> $size bytes" -ForegroundColor Green
        $passed++
    } else {
        Write-Host "  FAIL  $($f.Name)" -ForegroundColor Red
        $failed++
        $errors += $f.Name
    }
}

# --- Resumen ---
Write-Host ""
Write-Host "============================================" -ForegroundColor Cyan
Write-Host "  RESUMEN" -ForegroundColor Cyan
Write-Host "============================================" -ForegroundColor Cyan
Write-Host "  Total:   $total" -ForegroundColor White
Write-Host "  Passed:  $passed" -ForegroundColor Green
Write-Host "  Failed:  $failed" -ForegroundColor $(if ($failed -gt 0) { "Red" } else { "Green" })

if ($failed -gt 0) {
    Write-Host ""
    Write-Host "  Archivos con error:" -ForegroundColor Red
    foreach ($e in $errors) {
        Write-Host "    - $e" -ForegroundColor Red
    }
}

$pct = [math]::Round(($passed / $total) * 100, 1)
Write-Host ""
Write-Host "  Tasa: $pct% ($passed/$total)" -ForegroundColor $(if ($pct -eq 100) { "Green" } else { "Yellow" })
Write-Host ""
