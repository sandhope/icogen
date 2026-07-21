# One-click release script for Windows.
# Usage: .\release.ps1
#
# Reads the current version from the workspace Cargo.toml, creates an annotated
# Git tag (e.g. v0.0.1), pushes it to origin, then bumps the patch version in
# Cargo.toml (0.0.1 -> 0.0.2), updates Cargo.lock, and commits/pushes the bump.
#
# Cargo.toml is read/written as UTF-8 (no BOM) so non-ASCII characters such as
# the em dash in comments are preserved byte-for-byte.
$ErrorActionPreference = "Stop"

$ProjectRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$CargoToml = Join-Path $ProjectRoot "Cargo.toml"
$Utf8NoBom = New-Object System.Text.UTF8Encoding $false

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

# 3. Read current version and create the release tag (skip if it already exists).
$currentVersion = Get-WorkspaceVersion
$tag = "v$currentVersion"
Write-Host "Current version: $currentVersion"

$existing = git tag -l $tag
if ($existing) {
    Write-Host "Tag $tag already exists; skipping tag creation and push."
} else {
    Write-Host "Creating and pushing tag $tag..."
    git tag -a $tag -m "Release $tag"
    Assert-LastExit "git tag failed"
    git push origin $tag
    Assert-LastExit "git push (tag) failed"
}

# 4. Bump patch version in Cargo.toml, preserving all other bytes (UTF-8, no BOM).
$nextVersion = Bump-PatchVersion $currentVersion
Write-Host "Bumping version to $nextVersion..."
$content = [System.IO.File]::ReadAllText($CargoToml, [System.Text.Encoding]::UTF8)
$content = $content -replace '(?m)^version\s*=\s*"\d+\.\d+\.\d+"', "version = `"$nextVersion`""
[System.IO.File]::WriteAllText($CargoToml, $content, $Utf8NoBom)

# 5. Refresh Cargo.lock to match the new version.
Write-Host "Updating Cargo.lock..."
cargo check
Assert-LastExit "cargo check failed after bump"

# 6. Commit and push the version bump.
git add $CargoToml
if (Test-Path (Join-Path $ProjectRoot "Cargo.lock")) {
    git add (Join-Path $ProjectRoot "Cargo.lock")
}
git commit -m "Bump version to $nextVersion"
Assert-LastExit "git commit failed"
git push
Assert-LastExit "git push failed"

Write-Host ""
Write-Host "Done. Released $tag and bumped workspace version to $nextVersion."
