#!/usr/bin/env python3
"""Generate a multi-resolution Windows AppIcon.ico.

The .ico produced here is meant for:
  * system tray (notification area)  -> 16 / 24 / 32 px
  * taskbar / Alt-Tab                -> 32 / 48 px
  * Task Manager / Explorer          -> 48 / 64 / 128 / 256 px

It embeds the standard sizes and stores the 256px frame as PNG (Windows
Vista+), which keeps the file small while staying sharp on high-DPI displays.

Usage examples
--------------
    # From any source image (PNG/SVG raster/photo), keep transparency:
    python icogen_gen.py logo.png --output AppIcon.ico

    # No logo? Generate a colored tile with a letter:
    python icogen_gen.py --text "A" --bg "#2b6cff" --output AppIcon.ico

    # Opaque square background instead of transparent (cover/crop):
    python icogen_gen.py logo.png --mode cover --background "#111827"

    # Write a preview montage so you can eyeball every size:
    python icogen_gen.py --text "A" --preview preview.png
"""

from __future__ import annotations

import argparse
import io
import struct
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

# Standard Windows icon sizes (small -> large).
DEFAULT_SIZES = [16, 24, 32, 48, 64, 128, 256]

# Sizes >= this are stored as PNG inside the .ico container (Vista+).
PNG_THRESHOLD = 256


# --------------------------------------------------------------------------- #
# Helpers
# --------------------------------------------------------------------------- #
def parse_color(value: str | None):
    """Turn a color name / hex into an (R, G, B, 255) tuple, or None."""
    if value is None:
        return None
    from PIL import ImageColor

    r, g, b = ImageColor.getrgb(value)
    return (r, g, b, 255)


def load_font(px: int) -> ImageFont.ImageFont:
    """Find a usable TrueType font, falling back to PIL's bitmap font."""
    candidates = [
        r"C:\Windows\Fonts\arial.ttf",
        r"C:\Windows\Fonts\seguiemj.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/Library/Fonts/Arial.ttf",
    ]
    for path in candidates:
        if Path(path).exists():
            try:
                return ImageFont.truetype(path, px)
            except Exception:
                pass
    return ImageFont.load_default()


def make_square(img: Image.Image, size: int, mode: str,
                background, pad: float) -> Image.Image:
    """Return a square RGBA image of `size` with the source fitted/cropped."""
    src = img.convert("RGBA")
    w, h = src.size
    if mode == "cover":
        side = min(w, h)
        left, top = (w - side) // 2, (h - side) // 2
        src = src.crop((left, top, left + side, top + side))
        return src.resize((size, size), Image.LANCZOS)

    # contain: optional transparent padding, optional solid background
    bg = background if background is not None else (0, 0, 0, 0)
    canvas = Image.new("RGBA", (size, size), bg)
    inner = max(1, int(size * (1 - pad)))
    ratio = min(inner / w, inner / h)
    nw, nh = max(1, int(round(w * ratio))), max(1, int(round(h * ratio)))
    resized = src.resize((nw, nh), Image.LANCZOS)
    left, top = (size - nw) // 2, (size - nh) // 2
    canvas.alpha_composite(resized, (left, top))
    return canvas


