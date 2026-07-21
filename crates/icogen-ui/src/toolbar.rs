//! Custom titlebar / toolbar for the IcoGen GUI windows.
//!
//! GPUI 0.2.2 does not render a native menu bar on Windows (`set_menus` only
//! takes effect on macOS). We therefore hide the system titlebar and draw our
//! own: a draggable title area on the left and, on the right, theme/language
//! toggles plus the standard window controls (minimize, maximize, close).

use gpui::prelude::*;
use gpui::{div, px, App, ClickEvent, Context, Div, Stateful, WindowControlArea};

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

/// The custom titlebar for an IcoGen window.
///
/// * `title` is shown on the left and acts as a draggable region.
/// * The right side contains theme/language toggles and the standard window
///   controls (minimize, maximize, close). The draggable title area uses
///   `WindowControlArea::Drag`; the controls use normal GPUI click handlers so
///   their hover styling is fully customizable.
pub fn toolbar<T: 'static>(title: &str, t: &ThemeColors, cx: &mut Context<T>) -> Div {
    let is_dark = cx.global::<ThemeManager>().is_dark();
    let is_zh = cx.global::<I18nManager>().language_id == "zh-CN";

    // Show the *target* state: click to switch there.
    // Light → moon (switch to dark); dark → sun (switch to light).
    let theme_icon: &'static str = if is_dark { "\u{2600}" } else { "\u{263E}" };
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

    let min_btn = window_control_btn(t, t.surface_hover)
        .id("win-min")
        .on_click(cx.listener(|_, _, window, _| window.minimize_window()))
        .child("\u{2212}");
    let max_btn = window_control_btn(t, t.surface_hover)
        .id("win-max")
        .on_click(cx.listener(|_, _, window, _| window.zoom_window()))
        .child("\u{25A1}");
    let close_btn = window_control_btn(t, t.close_hover)
        .id("win-close")
        .hover(|s| s.text_color(color(0xffffff)))
        .on_click(cx.listener(|_, _, _, cx| cx.quit()))
        .child("\u{00D7}");

    // Draggable title region. It fills the remaining horizontal space so the
    // user can drag the window from anywhere on the left side.
    let drag_area = div()
        .id("titlebar-drag")
        .flex()
        .flex_1()
        .items_center()
        .h_full()
        .px(spacing::sm())
        .text_color(color(t.text_primary))
        .text_size(px(13.))
        .child(title.to_string())
        .window_control_area(WindowControlArea::Drag);

    div()
        .w_full()
        .h(px(36.))
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .pl(spacing::sm())
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
        .h(px(24.))
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

/// A window-control button (minimize / maximize / close).
///
/// The button fills the full titlebar height and has no rounded corners, so
/// its hover background spans the bar like native apps. `hover_bg` is the
/// background applied on hover. The glyph color is handled by each button's
/// own child (see `close_glyph`) because GPUI shapes text at layout time and
/// only the glyph's direct container re-lays-out on hover.
///
/// We use normal GPUI click handlers instead of `WindowControlArea` so the
/// hover styling (especially the white × on red) is fully under our control.
fn window_control_btn(t: &ThemeColors, hover_bg: u32) -> Div {
    div()
        .w(px(40.))
        .h_full()
        .flex()
        .items_center()
        .justify_center()
        .cursor_pointer()
        .text_color(color(t.text_secondary))
        .text_size(px(14.))
        .hover(|s| s.bg(color(hover_bg)))
}
