$compiler = 'c:\Users\andre\OneDrive\Documentos\ADead-BIB\src\rust\target\release\adB.exe'
$testDir = 'c:\Users\andre\OneDrive\Documentos\ADead-BIB\tests\bridge'
$outDir = Join-Path $testDir 'out'
if (!(Test-Path $outDir)) { New-Item -ItemType Directory -Path $outDir | Out-Null }

$tests = @(
    '01_console_hello',
    '02_string_ops',
    '03_math_logic',
    '04_memory_alloc',
    '05_control_flow',
    '06_structs_unions',
    '17_recursion_deep',
    '18_bitfields',
    '20_enum_switch',
    '21_linked_list',
    '23_float_math'
)

$results = @()

foreach ($t in $tests) {
    $src = Join-Path $testDir ($t + '.c')
    $exe = Join-Path $outDir ($t + '.exe')
    Write-Host '=========================================='
    Write-Host ('TEST: ' + $t)
    Write-Host '=========================================='

    Write-Host '--- Compiling ---'
    $compileOutput = & $compiler cc $src -o $exe 2>&1 | Out-String
    $compileExit = $LASTEXITCODE
    if ($compileOutput.Trim()) { Write-Host $compileOutput.Trim() }
    Write-Host ('Compile exit code: ' + $compileExit)

    if ($compileExit -ne 0) {
        Write-Host 'RESULT: COMPILE FAIL'
        $results += [PSCustomObject]@{Test=$t; Compile='FAIL'; Run='SKIP'; Status='FAIL'}
        continue
    }

    Write-Host '--- Running ---'
    $runOutput = & $exe 2>&1 | Out-String
    $runExit = $LASTEXITCODE
    if ($runOutput.Trim()) { Write-Host $runOutput.Trim() }
    Write-Host ('Run exit code: ' + $runExit)

    if ($runExit -eq 0) {
        Write-Host 'RESULT: PASS'
        $results += [PSCustomObject]@{Test=$t; Compile='OK'; Run='OK'; Status='PASS'}
    } else {
        Write-Host 'RESULT: RUN FAIL'
        $results += [PSCustomObject]@{Test=$t; Compile='OK'; Run='FAIL'; Status='FAIL'}
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
