<#
.SYNOPSIS
    PowerGIS Windows User Installer
.DESCRIPTION
    Installs PowerGIS to %LOCALAPPDATA%\Programs\PowerGIS, creates Start Menu
    and Desktop shortcuts named 'PowerGIS', adds the CLI to the user's PATH,
    and registers .rgis and .geojson file associations.
#>
[CmdletBinding()]
param(
    [string]$InstallDir = "$env:LOCALAPPDATA\Programs\PowerGIS"
)

$ErrorActionPreference = "Stop"

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "     PowerGIS Windows Setup Installer   " -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

# 1. Target directory
Write-Host "`n[1/5] Target Installation Directory:" -ForegroundColor Yellow
Write-Host "  -> $InstallDir"
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

# 2. Copy binaries and assets
Write-Host "`n[2/5] Copying PowerGIS Application Files..." -ForegroundColor Yellow
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path

# Candidate source directories
$CandidateDirs = @(
    $ScriptDir,
    (Join-Path $ScriptDir "..\..\target\release"),
    (Join-Path $ScriptDir "..\..\target\debug"),
    (Join-Path (Get-Location) "target\release"),
    "$env:LOCALAPPDATA\Programs\PowerGIS"
)

$FilesToCopy = @("powergis.exe", "powergis-cli.exe", "LICENSE.txt", "THIRD-PARTY-LICENSES.txt")
foreach ($f in $FilesToCopy) {
    $found = $null
    foreach ($dir in $CandidateDirs) {
        $testPath = Join-Path $dir $f
        if (Test-Path $testPath) {
            $found = $testPath
            break
        }
    }
    if ($found -and (Resolve-Path $found).Path -ne (Resolve-Path (Join-Path $InstallDir $f) -ErrorAction SilentlyContinue).Path) {
        Copy-Item -Path $found -Destination (Join-Path $InstallDir $f) -Force
        Write-Host "  -> Copied $f from $found"
    } elseif (Test-Path (Join-Path $InstallDir $f)) {
        Write-Host "  -> $f already present in $InstallDir"
    }
}

# Verify powergis.exe and powergis-cli.exe exist
if (-not (Test-Path (Join-Path $InstallDir "powergis.exe"))) {
    Write-Warning "powergis.exe was not found in $InstallDir"
}
if (-not (Test-Path (Join-Path $InstallDir "powergis-cli.exe"))) {
    Write-Warning "powergis-cli.exe was not found in $InstallDir"
}

# 3. Create Shortcuts
Write-Host "`n[3/5] Creating Start Menu & Desktop Shortcuts for PowerGIS..." -ForegroundColor Yellow
$WshShell = New-Object -ComObject WScript.Shell

# Desktop shortcut
$DesktopPath = [Environment]::GetFolderPath("Desktop")
$DesktopShortcut = $WshShell.CreateShortcut((Join-Path $DesktopPath "PowerGIS.lnk"))
$DesktopShortcut.TargetPath = Join-Path $InstallDir "powergis.exe"
$DesktopShortcut.WorkingDirectory = $InstallDir
$DesktopShortcut.Description = "PowerGIS - Open Source Geospatial Information System"
$DesktopShortcut.Save()
Write-Host "  -> Desktop Shortcut: PowerGIS.lnk"

# Start Menu shortcut
$StartMenuDir = Join-Path ([Environment]::GetFolderPath("Programs")) "PowerGIS"
if (-not (Test-Path $StartMenuDir)) {
    New-Item -ItemType Directory -Path $StartMenuDir -Force | Out-Null
}
$StartMenuShortcut = $WshShell.CreateShortcut((Join-Path $StartMenuDir "PowerGIS.lnk"))
$StartMenuShortcut.TargetPath = Join-Path $InstallDir "powergis.exe"
$StartMenuShortcut.WorkingDirectory = $InstallDir
$StartMenuShortcut.Description = "PowerGIS"
$StartMenuShortcut.Save()
Write-Host "  -> Start Menu Shortcut: PowerGIS\PowerGIS.lnk"

# 4. Configure PATH
Write-Host "`n[4/5] Adding PowerGIS to User PATH..." -ForegroundColor Yellow
$UserPath = [Environment]::GetEnvironmentVariable("Path", [EnvironmentVariableTarget]::User)
if ($UserPath -notlike "*$InstallDir*") {
    $NewPath = "$InstallDir;$UserPath"
    [Environment]::SetEnvironmentVariable("Path", $NewPath, [EnvironmentVariableTarget]::User)
    Write-Host "  -> Added $InstallDir to PATH."
} else {
    Write-Host "  -> Already in user PATH."
}

# 5. File Associations
Write-Host "`n[5/5] Registering File Associations (.rgis, .geojson)..." -ForegroundColor Yellow
try {
    New-Item -Path "HKCU:\Software\Classes\PowerGIS.Project" -Value "PowerGIS Project" -Force | Out-Null
    New-Item -Path "HKCU:\Software\Classes\PowerGIS.Project\DefaultIcon" -Value "`"$InstallDir\powergis.exe`",0" -Force | Out-Null
    New-Item -Path "HKCU:\Software\Classes\PowerGIS.Project\shell\open\command" -Value "`"$InstallDir\powergis.exe`" `"%1`"" -Force | Out-Null
    New-Item -Path "HKCU:\Software\Classes\.rgis" -Value "PowerGIS.Project" -Force | Out-Null
    New-Item -Path "HKCU:\Software\Classes\.rgisx" -Value "PowerGIS.Project" -Force | Out-Null
    Write-Host "  -> Successfully registered .rgis with PowerGIS."
} catch {
    Write-Warning "Could not register file associations: $_"
}

Write-Host "`n========================================" -ForegroundColor Green
Write-Host "     PowerGIS Installed Successfully!   " -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host "Launch PowerGIS from your Desktop or Start Menu, or run 'powergis' from any terminal.`n"
