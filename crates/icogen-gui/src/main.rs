//! `icogen-gui` — a graphical frontend for `icogen-core`, built with GPUI.
//!
//! GPUI 0.2.2 ships no built-in widgets, so buttons/checkboxes are hand-rolled
//! from `div` + click handlers. Source images are supplied by drag-and-drop,
//! the native file dialog, or a command-line argument.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod gui;
mod ui;

use std::path::PathBuf;

use icogen_core as core;
use icogen_ui::{settings, window_state};
use icogen_ui::i18n::I18nManager;
use icogen_ui::theme::ThemeManager;

use gpui::{
    App, AppContext, Application, Bounds, px, size, TitlebarOptions, WindowBounds, WindowOptions,
};

/// Identifier for persisting window placement (see `icogen_ui::window_state`).
const APP_ID: &str = "icogen-gui";

fn main() {
    // Optional args: `icogen-gui.exe [image] [-o out]`
    let args: Vec<String> = std::env::args().collect();
    let mut input_arg: Option<String> = None;
    let mut output_arg = "AppIcon.ico".to_string();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-o" | "--output" => {
                if let Some(v) = args.get(i + 1) {
                    output_arg = v.clone();
                }
                i += 2;
            }
            other if !other.starts_with('-') && input_arg.is_none() => {
                input_arg = Some(other.to_string());
                i += 1;
            }
            _ => i += 1,
        }
    }

    let platform = gpui_platform::current_platform(false);
    Application::with_platform(platform)
        .with_assets(icogen_ui::assets::Assets)
        .run(move |cx: &mut App| {
        // Load persisted preferences, then initialize theme + i18n globals.
        let prefs = settings::load();
        I18nManager::init(cx, &prefs.language_id);
        ThemeManager::init(cx, &prefs.theme_id);

        let default_bounds = Bounds::centered(None, size(px(960.), px(640.)), cx);
        let window_bounds =
            window_state::load(APP_ID, cx).unwrap_or(WindowBounds::Windowed(default_bounds));
        cx.open_window(
            WindowOptions {
                window_bounds: Some(window_bounds),
                titlebar: Some(TitlebarOptions {
                    title: Some("IcoGen".into()),
                    appears_transparent: true,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |window, cx| {
                let mut gui = gui::Gui::new();
                gui.output = output_arg.clone();
                if let Some(p) = &input_arg {
                    if let Ok(image) = core::load_image(p) {
                        gui.set_source(PathBuf::from(p), image);
                    }
                }
                cx.new(|cx| {
                    cx.observe_window_bounds(window, move |_, window, _| {
                        window_state::save(APP_ID, window.window_bounds());
                    })
                    .detach();
                    gui
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
