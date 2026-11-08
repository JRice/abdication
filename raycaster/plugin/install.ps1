# Build Raycaster and install it for the current user (no admin needed).
#
#   powershell -ExecutionPolicy Bypass -File plugin\install.ps1
#
# Close any project using Raycaster first: Windows won't overwrite a plugin
# the DAW has loaded. Bitwig keeps plugin processes alive, so quit it fully.

$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot

$env:RUSTFLAGS = "-C target-cpu=x86-64-v2"
cargo run --package xtask --release -- bundle raycaster --release
if ($LASTEXITCODE -ne 0) { throw "Build failed." }

$clapDir = Join-Path $env:LOCALAPPDATA "Programs\Common\CLAP"
$vst3Dir = Join-Path $env:LOCALAPPDATA "Programs\Common\VST3"
New-Item -ItemType Directory -Force -Path $clapDir, $vst3Dir | Out-Null

Copy-Item "target\bundled\Raycaster.clap" $clapDir -Force
Copy-Item "target\bundled\Raycaster.vst3" $vst3Dir -Recurse -Force

Write-Host ""
Write-Host "Installed:"
Write-Host "  CLAP -> $clapDir\Raycaster.clap"
Write-Host "  VST3 -> $vst3Dir\Raycaster.vst3"
Write-Host "Rescan plugins in your DAW."
