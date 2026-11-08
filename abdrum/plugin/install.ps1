# Build ABDrum and install it for the current user (no admin needed).
#
#   powershell -ExecutionPolicy Bypass -File plugin\install.ps1
#
# Close any project using ABDrum first: Windows won't overwrite a plugin
# the DAW has loaded.

$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot

# x86-64-v2 (any x86 CPU from roughly the last 15 years) adds SSE4.1's
# rounding instructions; without them floor() runs in software, and the
# DSP calls it constantly. (Abdication sets this in .cargo/config.toml.)
$env:RUSTFLAGS = "-C target-cpu=x86-64-v2"
# With drag-and-drop export (Windows-only code, written without a Windows
# machine to compile it on: if it won't build, the plugin still builds
# without it, and the export button saves files instead of dragging them).
$ErrorActionPreference = "Continue"
cargo run --package xtask --release -- bundle abdrum --release --features drag
if ($LASTEXITCODE -ne 0) {
    Write-Warning "Drag-and-drop export didn't build (see the errors above); building without it."
    cargo run --package xtask --release -- bundle abdrum --release
    if ($LASTEXITCODE -ne 0) { throw "Build failed." }
}
$ErrorActionPreference = "Stop"

$clapDir = Join-Path $env:LOCALAPPDATA "Programs\Common\CLAP"
$vst3Dir = Join-Path $env:LOCALAPPDATA "Programs\Common\VST3"
New-Item -ItemType Directory -Force -Path $clapDir, $vst3Dir | Out-Null

Copy-Item "target\bundled\ABDrum.clap" $clapDir -Force
Copy-Item "target\bundled\ABDrum.vst3" $vst3Dir -Recurse -Force

Write-Host ""
Write-Host "Installed:"
Write-Host "  CLAP -> $clapDir\ABDrum.clap"
Write-Host "  VST3 -> $vst3Dir\ABDrum.vst3"
Write-Host "Rescan plugins in your DAW."
