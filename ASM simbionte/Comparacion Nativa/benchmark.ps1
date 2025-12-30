# BENCHMARK NATIVO: C++ vs Rust vs ADead-BIB
# Sin Python - Cada lenguaje en su rol nativo

Write-Host ""
Write-Host "======================================================================"
Write-Host "BENCHMARK NATIVO: C++ vs Rust vs ADead-BIB"
Write-Host "======================================================================"
Write-Host ""
Write-Host "Cada lenguaje compila y ejecuta NATIVAMENTE"
Write-Host "Sin Python, sin FFI, sin intermediarios"
Write-Host ""

$results = @{}

# 1. RUST
Write-Host "======================================================================"
Write-Host "[RUST]"
Write-Host "======================================================================"
Write-Host ""

Write-Host "Compilando Rust..."
Push-Location "rust"
cargo build --release 2>&1 | Out-Null
Pop-Location

if (Test-Path "rust\target\release\counter.exe") {
    Write-Host "Ejecutando Rust..."
    $rustTime = Measure-Command { 
        $rustOutput = & "rust\target\release\counter.exe" 
    }
    Write-Host $rustOutput
    Write-Host "Tiempo medido: $($rustTime.TotalSeconds.ToString('F3'))s"
    $results["Rust"] = $rustTime.TotalSeconds
} else {
    Write-Host "Error compilando Rust"
}
Write-Host ""

# 2. ADead-BIB
Write-Host "======================================================================"
Write-Host "[ADead-BIB]"
Write-Host "======================================================================"
Write-Host ""

Write-Host "Compilando ADead-BIB..."
$adeadCompiler = "..\..\target\release\adeadc.exe"
if (-not (Test-Path $adeadCompiler)) {
    $adeadCompiler = "..\..\target\debug\adeadc.exe"
}

& $adeadCompiler build "adead\counter.adB" -o "adead\counter.exe" 2>&1 | Out-Null

if (Test-Path "adead\counter.exe") {
    Write-Host "Ejecutando ADead-BIB..."
    $adeadTime = Measure-Command { 
        $adeadOutput = & "adead\counter.exe" 
    }
    Write-Host "Resultado: $adeadOutput"
    Write-Host "Tiempo medido: $($adeadTime.TotalSeconds.ToString('F3'))s"
    $results["ADead-BIB"] = $adeadTime.TotalSeconds
} else {
    Write-Host "Error compilando ADead-BIB"
}
Write-Host ""

# 3. C++
Write-Host "======================================================================"
Write-Host "[C++]"
Write-Host "======================================================================"
Write-Host ""

$cppCompiled = $false

$clPath = Get-Command cl -ErrorAction SilentlyContinue
if ($clPath) {
    Write-Host "Compilando C++ con MSVC..."
    Push-Location "cpp"
    cl /O2 /EHsc counter.cpp /Fe:counter.exe 2>&1 | Out-Null
    Pop-Location
    $cppCompiled = $true
}

if (-not $cppCompiled) {
    $gppPath = Get-Command g++ -ErrorAction SilentlyContinue
    if ($gppPath) {
        Write-Host "Compilando C++ con g++..."
        Push-Location "cpp"
        g++ -O3 counter.cpp -o counter.exe 2>&1 | Out-Null
        Pop-Location
        $cppCompiled = $true
    }
}

if (-not $cppCompiled) {
    $clangPath = Get-Command clang++ -ErrorAction SilentlyContinue
    if ($clangPath) {
        Write-Host "Compilando C++ con clang++..."
        Push-Location "cpp"
        clang++ -O3 counter.cpp -o counter.exe 2>&1 | Out-Null
        Pop-Location
        $cppCompiled = $true
    }
}

if ($cppCompiled -and (Test-Path "cpp\counter.exe")) {
    Write-Host "Ejecutando C++..."
    $cppTime = Measure-Command { 
        $cppOutput = & "cpp\counter.exe" 
    }
    Write-Host $cppOutput
    Write-Host "Tiempo medido: $($cppTime.TotalSeconds.ToString('F3'))s"
    $results["C++"] = $cppTime.TotalSeconds
} else {
    Write-Host "No hay compilador C++ disponible (cl, g++, clang++)"
}
Write-Host ""

# RESUMEN
Write-Host "======================================================================"
Write-Host "RESUMEN FINAL"
Write-Host "======================================================================"
Write-Host ""

$sortedResults = $results.GetEnumerator() | Sort-Object Value

Write-Host ("{0,-20} {1,-15} {2,-15}" -f "Lenguaje", "Tiempo", "Ranking")
Write-Host ("-" * 50)

$rank = 1
$fastest = $null
foreach ($r in $sortedResults) {
    if ($rank -eq 1) { $fastest = $r.Value }
    $ratio = if ($fastest -gt 0) { ($r.Value / $fastest).ToString("F2") + "x" } else { "1.00x" }
    Write-Host ("{0,-20} {1,-15} {2}" -f $r.Key, "$($r.Value.ToString('F3'))s", $ratio)
    $rank++
}

Write-Host ""
Write-Host "======================================================================"
Write-Host "CONCLUSION"
Write-Host "======================================================================"
Write-Host ""

if ($results.Count -gt 0) {
    $winner = ($sortedResults | Select-Object -First 1).Key
    Write-Host "Mas rapido: $winner"
    Write-Host ""
    
    if ($results.ContainsKey("ADead-BIB") -and $results.ContainsKey("Rust")) {
        $diff = (($results["ADead-BIB"] - $results["Rust"]) / $results["Rust"] * 100)
        if ($diff -gt 0) {
            Write-Host "ADead-BIB es $($diff.ToString('F1'))% mas lento que Rust"
        } else {
            Write-Host "ADead-BIB es $([Math]::Abs($diff).ToString('F1'))% mas rapido que Rust"
        }
    }
}

Write-Host ""
