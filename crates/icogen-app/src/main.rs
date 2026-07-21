//! `icogen-app` — a unified GPUI frontend for IcoGen that combines the
//! `Ico` (AppIcon.ico) and `Assets` (asset PNG set) tools in a single window,
//! switched via a top tab bar.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod gui;
mod ui;

use icogen_ui::{settings, window_state};
use icogen_ui::i18n::I18nManager;
use icogen_ui::theme::ThemeManager;

use gpui::{App, AppContext, Application, Bounds, px, size, TitlebarOptions, WindowBounds, WindowOptions};

/// Identifier for persisting window placement (see `icogen_ui::window_state`).
const APP_ID: &str = "icogen-app";

fn main() {
    let platform = gpui_platform::current_platform(false);
    Application::with_platform(platform)
        .with_assets(icogen_ui::assets::Assets)
        .run(move |cx: &mut App| {
            // Load persisted preferences, then initialize theme + i18n globals.
            let prefs = settings::load();
            I18nManager::init(cx, &prefs.language_id);
            ThemeManager::init(cx, &prefs.theme_id);

            let default_bounds = Bounds::centered(None, size(px(1200.), px(760.)), cx);
            let window_bounds =
                window_state::load(APP_ID, cx).unwrap_or(WindowBounds::Windowed(default_bounds));
                println!("window_bounds: {:?}", window_bounds);
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
                |_window, cx| cx.new(|_cx| gui::Gui::new()),
            )
            .unwrap();
            cx.activate(true);
        });
}
