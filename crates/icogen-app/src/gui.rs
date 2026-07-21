//! Combined `Gui` state model for the unified IcoGen app.
//!
//! Holds a single shared source image plus the two tools' settings
//! (`Ico` → AppIcon.ico, `Assets` → asset PNG set). The active tool is
//! selected via the top tab bar (`tool`).

use std::path::{Path, PathBuf};

use icogen_core as core;
use icogen_core::Mode;
use icogen_ui::i18n::I18nStrings;
use image::RgbaImage;

use gpui::{ClickEvent, Window};

/// Which tool is active in the top tab bar.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tool {
    Ico,
    Assets,
}

/// Preset background colors offered as clickable swatches (Ico mode).
pub const PRESETS: &[(&str, image::Rgba<u8>)] = &[
    ("blue", image::Rgba([43, 108, 255, 255])),
    ("slate", image::Rgba([17, 24, 39, 255])),
    ("red", image::Rgba([220, 38, 38, 255])),
    ("green", image::Rgba([22, 163, 74, 255])),
    ("amber", image::Rgba([217, 119, 6, 255])),
    ("white", image::Rgba([255, 255, 255, 255])),
];

/// (file name, width, height) — the 8 WinUI 3 / App SDK asset PNGs.
pub const TARGETS: &[(&str, u32, u32)] = &[
    ("Square150x150Logo.scale-200.png", 300, 300),
    ("Square44x44Logo.scale-200.png", 88, 88),
    ("Square44x44Logo.targetsize-24_altform-unplated.png", 24, 24),
    ("Square44x44Logo.targetsize-48_altform-lightunplated.png", 48, 48),
    ("LockScreenLogo.scale-200.png", 48, 48),
    ("StoreLogo.png", 50, 50),
    ("Wide310x150Logo.scale-200.png", 620, 300),
    ("SplashScreen.scale-200.png", 1240, 600),
];

pub struct Gui {
    /// Active tool selected in the top tab bar.
    pub tool: Tool,
    /// Shared source image for both tools.
    pub src_path: Option<PathBuf>,
    pub src_image: Option<RgbaImage>,
    // --- Ico mode ---
    pub output: String,
    pub sizes: Vec<u32>,
    pub size_on: Vec<bool>,
    pub fit: Mode,
    pub transparent: bool,
    pub bg_color: image::Rgba<u8>,
    // --- Assets mode ---
    pub out_dir: String,
    pub target_on: Vec<bool>,
    // --- Shared status (only one tool is visible at a time) ---
    pub status: String,
    /// Whether `status` is an error (drives its color in the UI).
    pub status_is_error: bool,
}

impl Gui {
    pub fn new() -> Self {
        let sizes = core::DEFAULT_SIZES.to_vec();
        Gui {
            tool: Tool::Ico,
            src_path: None,
            src_image: None,
            output: "AppIcon.ico".to_string(),
            size_on: vec![true; sizes.len()],
            sizes,
            fit: Mode::Contain,
            transparent: true,
            bg_color: image::Rgba([43, 108, 255, 255]),
            out_dir: "Assets".to_string(),
            target_on: vec![true; TARGETS.len()],
            status: String::new(),
            status_is_error: false,
        }
    }

    pub fn set_source(&mut self, path: PathBuf, image: RgbaImage) {
        self.src_path = Some(path);
        self.src_image = Some(image);
        self.status.clear();
        self.status_is_error = false;
    }

    /// Open the native file dialog and load the chosen image as the source.
    /// Returns `true` if a new image was loaded.
    pub fn pick_source(&mut self, s: &I18nStrings) -> bool {
        let picked = rfd::FileDialog::new()
            .add_filter(s.image_filter, &["png", "jpg", "jpeg", "bmp", "gif", "webp"])
            .pick_file();
        match picked {
            Some(path) => match core::load_image(path.to_str().unwrap_or("")) {
                Ok(image) => {
                    self.set_source(path, image);
                    true
                }
                Err(e) => {
                    self.status = format!("{}{e}", s.error_loading_image);
                    self.status_is_error = true;
                    false
                }
            },
            None => false,
        }
    }

    /// Open the native save dialog to choose the `.ico` output path.
    /// Returns `true` if a path was chosen.
    pub fn pick_output(&mut self) -> bool {
        let picked = rfd::FileDialog::new()
            .add_filter("ICO", &["ico"])
            .set_file_name(&self.output)
            .save_file();
        if let Some(path) = picked {
            self.output = path.to_string_lossy().into_owned();
            true
        } else {
            false
        }
    }

    /// Open the native folder dialog to choose the output directory.
    /// Returns `true` if a directory was chosen.
    pub fn pick_out_dir(&mut self) -> bool {
        let picked = rfd::FileDialog::new()
            .set_directory(&self.out_dir)
            .pick_folder();
        if let Some(path) = picked {
            self.out_dir = path.to_string_lossy().into_owned();
            true
        } else {
            false
        }
    }

    /// Build the `.ico` from the current Ico-mode settings.
    pub fn generate_ico(&mut self, _: &ClickEvent, _: &mut Window, s: &I18nStrings) {
        let src = match &self.src_image {
            Some(img) => img.clone(),
            None => {
                self.status = s.load_image_first.into();
                self.status_is_error = false;
                return;
            }
        };

        let active: Vec<u32> = self
            .sizes
            .iter()
            .copied()
            .zip(self.size_on.iter().copied())
            .filter(|(_, on)| *on)
            .map(|(sz, _)| sz)
            .collect();

        if active.is_empty() {
            self.status = s.select_at_least_one_size.into();
            self.status_is_error = false;
            return;
        }

        let bg = if self.transparent {
            None
        } else {
            Some(self.bg_color)
        };

        let frames = core::render_frames(&src, &active, self.fit, bg, 0.0);
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
                let list = active
                    .iter()
                    .map(|sz| format!("{sz}x{sz}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                self.status = format!("{}{}  ({})", s.saved_prefix, self.output, list);
                self.status_is_error = false;
            }
            Err(e) => {
                self.status = format!("{}{e}", s.error_prefix);
                self.status_is_error = true;
            }
        }
    }

    /// Write the enabled asset PNGs to `out_dir/`.
    pub fn generate_assets(&mut self, s: &I18nStrings) {
        let src = match &self.src_image {
            Some(img) => img.clone(),
            None => {
                self.status = s.load_image_first.into();
                self.status_is_error = false;
                return;
            }
        };

        let out_dir = Path::new(&self.out_dir);
        if let Err(e) = std::fs::create_dir_all(out_dir) {
            self.status = format!(
                "{}{}{}{e}",
                s.error_creating_prefix, self.out_dir, s.error_creating_middle
            );
            self.status_is_error = true;
            return;
        }

        let mut saved = 0;
        for (i, (name, w, h)) in TARGETS.iter().enumerate() {
            if !self.target_on[i] {
                continue;
            }
            let img = core::render_canvas(&src, *w, *h);
            let out = out_dir.join(name);
            match img.save(&out) {
                Ok(()) => saved += 1,
                Err(e) => {
                    self.status = format!("{}{name}{}{e}", s.error_writing_prefix, s.error_writing_middle);
                    self.status_is_error = true;
                    return;
                }
            }
        }

        if saved == 0 {
            self.status = s.select_at_least_one_target.into();
            self.status_is_error = false;
        } else {
            self.status = format!(
                "{}{}{}{}{}",
                s.saved_assets_prefix, saved, s.saved_assets_middle, self.out_dir, s.saved_assets_suffix
            );
            self.status_is_error = false;
        }
    }
}
