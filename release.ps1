# One-click release script for Windows.
# Usage: .\release.ps1
#
# Reads the current version from the workspace Cargo.toml, creates an annotated
# Git tag (e.g. v0.0.1), pushes it to origin, then bumps the patch version in
# Cargo.toml (0.0.1 -> 0.0.2), updates Cargo.lock, and commits/pushes the bump.
$ErrorActionPreference = "Stop"

$ProjectRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$CargoToml = Join-Path $ProjectRoot "Cargo.toml"

function Get-WorkspaceVersion {
    $line = Select-String -Path $CargoToml -Pattern '^version\s*=\s*"(\d+\.\d+\.\d+)"' | Select-Object -First 1
    if (-not $line) {
        throw "Could not find workspace version in $CargoToml"
    }
    return $line.Matches.Groups[1].Value
}

function Bump-PatchVersion($version) {
    $parts = $version -split '\.'
    $parts[2] = [int]$parts[2] + 1
    return $parts -join '.'
}

# 1. Ensure the working tree is clean so the release is reproducible.
$status = git status --porcelain
if ($status) {
    throw "Working tree is not clean. Commit or stash changes before releasing.`n$status"
}

# 2. Verify the project compiles before tagging.
Write-Host "Running cargo check..."
cargo check

# 3. Read current version and create the release tag.
$currentVersion = Get-WorkspaceVersion
$tag = "v$currentVersion"
Write-Host "Current version: $currentVersion"
Write-Host "Creating and pushing tag $tag..."
git tag -a $tag -m "Release $tag"
git push origin $tag

# 4. Bump patch version in Cargo.toml.
$nextVersion = Bump-PatchVersion $currentVersion
Write-Host "Bumping version to $nextVersion..."

$toml = Get-Content $CargoToml
$updated = $false
for ($i = 0; $i -lt $toml.Length; $i++) {
    if ($toml[$i] -match '^version\s*=\s*"\d+\.\d+\.\d+"') {
        $toml[$i] = "version = `"$nextVersion`""
        $updated = $true
        break
    }
}
if (-not $updated) {
    throw "Could not update version line in $CargoToml"
}
Set-Content -Path $CargoToml -Value $toml

# 5. Refresh Cargo.lock to match the new version.
Write-Host "Updating Cargo.lock..."
cargo check

# 6. Commit and push the version bump.
git add $CargoToml
if (Test-Path (Join-Path $ProjectRoot "Cargo.lock")) {
    git add (Join-Path $ProjectRoot "Cargo.lock")
}
git commit -m "Bump version to $nextVersion [skip ci]"
git push

Write-Host ""
Write-Host "Done. Released $tag and bumped workspace version to $nextVersion."
