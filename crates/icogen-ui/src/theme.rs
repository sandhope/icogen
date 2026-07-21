//! Centralized color palette and design tokens for the icogen GUIs.
//!
//! All shared components and both front-ends draw from these constants so the
//! two tools stay visually in sync. The palette is a clean, modern light theme
//! with an indigo accent — no pinks or overly warm tones.

/// Hex color constants. Use with `crate::color::color()`.
pub mod colors {
    /// Indigo-600 — primary accent for buttons, selected pills, active states.
    pub const ACCENT: u32 = 0x4f46e5;
    /// Indigo-700 — stronger accent for emphasis / pressed states.
    pub const ACCENT_DARK: u32 = 0x4338ca;
    /// Indigo-500 — lighter accent for highlights.
    pub const ACCENT_LIGHT: u32 = 0x6366f1;

    /// Slate-50 — window / page background.
    pub const BG: u32 = 0xf8fafc;
    /// Slate-100 — subtle surfaces (unselected pills, empty drop zones).
    pub const SURFACE: u32 = 0xf1f5f9;
    /// Slate-200 — hover / pressed surface state.
    pub const SURFACE_HOVER: u32 = 0xe2e8f0;

    /// Pure white — cards, image preview backgrounds.
    pub const CARD: u32 = 0xffffff;

    /// Slate-200 — default borders.
    pub const BORDER: u32 = 0xe2e8f0;
    /// Slate-300 — stronger borders (drop zones, selected swatches).
    pub const BORDER_STRONG: u32 = 0xcbd5e1;

    /// Slate-900 — primary text.
    pub const TEXT_PRIMARY: u32 = 0x0f172a;
    /// Slate-600 — secondary text (labels, values).
    pub const TEXT_SECONDARY: u32 = 0x475569;
    /// Slate-500 — muted captions / hints.
    pub const TEXT_MUTED: u32 = 0x64748b;

    /// Emerald-500 — success status.
    pub const SUCCESS: u32 = 0x10b981;

    /// White as a design token.
    pub const WHITE: u32 = 0xffffff;
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
