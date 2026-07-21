//! Rendering for the combined `Gui`: layout, tab bar, panels, and wiring.

use std::sync::Arc;

use icogen_core as core;
use icogen_core::Mode;
use image::RgbaImage;

use gpui::prelude::*;
use gpui::{ClickEvent, Context, Div, ExternalPaths, Render, RenderImage, Stateful, Window, div, img, px};

use icogen_ui::color::{color, hex_rgb};
use icogen_ui::components::{
    card, drop_hint, drop_icon, drop_zone, folder_button, helper_text, section_label,
    style_button, style_pill, target_card, wide_indicator,
};
use icogen_ui::i18n::{I18nManager, I18nStrings};
use icogen_ui::theme::radii;
use icogen_ui::theme::spacing;
use icogen_ui::theme::{ThemeColors, ThemeManager};
use icogen_ui::toolbar;

use crate::gui::{Gui, PRESETS, TARGETS, Tool};

/// Wrap an RGBA image as an `Arc<RenderImage>` for preview display.
fn render_img(img: &RgbaImage) -> Arc<RenderImage> {
    Arc::new(RenderImage::new(vec![image::Frame::new(img.clone())]))
}

impl Render for Gui {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let t = cx.global::<ThemeManager>().colors;
        let s = cx.global::<I18nManager>().strings().clone();

        let bar = toolbar::toolbar("IcoGen", &t, window, cx);
        let tabs = self.tab_bar(&t, &s, cx);
        let source = self.source_panel(&t, &s, cx);
        let controls = self.controls_panel(&t, &s, cx);

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(color(t.bg))
            .text_color(color(t.text_primary))
            .child(bar)
            .child(tabs)
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
    /// Top tab bar: switch between the `Ico` and `Assets` tools.
    fn tab_bar(&mut self, t: &ThemeColors, s: &I18nStrings, cx: &mut Context<Self>) -> Div {
        let ico = tool_tab(
            div()
                .id("tab-ico")
                .cursor_pointer()
                .on_click(cx.listener(|this, _: &ClickEvent, _: &mut Window, cx| {
                    this.tool = Tool::Ico;
                    cx.notify();
                })),
            self.tool == Tool::Ico,
            s.tab_ico,
            t,
        );
        let assets = tool_tab(
            div()
                .id("tab-assets")
                .cursor_pointer()
                .on_click(cx.listener(|this, _: &ClickEvent, _: &mut Window, cx| {
                    this.tool = Tool::Assets;
                    cx.notify();
                })),
            self.tool == Tool::Assets,
            s.tab_assets,
            t,
        );
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(spacing::sm())
            .px(spacing::lg())
            .py(spacing::sm())
            .child(ico)
            .child(assets)
    }

    /// Shared source panel (identical for both tools).
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

    /// Mode-specific controls: render the active tool's panel.
    fn controls_panel(
        &mut self,
        t: &ThemeColors,
        s: &I18nStrings,
        cx: &mut Context<Self>,
    ) -> Div {
        match self.tool {
            Tool::Ico => self.ico_controls(&t, &s, cx),
            Tool::Assets => self.assets_controls(&t, &s, cx),
        }
    }

