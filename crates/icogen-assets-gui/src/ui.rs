//! Rendering for `Gui`: layout, panels, and event wiring.

use std::sync::Arc;

use icogen_core as core;
use image::RgbaImage;

use gpui::prelude::*;
use gpui::{
    ClickEvent, Context, Div, ExternalPaths, Render, RenderImage, Stateful, Window, div, img, px,
};

use icogen_ui::color::color;
use icogen_ui::components::{
    card, drop_hint, drop_icon, drop_zone, folder_button, helper_text, section_label,
    style_button, target_card, wide_indicator,
};
use icogen_ui::i18n::{I18nManager, I18nStrings};
use icogen_ui::theme::radii;
use icogen_ui::theme::spacing;
use icogen_ui::theme::{ThemeColors, ThemeManager};
use icogen_ui::toolbar;

use crate::gui::{Gui, TARGETS};

/// Wrap an RGBA image as an `Arc<RenderImage>` for preview display.
fn render_img(img: &RgbaImage) -> Arc<RenderImage> {
    Arc::new(RenderImage::new(vec![image::Frame::new(img.clone())]))
}

impl Render for Gui {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = cx.global::<ThemeManager>().colors;
        let s = cx.global::<I18nManager>().strings().clone();

        let source = self.source_panel(&t, &s, cx);
        let controls = self.controls_panel(&t, &s, cx);
        let bar = toolbar::toolbar("IcoGen Assets", &t, window, cx);
        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(color(t.bg))
            .text_color(color(t.text_primary))
            .child(bar)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_1()
                    .gap(spacing::lg())
                    .p(spacing::lg())
                    .pt(spacing::md())
                    .child(source)
                    .child(controls),
            )
    }
}

impl Gui {
    fn source_panel(
        &mut self,
        t: &ThemeColors,
        s: &I18nStrings,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + use<> {
        let pick_strings = s.clone();
        let drop = drop_zone(t)
            .id("drop")
            .on_drop(cx.listener(|this, paths: &ExternalPaths, _: &mut Window, cx| {
                if let Some(p) = paths.paths().first() {
                    if let Ok(image) = core::load_image(p.to_str().unwrap_or("")) {
                        this.set_source(p.clone(), image);
                        cx.notify();
                    }
                }
            }))
            .on_click(cx.listener(move |this, _: &ClickEvent, _: &mut Window, cx| {
                if this.pick_source(&pick_strings) {
                    cx.notify();
                }
            }))
            .child(if let Some(buf) = &self.src_image {
                div()
                    .w(px(180.))
                    .h(px(180.))
                    .bg(color(t.card))
                    .border_1()
                    .border_color(color(t.border))
                    .rounded(radii::lg())
                    .shadow_sm()
                    .child(img(render_img(buf)).w(px(180.)).h(px(180.)))
            } else {
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .child(drop_icon(t))
                    .child(drop_hint(s.drop_hint, t))
            })
            .cursor_pointer();

        card(t)
            .w(px(360.))
            .flex_none()
            .flex()
            .flex_col()
            .gap(spacing::md())
            .child(drop)
            .child(
                div()
                    .child(format!(
                        "{}",
                        self.src_path
                            .as_ref()
                            .map(|p| p.display().to_string())
                            .unwrap_or_else(|| s.no_file_selected.to_string())
                    ))
                    .text_size(px(12.))
                    .text_color(color(t.text_muted))
                    .truncate(),
            )
            .child(helper_text(s.supported_formats, t))
    }

    fn controls_panel(
        &mut self,
        t: &ThemeColors,
        s: &I18nStrings,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + use<> {
        // Lay out the targets as 2 per row (4 rows for the 8 built-in targets).
        // Group them into row containers of 2 so the grid is deterministic
        // regardless of panel width, instead of relying on flex-wrap.
        let mut rows: Vec<Div> = Vec::new();
        let mut idx = 0;
        while idx < TARGETS.len() {
            let mut row_children: Vec<Stateful<Div>> = Vec::new();
            for j in idx..(idx + 2).min(TARGETS.len()) {
                let (name, w, h) = TARGETS[j];
                let on = self.target_on[j];
                let is_wide = w > h;
                let display_name = name.strip_suffix(".png").unwrap_or(name);
                let card = target_card(
                    div()
                        .id(("tgt", j as u32))
                        .min_w(px(160.))
                        .flex_1()
                        .on_click(cx.listener(move |this, _: &ClickEvent, _: &mut Window, cx| {
                            this.target_on[j] = !this.target_on[j];
                            cx.notify();
                        })),
                    on,
                    t,
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .justify_center()
                        .min_w(px(0.))
                        .flex_1()
                        .mr(spacing::sm())
                        .child(
                            div()
                                .child(display_name.to_string())
                                .text_size(px(11.))
                                .font_weight(if on {
                                    gpui::FontWeight::MEDIUM
                                } else {
                                    gpui::FontWeight::NORMAL
                                })
                                .text_color(if on { color(t.accent) } else { color(t.text_primary) })
                                .truncate(),
                        )
                        .child(
                            div()
                                .child(format!("{}×{h}", w))
                                .text_size(px(10.))
                                .text_color(if on {
                                    color(t.accent).opacity(0.7)
                                } else {
                                    color(t.text_muted)
                                }),
                        ),
                )
                .when(is_wide, |this| this.child(wide_indicator(t)));
                row_children.push(card);
            }
            rows.push(
                div()
                    .flex()
                    .flex_row()
                    .gap(spacing::sm())
                    .children(row_children),
            );
            idx += 2;
        }

        let gen_strings = s.clone();
        let generate = style_button(div().id("generate-assets").cursor_pointer().on_click(cx.listener(
            move |this, _: &ClickEvent, _: &mut Window, cx| {
                this.generate(&gen_strings);
                cx.notify();
            },
        )), t)
        .child(div().child(s.generate_assets).text_size(px(14.)));

        let status_color = if self.status_is_error { t.error } else { t.success };

        card(t)
            .flex_1()
            .flex()
            .flex_col()
            .gap(spacing::lg())
            .child(section_label(s.output_directory, t))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(spacing::sm())
                    .child(
                        div()
                            .flex_1()
                            .h(px(36.))
                            .px(spacing::md())
                            .flex()
                            .items_center()
                            .bg(color(t.surface))
                            .border_1()
                            .border_color(color(t.border))
                            .rounded(radii::md())
                            .child(
                                div()
                                    .child(self.out_dir.clone())
                                    .text_size(px(13.))
                                    .text_color(color(t.text_primary))
                                    .truncate(),
                            ),
                    )
                    .child(
                        folder_button(
                            div()
                                .id("pick-out-dir")
                                .cursor_pointer()
                                .on_click(cx.listener(|this, _: &ClickEvent, _: &mut Window, cx| {
                                    if this.pick_out_dir() {
                                        cx.notify();
                                    }
                                })),
                            t,
                        ),
                    ),
            )
            .child(section_label(s.targets, t))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(spacing::sm())
                    .children(rows),
            )
            .child(div().flex_1())
            .child(generate)
            .child(
                div()
                    .child(self.status.clone())
                    .text_size(px(12.))
                    .text_color(color(status_color))
                    .text_center(),
            )
    }
}
