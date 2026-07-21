# One-click release script for Windows (cargo-release style).
# Usage: .\release.ps1
#
# Flow (mirrors `cargo-release`):
#   1. Read the current workspace version from Cargo.toml.
#      - If it carries a `-dev` pre-release suffix (e.g. 0.0.4-dev) the release
#        version is that base version (0.0.4).
#      - Otherwise the patch component is bumped (0.0.2 -> 0.0.3).
#   2. Write the release version, commit "Release X.Y.Z".
#   3. Create + push the annotated tag vX.Y.Z (this is what CI builds).
#   4. Bump to the next development version X.Y.(Z+1)-dev, commit
#      "Starting next development iteration X.Y.Z-dev", and push.
#
# Cargo.toml is read/written as UTF-8 (no BOM) so non-ASCII characters such as
# the em dash in comments are preserved byte-for-byte.
$ErrorActionPreference = "Stop"

$ProjectRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$CargoToml = Join-Path $ProjectRoot "Cargo.toml"
$Utf8NoBom = New-Object System.Text.UTF8Encoding $false

function Get-WorkspaceVersion {
    $line = Select-String -Path $CargoToml -Pattern '^version\s*=\s*"(\d+\.\d+\.\d+(?:-[0-9A-Za-z.]+)?)"' | Select-Object -First 1
    if (-not $line) {
        throw "Could not find workspace version in $CargoToml"
    }
    return $line.Matches.Groups[1].Value
}

function Bump-PatchVersion($version) {
    # Strip any pre-release suffix first, then bump the patch component.
    $base = ($version -split '-', 2)[0]
    $parts = $base -split '\.'
    $parts[2] = [int]$parts[2] + 1
    return $parts -join '.'
}

function Set-WorkspaceVersion($version) {
    $content = [System.IO.File]::ReadAllText($CargoToml, [System.Text.Encoding]::UTF8)
    $content = $content -replace '(?m)^version\s*=\s*"\d+\.\d+\.\d+(?:-[0-9A-Za-z.]+)?"', "version = `"$version`""
    [System.IO.File]::WriteAllText($CargoToml, $content, $Utf8NoBom)
}

function Assert-LastExit($msg) {
    if ($LASTEXITCODE -ne 0) {
        throw "$msg (exit code $LASTEXITCODE)"
    }
}

# 1. Ensure the working tree is clean so the release is reproducible.
$status = git status --porcelain
if ($status) {
    throw "Working tree is not clean. Commit or stash changes before releasing.`n$status"
}

# 2. Verify the project compiles before tagging.
Write-Host "Running cargo check..."
cargo check
Assert-LastExit "cargo check failed"

# 3. Determine the release version.
#    -dev suffix -> drop it; plain version -> bump patch.
$currentVersion = Get-WorkspaceVersion
if ($currentVersion -match '-') {
    $releaseVersion = ($currentVersion -split '-', 2)[0]
} else {
    $releaseVersion = Bump-PatchVersion $currentVersion
}
$tag = "v$releaseVersion"
Write-Host "Current version: $currentVersion"
Write-Host "Release version: $releaseVersion"

$existing = git tag -l $tag
if ($existing) {
    throw "Tag $tag already exists. Aborting to avoid re-releasing."
}

# 4. Write the release version and commit it.
Set-WorkspaceVersion $releaseVersion
Write-Host "Updating Cargo.lock..."
cargo check
Assert-LastExit "cargo check failed after setting release version"

git add $CargoToml
if (Test-Path (Join-Path $ProjectRoot "Cargo.lock")) {
    git add (Join-Path $ProjectRoot "Cargo.lock")
}
git commit -m "Release $releaseVersion"
Assert-LastExit "git commit (release) failed"

# 5. Create and push the release tag (this is what CI builds).
Write-Host "Creating and pushing tag $tag..."
git tag -a $tag -m "Release $releaseVersion"
Assert-LastExit "git tag failed"
git push origin $tag
Assert-LastExit "git push (tag) failed"

# 6. Bump to the next development version (X.Y.(Z+1)-dev) and commit.
$nextDev = "$(Bump-PatchVersion $releaseVersion)-dev"
Write-Host "Starting next development iteration $nextDev..."
Set-WorkspaceVersion $nextDev
cargo check
Assert-LastExit "cargo check failed after dev bump"

git add $CargoToml
if (Test-Path (Join-Path $ProjectRoot "Cargo.lock")) {
    git add (Join-Path $ProjectRoot "Cargo.lock")
}
git commit -m "Starting next development iteration $nextDev"
Assert-LastExit "git commit (dev bump) failed"
git push
Assert-LastExit "git push failed"

Write-Host ""
Write-Host "Done. Released $tag and bumped workspace version to $nextDev."
