//! Reusable styled components shared by the icogen GUIs.
//!
//! `card`, `section_label`, `drop_zone`, `drop_icon`, `drop_hint`,
//! `style_button`, `style_pill`, `folder_button`, `target_card` and
//! `wide_indicator` are styled against the active theme so both front-ends
//! stay visually in sync.

use gpui::prelude::*;
use gpui::{div, px, svg, white, Div, SharedString, Stateful};

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
                .text_size(px(12.))
                .text_color(color(t.text_secondary))
                .font_weight(gpui::FontWeight::MEDIUM),
        )
}

/// Apply the shared primary-button look to a `Stateful<Div>` the caller built.
/// Solid accent fill, white label, generous padding.
pub fn style_button(div: Stateful<Div>, t: &ThemeColors) -> Stateful<Div> {
    div.h(px(44.))
        .w_full()
        .flex()
        .items_center()
        .justify_center()
        .rounded(radii::lg())
        .bg(color(t.accent))
        .text_color(white())
        .cursor_pointer()
        .font_weight(gpui::FontWeight::MEDIUM)
        .hover(|s| s.bg(color(t.accent_hover)))
        .active(|s| s.bg(color(t.accent_dark)))
}

/// Apply the shared selectable-pill look. `selected` toggles the filled style.
/// Unselected: light surface, 1px border, muted text.
/// Selected:   accent tint, 2px accent border, accent text.
pub fn style_pill(div: Stateful<Div>, selected: bool, t: &ThemeColors) -> Stateful<Div> {
    div.h(px(32.))
        .px(spacing::md())
        .flex()
        .items_center()
        .justify_center()
        .rounded(radii::md())
        .border(if selected { gpui::px(2.) } else { gpui::px(1.) })
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

/// A small folder icon button used next to output paths.
/// The caller supplies a `Stateful<Div>` with `.id(...)` and `.on_click(...)`.
/// The folder glyph is rendered from the embedded `icons/folder.svg` and tinted
/// with the button's `text_color`; it is the direct child of the flex button so
/// it stays perfectly centered (GPUI paints an SVG as an alpha mask filled by
/// `text_color`, so the color inside the file is ignored).
pub fn folder_button(btn: Stateful<Div>, t: &ThemeColors) -> Stateful<Div> {
    btn.w(px(36.))
        .h(px(36.))
        .flex()
        .items_center()
        .justify_center()
        .rounded(radii::md())
        .bg(color(t.surface))
        .border_1()
        .border_color(color(t.border))
        .text_color(color(t.text_secondary))
        .cursor_pointer()
        .hover(|s| s.bg(color(t.surface_hover)))
        .child(
            svg()
                .path("icons/folder.svg")
                .w(px(16.))
                .h(px(16.))
                .text_color(color(t.text_secondary)),
        )
}

/// A selectable target card for the Assets GUI. Shows a name and a size
/// subtitle; wide targets include a small wide-screen glyph on the right.
pub fn target_card(
    div: Stateful<Div>,
    selected: bool,
    t: &ThemeColors,
) -> Stateful<Div> {
    div.h(px(48.))
        .px(spacing::md())
        .flex()
        .items_center()
        .justify_between()
        .rounded(radii::md())
        .border(if selected { gpui::px(1.) } else { gpui::px(1.) })
        .border_color(if selected {
            color(t.accent)
        } else {
            color(t.border)
        })
        .bg(if selected {
            color(t.accent_tint)
        } else {
            color(t.card)
        })
        .cursor_pointer()
}

/// Small wide-screen glyph placed inside wide target cards.
pub fn wide_indicator(t: &ThemeColors) -> Div {
    div()
        .w(px(18.))
        .h(px(12.))
        .flex()
        .items_center()
        .justify_center()
        .child(
            div()
                .w_full()
                .h_full()
                .rounded(px(2.))
                .border_1()
                .border_color(color(t.accent))
                .child(
                    div()
                        .mt(px(3.))
                        .mx(px(2.))
                        .h(px(4.))
                        .rounded(px(1.))
                        .bg(color(t.accent).opacity(0.3)),
                ),
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
        .bg(color(t.surface))
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .cursor_pointer()
        .hover(|s| s.border_color(color(t.accent)).bg(color(t.accent_tint)))
}

/// Upload icon (an arrow rising out of a tray) shown in the center of an empty
/// drop zone. Rendered from the embedded `icons/upload.svg` and tinted with the
/// muted text color (GPUI paints an SVG as an alpha mask filled by `text_color`).
pub fn drop_icon(t: &ThemeColors) -> Div {
    div()
        .mb(spacing::md())
        .flex()
        .items_center()
        .justify_center()
        .child(
            svg()
                .path("icons/upload.svg")
                .w(px(40.))
                .h(px(40.))
                .text_color(color(t.text_muted).opacity(0.6)),
        )
}

/// Small helper for centering placeholder text inside a drop zone.
pub fn drop_hint(text: impl Into<SharedString>, t: &ThemeColors) -> Div {
    div()
        .child(text.into())
        .text_size(px(13.))
        .text_color(color(t.text_muted))
        .text_center()
        .line_height(px(18.))
}

/// Tiny muted helper text (e.g. "Supports PNG, JPG, SVG, WebP").
pub fn helper_text(text: impl Into<SharedString>, t: &ThemeColors) -> Div {
    div()
        .child(text.into())
        .text_size(px(11.))
        .text_color(color(t.text_muted))
}
