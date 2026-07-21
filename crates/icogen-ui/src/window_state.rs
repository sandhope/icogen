//! Window placement persistence: GUIs reopen at the size and position they had
//! when last closed.
//!
//! The state is stored as a small text file (`windowed|maximized x y w h`,
//! logical pixels) under `%APPDATA%/icogen/`. Fullscreen is never persisted —
//! the last windowed/maximized placement wins instead.

use std::path::PathBuf;

use gpui::{App, Bounds, Point, Size, WindowBounds, px};

/// Smallest restored size we accept; anything smaller is treated as corrupt.
const MIN_SIZE: f32 = 100.0;

fn state_path(app_id: &str) -> Option<PathBuf> {
    let appdata = std::env::var_os("APPDATA")?;
    Some(PathBuf::from(appdata).join("icogen").join(format!("{app_id}.window")))
}

/// Persist the current window placement. Best-effort: I/O errors are ignored
/// so a read-only profile can never break the app.
pub fn save(app_id: &str, bounds: WindowBounds) {
    let (kind, b) = match bounds {
        WindowBounds::Windowed(b) => ("windowed", b),
        WindowBounds::Maximized(b) => ("maximized", b),
        // Keep the last non-fullscreen placement.
        WindowBounds::Fullscreen(_) => return,
    };
    let Some(path) = state_path(app_id) else { return };
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let content = format!(
        "{kind} {} {} {} {}\n",
        f32::from(b.origin.x).round() as i32,
        f32::from(b.origin.y).round() as i32,
        f32::from(b.size.width).round() as i32,
        f32::from(b.size.height).round() as i32,
    );
    let _ = std::fs::write(path, content);
}

/// Load the saved placement, if any. Returns `None` when the file is missing,
/// malformed, suspiciously small, or no longer intersects any connected
/// display (e.g. the window was last on a monitor that is now unplugged).
pub fn load(app_id: &str, cx: &App) -> Option<WindowBounds> {
    let path = state_path(app_id)?;
    let text = std::fs::read_to_string(path).ok()?;
    let mut parts = text.split_whitespace();
    let kind = parts.next()?;
    let x = parts.next()?.parse::<f32>().ok()?;
    let y = parts.next()?.parse::<f32>().ok()?;
    let w = parts.next()?.parse::<f32>().ok()?;
    let h = parts.next()?.parse::<f32>().ok()?;
    if w < MIN_SIZE || h < MIN_SIZE {
        return None;
    }

    let bounds = Bounds {
        origin: Point { x: px(x), y: px(y) },
        size: Size { width: px(w), height: px(h) },
    };
    let visible = cx.displays().iter().any(|display| display.bounds().intersects(&bounds));
    if !visible {
        return None;
    }

    Some(match kind {
        "maximized" => WindowBounds::Maximized(bounds),
        _ => WindowBounds::Windowed(bounds),
    })
}
