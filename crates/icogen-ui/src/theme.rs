//! Centralized color palette, design tokens, and runtime theme management.
//!
//! Design language:
//!   - Calm, neutral slate surfaces (no warm tints, no pink).
//!   - A single, restrained indigo accent used only for primary actions and
//!     the selected state.
//!   - Status colors are muted: success is a deep emerald, error a desaturated
//!     rust, never a saturated red.

use gpui::{App, Global};

/// A complete color palette. All fields are `0xRRGGBB` hex values — pass them
/// through `crate::color::color()` to get a GPUI `Hsla`.
#[derive(Clone, Copy, Debug)]
pub struct ThemeColors {
    // --- Brand accent ---
    pub accent: u32,
    pub accent_dark: u32,
    pub accent_hover: u32,
    pub accent_tint: u32,

    // --- Neutral surfaces ---
    pub bg: u32,
    pub surface: u32,
    pub surface_hover: u32,
    pub card: u32,

    // --- Borders ---
    pub border: u32,
    pub border_strong: u32,

    // --- Text ---
    pub text_primary: u32,
    pub text_secondary: u32,
    pub text_muted: u32,

    // --- Status ---
    pub success: u32,
    pub error: u32,

    // --- Window control (close button) hover ---
    pub close_hover: u32,

    // --- Drop zone icon ---
    pub drop_icon_bg: u32,
    pub drop_icon_fg: u32,
}

impl ThemeColors {
    /// Light palette (slate-50 background, indigo-600 accent).
    pub fn light() -> Self {
        Self {
            accent: 0x4f46e5,
            accent_dark: 0x4338ca,
            accent_hover: 0x6366f1,
            accent_tint: 0xeef2ff,

            bg: 0xf8fafc,
            surface: 0xf1f5f9,
            surface_hover: 0xe2e8f0,
            card: 0xffffff,

            border: 0xe2e8f0,
            border_strong: 0xcbd5e1,

            text_primary: 0x0f172a,
            text_secondary: 0x475569,
            text_muted: 0x64748b,

            success: 0x059669,
            error: 0x57534e,

            close_hover: 0xe81123,

            drop_icon_bg: 0xe2e8f0,
            drop_icon_fg: 0x64748b,
        }
    }

    /// Dark palette (slate-900 background, indigo-400 accent).
    pub fn dark() -> Self {
        Self {
            accent: 0x818cf8,
            accent_dark: 0xa5b4fc,
            accent_hover: 0x6366f1,
            accent_tint: 0x1e1b4b,

            bg: 0x0f172a,
            surface: 0x1e293b,
            surface_hover: 0x334155,
            card: 0x1e293b,

            border: 0x334155,
            border_strong: 0x475569,

            text_primary: 0xf1f5f9,
            text_secondary: 0xcbd5e1,
            text_muted: 0x94a3b8,

            success: 0x34d399,
            error: 0xa8a29e,

            close_hover: 0xc42b1c,

            drop_icon_bg: 0x334155,
            drop_icon_fg: 0x94a3b8,
        }
    }
}

/// Runtime theme state, stored as a GPUI global singleton.
pub struct ThemeManager {
    pub colors: ThemeColors,
    pub theme_id: String,
}

impl Global for ThemeManager {}

impl ThemeManager {
    /// Initialize the global with the given theme id ("light" or "dark").
    pub fn init(cx: &mut App, theme_id: &str) {
        let colors = match theme_id {
            "dark" => ThemeColors::dark(),
            _ => ThemeColors::light(),
        };
        let id = if theme_id == "dark" { "dark" } else { "light" };
        cx.set_global(Self {
            colors,
            theme_id: id.to_string(),
        });
    }

    /// Switch theme. Returns `true` if the theme actually changed.
    pub fn set_theme(&mut self, theme_id: &str) -> bool {
        let id = if theme_id == "dark" { "dark" } else { "light" };
        if self.theme_id == id {
            return false;
        }
        self.colors = match id {
            "dark" => ThemeColors::dark(),
            _ => ThemeColors::light(),
        };
        self.theme_id = id.to_string();
        true
    }

    pub fn is_dark(&self) -> bool {
        self.theme_id == "dark"
    }
}

/// Common corner radii.
pub mod radii {
    use gpui::Pixels;

    pub fn sm() -> Pixels {
        gpui::px(6.)
    }
    pub fn md() -> Pixels {
        gpui::px(8.)
    }
    pub fn lg() -> Pixels {
        gpui::px(10.)
    }
    pub fn xl() -> Pixels {
        gpui::px(12.)
    }
    pub fn xl2() -> Pixels {
        gpui::px(16.)
    }
}

/// Common spacing values.
pub mod spacing {
    use gpui::Pixels;

    pub fn xs() -> Pixels {
        gpui::px(4.)
    }
    pub fn sm() -> Pixels {
        gpui::px(8.)
    }
    pub fn md() -> Pixels {
        gpui::px(12.)
    }
    pub fn lg() -> Pixels {
        gpui::px(16.)
    }
    pub fn xl() -> Pixels {
        gpui::px(20.)
    }
    pub fn xl2() -> Pixels {
        gpui::px(24.)
    }
}
