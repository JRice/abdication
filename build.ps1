# Build and install every plugin in this repo, then zip their VST3 bundles.
#
#   powershell -ExecutionPolicy Bypass -File build.ps1                  # all of them
#   powershell -ExecutionPolicy Bypass -File build.ps1 calls abdication # just these, zip still has all
#
# The building is build-all.ps1's (each plugin\install.ps1 in its own process;
# plugins already up to date take seconds). Then every plugin's
# plugin\target\bundled\*.vst3 goes into "evils yyyyMMdd.zip" here, replacing
# one from earlier the same day. A plugin whose build failed goes in as it last
# built, and is named in the summary. Quit Bitwig fully first: Windows won't
# overwrite a loaded plugin.

param([Parameter(ValueFromRemainingArguments)] [string[]] $Only)

$ErrorActionPreference = "Stop"
$shell = (Get-Process -Id $PID).Path

& $shell -NoProfile -ExecutionPolicy Bypass -File (Join-Path $PSScriptRoot "build-all.ps1") @Only
$buildFailed = $LASTEXITCODE -ne 0

$bundles = Get-ChildItem $PSScriptRoot -Directory |
    ForEach-Object { Join-Path $_.FullName "plugin\target\bundled" } |
    Where-Object { Test-Path $_ } |
    ForEach-Object { Get-ChildItem $_ -Directory -Filter "*.vst3" }

if (-not $bundles) { throw "No .vst3 bundles found under */plugin/target/bundled." }

$zip = Join-Path $PSScriptRoot ("evils {0}.zip" -f (Get-Date -Format "yyyyMMdd"))
Compress-Archive -Path $bundles.FullName -DestinationPath $zip -Force

Write-Host ""
Write-Host "Zipped $($bundles.Count) VST3 bundles -> $zip" -ForegroundColor Green
$bundles | ForEach-Object { Write-Host "  $($_.Name)  (built $($_.LastWriteTime.ToString('yyyy-MM-dd HH:mm')))" }
if ($buildFailed) {
    Write-Host "Some builds failed (see the table above): the zip holds their last good bundles." -ForegroundColor Red
    exit 1
}
