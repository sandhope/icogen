# Icogen

> English ｜ **中文文档:** [README.zh-CN.md](README.zh-CN.md)

Generate Windows app icons (`AppIcon.ico`) and WinUI 3 / Windows App SDK asset
PNGs from a single source image. Each task ships as a CLI and a GPUI-based GUI —
four standalone `.exe` files in total, with zero runtime dependencies.

## Quick start

Download the prebuilt Windows binaries from [GitHub Releases](../../releases)
(or [build from source](#build-from-source)), then:

```powershell
# AppIcon.ico from an image
icogen.exe logo.png

# WinUI 3 / Windows App SDK asset PNGs (written to Assets\)
icogen-assets.exe logo.png
```

Prefer clicking? Run `icogen-gui.exe` / `icogen-assets-gui.exe`, drag an image
onto the window (or click to browse), pick options, and generate.

## Usage

### `icogen` — AppIcon.ico generator

```text
icogen <input image> [options]
```

| Option | Description | Default |
| --- | --- | --- |
| `-o, --output <path>` | Output `.ico` path | `AppIcon.ico` |
| `-s, --sizes <list>` | Embedded sizes, comma-separated | `16,24,32,48,64,128,256` |
| `--mode <mode>` | `contain` (fit, transparent padding) or `cover` (crop to fill) | `contain` |
| `-b, --background <color>` | Solid background, e.g. `#1e293b` or `white` | transparent |
| `--pad <ratio>` | Inner padding in contain mode, 0–1 | `0` |
| `--verify` | Read the `.ico` back and verify its frames | off |

Examples:

```powershell
icogen logo.png
icogen logo.png -o AppIcon.ico --mode cover
icogen logo.png -b "#111827" --mode cover --verify
```

### `icogen-assets` — WinUI 3 asset generator

```text
icogen-assets <input image> [options]
```

| Option | Description | Default |
| --- | --- | --- |
| `-o, --output <dir>` | Output directory | `Assets` |

Writes the eight platform PNGs into the output directory, relative to the
current directory — run it from your project root.

### GUI

`icogen-gui` and `icogen-assets-gui` expose the same functionality in a GPUI
window: drag & drop or a native file dialog, per-size / per-target toggles, a
live preview of every generated frame, and an editable output path. `icogen-gui`
additionally offers contain/cover mode, background color presets and inner
padding. Both accept an optional image path and `-o <output>` on the command
line.

## What it generates

### AppIcon.ico

A single `.ico` embedding seven frames — `16 / 24 / 32 / 48 / 64 / 128 / 256` px:

- The **256 px** frame is stored as PNG (small file, crisp on high-DPI screens).
- Smaller frames are stored as 32-bit BGRA bitmaps with alpha.

![AppIcon.ico frames from 16 to 256 px](samples/preview.png)

### WinUI 3 assets

Eight PNGs written to `Assets/`:

| Output | Size |
| --- | --- |
| `Square150x150Logo.scale-200.png` | 300×300 |
| `Square44x44Logo.scale-200.png` | 88×88 |
| `Square44x44Logo.targetsize-24_altform-unplated.png` | 24×24 |
| `Square44x44Logo.targetsize-48_altform-lightunplated.png` | 48×48 |
| `LockScreenLogo.scale-200.png` | 48×48 |
| `StoreLogo.png` | 50×50 |
| `Wide310x150Logo.scale-200.png` | 620×300 |
| `SplashScreen.scale-200.png` | 1240×600 |

Square targets are scaled directly; wide targets are scaled to fit and centered
on a transparent canvas.

## Build from source

Requires the [Rust toolchain](https://rustup.rs/) and the Windows 10/11 SDK
(`rc.exe` embeds the window icon into the GUI binaries) — both compile-time only.

One-click build — compiles all four binaries and copies them into `dist/`:

```powershell
# Windows PowerShell
.\build-dist.ps1

# or Git Bash
./build-dist.sh
```

Manual build — outputs land in `dist/release/`:

```bash
cargo build --release
```

Size optimization is enabled (`opt-level=z` + LTO + strip). `.cargo/config.toml`
keeps every build artifact inside the project-local, gitignored `dist/`
directory instead of the global cargo home.

## Project layout

```
icogen/
├── assets/                 # app icon embedded into the GUI binaries
│   └── app.ico
├── crates/                 # all binaries + libs
│   ├── icogen/             # icogen.exe — AppIcon.ico CLI
│   ├── icogen-assets/      # icogen-assets.exe — WinUI 3 assets CLI
│   ├── icogen-assets-gui/  # icogen-assets-gui.exe — WinUI 3 assets GUI
│   ├── icogen-core/        # shared logic (lib)
│   ├── icogen-gui/         # icogen-gui.exe — AppIcon.ico GUI
│   └── icogen-ui/          # shared GPUI components & colors (lib)
├── dist/                   # build output & shipped binaries (gitignored)
├── samples/                # demo input and output
│   ├── AppIcon.ico
│   ├── logo.png
│   └── preview.png
├── scripts/                # Python reference implementations
│   ├── gen-assets.py
│   ├── icogen_gen.py
│   └── requirements.txt
├── build-dist.ps1
├── build-dist.sh
├── Cargo.lock
├── Cargo.toml              # Cargo workspace root
├── LICENSE
├── README.md
└── README.zh-CN.md
```

`icogen` = **ico**n **gen**erator. CLI binaries take the bare product name and
GUI variants add `-gui`, so directory, package and `.exe` names always match.
`icogen-core` holds the shared image-loading, resizing and ICO/PNG encoding
logic; `icogen-ui` holds the shared GPUI components and color palette so both
GUIs stay in sync.

## Python reference scripts

`scripts/` contains two Python implementations for development, tweaking and
batch use (Python 3.8+, Pillow):

```bash
pip install -r scripts/requirements.txt
python scripts/icogen_gen.py --help
python scripts/gen-assets.py --help
```

The Rust binaries are recommended for end-user distribution because they are
self-contained; reach for the scripts when you want to adjust behavior.

## CI & releases

Two GitHub Actions workflows live in `.github/workflows/`:

- **`ci.yml`** — builds the workspace on every push / pull request and uploads
  the four `.exe` files as artifact `icogen-windows`.
- **`release.yml`** — publishes a GitHub Release when a `v*` tag is pushed (or
  manually from the Actions tab):

  ```bash
  git tag v1.0.0
  git push origin v1.0.0
  ```

## Notes

- **Icon cache**: Windows caches tray/taskbar icons. After replacing
  `AppIcon.ico`, restart `explorer.exe` or rebuild the icon cache if the UI does
  not update immediately.
- **Source quality**: icons end up tiny, so a crisp, simple source image
  (512×512+) yields cleaner 16 px / 24 px results.
