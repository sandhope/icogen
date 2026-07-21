//! Custom titlebar / toolbar for the IcoGen GUI windows.
//!
//! GPUI 0.2.2 does not render a native menu bar on Windows (`set_menus` only
//! takes effect on macOS). We therefore hide the system titlebar and draw our
//! own: a draggable title area on the left and, on the right, theme/language
//! toggles plus the standard window controls (minimize, maximize, close).

use gpui::prelude::*;
use gpui::{div, img, px, svg, App, ClickEvent, Context, Div, Hsla, Rgba, Stateful, Window, WindowControlArea};

use crate::color::color;
use crate::i18n::I18nManager;
use crate::settings;
use crate::theme::{radii, spacing, ThemeColors, ThemeManager};

/// Switch to the opposite theme, persist the choice, and repaint all windows.
pub fn toggle_theme(cx: &mut App) {
    let next = if cx.global::<ThemeManager>().is_dark() {
        "light"
    } else {
        "dark"
    };
    if cx.global_mut::<ThemeManager>().set_theme(next) {
        persist(cx);
        cx.refresh_windows();
    }
}

/// Switch to the other language, persist the choice, and repaint all windows.
pub fn toggle_language(cx: &mut App) {
    let next = if cx.global::<I18nManager>().language_id == "zh-CN" {
        "en-US"
    } else {
        "zh-CN"
    };
    if cx.global_mut::<I18nManager>().set_language(next) {
        persist(cx);
        cx.refresh_windows();
    }
}

fn persist(cx: &App) {
    settings::save(&settings::Settings {
        theme_id: cx.global::<ThemeManager>().theme_id.clone(),
        language_id: cx.global::<I18nManager>().language_id.clone(),
    });
}

/// Window caption buttons, copied from Zed's `WindowsWindowControls` /
/// `WindowsCaptionButton` (crates/platform_title_bar). Using `WindowControlArea`
/// lets the OS handle minimize / maximize / restore / close natively — so the
/// maximize button toggles and the glyph swaps to the "restore" icon when the
/// window is maximized — while the styling (incl. the white × on red) stays
/// under our control via hover/active styles.
#[derive(Clone, Copy, PartialEq, Eq)]
enum WindowControlButton {
    Minimize,
    Restore,
    Maximize,
    Close,
}

impl WindowControlButton {
    fn id(&self) -> &'static str {
        match self {
            WindowControlButton::Minimize => "win-min",
            WindowControlButton::Restore | WindowControlButton::Maximize => "win-max",
            WindowControlButton::Close => "win-close",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            WindowControlButton::Minimize => "\u{e921}",
            WindowControlButton::Restore => "\u{e923}",
            WindowControlButton::Maximize => "\u{e922}",
            WindowControlButton::Close => "\u{e8bb}",
        }
    }

    fn control_area(&self) -> WindowControlArea {
        match self {
            WindowControlButton::Close => WindowControlArea::Close,
            WindowControlButton::Minimize => WindowControlArea::Min,
            WindowControlButton::Restore | WindowControlButton::Maximize => WindowControlArea::Max,
        }
    }

    /// Render the button. `t` supplies theme colors; the close button uses the
    /// Windows red regardless of theme.
    fn render(self, t: &ThemeColors) -> Stateful<Div> {
        let (hover_bg, hover_fg, active_bg, active_fg) = match self {
            WindowControlButton::Close => {
                // Windows close red (#E81123).
                let c: Hsla = Rgba {
                    r: 232.0 / 255.0,
                    g: 17.0 / 255.0,
                    b: 35.0 / 255.0,
                    a: 1.0,
                }
                .into();
                (c, color(0xffffff), c.opacity(0.8), color(0xffffff).opacity(0.8))
            }
            _ => (
                color(t.surface_hover),
                color(t.text_primary),
                color(t.surface_hover),
                color(t.text_primary),
            ),
        };

        div()
            .id(self.id())
            .flex()
            .items_center()
            .justify_center()
            .content_center()
            .occlude()
            .w(px(40.))
            .h_full()
            .text_size(px(10.))
            .font_family(get_font())
            .hover(|s| s.bg(hover_bg).text_color(hover_fg))
            .active(|s| s.bg(active_bg).text_color(active_fg))
            .window_control_area(self.control_area())
            .child(self.icon())
    }
}

