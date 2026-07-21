//! `icogen-assets` — command-line frontend for generating the 8 WinUI 3 /
//! Windows App SDK asset PNGs.
//!
//! Generates the 8 WinUI 3 / Windows App SDK asset PNGs from a single square
//! source image (e.g. 1024x1024). Square targets are scaled directly; non-square
//! targets (Wide / SplashScreen) are scaled to fit and centered on a transparent
//! canvas. `AppIcon.ico` is not touched.
//!
//! Output lands in the directory given with `-o` (default `Assets/`, relative
//! to the current working directory — run this from the project root).

use std::path::PathBuf;
use std::process::exit;

use icogen_core::{load_image, render_canvas};

/// (file name, width, height)
const TARGETS: &[(&str, u32, u32)] = &[
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

struct Options {
    input: Option<String>,
    out_dir: String,
}

fn print_help() {
    println!(
        r##"icogen-assets — generate WinUI 3 / Windows App SDK asset PNGs (standalone, zero runtime deps)

Usage:
    icogen-assets <input image> [options]

Options:
    -o, --output <dir>   Output directory (default: Assets)
    -h, --help           Show help

Examples:
    icogen-assets logo.png
    icogen-assets logo.png -o src/Assets
"##
    );
}

fn die(msg: &str) -> ! {
    eprintln!("error: {msg}");
    exit(1);
}

fn parse_args() -> Options {
    let mut args = std::env::args().skip(1);
    let mut opts = Options {
        input: None,
        out_dir: "Assets".to_string(),
    };

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                exit(0);
            }
            "-o" | "--output" => {
                opts.out_dir = args.next().unwrap_or_else(|| die("--output requires a value"));
            }
            other if other.starts_with('-') => die(&format!("unknown option: {other}")),
            other => {
                if opts.input.is_some() {
                    die("only one input image allowed");
                }
                opts.input = Some(other.to_string());
            }
        }
    }
    opts
}

fn main() {
    let opts = parse_args();

    let input = opts.input.clone().unwrap_or_else(|| {
        print_help();
        die("missing input image");
    });

    let src = load_image(&input).unwrap_or_else(|e| die(&e));
    println!("source: {input} ({}x{})", src.width(), src.height());

    for (name, w, h) in TARGETS {
        let out = PathBuf::from(&opts.out_dir).join(name);
        if let Some(parent) = out.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                eprintln!("  failed to create {parent:?}: {e}");
                continue;
            }
        }

        let img = render_canvas(&src, *w, *h);
        match img.save(&out) {
            Ok(()) => println!("  -> {} ({w}x{h})", out.display()),
            Err(e) => eprintln!("  failed {}: {e}", out.display()),
        }
    }

    println!("done (AppIcon.ico not modified).");
}
