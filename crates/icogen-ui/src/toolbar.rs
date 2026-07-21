//! In-window toolbar strip for switching theme and language.
//!
//! GPUI 0.2.2 does not render a native menu bar on Windows (`set_menus` only
//! takes effect on macOS), so both GUIs show a thin toolbar under the native
//! titlebar instead. The toolbar reads the current theme/language from the
//! GPUI globals and toggles them on click, persisting the choice each time.

use gpui::prelude::*;
use gpui::{div, px, App, ClickEvent, Context, Div, Stateful};

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

/// A thin toolbar strip (right-aligned) with a theme toggle and a language
/// toggle. Each button shows an icon for the *current* value; clicking
/// switches it.
pub fn toolbar<T: 'static>(t: &ThemeColors, cx: &mut Context<T>) -> Div {
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

    div()
        .w_full()
        .flex()
        .flex_row()
        .items_center()
        .justify_end()
        .gap(spacing::sm())
        .px(spacing::lg())
        .py(spacing::sm())
        .bg(color(t.card))
        .border_b_1()
        .border_color(color(t.border))
        .child(theme_btn)
        .child(lang_btn)
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
