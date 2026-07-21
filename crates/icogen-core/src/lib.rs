//! Core logic shared by the `icogen`, `icogen-gui`, `icogen-assets` and
//! `icogen-assets-gui` front-ends.
//!
//! Generates a multi-resolution Windows `AppIcon.ico` from an image:
//!   - the 256px frame is stored as PNG (Windows Vista+),
//!   - smaller frames are stored as 32-bit BGRA BMP with an alpha channel.
//!
//! The embedded resolutions let Windows pick the right one for the system
//! tray, taskbar, Alt-Tab, Task Manager and Explorer.

use std::io::Write;

use image::imageops::{self, FilterType};
use image::{Frame, Rgba, RgbaImage};

/// Default set of resolutions embedded in the `.ico`.
pub const DEFAULT_SIZES: &[u32] = &[16, 24, 32, 48, 64, 128, 256];

/// How the source image is fitted into each square canvas.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    /// Fit entirely inside, keeping aspect ratio (transparent padding).
    Contain,
    /// Cover the whole canvas, cropping overflow.
    Cover,
}

/// Decode an image file into an RGBA buffer.
pub fn load_image(path: &str) -> Result<RgbaImage, String> {
    let img = image::open(path)
        .map_err(|e| format!("cannot open image '{path}': {e}"))?
        .to_rgba8();
    Ok(img)
}

/// Parse a color from a hex (`#rrggbb` / `#rrggbbaa`) or named string.
pub fn parse_color(s: &str) -> Option<Rgba<u8>> {
    let named = match s.to_ascii_lowercase().as_str() {
        "white" => Some((255, 255, 255)),
        "black" => Some((0, 0, 0)),
        "red" => Some((255, 0, 0)),
        "green" => Some((0, 128, 0)),
        "blue" => Some((0, 0, 255)),
        "transparent" | "none" => return Some(Rgba([0, 0, 0, 0])),
        _ => None,
    };
    if let Some((r, g, b)) = named {
        return Some(Rgba([r, g, b, 255]));
    }
    let h = s.strip_prefix('#').unwrap_or(s);
    match h.len() {
        6 => {
            let r = u8::from_str_radix(&h[0..2], 16).ok()?;
            let g = u8::from_str_radix(&h[2..4], 16).ok()?;
            let b = u8::from_str_radix(&h[4..6], 16).ok()?;
            Some(Rgba([r, g, b, 255]))
        }
        8 => {
            let r = u8::from_str_radix(&h[0..2], 16).ok()?;
            let g = u8::from_str_radix(&h[2..4], 16).ok()?;
            let b = u8::from_str_radix(&h[4..6], 16).ok()?;
            let a = u8::from_str_radix(&h[6..8], 16).ok()?;
            Some(Rgba([r, g, b, a]))
        }
        _ => None,
    }
}

/// Render the source into a `size`×`size` RGBA canvas.
pub fn render_frame(
    src: &RgbaImage,
    size: u32,
    mode: Mode,
    background: Option<Rgba<u8>>,
    pad: f32,
) -> RgbaImage {
    let bg = background.unwrap_or(Rgba([0, 0, 0, 0]));
    let mut canvas = RgbaImage::from_pixel(size, size, bg);

    let (sw, sh) = (src.width() as f32, src.height() as f32);

    match mode {
        Mode::Contain => {
            let inner = (size as f32) * (1.0 - 2.0 * pad);
            let ratio = (inner / sw).min(inner / sh);
            let nw = (sw * ratio).round().max(1.0) as u32;
            let nh = (sh * ratio).round().max(1.0) as u32;
            let resized = imageops::resize(src, nw, nh, FilterType::Lanczos3);
            let x = ((size - nw) / 2) as i64;
            let y = ((size - nh) / 2) as i64;
            imageops::overlay(&mut canvas, &resized, x, y);
        }
        Mode::Cover => {
            let ratio = (size as f32 / sw).max(size as f32 / sh);
            let nw = (sw * ratio).round().max(size as f32) as u32;
            let nh = (sh * ratio).round().max(size as f32) as u32;
            let resized = imageops::resize(src, nw, nh, FilterType::Lanczos3);
            let cx = (nw - size) / 2;
            let cy = (nh - size) / 2;
            let cropped = imageops::crop_imm(&resized, cx, cy, size, size).to_image();
            imageops::overlay(&mut canvas, &cropped, 0, 0);
        }
    }
    canvas
}

/// Render the source onto a `w`×`h` RGBA canvas: scale to fit (keep aspect
/// ratio, transparent padding), then center. For square canvases with a square
/// source this is identical to a direct resize (matches `gen-assets.py`).
pub fn render_canvas(src: &RgbaImage, w: u32, h: u32) -> RgbaImage {
    let mut canvas = RgbaImage::from_pixel(w, h, Rgba([0, 0, 0, 0]));
    let (sw, sh) = (src.width() as f32, src.height() as f32);

    let scale = (w as f32 / sw).min(h as f32 / sh);
    let nw = (sw * scale).max(1.0).round() as u32;
    let nh = (sh * scale).max(1.0).round() as u32;
    let resized = imageops::resize(src, nw, nh, FilterType::Lanczos3);
    let x = ((w - nw) / 2) as i64;
    let y = ((h - nh) / 2) as i64;
    imageops::overlay(&mut canvas, &resized, x, y);

    canvas
}

