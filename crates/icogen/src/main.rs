//! `icogen` — command-line frontend for `icogen-core`.
//!
//! Usage:
//!     icogen <input image> [options]
//!
//! Produces a standalone, dependency-free `.exe` for end users.

use std::process::exit;

use icogen_core::{self as core, Mode};

struct Options {
    input: Option<String>,
    output: String,
    sizes: Vec<u32>,
    mode: Mode,
    background: Option<image::Rgba<u8>>,
    pad: f32,
    verify: bool,
}

fn print_help() {
    println!(
        r##"icogen — generate a multi-resolution Windows AppIcon.ico (standalone, zero runtime deps)

Usage:
    icogen <input image> [options]

Options:
    -o, --output <path>       Output .ico path (default: AppIcon.ico)
    -s, --sizes <list>        Embedded sizes, comma-separated (default: 16,24,32,48,64,128,256)
        --mode <mode>         contain (fit, transparent padding) | cover (crop to fill) (default: contain)
    -b, --background <color>  Solid background, e.g. #1e293b or white (default: transparent)
        --pad <ratio>         Inner padding in contain mode, 0~1 (default: 0)
        --verify              Read back and verify frames after generation
    -h, --help                Show help

Examples:
    icogen logo.png
    icogen logo.png -o AppIcon.ico --mode cover
    icogen logo.png -b "#111827" --mode cover --verify
"##
    );
}

fn die(msg: &str) -> ! {
    eprintln!("error: {msg}");
    exit(1);
}

fn parse_args() -> Options {
    let mut args = std::env::args().skip(1).peekable();
    let mut opts = Options {
        input: None,
        output: "AppIcon.ico".to_string(),
        sizes: core::DEFAULT_SIZES.to_vec(),
        mode: Mode::Contain,
        background: None,
        pad: 0.0,
        verify: false,
    };

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                exit(0);
            }
            "-o" | "--output" => {
                opts.output = args.next().unwrap_or_else(|| die("--output requires a value"));
            }
            "-s" | "--sizes" => {
                let raw = args.next().unwrap_or_else(|| die("--sizes requires a value"));
                let mut v: Vec<u32> = raw
                    .split(',')
                    .filter(|x| !x.trim().is_empty())
                    .map(|x| x.trim().parse::<u32>().unwrap_or_else(|_| die("invalid number in --sizes")))
                    .collect();
                v.sort_unstable();
                v.dedup();
                if v.is_empty() || v.iter().any(|&s| s == 0 || s > 256) {
                    die("--sizes must be integers in 1..=256");
                }
                opts.sizes = v;
            }
            "--mode" => {
                opts.mode = match args.next().as_deref() {
                    Some("contain") => Mode::Contain,
                    Some("cover") => Mode::Cover,
                    _ => die("--mode must be contain or cover"),
                };
            }
            "-b" | "--background" => {
                let c = args.next().unwrap_or_else(|| die("--background requires a value"));
                opts.background = Some(core::parse_color(&c).unwrap_or_else(|| die("cannot parse color")));
            }
            "--pad" => {
                let p = args.next().unwrap_or_else(|| die("--pad requires a value"));
                opts.pad = p.parse::<f32>().unwrap_or_else(|_| die("--pad must be a number"));
                if !(0.0..1.0).contains(&opts.pad) {
                    die("--pad must be in [0,1)");
                }
            }
            "--verify" => opts.verify = true,
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

    let src = core::load_image(&input).unwrap_or_else(|e| die(&e));

    let frames = core::render_frames(&src, &opts.sizes, opts.mode, opts.background, opts.pad);

    let encoded: Vec<(u32, Vec<u8>)> = frames
        .iter()
        .map(|(size, img)| {
            let data = if *size >= 256 {
                core::encode_frame_png(img)
            } else {
                core::encode_frame_bmp(img)
            };
            (*size, data)
        })
        .collect();

    core::encode_ico(&encoded, &opts.output)
        .unwrap_or_else(|e| die(&format!("failed to write '{}': {e}", opts.output)));

    println!(
        "generated {} (embedded sizes: {})",
        opts.output,
        opts.sizes
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );

    if opts.verify {
        match core::verify_ico(&opts.output) {
            Ok(dims) => {
                let list = dims
                    .iter()
                    .map(|d| format!("{d}x{d}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                println!("verified: {} frames [{}]", dims.len(), list);
            }
            Err(e) => eprintln!("verify warning: {e}"),
        }
    }
}
