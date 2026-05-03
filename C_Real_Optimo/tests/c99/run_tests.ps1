$tests = @(
    "01_variables",
    "02_arithmetic", 
    "03_if_else",
    "04_while_loop",
    "05_for_loop",
    "06_functions",
    "07_recursion",
    "08_pointers",
    "09_arrays",
    "10_structs"
)

$passed = 0
$failed = 0
$results = @()

foreach ($test in $tests) {
    Write-Host "`n=== $test ===" -ForegroundColor Cyan
    
    # Compile
    $compileResult = & ..\..\..\target\debug\adB.exe cc "$test.c" -o "$test.exe" 2>&1
    $compileCode = $LASTEXITCODE
    
    if ($compileCode -ne 0) {
        Write-Host "  COMPILE FAIL" -ForegroundColor Red
        $results += "$test : COMPILE FAIL"
        $failed++
        continue
    }
    
    Write-Host "  Compiled OK" -ForegroundColor Green
    
    # Run
    & ".\$test.exe" 2>&1 | Out-Null
    $runCode = $LASTEXITCODE
    
    if ($runCode -eq 0) {
        Write-Host "  RUN OK (exit=0)" -ForegroundColor Green
        $results += "$test : PASS"
        $passed++
    } else {
        Write-Host "  RUN FAIL (exit=$runCode)" -ForegroundColor Yellow
        $results += "$test : RUN FAIL (exit=$runCode)"
        $failed++
    }
}

Write-Host "`n========== SUMMARY ==========" -ForegroundColor White
foreach ($r in $results) {
    if ($r -match "PASS") {
        Write-Host $r -ForegroundColor Green
    } else {
        Write-Host $r -ForegroundColor Red
    }
}
Write-Host "`nPassed: $passed / $($passed + $failed)" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Yellow" })