/// Render every requested size; returns `(size, RGBA frame)` pairs for preview.
pub fn render_frames(
    src: &RgbaImage,
    sizes: &[u32],
    mode: Mode,
    background: Option<Rgba<u8>>,
    pad: f32,
) -> Vec<(u32, RgbaImage)> {
    sizes
        .iter()
        .map(|&size| (size, render_frame(src, size, mode, background, pad)))
        .collect()
}

/// Encode one frame as raw PNG bytes (used for the 256px entry).
pub fn encode_frame_png(img: &RgbaImage) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    {
        let enc = image::codecs::png::PngEncoder::new(&mut buf);
        image::ImageEncoder::write_image(
            enc,
            img.as_raw(),
            img.width(),
            img.height(),
            image::ExtendedColorType::Rgba8,
        )
        .expect("PNG encode failed");
    }
    buf
}

/// Encode one frame as a 32-bit BGRA BMP DIB (XOR bitmap + zero AND mask),
/// which is what an ICO entry expects for non-PNG frames.
pub fn encode_frame_bmp(img: &RgbaImage) -> Vec<u8> {
    let w = img.width();
    let h = img.height();
    let mut out: Vec<u8> = Vec::new();

    // BITMAPINFOHEADER (40 bytes). Height is doubled: XOR bitmap + AND mask.
    out.extend_from_slice(&40u32.to_le_bytes()); // biSize
    out.extend_from_slice(&(w as i32).to_le_bytes()); // biWidth
    out.extend_from_slice(&((h * 2) as i32).to_le_bytes()); // biHeight (xor+and)
    out.extend_from_slice(&1u16.to_le_bytes()); // biPlanes
    out.extend_from_slice(&32u16.to_le_bytes()); // biBitCount
    out.extend_from_slice(&0u32.to_le_bytes()); // biCompression = BI_RGB
    out.extend_from_slice(&0u32.to_le_bytes()); // biSizeImage (0 ok for BI_RGB)
    out.extend_from_slice(&0i32.to_le_bytes()); // biXPelsPerMeter
    out.extend_from_slice(&0i32.to_le_bytes()); // biYPelsPerMeter
    out.extend_from_slice(&0u32.to_le_bytes()); // biClrUsed
    out.extend_from_slice(&0u32.to_le_bytes()); // biClrImportant

    // XOR bitmap: 32-bit BGRA, rows bottom-up.
    for y in (0..h).rev() {
        for x in 0..w {
            let p = img.get_pixel(x, y).0; // RGBA
            out.push(p[2]); // B
            out.push(p[1]); // G
            out.push(p[0]); // R
            out.push(p[3]); // A
        }
    }

    // AND mask: 1 bit per pixel, rows padded to 32-bit boundary. All zero
    // because alpha in the XOR bitmap already carries transparency.
    let row_bytes = (((w + 31) / 32) * 4) as usize;
    out.extend(std::iter::repeat(0u8).take(row_bytes * h as usize));

    out
}

/// Encode PNG-or-BMP frames into a complete `.ico` file.
pub fn encode_ico(frames: &[(u32, Vec<u8>)], path: &str) -> std::io::Result<()> {
    let mut file = std::fs::File::create(path)?;
    let count = frames.len() as u16;

    // ICONDIR
    file.write_all(&0u16.to_le_bytes())?; // reserved
    file.write_all(&1u16.to_le_bytes())?; // type = icon
    file.write_all(&count.to_le_bytes())?;

    // Offset to first image data = 6 (header) + 16 * count (entries).
    let mut offset: u32 = 6 + 16 * count as u32;

    // ICONDIRENTRY for each frame.
    for (size, data) in frames {
        let dim = if *size >= 256 { 0u8 } else { *size as u8 };
        file.write_all(&[dim])?; // width
        file.write_all(&[dim])?; // height
        file.write_all(&[0u8])?; // color count
        file.write_all(&[0u8])?; // reserved
        file.write_all(&1u16.to_le_bytes())?; // planes
        file.write_all(&32u16.to_le_bytes())?; // bit count
        file.write_all(&(data.len() as u32).to_le_bytes())?; // bytes in resource
        file.write_all(&offset.to_le_bytes())?; // image offset
        offset += data.len() as u32;
    }

    // Image data.
    for (_size, data) in frames {
        file.write_all(data)?;
    }
    Ok(())
}

