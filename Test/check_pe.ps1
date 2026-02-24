$file = $args[0]
if (-not $file) { $file = "test01.exe" }
$bytes = [System.IO.File]::ReadAllBytes($file)
Write-Host "Size: $($bytes.Length)"
Write-Host "MZ: $([char]$bytes[0])$([char]$bytes[1])"
$lfanew = [BitConverter]::ToInt32($bytes, 0x3C)
Write-Host "e_lfanew: 0x$($lfanew.ToString('X'))"
Write-Host "PE sig: $([char]$bytes[$lfanew])$([char]$bytes[$lfanew+1])"
$machine = [BitConverter]::ToUInt16($bytes, $lfanew+4)
Write-Host "Machine: 0x$($machine.ToString('X4'))"
$numSec = [BitConverter]::ToUInt16($bytes, $lfanew+6)
Write-Host "Sections: $numSec"
$optHdrSize = [BitConverter]::ToUInt16($bytes, $lfanew+20)
Write-Host "OptHdrSize: $optHdrSize"
$chars = [BitConverter]::ToUInt16($bytes, $lfanew+22)
Write-Host "Characteristics: 0x$($chars.ToString('X4'))"
$optMagic = [BitConverter]::ToUInt16($bytes, $lfanew+24)
Write-Host "OptMagic: 0x$($optMagic.ToString('X4'))"
$entry = [BitConverter]::ToUInt32($bytes, $lfanew+40)
Write-Host "EntryRVA: 0x$($entry.ToString('X'))"
$imgBase = [BitConverter]::ToUInt64($bytes, $lfanew+48)
Write-Host "ImageBase: 0x$($imgBase.ToString('X'))"
$secAlign = [BitConverter]::ToUInt32($bytes, $lfanew+56)
Write-Host "SectionAlign: 0x$($secAlign.ToString('X'))"
$fileAlign = [BitConverter]::ToUInt32($bytes, $lfanew+60)
Write-Host "FileAlign: 0x$($fileAlign.ToString('X'))"
$sizeImg = [BitConverter]::ToUInt32($bytes, $lfanew+80)
Write-Host "SizeOfImage: 0x$($sizeImg.ToString('X'))"
$sizeHdr = [BitConverter]::ToUInt32($bytes, $lfanew+84)
Write-Host "SizeOfHeaders: 0x$($sizeHdr.ToString('X'))"
$subsys = [BitConverter]::ToUInt16($bytes, $lfanew+92)
Write-Host "Subsystem: $subsys"
$numDir = [BitConverter]::ToUInt32($bytes, $lfanew+132)
Write-Host "NumberOfRvaAndSizes: $numDir"
# Import directory
$impRva = [BitConverter]::ToUInt32($bytes, $lfanew+144)
$impSize = [BitConverter]::ToUInt32($bytes, $lfanew+148)
Write-Host "ImportDir RVA: 0x$($impRva.ToString('X')), Size: $impSize"
# IAT directory (index 12)
$iatRva = [BitConverter]::ToUInt32($bytes, $lfanew+232)
$iatSize = [BitConverter]::ToUInt32($bytes, $lfanew+236)
Write-Host "IAT Dir RVA: 0x$($iatRva.ToString('X')), Size: $iatSize"
# DllCharacteristics
$dllChars = [BitConverter]::ToUInt16($bytes, $lfanew+94)
Write-Host "DllCharacteristics: 0x$($dllChars.ToString('X4'))"
