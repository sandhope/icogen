#!/usr/bin/env bash
# One-click release script for Linux/macOS/WSL (cargo-release style).
# Usage: ./release.sh
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
set -euo pipefail

ProjectRoot="$(cd "$(dirname "$0")" && pwd)"
CargoToml="$ProjectRoot/Cargo.toml"

get_workspace_version() {
    local version
    version=$(grep -E '^version[[:space:]]*=[[:space:]]*"[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.]+)?"' "$CargoToml" | head -1 \
        | sed -E 's/.*"([0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.]+)?)".*/\1/')
    if [ -z "$version" ]; then
        echo "Could not find workspace version in $CargoToml" >&2
        exit 1
    fi
    echo "$version"
}

bump_patch_version() {
    # Strip any pre-release suffix first, then bump the patch component.
    local version="${1%%-*}"
    local major minor patch
    IFS='.' read -r major minor patch <<< "$version"
    echo "$major.$minor.$((patch + 1))"
}

set_workspace_version() {
    local version="$1"
    sed -i -E "0,/^version[[:space:]]*=[[:space:]]*\"[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.]+)?\"/s//version = \"$version\"/" "$CargoToml"
}

# 1. Ensure the working tree is clean so the release is reproducible.
if [ -n "$(git status --porcelain)" ]; then
    echo "Working tree is not clean. Commit or stash changes before releasing." >&2
    exit 1
fi

# 2. Verify the project compiles before tagging.
echo "Running cargo check..."
cargo check

# 3. Determine the release version.
#    -dev suffix -> drop it; plain version -> bump patch.
CurrentVersion=$(get_workspace_version)
case "$CurrentVersion" in
    *-*) ReleaseVersion="${CurrentVersion%%-*}" ;;
    *)   ReleaseVersion=$(bump_patch_version "$CurrentVersion") ;;
esac
Tag="v$ReleaseVersion"
echo "Current version: $CurrentVersion"
echo "Release version: $ReleaseVersion"

if git rev-parse "$Tag" >/dev/null 2>&1; then
    echo "Tag $Tag already exists. Aborting to avoid re-releasing." >&2
    exit 1
fi

# 4. Write the release version and commit it.
set_workspace_version "$ReleaseVersion"
echo "Updating Cargo.lock..."
cargo check
git add "$CargoToml"
[ -f "$ProjectRoot/Cargo.lock" ] && git add "$ProjectRoot/Cargo.lock"
git commit -m "Release $ReleaseVersion"

# 5. Create and push the release tag (this is what CI builds).
echo "Creating and pushing tag $Tag..."
git tag -a "$Tag" -m "Release $ReleaseVersion"
git push origin "$Tag"

# 6. Bump to the next development version (X.Y.(Z+1)-dev) and commit.
NextDev="$(bump_patch_version "$ReleaseVersion")-dev"
echo "Starting next development iteration $NextDev..."
set_workspace_version "$NextDev"
cargo check
git add "$CargoToml"
[ -f "$ProjectRoot/Cargo.lock" ] && git add "$ProjectRoot/Cargo.lock"
git commit -m "Starting next development iteration $NextDev"
git push

echo ""
echo "Done. Released $Tag and bumped workspace version to $NextDev."
