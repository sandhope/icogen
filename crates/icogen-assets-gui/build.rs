//! Embeds the application icon (resource ID 1) so the window shows an icon
//! in the taskbar and title bar. GPUI loads it via `LoadImageW(module, 1)`.

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    if !env::var("TARGET").unwrap().contains("windows") {
        return;
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let res_path = out_dir.join("app.res");

    let rc = find_rc().expect("rc.exe not found — install the Windows 10/11 SDK");
    let status = Command::new(&rc)
        .args(["/nologo", "/fo"])
        .arg(&res_path)
        .arg("app.rc")
        .status()
        .expect("failed to execute rc.exe");
    assert!(status.success(), "rc.exe failed to compile app.rc");

    println!("cargo:rerun-if-changed=app.rc");
    println!("cargo:rerun-if-changed=../../assets/app.ico");
    println!("cargo:rustc-link-arg={}", res_path.display());
}

/// Locate rc.exe: prefer the environment (VS developer prompt), then scan
/// installed SDK versions and pick the newest.
fn find_rc() -> Option<PathBuf> {
    if let Ok(sdk_dir) = env::var("WindowsSdkDir") {
        for candidate in [
            PathBuf::from(&sdk_dir).join("bin").join("x64").join("rc.exe"),
            PathBuf::from(&sdk_dir)
                .join("bin")
                .join(env::var("WindowsSDKVersion").unwrap_or_default().trim_end_matches('\\'))
                .join("x64")
                .join("rc.exe"),
        ] {
            if candidate.exists() {
                return Some(candidate);
            }
        }
    }

    let base = PathBuf::from(r"C:\Program Files (x86)\Windows Kits\10\bin");
    let mut versions: Vec<PathBuf> = std::fs::read_dir(&base)
        .ok()?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.join("x64").join("rc.exe").exists())
        .collect();
    versions.sort();
    versions.pop().map(|path| path.join("x64").join("rc.exe"))
}
