//! Reusable styled components shared by the icogen GUIs.
//!
//! `card` and `section_label` are plain functions returning a styled `Div`.
//! `style_button` / `style_pill` take a `Stateful<Div>` the caller built (with
//! `.id(...)` and `.on_click(...)` already attached) and apply the shared look.
//! Editing the styling here restyles both front-ends from one place.
//!
//! Every function takes a `&ThemeColors` so the look follows the active theme.

use gpui::prelude::*;
use gpui::{div, px, white, Div, SharedString, Stateful};

use crate::color::color;
use crate::theme::radii;
use crate::theme::spacing;
use crate::theme::ThemeColors;

/// Rounded card with a subtle border and soft shadow — the standard panel
/// container.
pub fn card(t: &ThemeColors) -> Div {
    div()
        .bg(color(t.card))
        .rounded(radii::xl2())
        .border_1()
        .border_color(color(t.border))
        .shadow_sm()
        .p(spacing::xl())
}

/// Small muted section heading used above groups of controls.
pub fn section_label(text: impl Into<SharedString>, t: &ThemeColors) -> Div {
    div()
        .mb(spacing::sm())
        .child(
            div()
                .child(text.into())
                .text_size(px(11.))
                .text_color(color(t.text_muted))
                .font_weight(gpui::FontWeight::MEDIUM),
        )
}

/// Apply the shared primary-button look to a `Stateful<Div>` the caller built.
/// Solid accent fill, white label, generous padding.
pub fn style_button(div: Stateful<Div>, t: &ThemeColors) -> Stateful<Div> {
    div.px(spacing::xl())
        .py(spacing::md())
        .rounded(radii::lg())
        .bg(color(t.accent))
        .text_color(white())
        .shadow_sm()
        .cursor_pointer()
        .font_weight(gpui::FontWeight::MEDIUM)
}

/// Apply the shared selectable-pill look. `selected` toggles the filled style.
/// Unselected: light surface, border, muted text.
/// Selected:   accent tint, accent border, accent text.
pub fn style_pill(div: Stateful<Div>, selected: bool, t: &ThemeColors) -> Stateful<Div> {
    div.px(spacing::md())
        .py(spacing::sm())
        .rounded(radii::md())
        .border_1()
        .border_color(if selected {
            color(t.accent)
        } else {
            color(t.border)
        })
        .bg(if selected {
            color(t.accent_tint)
        } else {
            color(t.surface)
        })
        .text_color(if selected {
            color(t.accent)
        } else {
            color(t.text_secondary)
        })
        .font_weight(if selected {
            gpui::FontWeight::MEDIUM
        } else {
            gpui::FontWeight::NORMAL
        })
        .cursor_pointer()
}

/// A full-width header bar with a sober accent icon and title.
pub fn header(title: impl Into<SharedString>, t: &ThemeColors) -> Div {
    div()
        .w_full()
        .flex()
        .flex_row()
        .items_center()
        .gap(spacing::md())
        .p(spacing::lg())
        .bg(color(t.card))
        .border_b_1()
        .border_color(color(t.border))
        .child(
            div()
                .w(px(24.))
                .h(px(24.))
                .rounded(radii::md())
                .bg(color(t.accent)),
        )
        .child(
            div()
                .child(title.into())
                .text_size(px(16.))
                .text_color(color(t.text_primary))
                .font_weight(gpui::FontWeight::MEDIUM),
        )
}

/// A dashed drop-zone container. The caller supplies the dynamic background and
/// children (placeholder text or image preview).
pub fn drop_zone(t: &ThemeColors) -> Div {
    div()
        .w_full()
        .h(px(240.))
        .rounded(radii::lg())
        .border_2()
        .border_dashed()
        .border_color(color(t.border_strong))
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
}

/// The rounded square shown in the center of an empty drop zone.
pub fn drop_icon(t: &ThemeColors) -> Div {
    div()
        .w(px(56.))
        .h(px(56.))
        .rounded(radii::lg())
        .bg(color(t.drop_icon_bg))
        .flex()
        .items_center()
        .justify_center()
        .child(
            div()
                .w(px(22.))
                .h(px(22.))
                .rounded(radii::sm())
                .bg(color(t.drop_icon_fg)),
        )
        .mb(spacing::sm())
}

/// Small helper for centering placeholder text inside a drop zone.
pub fn drop_hint(text: impl Into<SharedString>, t: &ThemeColors) -> Div {
    div()
        .child(text.into())
        .text_size(px(14.))
        .text_color(color(t.text_muted))
        .text_center()
}
