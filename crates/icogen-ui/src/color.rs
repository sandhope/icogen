//! Color helpers: build a GPUI `Hsla` from a `0xRRGGBB` value, plus packing helpers.

use gpui::Hsla;
use image::Rgba;

/// Build a GPUI color (`Hsla`) from a `0xRRGGBB` value, satisfying
/// `bg()` / `text_color()` which accept `impl Into<Hsla>`.
pub fn color(hex: u32) -> Hsla {
    let r = ((hex >> 16) & 0xff) as f32 / 255.0;
    let g = ((hex >> 8) & 0xff) as f32 / 255.0;
    let b = (hex & 0xff) as f32 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;
    let d = max - min;
    let s = if d == 0.0 {
        0.0
    } else {
        d / (1.0 - (2.0 * l - 1.0).abs())
    };
    let h = if d == 0.0 {
        0.0
    } else {
        let mut hh = if (max - r).abs() < f32::EPSILON {
            ((g - b) / d) % 6.0
        } else if (max - g).abs() < f32::EPSILON {
            (b - r) / d + 2.0
        } else {
            (r - g) / d + 4.0
        };
        hh *= 60.0;
        if hh < 0.0 {
            hh += 360.0;
        }
        hh
    };
    Hsla { h, s, l, a: 1.0 }
}

/// Pack an `Rgba<u8>` into `0xRRGGBB` for `color()`.
pub fn hex_rgb(c: Rgba<u8>) -> u32 {
    ((c[0] as u32) << 16) | ((c[1] as u32) << 8) | (c[2] as u32)
}
