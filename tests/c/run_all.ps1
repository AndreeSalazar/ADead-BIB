$compiler = "c:\Users\andre\OneDrive\Documentos\ADead-BIB\src\rust\target\release\adB.exe"
$fixtures = "c:\Users\andre\OneDrive\Documentos\ADead-BIB\tests\c\fixtures"
$outdir   = "c:\Users\andre\OneDrive\Documentos\ADead-BIB\tests\c\output"

if (-not (Test-Path $outdir)) { New-Item -ItemType Directory -Path $outdir -Force | Out-Null }

$results = @()
$files = Get-ChildItem "$fixtures\*.c" | Sort-Object Name

foreach ($f in $files) {
    $name = $f.BaseName
    $exe  = "$outdir\$name.exe"
    
    # Compile
    $compOut = & $compiler cc $f.FullName -o $exe 2>&1 | Out-String
    $compOk  = Test-Path $exe
    
    if (-not $compOk) {
        $results += [PSCustomObject]@{
            Test     = $name
            Compiled = "FAIL"
            ExitCode = "N/A"
            Output   = "(compile failed)"
        }
        continue
    }
    
    # Run with timeout
    try {
        $pinfo = New-Object System.Diagnostics.ProcessStartInfo
        $pinfo.FileName = $exe
        $pinfo.RedirectStandardOutput = $true
        $pinfo.RedirectStandardError  = $true
        $pinfo.UseShellExecute = $false
        $pinfo.CreateNoWindow = $true
        $proc = [System.Diagnostics.Process]::Start($pinfo)
        $stdout = $proc.StandardOutput.ReadToEnd()
        $stderr = $proc.StandardError.ReadToEnd()
        $finished = $proc.WaitForExit(5000) # 5 sec timeout
        if (-not $finished) {
            $proc.Kill()
            $results += [PSCustomObject]@{
                Test     = $name
                Compiled = "OK"
                ExitCode = "TIMEOUT"
                Output   = "(killed after 5s)"
            }
        } else {
            $results += [PSCustomObject]@{
                Test     = $name
                Compiled = "OK"
                ExitCode = $proc.ExitCode
                Output   = ($stdout + $stderr).Trim()
            }
        }
    } catch {
        $results += [PSCustomObject]@{
            Test     = $name
            Compiled = "OK"
            ExitCode = "CRASH"
            Output   = $_.Exception.Message
        }
    }
}

# Print summary
Write-Host "`n=========================================="
Write-Host " ADead-BIB C Test Suite — Runtime Results"
Write-Host "==========================================`n"

$pass = 0; $fail = 0; $crash = 0; $noout = 0

foreach ($r in $results) {
    $status = if ($r.Compiled -eq "FAIL") { "COMPILE_FAIL" }
              elseif ($r.ExitCode -eq "TIMEOUT") { "TIMEOUT" }
              elseif ($r.ExitCode -eq "CRASH") { "CRASH" }
              elseif ($r.ExitCode -eq 0 -and $r.Output.Length -gt 0) { "PASS" }
              elseif ($r.ExitCode -eq 0 -and $r.Output.Length -eq 0) { "NO_OUTPUT" }
              else { "EXIT_$($r.ExitCode)" }
    
    $icon = switch ($status) {
        "PASS"         { "[OK]" }
        "NO_OUTPUT"    { "[--]" }
        "COMPILE_FAIL" { "[CF]" }
        "TIMEOUT"      { "[TO]" }
        "CRASH"        { "[!!]" }
        default        { "[EC]" }
    }
    
    if ($status -eq "PASS") { $pass++ }
    elseif ($status -eq "NO_OUTPUT") { $noout++ }
    elseif ($status -eq "CRASH" -or $status -eq "TIMEOUT") { $crash++ }
    else { $fail++ }
    
    $outPreview = $r.Output
    if ($outPreview.Length -gt 120) { $outPreview = $outPreview.Substring(0,120) + '...' }
    $outPreview = $outPreview.Replace([char]13+[char]10, ' | ').Replace([char]10, ' | ')
    
    $line = $icon + ' ' + $r.Test + ' [exit=' + $r.ExitCode + ']'
    Write-Host $line
    if ($r.Output.Length -gt 0) {
        Write-Host ('     ' + $outPreview)
    }
}

Write-Host ''
Write-Host '------------------------------------------'
Write-Host ('PASS=' + $pass + '  NO_OUTPUT=' + $noout + '  FAIL=' + $fail + '  CRASH=' + $crash + '  TOTAL=' + $results.Count)
Write-Host '------------------------------------------'
