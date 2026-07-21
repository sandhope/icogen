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
    card, drop_hint, drop_icon, drop_zone, section_label, style_button, style_pill,
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
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = cx.global::<ThemeManager>().colors;
        let s = cx.global::<I18nManager>().strings().clone();

        let source = self.source_panel(&t, &s, cx);
        let controls = self.controls_panel(&t, &s, cx);
        let result = self.result_panel(&t);
        let bar = toolbar::toolbar(&t, cx);
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
            .child(result)
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
            .bg(if self.src_image.is_some() {
                color(t.card)
            } else {
                color(t.surface)
            })
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
            });

        card(t)
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
                            .unwrap_or_else(|| s.no_file_selected.to_string())
                    ))
                    .text_size(px(12.))
                    .text_color(color(t.text_muted))
                    .truncate(),
            )
    }

    fn controls_panel(
        &mut self,
        t: &ThemeColors,
        s: &I18nStrings,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + use<> {
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
                    t,
                )
                .child(div().child(format!("{name}\n{}x{h}", *w)).text_size(px(12.)))
            })
            .collect();

        let gen_strings = s.clone();
        let generate = style_button(div().id("generate-assets").cursor_pointer().on_click(cx.listener(
            move |this, _: &ClickEvent, _: &mut Window, cx| {
                this.generate(&gen_strings);
                cx.notify();
            },
        )), t)
        .w_full()
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
                    .child(self.out_dir.clone())
                    .text_size(px(13.))
                    .text_color(color(t.text_secondary))
                    .truncate(),
            )
            .child(section_label(s.targets, t))
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
                    .text_color(color(status_color)),
            )
    }

    fn result_panel(&self, t: &ThemeColors) -> impl IntoElement + use<> {
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
                                    .border_color(color(t.border))
                                    .rounded(radii::md())
                                    .bg(color(t.card))
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
                                    .text_color(color(t.text_muted))
                                    .text_center(),
                            )
                    })
                    .collect();
                card(t)
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
