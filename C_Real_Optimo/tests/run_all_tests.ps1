# run_all_tests.ps1 — Harness de tests automatizado para ADead-BIB v12.0
# Verifica que cada .c se compile y produzca el exit code esperado.
# Aborta cualquier .exe que cuelgue >5 segundos.

param(
    [string]$Filter = "*",
    [int]$TimeoutSec = 5
)

$ErrorActionPreference = "Continue"
$root = Split-Path $PSScriptRoot -Parent
$compiler = Join-Path $root "target\release\adB.exe"

if (-not (Test-Path $compiler)) {
    Write-Host "Compiler not found at $compiler. Run 'cargo build --release' first." -ForegroundColor Red
    exit 1
}

# (test_file, expected_exit_code)
$tests = @(
    @{f = "c99/01_variables.c";  e = 0},
    @{f = "c99/02_arithmetic.c"; e = 0},
    @{f = "c99/03_if_else.c";    e = 0},
    @{f = "c99/04_while_loop.c"; e = 0},
    @{f = "c99/05_for_loop.c";   e = 0},
    @{f = "c99/06_functions.c";  e = 0},
    @{f = "c99/07_recursion.c";  e = 0},
    @{f = "c99/08_pointers.c";   e = 0},
    @{f = "c99/09_arrays.c";     e = 0},
    @{f = "c99/10_structs.c";    e = 0},
    @{f = "hello.c";             e = 0}
)

$pass = 0; $fail = 0; $hang = 0; $compileFail = 0
$results = @()

Write-Host "`n=== ADead-BIB Test Harness v12.0 ===" -ForegroundColor Cyan
Write-Host "Compiler: $compiler"
Write-Host "Timeout: ${TimeoutSec}s per test`n"

foreach ($t in $tests) {
    if ($t.f -notlike $Filter) { continue }

    $src = Join-Path $PSScriptRoot $t.f
    if (-not (Test-Path $src)) {
        Write-Host "[skip] $($t.f) — file not found" -ForegroundColor DarkGray
        continue
    }

    $exe = Join-Path $env:TEMP ("adeb_test_" + ([System.IO.Path]::GetFileNameWithoutExtension($t.f)) + ".exe")

    # 1. Compile
    $compileOut = & $compiler cc $src -o $exe 2>&1
    if ($LASTEXITCODE -ne 0 -or -not (Test-Path $exe)) {
        $compileFail++
        Write-Host "[FAIL-COMPILE] $($t.f)" -ForegroundColor Red
        $compileOut | Where-Object { $_ -match "error" } | ForEach-Object { Write-Host "    $_" -ForegroundColor DarkRed }
        continue
    }

    # 2. Run with timeout
    $proc = Start-Process -FilePath $exe -PassThru -NoNewWindow -RedirectStandardOutput "NUL"
    $finished = $proc.WaitForExit($TimeoutSec * 1000)

    if (-not $finished) {
        $proc.Kill()
        $hang++
        Write-Host "[HANG]         $($t.f) — killed after ${TimeoutSec}s" -ForegroundColor Yellow
        Remove-Item $exe -ErrorAction SilentlyContinue
        continue
    }

    $code = $proc.ExitCode
    if ($code -eq $t.e) {
        $pass++
        Write-Host "[PASS]         $($t.f) — exit=$code" -ForegroundColor Green
    } else {
        $fail++
        Write-Host "[FAIL]         $($t.f) — exit=$code (expected $($t.e))" -ForegroundColor Red
    }
    Remove-Item $exe -ErrorAction SilentlyContinue
}

$total = $pass + $fail + $hang + $compileFail
Write-Host "`n=== Summary ===" -ForegroundColor Cyan
Write-Host "  PASS:        $pass / $total" -ForegroundColor Green
Write-Host "  FAIL:        $fail / $total" -ForegroundColor Red
Write-Host "  HANG:        $hang / $total" -ForegroundColor Yellow
Write-Host "  COMPILE-FAIL:$compileFail / $total" -ForegroundColor DarkRed

if ($pass -eq $total) {
    Write-Host "`n[OK] ALL TESTS PASSED" -ForegroundColor Green
    exit 0
} else {
    exit 1
}
