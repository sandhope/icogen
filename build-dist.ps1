# Build all four binaries and copy them into dist/ (project root).
# Run from anywhere:  .\build-dist.ps1
$ErrorActionPreference = "Stop"

$ProjectRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$DistDir     = Join-Path $ProjectRoot "dist"
$ReleaseDir  = Join-Path $DistDir "release"

cargo build --release `
    -p icogen `
    -p icogen-gui `
    -p icogen-assets `
    -p icogen-assets-gui

if (-not (Test-Path $ReleaseDir)) {
    throw "Build output not found at $ReleaseDir"
}

$exes = @("icogen.exe", "icogen-gui.exe", "icogen-assets.exe", "icogen-assets-gui.exe")
foreach ($exe in $exes) {
    $src = Join-Path $ReleaseDir $exe
    if (-not (Test-Path $src)) { throw "Missing build artifact: $src" }
    Copy-Item $src $DistDir -Force
    Write-Host "OK  copied $exe -> dist/"
}

Write-Host ""
Write-Host "Done. Binaries are in dist/:"
foreach ($exe in $exes) { Write-Host "  dist/$exe" }