/// Segoe Fluent Icons (Windows 11) glyphs for the caption buttons.
fn get_font() -> &'static str {
    "Segoe Fluent Icons"
}

/// The custom titlebar for an IcoGen window.
///
/// * `title` is shown on the left and acts as a draggable region.
/// * The right side contains theme/language toggles and the standard window
///   controls (minimize, maximize, close). The draggable title area uses
///   `WindowControlArea::Drag`; the controls use `WindowControlArea` so the OS
///   handles them natively (maximize toggles, close behaves like the real
///   caption button) while hover/active styling is ours.
pub fn toolbar<T: 'static>(title: &str, t: &ThemeColors, window: &mut Window, cx: &mut Context<T>) -> Div {
    let is_dark = cx.global::<ThemeManager>().is_dark();
    let is_zh = cx.global::<I18nManager>().language_id == "zh-CN";

    // Show the *target* state: click to switch there.
    // Light → moon (switch to dark); dark → sun (switch to light).
    // Rendered as an embedded SVG icon (GPUI tints it via the button's
    // `text_color`, which it inherits, so hover states follow automatically).
    let theme_icon = (if is_dark {
        svg().path("icons/sun.svg")
    } else {
        svg().path("icons/moon.svg")
    })
    .w(px(16.))
    .h(px(16.))
    .text_color(color(t.text_secondary));
    // Chinese → "EN" (switch to English); English → 中 (switch to Chinese).
    let lang_label: &'static str = if is_zh { "EN" } else { "\u{4E2D}" };

    let theme_btn = toolbar_icon_btn(
        div()
            .id("toolbar-theme")
            .on_click(cx.listener(|_, _: &ClickEvent, _, cx| toggle_theme(cx))),
        t,
    )
    .child(theme_icon);

    let lang_btn = toolbar_icon_btn(
        div()
            .id("toolbar-lang")
            .on_click(cx.listener(|_, _: &ClickEvent, _, cx| toggle_language(cx))),
        t,
    )
    .child(lang_label);

    let min_btn = WindowControlButton::Minimize.render(t);
    let max_btn = (if window.is_maximized() {
        WindowControlButton::Restore
    } else {
        WindowControlButton::Maximize
    })
    .render(t);
    let close_btn = WindowControlButton::Close.render(t);

    // App logo: the embedded `assets/logo.png` (small rounded square with a
    // leaf glyph), decoded once and cached.
    let logo = img(crate::logo::app_logo())
        .id("titlebar-logo")
        .w(px(18.))
        .h(px(18.));

    // Draggable title region. It fills the remaining horizontal space so the
    // user can drag the window from anywhere on the left side.
    let drag_area = div()
        .id("titlebar-drag")
        .flex()
        .flex_1()
        .items_center()
        .h_full()
        .px(spacing::sm())
        .gap(spacing::sm())
        .child(logo)
        .child(
            div()
                .child(title.to_string())
                .text_color(color(t.text_primary))
                .text_size(px(13.))
                .font_weight(gpui::FontWeight::MEDIUM),
        )
        .window_control_area(WindowControlArea::Drag);

    div()
        .w_full()
        .h(px(40.))
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .pl(spacing::md())
        .bg(color(t.card))
        .border_b_1()
        .border_color(color(t.border))
        .child(drag_area)
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .h_full()
                .gap(spacing::sm())
                .child(theme_btn)
                .child(lang_btn)
                .child(div().w(px(8.)))
                .child(
                    // Native-style window controls: contiguous, full-bar-height
                    // rectangles so the hover background spans the titlebar.
                    div()
                        .flex()
                        .flex_row()
                        .h_full()
                        .items_center()
                        .child(min_btn)
                        .child(max_btn)
                        .child(close_btn),
                ),
        )
}

/// Shared compact icon-button look for the toolbar buttons.
fn toolbar_icon_btn(div: Stateful<Div>, t: &ThemeColors) -> Stateful<Div> {
    div.w(px(32.))
        .h(px(30.))
        .flex()
        .items_center()
        .justify_center()
        .rounded(radii::md())
        .border_1()
        .border_color(color(t.border))
        .bg(color(t.surface))
        .text_color(color(t.text_secondary))
        .text_size(px(14.))
        .cursor_pointer()
}


