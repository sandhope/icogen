//! Reusable styled components shared by the icogen GUIs.
//!
//! `card` and `section_label` are plain functions returning a styled `Div`.
//! `style_button` / `style_pill` take a `Stateful<Div>` the caller built (with
//! `.id(...)` and `.on_click(...)` already attached) and apply the shared look.
//! Editing the styling here restyles both front-ends from one place.

use gpui::prelude::*;
use gpui::{div, px, white, Div, SharedString, Stateful};

use crate::color::color;
use crate::theme::colors;
use crate::theme::radii;
use crate::theme::spacing;

/// White rounded card with a subtle border and soft shadow — the standard panel
/// container.
pub fn card() -> Div {
    div()
        .bg(color(colors::CARD))
        .rounded(radii::xl2())
        .border_1()
        .border_color(color(colors::BORDER))
        .shadow_sm()
        .p(spacing::xl())
}

/// Small muted section heading used above groups of controls.
pub fn section_label(text: impl Into<SharedString>) -> Div {
    div()
        .mb(spacing::sm())
        .child(
            div()
                .child(text.into())
                .text_size(px(11.))
                .text_color(color(colors::TEXT_MUTED)),
        )
}

/// Apply the shared primary-button look to a `Stateful<Div>` the caller built.
/// The caller must create the element with `div().id(...)` (which yields the
/// `Stateful<Div>` that `on_click` / `cursor_pointer` live on) and attach
/// `.on_click(...)` before passing it in.
pub fn style_button(div: Stateful<Div>) -> Stateful<Div> {
    div.px(spacing::xl())
        .py(spacing::md())
        .rounded(radii::lg())
        .bg(color(colors::ACCENT))
        .text_color(white())
        .shadow_sm()
        .cursor_pointer()
}

/// Apply the shared selectable-pill look. `selected` toggles the filled style.
/// Same call-site contract as `style_button`: pass a `Stateful<Div>` that
/// already has `.on_click(...)` attached.
pub fn style_pill(div: Stateful<Div>, selected: bool) -> Stateful<Div> {
    div.px(spacing::md())
        .py(spacing::sm())
        .rounded(radii::md())
        .border_1()
        .border_color(if selected {
            color(colors::ACCENT)
        } else {
            color(colors::BORDER)
        })
        .bg(if selected {
            color(colors::ACCENT)
        } else {
            color(colors::SURFACE)
        })
        .text_color(if selected { white() } else { color(colors::TEXT_SECONDARY) })
        .cursor_pointer()
}

/// A full-width header bar with an accent icon and title.
pub fn header(title: impl Into<SharedString>) -> Div {
    div()
        .w_full()
        .flex()
        .flex_row()
        .items_center()
        .gap(spacing::md())
        .p(spacing::lg())
        .bg(color(colors::CARD))
        .border_b_1()
        .border_color(color(colors::BORDER))
        .child(
            div()
                .w(px(24.))
                .h(px(24.))
                .rounded(radii::md())
                .bg(color(colors::ACCENT)),
        )
        .child(
            div()
                .child(title.into())
                .text_size(px(16.))
                .text_color(color(colors::TEXT_PRIMARY)),
        )
}

/// A dashed drop-zone container. The caller supplies the dynamic background and
/// children (placeholder text or image preview).
pub fn drop_zone() -> Div {
    div()
        .w_full()
        .h(px(240.))
        .rounded(radii::lg())
        .border_2()
        .border_dashed()
        .border_color(color(colors::BORDER_STRONG))
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
}

/// Small helper for centering placeholder text inside a drop zone.
pub fn drop_hint(text: impl Into<SharedString>) -> Div {
    div()
        .child(text.into())
        .text_size(px(14.))
        .text_color(color(colors::TEXT_MUTED))
        .text_center()
}
