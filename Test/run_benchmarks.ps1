# ============================================================
# ADead-BIB -- Benchmark Runner
# ============================================================
# Compila y ejecuta toda la bateria de benchmarks
# Mide tiempo de compilacion + ejecucion
#
# Uso: powershell -ExecutionPolicy Bypass -File run_benchmarks.ps1
# ============================================================

$ErrorActionPreference = "Continue"
$compiler = "..\target\release\adeadc.exe"

# Verificar que el compilador existe
if (-not (Test-Path $compiler)) {
    Write-Host "[!] Compilador no encontrado en $compiler"
    Write-Host "    Intentando con: cargo build --release"
    Push-Location ..
    cargo build --release
    Pop-Location
    if (-not (Test-Path $compiler)) {
        Write-Host "[ERROR] No se pudo compilar. Abortando."
        exit 1
    }
}

Write-Host ""
Write-Host "============================================================"
Write-Host "        ADead-BIB -- Benchmark Suite v1.0"
Write-Host "============================================================"
Write-Host ""

$benchmarks = @(
    @{ file = "bench_01_cpu_integers.adB";     name = "01 CPU Integer ALU" },
    @{ file = "bench_02_cpu_floats.adB";       name = "02 CPU Float FixedPoint" },
    @{ file = "bench_03_optimizer.adB";        name = "03 Compiler Optimizations" },
    @{ file = "bench_04_memory.adB";           name = "04 Memory Registers" },
    @{ file = "bench_05_branch.adB";           name = "05 Branch Prediction" },
    @{ file = "bench_06_bitwise.adB";          name = "06 Bitwise Operations" },
    @{ file = "bench_07_real_algorithms.adB";  name = "07 Real Algorithms" },
    @{ file = "bench_08_math_intense.adB";     name = "08 Intensive Math" },
    @{ file = "bench_09_stress.adB";           name = "09 Stress Tests" },
    @{ file = "bench_10_data_structures.adB";  name = "10 Data Structures" },
    @{ file = "bench_11_sorting.adB";          name = "11 Sorting Algorithms" },
    @{ file = "bench_12_crypto.adB";           name = "12 Crypto & Hashing" },
    @{ file = "bench_13_pathfinding.adB";      name = "13 Pathfinding & Graphs" },
    @{ file = "bench_14_oop_patterns.adB";     name = "14 OOP-Light Patterns" }
)

$results = @()
$totalCompile = 0
$totalRun = 0

foreach ($bench in $benchmarks) {
    $file = $bench.file
    $name = $bench.name
    $exe = $file -replace '\.adB$', '.exe'

    if (-not (Test-Path $file)) {
        Write-Host "[SKIP] $name -- archivo no encontrado: $file"
        continue
    }

    Write-Host "------------------------------------------------------------"
    Write-Host "[BUILD] $name ..."

    # Compilar
    $compileTime = Measure-Command {
        $compileOutput = & $compiler build $file -o $exe 2>&1
    }
    $compileMs = [math]::Round($compileTime.TotalMilliseconds, 2)
    $totalCompile += $compileMs

    if (-not (Test-Path $exe)) {
        Write-Host "  [FAIL] Compilacion fallo"
        Write-Host "  $compileOutput"
        continue
    }

    $fileSize = (Get-Item $exe).Length
    Write-Host "  [OK] $exe -- $fileSize bytes -- ${compileMs}ms"

    # Ejecutar
    Write-Host "[RUN]   $name ..."
    $runTime = Measure-Command {
        $runOutput = & ".\$exe" 2>&1
    }
    $runMs = [math]::Round($runTime.TotalMilliseconds, 2)
    $totalRun += $runMs

    foreach ($line in $runOutput) {
        Write-Host "  $line"
    }
    Write-Host "  [TIME] ${runMs}ms"

    $results += [PSCustomObject]@{
        Benchmark    = $name
        CompileMs    = $compileMs
        RunMs        = $runMs
        BinaryBytes  = $fileSize
    }

    # Limpiar exe
    Remove-Item $exe -ErrorAction SilentlyContinue
}

