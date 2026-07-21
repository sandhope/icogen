//! Centralized color palette and design tokens for the icogen GUIs.
//!
//! All shared components and both front-ends draw from these constants so the
//! two tools stay visually in sync.
//!
//! Design language:
//!   - Calm, neutral slate surfaces (no warm tints, no pink).
//!   - A single, restrained indigo accent used only for primary actions and
//!     the selected state. Selected pills are not filled in solid accent —
//!     they use an accent-tinted surface with accent text for a quieter look.
//!   - Status colors are muted: success is a deep emerald, error a desaturated
//!     rust, never a saturated red.

/// Hex color constants. Use with `crate::color::color()`.
pub mod colors {
    // --- Brand accent: a single, restrained indigo ---
    /// Indigo-600 — primary accent (main action button text/icon).
    pub const ACCENT: u32 = 0x4f46e5;
    /// Indigo-700 — pressed / emphasis.
    pub const ACCENT_DARK: u32 = 0x4338ca;
    /// Indigo-500 — hover on the main button.
    pub const ACCENT_HOVER: u32 = 0x6366f1;
    /// Indigo-50 — selected pill surface (very light tint, not a fill).
    pub const ACCENT_TINT: u32 = 0xeef2ff;

    // --- Neutral surfaces (slate scale) ---
    /// Slate-50 — window / page background.
    pub const BG: u32 = 0xf8fafc;
    /// Slate-100 — subtle surfaces (unselected pills, empty drop zones).
    pub const SURFACE: u32 = 0xf1f5f9;
    /// Slate-200 — hover / pressed surface state.
    pub const SURFACE_HOVER: u32 = 0xe2e8f0;

    /// Pure white — cards, image preview backgrounds.
    pub const CARD: u32 = 0xffffff;

    // --- Borders ---
    /// Slate-200 — default borders.
    pub const BORDER: u32 = 0xe2e8f0;
    /// Slate-300 — stronger borders (drop zones).
    pub const BORDER_STRONG: u32 = 0xcbd5e1;

    // --- Text ---
    /// Slate-900 — primary text.
    pub const TEXT_PRIMARY: u32 = 0x0f172a;
    /// Slate-600 — secondary text (labels, values).
    pub const TEXT_SECONDARY: u32 = 0x475569;
    /// Slate-500 — muted captions / hints.
    pub const TEXT_MUTED: u32 = 0x64748b;

    // --- Status (muted, never saturated) ---
    /// Emerald-600 — success status.
    pub const SUCCESS: u32 = 0x059669;
    /// Stone-600 — error status (desaturated rust, not a bright red).
    pub const ERROR: u32 = 0x57534e;
    /// Slate-700 — drop-zone idle icon background (sober neutral).
    pub const DROP_ICON_BG: u32 = 0xe2e8f0;
    /// Slate-500 — drop-zone idle icon foreground.
    pub const DROP_ICON_FG: u32 = 0x64748b;

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
