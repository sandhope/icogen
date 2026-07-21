#!/usr/bin/env bash
# Build all four binaries and copy them into dist/ (project root).
# Run from anywhere:  ./build-dist.sh
set -euo pipefail

ProjectRoot="$(cd "$(dirname "$0")" && pwd)"
ReleaseDir="$ProjectRoot/dist/release"
DistDir="$ProjectRoot/dist"

cargo build --release \
    -p icogen \
    -p icogen-gui \
    -p icogen-assets \
    -p icogen-assets-gui

EXES=(icogen.exe icogen-gui.exe icogen-assets.exe icogen-assets-gui.exe)
for exe in "${EXES[@]}"; do
    src="$ReleaseDir/$exe"
    [ -f "$src" ] || { echo "Missing build artifact: $src" >&2; exit 1; }
    cp -f "$src" "$DistDir/"
    echo "OK  copied $exe -> dist/"
done

echo
echo "Done. Binaries are in dist/:"
printf '  dist/%s\n' "${EXES[@]}"
