# Icogen — Windows icon & asset toolkit

Generate Windows app icons and WinUI 3 assets from a single source image, with zero runtime dependencies.

This release ships five standalone executables:

### GUI

| Binary | Purpose |
| --- | --- |
| **`icogen-app.exe`** | ⭐ Recommended. Combined tool — a top tab bar switches between the **Ico** and **Assets** workflows over one shared source image. |
| **`icogen-gui.exe`** | AppIcon.ico only, with contain/cover mode, background color, padding and per-size preview. |
| **`icogen-assets-gui.exe`** | WinUI 3 / Windows App SDK asset PNGs only, with per-target toggles. |

### CLI

| Binary | Purpose |
| --- | --- |
| **`icogen.exe`** | Generate AppIcon.ico with configurable sizes, scale mode and background. |
| **`icogen-assets.exe`** | Generate the eight WinUI 3 asset PNGs into `Assets\`. |

### Quick start

```powershell
# GUI (recommended)
icogen-app.exe

# CLI
icogen.exe logo.png
icogen-assets.exe logo.png
```

> All binaries are self-contained — no .NET or other runtime required.

### What's new

- Added the combined `icogen-app.exe` with a top tab bar switching Ico / Assets over one shared source image.
- Refreshed GUI design: drop zone, rounded primary button, SVG icons (upload / folder / theme), and a 2×4 target grid for the 8 assets.
- Build scripts and docs updated for the five binaries.

---

# Icogen — Windows 图标 & 资源生成工具集

从单张源图生成 Windows 应用图标和 WinUI 3 资源，零运行时依赖。

本次发布包含 5 个独立可执行文件：

### 图形界面

| 程序 | 用途 |
| --- | --- |
| **`icogen-app.exe`** | ⭐ 推荐。整合工具，顶部标签栏在「图标」与「资源」两种工作流间切换，复用同一份源图。 |
| **`icogen-gui.exe`** | 仅生成 `AppIcon.ico`，支持 contain/cover 模式、背景色、内边距和逐尺寸预览。 |
| **`icogen-assets-gui.exe`** | 仅生成 WinUI 3 / Windows App SDK 资源 PNG，逐目标开关。 |

### 命令行

| 程序 | 用途 |
| --- | --- |
| **`icogen.exe`** | 生成 `AppIcon.ico`，可配置尺寸、缩放模式、背景色。 |
| **`icogen-assets.exe`** | 生成 8 个 WinUI 3 资源 PNG 到 `Assets\` 目录。 |

### 快速开始

```powershell
# 图形界面（推荐）
icogen-app.exe

# 命令行
icogen.exe logo.png
icogen-assets.exe logo.png
```

> 全部为独立二进制，无需安装 .NET 或其他运行时。

### 更新说明

- 新增整合程序 `icogen-app.exe`（顶部标签栏切换 Ico / Assets，复用同一份源图）。
- 图形界面升级为新设计：拖放区、圆角主按钮、SVG 图标（上传 / 文件夹 / 主题）、8 个目标资源按 2×4 网格排布。
- 构建脚本与文档已同步为 5 个二进制。
