# Build and install every plugin in this repo for the current user.
#
#   powershell -ExecutionPolicy Bypass -File build-all.ps1                    # all of them
#   powershell -ExecutionPolicy Bypass -File build-all.ps1 argent raycaster   # just these
#
# Runs each folder's plugin\install.ps1 in its own PowerShell process, so one
# script's settings (ABDrum's RUSTFLAGS, the working folder) don't leak into the
# next. A failed build doesn't stop the rest: the summary lists it, and the script
# exits non-zero. Quit Bitwig fully first: Windows won't overwrite a loaded plugin.

param([Parameter(ValueFromRemainingArguments)] [string[]] $Only)

$ErrorActionPreference = "Stop"
$shell = (Get-Process -Id $PID).Path   # the same PowerShell this was started with

function Get-InstallScript($folder) { Join-Path $folder.FullName "plugin\install.ps1" }

$plugins = Get-ChildItem $PSScriptRoot -Directory |
    Where-Object { Test-Path (Get-InstallScript $_) }

if ($Only) {
    $unknown = $Only | Where-Object { $plugins.Name -notcontains $_ }
    if ($unknown) { throw "No plugin named: $($unknown -join ', '). Available: $($plugins.Name -join ', ')" }
    $plugins = $plugins | Where-Object { $Only -contains $_.Name }
}

$results = [Collections.Generic.List[object]]::new()
foreach ($plugin in $plugins) {
    Write-Host "`n=== $($plugin.Name) ===" -ForegroundColor Cyan
    $clock = [Diagnostics.Stopwatch]::StartNew()
    & $shell -NoProfile -ExecutionPolicy Bypass -File (Get-InstallScript $plugin)
    $results.Add([pscustomobject]@{
        Plugin  = $plugin.Name
        Result  = if ($LASTEXITCODE -eq 0) { "ok" } else { "FAILED" }
        Minutes = [math]::Round($clock.Elapsed.TotalMinutes, 1)
    })
}

$results | Format-Table -AutoSize | Out-Host
$failed = @($results | Where-Object Result -ne "ok")
if ($failed) {
    Write-Host "Failed: $($failed.Plugin -join ', ')" -ForegroundColor Red
    exit 1
}
Write-Host "All installed. Rescan plugins in Bitwig." -ForegroundColor Green
