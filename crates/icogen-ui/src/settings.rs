//! User preferences persistence: theme and language choices survive restarts.
//!
//! Stored as a small plain-text file (`theme=<id>\nlanguage=<id>`) under
//! `%APPDATA%/icogen/settings`, consistent with the `window_state` approach.

use std::path::PathBuf;

/// A loaded settings snapshot.
#[derive(Clone, Debug)]
pub struct Settings {
    pub theme_id: String,
    pub language_id: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme_id: "light".to_string(),
            language_id: crate::i18n::detect_system_language().to_string(),
        }
    }
}

fn settings_path() -> Option<PathBuf> {
    let appdata = std::env::var_os("APPDATA")?;
    Some(PathBuf::from(appdata).join("icogen").join("settings"))
}

/// Load persisted settings. Falls back to defaults (light theme, system
/// locale language) when the file is missing or malformed.
pub fn load() -> Settings {
    let Some(path) = settings_path() else {
        return Settings::default();
    };
    let Ok(text) = std::fs::read_to_string(path) else {
        return Settings::default();
    };

    let mut theme_id = None;
    let mut language_id = None;
    for line in text.lines() {
        let line = line.trim();
        if let Some(v) = line.strip_prefix("theme=") {
            theme_id = Some(v.trim().to_string());
        } else if let Some(v) = line.strip_prefix("language=") {
            language_id = Some(v.trim().to_string());
        }
    }

    let defaults = Settings::default();
    Settings {
        theme_id: theme_id.unwrap_or(defaults.theme_id),
        language_id: language_id.unwrap_or(defaults.language_id),
    }
}

/// Persist the current settings. Best-effort: I/O errors are silently ignored.
pub fn save(settings: &Settings) {
    let Some(path) = settings_path() else { return };
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let content = format!("theme={}\nlanguage={}\n", settings.theme_id, settings.language_id);
    let _ = std::fs::write(path, content);
}
