# Build Amara and install it for the current user (no admin needed).
#
#   powershell -ExecutionPolicy Bypass -File plugin\install.ps1
#
# Close any project using Amara first: Windows won't overwrite a plugin
# the DAW has loaded.

$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot

$env:RUSTFLAGS = "-C target-cpu=x86-64-v2"
cargo run --package xtask --release -- bundle amara --release
if ($LASTEXITCODE -ne 0) { throw "Build failed." }

$clapDir = Join-Path $env:LOCALAPPDATA "Programs\Common\CLAP"
$vst3Dir = Join-Path $env:LOCALAPPDATA "Programs\Common\VST3"
New-Item -ItemType Directory -Force -Path $clapDir, $vst3Dir | Out-Null

Copy-Item "target\bundled\Amara.clap" $clapDir -Force
Copy-Item "target\bundled\Amara.vst3" $vst3Dir -Recurse -Force

Write-Host ""
Write-Host "Installed:"
Write-Host "  CLAP -> $clapDir\Amara.clap"
Write-Host "  VST3 -> $vst3Dir\Amara.vst3"
Write-Host "Rescan plugins in your DAW."