# --------------------------------------------------------------------------- #
# ICO encoding (manual, spec-compliant)
# --------------------------------------------------------------------------- #
def encode_bmp(img: Image.Image) -> bytes:
    """Encode a square RGBA image as a 32-bit BGRA BMP (XOR + AND mask)."""
    img = img.convert("RGBA")
    w, h = img.size
    rgba = img.tobytes()  # top-to-bottom, RGBA
    row_bytes = w * 4

    out = bytearray()
    for y in range(h - 1, -1, -1):  # BMP rows are stored bottom-up
        start = y * row_bytes
        row = rgba[start:start + row_bytes]
        for i in range(0, row_bytes, 4):
            r, g, b, a = row[i], row[i + 1], row[i + 2], row[i + 3]
            out += bytes((b, g, r, a))  # -> BGRA

    # 1-bpp AND mask, one row padded to a 32-bit boundary, all zeros (alpha wins)
    and_row = ((w + 31) // 32) * 4
    and_mask = b"\x00" * (and_row * h)

    header = struct.pack(
        "<IiiHHIIiiII",
        40,                       # biSize
        w,                        # biWidth
        h * 2,                    # biHeight (XOR + AND)
        1,                        # biPlanes
        32,                       # biBitCount
        0,                        # biCompression (BI_RGB)
        len(out) + len(and_mask), # biSizeImage
        0, 0, 0, 0,               # x/y ppm, clrUsed, clrImportant
    )
    return header + out + and_mask


def encode_png(img: Image.Image) -> bytes:
    buf = io.BytesIO()
    img.convert("RGBA").save(buf, format="PNG")
    return buf.getvalue()


def save_ico(images: dict[int, Image.Image], path: str | Path) -> None:
    """Write an ICONDIR + entries + frame data from a {size: Image} dict."""
    sizes = sorted(images)
    entries, blobs = [], []
    offset = 6 + 16 * len(sizes)

    for size in sizes:
        blob = encode_png(images[size]) if size >= PNG_THRESHOLD else encode_bmp(images[size])
        entries.append((size, blob))
        blobs.append(blob)

    out = bytearray(struct.pack("<HHH", 0, 1, len(entries)))  # ICONDIR
    for size, blob in entries:
        w = 0 if size >= 256 else size  # 256 encoded as 0 per spec
        out += struct.pack("<BBBBHHII", w, w, 0, 0, 1, 32, len(blob), offset)
        offset += len(blob)
    for blob in blobs:
        out += blob

    Path(path).write_bytes(out)


# --------------------------------------------------------------------------- #
# High-level builders
# --------------------------------------------------------------------------- #
def build_from_source(source, sizes, mode, background, pad) -> dict[int, Image.Image]:
    base = Image.open(source)
    return {s: make_square(base, s, mode, background, pad) for s in sizes}


def build_from_text(text, sizes, bg, fg) -> dict[int, Image.Image]:
    hi = 1024
    canvas = Image.new("RGBA", (hi, hi), (0, 0, 0, 0))
    draw = ImageDraw.Draw(canvas)
    draw.rounded_rectangle([0, 0, hi, hi], radius=hi // 6, fill=parse_color(bg) or (43, 108, 255, 255))
    font = load_font(int(hi * 0.62))
    bbox = draw.textbbox((0, 0), text, font=font)
    tw, th = bbox[2] - bbox[0], bbox[3] - bbox[1]
    draw.text(((hi - tw) / 2 - bbox[0], (hi - th) / 2 - bbox[1]), text,
              font=font, fill=parse_color(fg) or (255, 255, 255, 255))
    # crop-to-fill each target size so the tile stays crisp
    return {s: make_square(canvas, s, "cover", None, 0.0) for s in sizes}


def checker(size: int) -> Image.Image:
    tile = max(4, size // 8)
    img = Image.new("RGBA", (size, size), (255, 255, 255, 255))
    px = img.load()
    for y in range(size):
        for x in range(size):
            if ((x // tile) + (y // tile)) % 2 == 0:
                px[x, y] = (214, 214, 214, 255)
    return img


def save_preview(images: dict[int, Image.Image], path: str | Path, display: int = 64) -> None:
    sizes = sorted(images)
    pad = 10
    w = len(sizes) * display + (len(sizes) + 1) * pad
    h = display + 2 * pad + 16
    canvas = Image.new("RGBA", (w, h), (245, 245, 245, 255))
    draw = ImageDraw.Draw(canvas)
    x = pad
    for s in sizes:
        cell = checker(display)
        cell.alpha_composite(images[s].resize((display, display), Image.LANCZOS))
        canvas.alpha_composite(cell, (x, pad))
        draw.text((x, pad + display + 2), f"{s}", fill=(60, 60, 60, 255))
        x += display + pad
    canvas.convert("RGB").save(path, format="PNG")


# --------------------------------------------------------------------------- #
# CLI
# --------------------------------------------------------------------------- #
def main() -> None:
    p = argparse.ArgumentParser(
        description="Generate a multi-resolution Windows AppIcon.ico "
                    "(system tray / taskbar / Task Manager).")
    p.add_argument("source", nargs="?", help="Source image (PNG/JPG/...). "
                   "Skip and use --text to generate a placeholder icon.")
    p.add_argument("--text", help="Generate a tile icon with this text "
                   "(e.g. 'A' or 'App') instead of using a source image.")
    p.add_argument("--bg", default="#2b6cff", help="Tile background color "
                   "(--text mode). Hex or name. Default #2b6cff.")
    p.add_argument("--fg", default="white", help="Tile text color "
                   "(--text mode). Default white.")
    p.add_argument("--output", "-o", default="AppIcon.ico", help="Output .ico path.")
    p.add_argument("--sizes", default=",".join(map(str, DEFAULT_SIZES)),
                   help="Comma list of sizes. Default: 16,24,32,48,64,128,256")
    p.add_argument("--mode", choices=["contain", "cover"], default="contain",
                   help="Fit (contain, keeps transparency) or crop (cover).")
    p.add_argument("--background", default=None,
                   help="Solid background color for contain mode "
                        "(default: transparent).")
    p.add_argument("--pad", type=float, default=0.0,
                   help="Transparent padding fraction 0..0.5 for contain mode.")
    p.add_argument("--preview", default=None, help="Also write a PNG montage "
                   "of every size to this path.")
    p.add_argument("--verify", action="store_true", help="Read the .ico back "
                   "and print the embedded frame sizes.")
    args = p.parse_args()

    if not args.source and not args.text:
        p.error("provide a SOURCE image or use --text to generate a placeholder")

    sizes = [int(x) for x in args.sizes.split(",") if x.strip()]
    background = parse_color(args.background)

    if args.text:
        images = build_from_text(args.text, sizes, args.bg, args.fg)
    else:
        images = build_from_source(args.source, sizes, args.mode, background, args.pad)

    save_ico(images, args.output)
    print(f"Wrote {args.output} with sizes: {sorted(images)}")

    if args.preview:
        save_preview(images, args.preview)
        print(f"Wrote preview montage: {args.preview}")

    if args.verify:
        with Image.open(args.output) as im:
            print("Verified embedded sizes:", sorted(im.info.get("sizes", [])))


if __name__ == "__main__":
    main()