/// Lightweight self-check: re-read the ICONDIR we just wrote and return the
/// width of each embedded frame (256 is stored as 0 in the entry byte).
pub fn verify_ico(path: &str) -> Result<Vec<u32>, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("read-back failed: {e}"))?;
    if bytes.len() < 6 {
        return Err("file too small, invalid ICO".into());
    }
    if u16::from_le_bytes([bytes[2], bytes[3]]) != 1 {
        return Err("type field is not icon(1)".into());
    }
    let count = u16::from_le_bytes([bytes[4], bytes[5]]) as usize;
    let mut dims = Vec::with_capacity(count);
    for i in 0..count {
        let off = 6 + i * 16;
        if off + 16 > bytes.len() {
            return Err("directory entry out of bounds".into());
        }
        let w = bytes[off];
        dims.push(if w == 0 { 256 } else { w as u32 });
    }
    Ok(dims)
}

/// Build a `image::Frame` (used by front-ends to render a preview).
pub fn to_frame(img: &RgbaImage) -> Frame {
    Frame::new(img.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    const RED: Rgba<u8> = Rgba([255, 0, 0, 255]);
    const BLUE: Rgba<u8> = Rgba([0, 0, 255, 255]);

    /// Solid 100×50 opaque red image.
    fn wide_src() -> RgbaImage {
        RgbaImage::from_pixel(100, 50, RED)
    }

    #[test]
    fn parse_color_hex_and_named() {
        assert_eq!(parse_color("#1e293b"), Some(Rgba([30, 41, 59, 255])));
        assert_eq!(parse_color("#11223344"), Some(Rgba([17, 34, 51, 68])));
        assert_eq!(parse_color("white"), Some(Rgba([255, 255, 255, 255])));
        assert_eq!(parse_color("transparent"), Some(Rgba([0, 0, 0, 0])));
        assert_eq!(parse_color("#zzz"), None);
        assert_eq!(parse_color("notacolor"), None);
    }

    #[test]
    fn contain_pads_wide_source() {
        // 100×50 into 64×64 → scaled to 64×32, vertically centered.
        let out = render_frame(&wide_src(), 64, Mode::Contain, None, 0.0);
        assert_eq!((out.width(), out.height()), (64, 64));
        assert_eq!(*out.get_pixel(0, 0), Rgba([0, 0, 0, 0])); // padding
        assert_eq!(*out.get_pixel(32, 32), RED); // content
    }

    #[test]
    fn contain_fills_background() {
        let out = render_frame(&wide_src(), 64, Mode::Contain, Some(BLUE), 0.0);
        assert_eq!(*out.get_pixel(0, 0), BLUE);
        assert_eq!(*out.get_pixel(32, 32), RED);
    }

    #[test]
    fn cover_crops_to_fill() {
        // 100×50 into 64×64 → scaled to 128×64, sides cropped; no padding.
        let out = render_frame(&wide_src(), 64, Mode::Cover, None, 0.0);
        assert_eq!((out.width(), out.height()), (64, 64));
        assert_eq!(*out.get_pixel(0, 0), RED);
        assert_eq!(*out.get_pixel(63, 63), RED);
    }

    #[test]
    fn canvas_centers_on_transparent_background() {
        // 100×100 onto 300×100 → centered with 100px padding each side.
        let src = RgbaImage::from_pixel(100, 100, RED);
        let out = render_canvas(&src, 300, 100);
        assert_eq!((out.width(), out.height()), (300, 100));
        assert_eq!(*out.get_pixel(0, 50), Rgba([0, 0, 0, 0]));
        assert_eq!(*out.get_pixel(150, 50), RED);
    }

    #[test]
    fn bmp_frame_layout() {
        let img = RgbaImage::from_pixel(16, 16, RED);
        let bmp = encode_frame_bmp(&img);
        // 40-byte header + 16×16×4 XOR + 16 rows × 4-byte AND mask.
        assert_eq!(bmp.len(), 40 + 16 * 16 * 4 + 16 * 4);
        assert_eq!(&bmp[0..4], &40u32.to_le_bytes()); // biSize
        assert_eq!(&bmp[4..8], &16i32.to_le_bytes()); // biWidth
        assert_eq!(&bmp[8..12], &32i32.to_le_bytes()); // biHeight (doubled)
        assert_eq!(&bmp[14..16], &32u16.to_le_bytes()); // biBitCount
        // First XOR pixel (bottom-left) is BGRA red.
        assert_eq!(&bmp[40..44], &[0, 0, 255, 255]);
    }

    #[test]
    fn ico_roundtrip() {
        let small = RgbaImage::from_pixel(16, 16, RED);
        let large = RgbaImage::from_pixel(256, 256, RED);
        let frames = vec![
            (16u32, encode_frame_bmp(&small)),
            (256u32, encode_frame_png(&large)),
        ];

        let path = std::env::temp_dir().join(format!("icogen-test-{}.ico", std::process::id()));
        let path_str = path.to_str().unwrap();
        encode_ico(&frames, path_str).expect("encode_ico failed");

        let bytes = std::fs::read(&path).unwrap();
        assert_eq!(&bytes[0..4], &[0, 0, 1, 0]); // reserved + type = icon
        assert_eq!(u16::from_le_bytes([bytes[4], bytes[5]]), 2);
        assert_eq!(verify_ico(path_str), Ok(vec![16, 256]));

        std::fs::remove_file(&path).ok();
    }
}
