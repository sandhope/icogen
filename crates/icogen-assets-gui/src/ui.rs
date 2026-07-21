//! Rendering for `Gui`: layout, panels, and event wiring.

use std::sync::Arc;

use icogen_core as core;
use image::RgbaImage;

use gpui::prelude::*;
use gpui::{
    ClickEvent, Context, ExternalPaths, Render, RenderImage, Window, div, img, px,
};

use icogen_ui::color::color;
use icogen_ui::components::{
    card, drop_hint, drop_zone, section_label, style_button, style_pill,
};
use icogen_ui::theme::colors;
use icogen_ui::theme::radii;
use icogen_ui::theme::spacing;

use crate::gui::{Gui, TARGETS};

/// Wrap an RGBA image as an `Arc<RenderImage>` for preview display.
fn render_img(img: &RgbaImage) -> Arc<RenderImage> {
    Arc::new(RenderImage::new(vec![image::Frame::new(img.clone())]))
}

impl Render for Gui {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let source = self.source_panel(cx);
        let controls = self.controls_panel(cx);
        let result = self.result_panel();
        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(color(colors::BG))
            .text_color(color(colors::TEXT_PRIMARY))
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
            .child(result)
    }
}

impl Gui {
    fn source_panel(&mut self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let drop = drop_zone()
            .id("drop")
            .bg(if self.src_image.is_some() {
                color(colors::CARD)
            } else {
                color(colors::SURFACE)
            })
            .on_drop(cx.listener(|this, paths: &ExternalPaths, _: &mut Window, cx| {
                if let Some(p) = paths.paths().first() {
                    if let Ok(image) = core::load_image(p.to_str().unwrap_or("")) {
                        this.set_source(p.clone(), image);
                        cx.notify();
                    }
                }
            }))
            .on_click(cx.listener(|this, _: &ClickEvent, _: &mut Window, cx| {
                if this.pick_source() {
                    cx.notify();
                }
            }))
            .child(if let Some(buf) = &self.src_image {
                div()
                    .w(px(180.))
                    .h(px(180.))
                    .bg(color(colors::CARD))
                    .border_1()
                    .border_color(color(colors::BORDER))
                    .rounded(radii::lg())
                    .shadow_sm()
                    .child(img(render_img(buf)).w(px(180.)).h(px(180.)))
            } else {
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(spacing::sm())
                    .child(
                        div()
                            .w(px(48.))
                            .h(px(48.))
                            .rounded(radii::md())
                            .bg(color(colors::ACCENT_LIGHT))
                            .mb(spacing::sm()),
                    )
                    .child(drop_hint("Drag an image here\nor click to browse"))
            });

        card()
            .w(px(340.))
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
                            .unwrap_or_else(|| "No file selected".to_string())
                    ))
                    .text_size(px(12.))
                    .text_color(color(colors::TEXT_MUTED))
                    .truncate(),
            )
    }

    fn controls_panel(&mut self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let targets: Vec<_> = TARGETS
            .iter()
            .enumerate()
            .map(|(i, (name, w, h))| {
                let on = self.target_on[i];
                style_pill(
                    div()
                        .id(("tgt", i as u32))
                        .cursor_pointer()
                        .on_click(cx.listener(move |this, _: &ClickEvent, _: &mut Window, cx| {
                            this.target_on[i] = !this.target_on[i];
                            cx.notify();
                        })),
                    on,
                )
                .child(div().child(format!("{name}\n{}x{h}", *w)).text_size(px(12.)))
            })
            .collect();

        let generate = style_button(div().id("generate-assets").cursor_pointer().on_click(cx.listener(
            |this, _: &ClickEvent, _: &mut Window, cx| {
                this.generate();
                cx.notify();
            },
        )))
        .w_full()
        .child(div().child("Generate asset PNGs").text_size(px(15.)));

        card()
            .flex_1()
            .flex()
            .flex_col()
            .gap(spacing::lg())
            .child(section_label("Output directory"))
            .child(
                div()
                    .child(self.out_dir.clone())
                    .text_size(px(13.))
                    .text_color(color(colors::TEXT_SECONDARY))
                    .truncate(),
            )
            .child(section_label("Targets"))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap(spacing::sm())
                    .children(targets),
            )
            .child(div().flex_1())
            .child(generate)
            .child(
                div()
                    .child(self.status.clone())
                    .text_size(px(12.))
                    .text_color(color(colors::SUCCESS)),
            )
    }

    fn result_panel(&self) -> impl IntoElement + use<> {
        match &self.result_thumbs {
            None => div().h(px(0.)),
            Some(thumbs) => {
                let items: Vec<_> = thumbs
                    .iter()
                    .map(|(label, buf)| {
                        div()
                            .flex_none()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap(spacing::xs())
                            .child(
                                div()
                                    .w(px(80.))
                                    .h(px(80.))
                                    .border_1()
                                    .border_color(color(colors::BORDER))
                                    .rounded(radii::md())
                                    .bg(color(colors::CARD))
                                    .shadow_sm()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(img(render_img(buf)).w(px(72.)).h(px(72.))),
                            )
                            .child(
                                div()
                                    .child(label.clone())
                                    .text_size(px(10.))
                                    .text_color(color(colors::TEXT_MUTED))
                                    .text_center(),
                            )
                    })
                    .collect();
                card()
                    .p(spacing::lg())
                    .flex_none()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap(spacing::md())
                    .children(items)
            }
        }
    }
}