    /// Ico-mode controls: output path, sizes, fit, background.
    fn ico_controls(
        &mut self,
        t: &ThemeColors,
        s: &I18nStrings,
        cx: &mut Context<Self>,
    ) -> Div {
        let sizes: Vec<_> = self
            .sizes
            .iter()
            .enumerate()
            .map(|(i, &sz)| {
                let on = self.size_on[i];
                style_pill(
                    div()
                        .id(("sz", i as u32))
                        .cursor_pointer()
                        .on_click(cx.listener(move |this, _: &ClickEvent, _: &mut Window, cx| {
                            this.size_on[i] = !this.size_on[i];
                            cx.notify();
                        })),
                    on,
                    t,
                )
                .child(div().child(sz.to_string()).text_size(px(13.)))
            })
            .collect();

        let swatches: Vec<_> = PRESETS
            .iter()
            .enumerate()
            .map(|(idx, (_name, swatch))| {
                let c = *swatch;
                div()
                    .id(("bg", idx as u32))
                    .w(px(28.))
                    .h(px(28.))
                    .rounded(radii::sm())
                    .border_2()
                    .border_color(if self.bg_color == c {
                        color(t.text_primary)
                    } else {
                        color(t.border)
                    })
                    .bg(color(hex_rgb(c)))
                    .cursor_pointer()
                    .on_click(cx.listener(move |this, _: &ClickEvent, _: &mut Window, cx| {
                        this.transparent = false;
                        this.bg_color = c;
                        cx.notify();
                    }))
            })
            .collect();

        let mode_contain = style_pill(
            div().id("mode-contain").cursor_pointer().on_click(cx.listener(|this, _: &ClickEvent, _: &mut Window, cx| {
                this.fit = Mode::Contain;
                cx.notify();
            })),
            self.fit == Mode::Contain,
            t,
        )
        .child(div().child(s.contain).text_size(px(13.)));
        let mode_cover = style_pill(
            div().id("mode-cover").cursor_pointer().on_click(cx.listener(|this, _: &ClickEvent, _: &mut Window, cx| {
                this.fit = Mode::Cover;
                cx.notify();
            })),
            self.fit == Mode::Cover,
            t,
        )
        .child(div().child(s.cover).text_size(px(13.)));
        let toggle_transparent = style_pill(
            div().id("toggle-transparent").cursor_pointer().on_click(cx.listener(|this, _: &ClickEvent, _: &mut Window, cx| {
                this.transparent = !this.transparent;
                cx.notify();
            })),
            self.transparent,
            t,
        )
        .child(div().child(s.transparent).text_size(px(13.)));
        let toggle_opaque = style_pill(
            div().id("toggle-opaque").cursor_pointer().on_click(cx.listener(|this, _: &ClickEvent, _: &mut Window, cx| {
                this.transparent = !this.transparent;
                cx.notify();
            })),
            !self.transparent,
            t,
        )
        .child(div().child(s.opaque).text_size(px(13.)));

        let gen_strings = s.clone();
        let generate = style_button(div().id("generate").cursor_pointer().on_click(cx.listener(
            move |this, ev: &ClickEvent, window, cx| {
                this.generate_ico(ev, window, &gen_strings);
                cx.notify();
            },
        )), t)
        .child(div().child(s.generate_ico).text_size(px(14.)));

        let status_color = if self.status_is_error { t.error } else { t.success };

        card(t)
            .flex_1()
            .flex()
            .flex_col()
            .gap(spacing::lg())
            .child(section_label(s.output, t))
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
                            .border_1()
                            .border_color(color(t.border))
                            .rounded(radii::md())
                            .bg(color(t.card))
                            .child(
                                div()
                                    .child(self.output.clone())
                                    .text_size(px(13.))
                                    .text_color(color(t.text_primary))
                                    .truncate(),
                            ),
                    )
                    .child(
                        folder_button(
                            div()
                                .id("pick-output")
                                .cursor_pointer()
                                .on_click(cx.listener(|this, _: &ClickEvent, _: &mut Window, cx| {
                                    if this.pick_output() {
                                        cx.notify();
                                    }
                                })),
                            t,
                        ),
                    ),
            )
            .child(section_label(s.sizes, t))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap(spacing::sm())
                    .children(sizes),
            )
            .child(section_label(s.fit_mode, t))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(spacing::sm())
                    .child(mode_contain)
                    .child(mode_cover),
            )
            .child(section_label(s.background, t))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(spacing::sm())
                    .child(toggle_transparent)
                    .child(toggle_opaque),
            )
            .when(!self.transparent, |this| {
                this.child(section_label(s.color, t))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .flex_wrap()
                            .gap(spacing::sm())
                            .children(swatches),
                    )
            })
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

    /// Assets-mode controls: output directory + 2-per-row target grid.
    fn assets_controls(
        &mut self,
        t: &ThemeColors,
        s: &I18nStrings,
        cx: &mut Context<Self>,
    ) -> Div {
        // Lay out the 8 targets as 2 per row (4 rows). Group them into row
        // containers of 2 so the grid is deterministic regardless of width.
        let mut rows: Vec<Div> = Vec::new();
        let mut idx = 0;
        while idx < TARGETS.len() {
            let mut row_children: Vec<Stateful<Div>> = Vec::new();
            for j in idx..(idx + 2).min(TARGETS.len()) {
                let (name, w, h) = TARGETS[j];
                let on = self.target_on[j];
                let is_wide = w > h;
                let display_name = name.strip_suffix(".png").unwrap_or(name);
                let card_el = target_card(
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
                row_children.push(card_el);
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
                this.generate_assets(&gen_strings);
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

/// A single tab in the top tab bar, rendered with the shared `style_pill` look.
fn tool_tab(btn: Stateful<Div>, selected: bool, label: &'static str, t: &ThemeColors) -> Stateful<Div> {
    style_pill(btn, selected, t).child(div().child(label).text_size(px(13.)))
}
