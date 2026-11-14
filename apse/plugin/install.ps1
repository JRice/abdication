# Build Apse and install it for the current user (no admin needed).
#
#   powershell -ExecutionPolicy Bypass -File plugin\install.ps1
#
# Close any project using Apse first: Windows won't overwrite a plugin
# the DAW has loaded.

$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot

$env:RUSTFLAGS = "-C target-cpu=x86-64-v2"
cargo run --package xtask --release -- bundle apse --release
if ($LASTEXITCODE -ne 0) { throw "Build failed." }

$clapDir = Join-Path $env:LOCALAPPDATA "Programs\Common\CLAP"
$vst3Dir = Join-Path $env:LOCALAPPDATA "Programs\Common\VST3"
New-Item -ItemType Directory -Force -Path $clapDir, $vst3Dir | Out-Null

Copy-Item "target\bundled\Apse.clap" $clapDir -Force
Copy-Item "target\bundled\Apse.vst3" $vst3Dir -Recurse -Force

Write-Host ""
Write-Host "Installed:"
Write-Host "  CLAP -> $clapDir\Apse.clap"
Write-Host "  VST3 -> $vst3Dir\Apse.vst3"
Write-Host "Rescan plugins in your DAW."
