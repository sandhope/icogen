//! `Gui` state model and the `.ico` generation logic.

use std::path::PathBuf;

use icogen_core::{self as core, Mode};
use image::RgbaImage;

use gpui::{ClickEvent, Window};

/// Preset background colors offered as clickable swatches.
pub const PRESETS: &[(&str, image::Rgba<u8>)] = &[
    ("blue", image::Rgba([43, 108, 255, 255])),
    ("slate", image::Rgba([17, 24, 39, 255])),
    ("red", image::Rgba([220, 38, 38, 255])),
    ("green", image::Rgba([22, 163, 74, 255])),
    ("amber", image::Rgba([217, 119, 6, 255])),
    ("white", image::Rgba([255, 255, 255, 255])),
];

pub struct Gui {
    pub src_path: Option<PathBuf>,
    pub src_image: Option<RgbaImage>,
    pub output: String,
    pub sizes: Vec<u32>,
    pub size_on: Vec<bool>,
    pub mode: Mode,
    pub transparent: bool,
    pub bg_color: image::Rgba<u8>,
    pub status: String,
    pub result_frames: Option<Vec<(u32, RgbaImage)>>,
}

impl Gui {
    pub fn new() -> Self {
        let sizes = core::DEFAULT_SIZES.to_vec();
        Gui {
            src_path: None,
            src_image: None,
            output: "AppIcon.ico".to_string(),
            size_on: vec![true; sizes.len()],
            sizes,
            mode: Mode::Contain,
            transparent: true,
            bg_color: image::Rgba([43, 108, 255, 255]),
            status: String::new(),
            result_frames: None,
        }
    }

    pub fn set_source(&mut self, path: PathBuf, image: RgbaImage) {
        // Default the output path to a sibling AppIcon.ico when still unset.
        if self.output == "AppIcon.ico" {
            if let Some(parent) = path.parent() {
                if !parent.as_os_str().is_empty() {
                    self.output = parent.join("AppIcon.ico").to_string_lossy().into_owned();
                }
            }
        }
        self.src_path = Some(path);
        self.src_image = Some(image);
        self.result_frames = None;
    }

    /// Open the native file dialog and load the chosen image as the source.
    /// Returns `true` if a new image was loaded.
    pub fn pick_source(&mut self) -> bool {
        let picked = rfd::FileDialog::new()
            .add_filter("Image", &["png", "jpg", "jpeg", "bmp", "gif", "webp"])
            .pick_file();
        match picked {
            Some(path) => match core::load_image(path.to_str().unwrap_or("")) {
                Ok(image) => {
                    self.set_source(path, image);
                    true
                }
                Err(e) => {
                    self.status = format!("Error loading image: {e}");
                    false
                }
            },
            None => false,
        }
    }

    /// Build the `.ico` from the current settings.
    pub fn generate(&mut self, _: &ClickEvent, _: &mut Window) {
        let src = match &self.src_image {
            Some(img) => img.clone(),
            None => {
                self.status = "Load an image first (drag it on, or click Browse).".into();
                return;
            }
        };

        let active: Vec<u32> = self
            .sizes
            .iter()
            .copied()
            .zip(self.size_on.iter().copied())
            .filter(|(_, on)| *on)
            .map(|(s, _)| s)
            .collect();

        if active.is_empty() {
            self.status = "Select at least one size.".into();
            return;
        }

        let bg = if self.transparent {
            None
        } else {
            Some(self.bg_color)
        };

        let frames = core::render_frames(&src, &active, self.mode, bg, 0.0);
        let encoded: Vec<(u32, Vec<u8>)> = frames
            .iter()
            .map(|(size, im)| {
                let data = if *size >= 256 {
                    core::encode_frame_png(im)
                } else {
                    core::encode_frame_bmp(im)
                };
                (*size, data)
            })
            .collect();

        match core::encode_ico(&encoded, &self.output) {
            Ok(()) => {
                self.result_frames = Some(frames);
                let list = active
                    .iter()
                    .map(|s| format!("{s}x{s}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                self.status = format!("Saved {}  ({})", self.output, list);
            }
            Err(e) => {
                self.status = format!("Error: {e}");
            }
        }
    }
}
