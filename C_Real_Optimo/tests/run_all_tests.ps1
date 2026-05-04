# run_all_tests.ps1 - ADead-BIB v12.0 Test Harness
# Verify each .c compiles and produces expected exit code.

$ErrorActionPreference = "Continue"
$compiler = Join-Path (Split-Path $PSScriptRoot -Parent) "target\release\adB.exe"

if (-not (Test-Path $compiler)) {
    Write-Host "Compiler not found at $compiler" -ForegroundColor Red
    exit 1
}

$tests = @(
    @{ f = "c99\01_variables.c";  e = 0 },
    @{ f = "c99\02_arithmetic.c"; e = 0 },
    @{ f = "c99\03_if_else.c";    e = 0 },
    @{ f = "c99\04_while_loop.c"; e = 0 },
    @{ f = "c99\05_for_loop.c";   e = 0 },
    @{ f = "c99\06_functions.c";  e = 0 },
    @{ f = "c99\07_recursion.c";  e = 0 },
    @{ f = "c99\08_pointers.c";   e = 0 },
    @{ f = "c99\09_arrays.c";     e = 0 },
    @{ f = "c99\10_structs.c";    e = 0 },
    @{ f = "hello.c";             e = 0 }
)

$timeoutSec = 5
$pass = 0; $fail = 0; $hang = 0; $compileFail = 0

Write-Host ""
Write-Host "=== ADead-BIB Test Harness v12.0 ===" -ForegroundColor Cyan
Write-Host "Compiler: $compiler"
Write-Host ("Timeout: " + $timeoutSec + "s per test")
Write-Host ""

foreach ($t in $tests) {
    $src = Join-Path $PSScriptRoot $t.f
    if (-not (Test-Path $src)) {
        Write-Host ("[skip]         " + $t.f) -ForegroundColor DarkGray
        continue
    }

    $exeName = "adeb_test_" + ([System.IO.Path]::GetFileNameWithoutExtension($t.f)) + ".exe"
    $exe = Join-Path $env:TEMP $exeName

    & $compiler cc $src -o $exe 2>&1 | Out-Null
    $compileCode = $LASTEXITCODE

    if ($compileCode -ne 0 -or -not (Test-Path $exe)) {
        $compileFail++
        Write-Host ("[FAIL-COMPILE] " + $t.f) -ForegroundColor Red
        continue
    }

    $proc = Start-Process -FilePath $exe -PassThru -NoNewWindow -RedirectStandardOutput "NUL"
    $finished = $proc.WaitForExit($timeoutSec * 1000)

    if (-not $finished) {
        $proc.Kill()
        $hang++
        Write-Host ("[HANG]         " + $t.f + " (>" + $timeoutSec + "s)") -ForegroundColor Yellow
        if (Test-Path $exe) { Remove-Item $exe -Force -ErrorAction SilentlyContinue }
        continue
    }

    $code = $proc.ExitCode
    if ($code -eq $t.e) {
        $pass++
        Write-Host ("[PASS]         " + $t.f + " exit=" + $code) -ForegroundColor Green
    } else {
        $fail++
        Write-Host ("[FAIL]         " + $t.f + " exit=" + $code + " expected=" + $t.e) -ForegroundColor Red
    }
    if (Test-Path $exe) { Remove-Item $exe -Force -ErrorAction SilentlyContinue }
}

$total = $pass + $fail + $hang + $compileFail
Write-Host ""
Write-Host "=== Summary ===" -ForegroundColor Cyan
Write-Host ("  PASS:         " + $pass + " / " + $total) -ForegroundColor Green
Write-Host ("  FAIL:         " + $fail + " / " + $total) -ForegroundColor Red
Write-Host ("  HANG:         " + $hang + " / " + $total) -ForegroundColor Yellow
Write-Host ("  COMPILE-FAIL: " + $compileFail + " / " + $total) -ForegroundColor DarkRed

if ($pass -eq $total -and $total -gt 0) {
    Write-Host ""
    Write-Host "[OK] ALL TESTS PASSED" -ForegroundColor Green
    exit 0
} else {
    exit 1
}
