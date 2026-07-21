#!/usr/bin/env bash
# One-click release script for Linux/macOS/WSL.
# Usage: ./release.sh
#
# Reads the current version from the workspace Cargo.toml, creates an annotated
# Git tag (e.g. v0.0.1), pushes it to origin, then bumps the patch version in
# Cargo.toml (0.0.1 -> 0.0.2), updates Cargo.lock, and commits/pushes the bump.
set -euo pipefail

ProjectRoot="$(cd "$(dirname "$0")" && pwd)"
CargoToml="$ProjectRoot/Cargo.toml"

get_workspace_version() {
    local version
    version=$(grep -E '^version[[:space:]]*=[[:space:]]*"[0-9]+\.[0-9]+\.[0-9]+"' "$CargoToml" | head -1 \
        | sed -E 's/.*"([0-9]+\.[0-9]+\.[0-9]+)".*/\1/')
    if [ -z "$version" ]; then
        echo "Could not find workspace version in $CargoToml" >&2
        exit 1
    fi
    echo "$version"
}

bump_patch_version() {
    local version="$1"
    local major minor patch
    IFS='.' read -r major minor patch <<< "$version"
    echo "$major.$minor.$((patch + 1))"
}

# 1. Ensure the working tree is clean so the release is reproducible.
if [ -n "$(git status --porcelain)" ]; then
    echo "Working tree is not clean. Commit or stash changes before releasing." >&2
    exit 1
fi

# 2. Verify the project compiles before tagging.
echo "Running cargo check..."
cargo check

# 3. Read current version and create the release tag (skip if it already exists).
CurrentVersion=$(get_workspace_version)
Tag="v$CurrentVersion"
echo "Current version: $CurrentVersion"
if git rev-parse "$Tag" >/dev/null 2>&1; then
    echo "Tag $Tag already exists; skipping tag creation and push."
else
    echo "Creating and pushing tag $Tag..."
    git tag -a "$Tag" -m "Release $Tag"
    git push origin "$Tag"
fi

# 4. Bump patch version in Cargo.toml.
NextVersion=$(bump_patch_version "$CurrentVersion")
echo "Bumping version to $NextVersion..."
sed -i -E "0,/^version[[:space:]]*=[[:space:]]*\"[0-9]+\.[0-9]+\.[0-9]+\"/s//version = \"$NextVersion\"/" "$CargoToml"

# 5. Refresh Cargo.lock to match the new version.
echo "Updating Cargo.lock..."
cargo check

# 6. Commit and push the version bump.
git add "$CargoToml"
if [ -f "$ProjectRoot/Cargo.lock" ]; then
    git add "$ProjectRoot/Cargo.lock"
fi
git commit -m "Bump version to $NextVersion"
git push

echo ""
echo "Done. Released $Tag and bumped workspace version to $NextVersion."
