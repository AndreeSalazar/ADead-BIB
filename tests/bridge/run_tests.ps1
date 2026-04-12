# ADead-BIB Bridge Test Runner — Fase 1: C Standard Library
# Usage: powershell -File run_tests.ps1

$compiler = 'c:\Users\andre\OneDrive\Documentos\ADead-BIB\src\rust\target\release\adB.exe'
$testDir  = 'c:\Users\andre\OneDrive\Documentos\ADead-BIB\tests\bridge\fase1_libc'
$outDir   = 'c:\Users\andre\OneDrive\Documentos\ADead-BIB\tests\bridge\out'
if (!(Test-Path $outDir)) { New-Item -ItemType Directory -Path $outDir | Out-Null }

# Discover all .c files in fase1_libc/ sorted by name
$testFiles = Get-ChildItem -Path $testDir -Filter '*.c' | Sort-Object Name

$results = @()

foreach ($f in $testFiles) {
    $name = $f.BaseName
    $src  = $f.FullName
    $exe  = Join-Path $outDir ($name + '.exe')
    Write-Host '=========================================='
    Write-Host ('TEST: ' + $name)
    Write-Host '=========================================='

    Write-Host '--- Compiling ---'
    $compileOutput = & $compiler cc $src -o $exe 2>&1 | Out-String
    $compileExit = $LASTEXITCODE
    if ($compileOutput.Trim()) { Write-Host $compileOutput.Trim() }
    Write-Host ('Compile exit code: ' + $compileExit)

    if ($compileExit -ne 0) {
        Write-Host 'RESULT: COMPILE FAIL'
        $results += [PSCustomObject]@{Test=$name; Compile='FAIL'; Run='SKIP'; Status='FAIL'}
        continue
    }

    Write-Host '--- Running ---'
    $runOutput = & $exe 2>&1 | Out-String
    $runExit = $LASTEXITCODE
    if ($runOutput.Trim()) { Write-Host $runOutput.Trim() }
    Write-Host ('Run exit code: ' + $runExit)

    if ($runExit -eq 0) {
        Write-Host 'RESULT: PASS'
        $results += [PSCustomObject]@{Test=$name; Compile='OK'; Run='OK'; Status='PASS'}
    } else {
        Write-Host 'RESULT: RUN FAIL'
        $results += [PSCustomObject]@{Test=$name; Compile='OK'; Run='FAIL'; Status='FAIL'}
    }
}

Write-Host ''
Write-Host '=========================================='
Write-Host '           SUMMARY'
Write-Host '=========================================='
$results | Format-Table -AutoSize
$pass = ($results | Where-Object { $_.Status -eq 'PASS' }).Count
$fail = ($results | Where-Object { $_.Status -eq 'FAIL' }).Count
Write-Host ('Total: ' + $results.Count + '  PASS: ' + $pass + '  FAIL: ' + $fail)
