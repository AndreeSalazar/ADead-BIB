$compiler = "..\target\release\adeadc.exe"
$file = $args[0]
if (-not $file) { $file = "bench_03_optimizer.adB" }
$exe = $file -replace '\.adB$', '.exe'

Write-Host "Compiling $file ..."
& $compiler build $file -o $exe 2>&1 | Where-Object { $_ -notmatch 'Warning' }

if (Test-Path $exe) {
    Write-Host "Running $exe ..."
    $time = Measure-Command { & ".\$exe" 2>&1 | ForEach-Object { Write-Host $_ } }
    Write-Host "Time: $([math]::Round($time.TotalMilliseconds, 0))ms"
    Remove-Item $exe
} else {
    Write-Host "FAILED to compile"
}
