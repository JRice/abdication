# Build Abdication and install it for the current user (no admin needed).
#
#   powershell -ExecutionPolicy Bypass -File plugin\install.ps1
#
# Close any project using Abdication first: Windows won't overwrite a
# plugin the DAW has loaded.

$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot

cargo xtask bundle abdication --release
if ($LASTEXITCODE -ne 0) { throw "Build failed." }

$clapDir = Join-Path $env:LOCALAPPDATA "Programs\Common\CLAP"
$vst3Dir = Join-Path $env:LOCALAPPDATA "Programs\Common\VST3"
New-Item -ItemType Directory -Force -Path $clapDir, $vst3Dir | Out-Null

Copy-Item "target\bundled\Abdication.clap" $clapDir -Force
Copy-Item "target\bundled\Abdication.vst3" $vst3Dir -Recurse -Force

Write-Host ""
Write-Host "Installed:"
Write-Host "  CLAP -> $clapDir\Abdication.clap"
Write-Host "  VST3 -> $vst3Dir\Abdication.vst3"
Write-Host "Rescan plugins in your DAW."
