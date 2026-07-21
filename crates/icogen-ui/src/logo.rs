//! Embedded application logo (`assets/logo.png`), decoded once into a GPUI
//! `RenderImage` for display in the title bar.
//!
//! The file is compiled into the binary via `include_bytes!` so the logo is
//! always available regardless of the working directory at runtime.

use std::sync::{Arc, OnceLock};

use gpui::RenderImage;
use image::RgbaImage;

/// Raw bytes of `assets/logo.png`, embedded at compile time.
const LOGO_BYTES: &[u8] = include_bytes!("../../../assets/logo.png");

/// Process-wide cached logo. Decoded lazily on first use and reused for every
/// paint (the title bar re-renders frequently).
static APP_LOGO: OnceLock<Arc<RenderImage>> = OnceLock::new();

/// Return the application logo as a decoded `RenderImage`, computed once and
/// cached for the lifetime of the process.
pub fn app_logo() -> Arc<RenderImage> {
    APP_LOGO
        .get_or_init(|| {
            let dynamic = image::load_from_memory(LOGO_BYTES)
                .expect("failed to decode embedded assets/logo.png");
            let rgba: RgbaImage = dynamic.to_rgba8();
            Arc::new(RenderImage::new(vec![image::Frame::new(rgba)]))
        })
        .clone()
}
