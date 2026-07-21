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

use gpui::{
    App, AppContext, Application, Bounds, px, size, TitlebarOptions, WindowBounds, WindowOptions,
};

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

    Application::new().run(move |cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(820.), px(640.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("IcoGen".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| {
                let mut gui = gui::Gui::new();
                gui.output = output_arg.clone();
                if let Some(p) = &input_arg {
                    if let Ok(image) = core::load_image(p) {
                        gui.set_source(PathBuf::from(p), image);
                    }
                }
                cx.new(|_| gui)
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
