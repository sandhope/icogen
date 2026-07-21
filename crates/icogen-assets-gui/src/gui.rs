//! `Gui` state model and the asset-PNG generation logic.

use std::path::{Path, PathBuf};

use icogen_core as core;
use image::RgbaImage;

/// (file name, width, height) — the 8 WinUI 3 / App SDK asset PNGs.
pub const TARGETS: &[(&str, u32, u32)] = &[
    ("Square150x150Logo.scale-200.png", 300, 300),
    ("Square44x44Logo.scale-200.png", 88, 88),
    (
        "Square44x44Logo.targetsize-24_altform-unplated.png",
        24,
        24,
    ),
    (
        "Square44x44Logo.targetsize-48_altform-lightunplated.png",
        48,
        48,
    ),
    ("LockScreenLogo.scale-200.png", 48, 48),
    ("StoreLogo.png", 50, 50),
    ("Wide310x150Logo.scale-200.png", 620, 300),
    ("SplashScreen.scale-200.png", 1240, 600),
];

pub struct Gui {
    pub src_path: Option<PathBuf>,
    pub src_image: Option<RgbaImage>,
    pub out_dir: String,
    pub target_on: Vec<bool>,
    pub status: String,
    pub result_thumbs: Option<Vec<(String, RgbaImage)>>,
}

impl Gui {
    pub fn new() -> Self {
        Gui {
            src_path: None,
            src_image: None,
            out_dir: "Assets".to_string(),
            target_on: vec![true; TARGETS.len()],
            status: String::new(),
            result_thumbs: None,
        }
    }

    pub fn set_source(&mut self, path: PathBuf, image: RgbaImage) {
        self.src_path = Some(path);
        self.src_image = Some(image);
        self.result_thumbs = None;
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

    /// Write the enabled asset PNGs to `out_dir/`.
    pub fn generate(&mut self) {
        let src = match &self.src_image {
            Some(img) => img.clone(),
            None => {
                self.status = "Load an image first (drag it on, or click Browse).".into();
                return;
            }
        };

        let out_dir = Path::new(&self.out_dir);
        if let Err(e) = std::fs::create_dir_all(out_dir) {
            self.status = format!("Error creating {}: {e}", self.out_dir);
            return;
        }

        let mut saved = 0;
        let mut thumbs = Vec::new();
        for (i, (name, w, h)) in TARGETS.iter().enumerate() {
            if !self.target_on[i] {
                continue;
            }
            let img = core::render_canvas(&src, *w, *h);
            let out = out_dir.join(name);
            match img.save(&out) {
                Ok(()) => {
                    saved += 1;
                    thumbs.push((format!("{name} ({}x{h})", *w), img));
                }
                Err(e) => {
                    self.status = format!("Error writing {name}: {e}");
                    return;
                }
            }
        }

        self.result_thumbs = Some(thumbs);
        self.status = if saved == 0 {
            "Select at least one target.".into()
        } else {
            format!(
                "Saved {saved} asset PNGs to {}/  (AppIcon.ico unchanged)",
                self.out_dir
            )
        };
    }
}
