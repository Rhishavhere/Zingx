#Requires -RunAsAdministrator
<#
.SYNOPSIS
  Prepare Zingx USB boot partition on Windows (Partition 1 + 2 only for first boot test).

.PARAMETER DriveLetter
  Windows drive letter of the USB stick (e.g. E). The script finds the physical disk and repartitions it.

.EXAMPLE
  .\prepare-usb.ps1 -DriveLetter E
#>
param(
    [Parameter(Mandatory = $true)]
    [string]$DriveLetter
)

$ErrorActionPreference = "Stop"
$Root = Split-Path $PSScriptRoot -Parent
$Payload = Join-Path $Root "artifacts\zingx-usb"

if (-not (Test-Path $Payload)) {
    Write-Error "Missing $Payload. Build first: wsl bash scripts/build.sh"
}

$DriveLetter = $DriveLetter.TrimEnd(':').ToUpper()
$vol = Get-Volume -DriveLetter $DriveLetter
$diskNumber = (Get-Partition -DriveLetter $DriveLetter).DiskNumber

Write-Host "Target disk $diskNumber (currently ${DriveLetter}:)" -ForegroundColor Yellow
Write-Host "ALL DATA ON THIS DISK WILL BE DESTROYED." -ForegroundColor Red
$confirm = Read-Host "Type YES to continue"
if ($confirm -ne "YES") { exit 1 }

# Offline repartition: 1GB FAT32 boot + rest exFAT data
$script = @"
select disk $diskNumber
clean
convert gpt
create partition primary size=1024
format fs=fat32 quick label=ZINGX_BOOT
assign letter=B
create partition primary
format fs=exfat quick label=ZINGX_DATA
assign letter=C
"@
$script | diskpart | Out-Host

Start-Sleep -Seconds 3

$bootMount = "B:"
Write-Host "Copying boot payload to ${bootMount}\"
Copy-Item -Path "$Payload\*" -Destination "${bootMount}\" -Recurse -Force

$dataPart = Get-Partition -DiskNumber $diskNumber | Where-Object { $_.Type -ne 'System' -and $_.Size -gt 2GB } | Select-Object -First 1
if ($dataPart -and -not $dataPart.DriveLetter) {
    $dl = Get-Volume | Where-Object { $_.FileSystemLabel -eq 'ZINGX_DATA' } | Select-Object -First 1
    if ($dl) {
        $dataLetter = $dl.DriveLetter
        New-Item -ItemType Directory -Force -Path "${dataLetter}:\models", "${dataLetter}:\chats", "${dataLetter}:\notes" | Out-Null
    }
}

Write-Host "Done. Set BIOS to USB boot (UEFI). Secure Boot may need to be disabled." -ForegroundColor Green