# ============================================================
# Resultados finales
# ============================================================
Write-Host ""
Write-Host "============================================================"
Write-Host "                    RESULTADOS"
Write-Host "============================================================"
Write-Host ""

$results | Format-Table -AutoSize

Write-Host "------------------------------------------------------------"
Write-Host "Total compilacion: ${totalCompile}ms"
Write-Host "Total ejecucion:   ${totalRun}ms"
$totalGeneral = [math]::Round($totalCompile + $totalRun, 2)
Write-Host "Total general:     ${totalGeneral}ms"
Write-Host ""

# ============================================================
# Comparacion con Rust (si esta disponible)
# ============================================================
if (Test-Path "bench_comparison_rust.rs") {
    Write-Host "------------------------------------------------------------"
    Write-Host "[RUST] Compilando benchmark Rust para comparacion..."

    $rustCompile = Measure-Command {
        $rustOut = rustc -O bench_comparison_rust.rs -o bench_rust.exe 2>&1
    }

    if (Test-Path "bench_rust.exe") {
        $rustCompileMs = [math]::Round($rustCompile.TotalMilliseconds, 2)
        Write-Host "  [OK] Rust compilado en ${rustCompileMs}ms"
        Write-Host ""
        Write-Host "[RUST] Ejecutando benchmark Rust..."

        $rustRun = Measure-Command {
            $rustOutput = & ".\bench_rust.exe" 2>&1
        }

        foreach ($line in $rustOutput) {
            Write-Host "  $line"
        }

        $rustRunMs = [math]::Round($rustRun.TotalMilliseconds, 2)

        Write-Host ""
        Write-Host "Rust total ejecucion:      ${rustRunMs}ms"
        Write-Host "ADead-BIB total ejecucion: ${totalRun}ms"

        if ($rustRunMs -gt 0) {
            $ratio = [math]::Round($totalRun / $rustRunMs, 2)
            Write-Host "Ratio ADead/Rust:          ${ratio}x"
        }

        Remove-Item bench_rust.exe -ErrorAction SilentlyContinue
    } else {
        Write-Host "  [!] rustc no disponible o compilacion fallo"
        Write-Host "  $rustOut"
    }
}

# ============================================================
# Comparacion con C (si gcc esta disponible)
# ============================================================
if (Test-Path "bench_comparison_c.c") {
    Write-Host "------------------------------------------------------------"
    Write-Host "[C]    Compilando benchmark C para comparacion..."

    $cCompile = Measure-Command {
        $cOut = gcc -O3 bench_comparison_c.c -o bench_c.exe 2>&1
    }

    if (Test-Path "bench_c.exe") {
        $cCompileMs = [math]::Round($cCompile.TotalMilliseconds, 2)
        Write-Host "  [OK] C compilado en ${cCompileMs}ms"
        Write-Host ""
        Write-Host "[C]    Ejecutando benchmark C..."

        $cRun = Measure-Command {
            $cOutput = & ".\bench_c.exe" 2>&1
        }

        foreach ($line in $cOutput) {
            Write-Host "  $line"
        }

        $cRunMs = [math]::Round($cRun.TotalMilliseconds, 2)

        Write-Host ""
        Write-Host "C total ejecucion:         ${cRunMs}ms"
        Write-Host "ADead-BIB total ejecucion: ${totalRun}ms"

        if ($cRunMs -gt 0) {
            $ratioC = [math]::Round($totalRun / $cRunMs, 2)
            Write-Host "Ratio ADead/C:             ${ratioC}x"
        }

        Remove-Item bench_c.exe -ErrorAction SilentlyContinue
    } else {
        Write-Host "  [!] gcc no disponible o compilacion fallo"
        Write-Host "  $cOut"
    }
}

Write-Host ""
Write-Host "[DONE] Benchmark suite completo."
