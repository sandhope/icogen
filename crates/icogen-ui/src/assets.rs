//! Embedded asset source for GPUI's `svg()` element.
//!
//! GPUI resolves `svg().path("...")` through the application's [`AssetSource`].
//! We embed the icon files at compile time (via `include_bytes!`) and serve
//! them from memory, so icons are always available regardless of the working
//! directory at runtime. Register this with `Application::with_assets(Assets)`.
//!
//! Note: GPUI renders an SVG as a single-color alpha mask tinted by the
//! element's `text_color` — the colors inside the SVG file are ignored, only
//! its shape (coverage) matters.

use std::borrow::Cow;

use gpui::{AssetSource, Result, SharedString};

/// Serves the embedded icon assets used across the icogen GUIs.
pub struct Assets;

/// Upload arrow icon for the empty drop zone.
const UPLOAD_SVG: &[u8] = include_bytes!("../../../assets/icons/upload.svg");

/// Folder icon shown on the output-path picker buttons.
const FOLDER_SVG: &[u8] = include_bytes!("../../../assets/icons/folder.svg");

/// Sun icon for the light-theme state of the toolbar theme toggle.
const SUN_SVG: &[u8] = include_bytes!("../../../assets/icons/sun.svg");

/// Moon icon for the dark-theme state of the toolbar theme toggle.
const MOON_SVG: &[u8] = include_bytes!("../../../assets/icons/moon.svg");

/// All embedded asset paths, kept in one place for `load`/`list`.
const ENTRIES: &[(&str, &[u8])] = &[
    ("icons/upload.svg", UPLOAD_SVG),
    ("icons/folder.svg", FOLDER_SVG),
    ("icons/sun.svg", SUN_SVG),
    ("icons/moon.svg", MOON_SVG),
];

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        let key = path.trim_start_matches('/');
        Ok(ENTRIES
            .iter()
            .find(|(p, _)| *p == key)
            .map(|(_, bytes)| Cow::Borrowed(*bytes)))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let prefix = path.trim_start_matches('/');
        Ok(ENTRIES
            .iter()
            .filter(|(p, _)| p.starts_with(prefix))
            .map(|(p, _)| SharedString::from(*p))
            .collect())
    }
}
