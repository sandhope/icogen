//! Internationalization: UI strings for both GUIs, with English and
//! Simplified Chinese language packs.
//!
//! `I18nManager` is a GPUI global singleton. On first launch the system
//! locale is detected via `sys-locale`; afterwards the user's choice is
//! persisted by `crate::settings`.

use std::sync::Arc;

use gpui::{App, Global};

/// All user-facing strings shared by `icogen-gui` and `icogen-assets-gui`.
#[derive(Clone, Debug)]
pub struct I18nStrings {
    // --- Common ---
    pub drop_hint: &'static str,
    pub no_file_selected: &'static str,
    pub image_filter: &'static str,
    pub supported_formats: &'static str,

    // --- icogen-gui ---
    pub output: &'static str,
    pub sizes: &'static str,
    pub fit_mode: &'static str,
    pub background: &'static str,
    pub color: &'static str,
    pub contain: &'static str,
    pub cover: &'static str,
    pub transparent: &'static str,
    pub opaque: &'static str,
    pub generate_ico: &'static str,
    pub load_image_first: &'static str,
    pub select_at_least_one_size: &'static str,
    pub error_loading_image: &'static str,
    /// Prefix for "Saved <output>  (<sizes>)".
    pub saved_prefix: &'static str,
    /// Prefix for "Error: <err>".
    pub error_prefix: &'static str,

    // --- icogen-assets-gui ---
    pub output_directory: &'static str,
    pub targets: &'static str,
    pub generate_assets: &'static str,
    pub select_at_least_one_target: &'static str,
    /// Prefix for "Error creating <dir>: <err>".
    pub error_creating_prefix: &'static str,
    /// Middle for "Error creating <dir>[middle]<err>".
    pub error_creating_middle: &'static str,
    /// Prefix for "Error writing <name>: <err>".
    pub error_writing_prefix: &'static str,
    /// Middle for "Error writing <name>[middle]<err>".
    pub error_writing_middle: &'static str,
    /// Prefix for "Saved <n> asset PNGs to <dir>/ ...".
    pub saved_assets_prefix: &'static str,
    /// Middle for "Saved <n>[middle]<dir>/ ...".
    pub saved_assets_middle: &'static str,
    /// Suffix for "Saved <n> asset PNGs to <dir>/[suffix]".
    pub saved_assets_suffix: &'static str,

    // --- Menu ---
    pub menu_theme: &'static str,
    pub menu_theme_light: &'static str,
    pub menu_theme_dark: &'static str,
    pub menu_language: &'static str,
    pub menu_lang_zh: &'static str,
    pub menu_lang_en: &'static str,
}

impl I18nStrings {
    pub fn en() -> Self {
        Self {
            drop_hint: "Drag an image here\nor click to browse",
            no_file_selected: "No file selected",
            image_filter: "Image",
            supported_formats: "Supports PNG, JPG, SVG, WebP formats",

            output: "Output",
            sizes: "Sizes",
            fit_mode: "Fit mode",
            background: "Background",
            color: "Color",
            contain: "Contain",
            cover: "Cover",
            transparent: "Transparent",
            opaque: "Opaque",
            generate_ico: "Generate AppIcon.ico",
            load_image_first: "Load an image first (drag it on, or click Browse).",
            select_at_least_one_size: "Select at least one size.",
            error_loading_image: "Error loading image: ",
            saved_prefix: "Saved ",
            error_prefix: "Error: ",

            output_directory: "Output directory",
            targets: "Targets",
            generate_assets: "Generate asset PNGs",
            select_at_least_one_target: "Select at least one target.",
            error_creating_prefix: "Error creating ",
            error_creating_middle: ": ",
            error_writing_prefix: "Error writing ",
            error_writing_middle: ": ",
            saved_assets_prefix: "Saved ",
            saved_assets_middle: " asset PNGs to ",
            saved_assets_suffix: "/  (AppIcon.ico unchanged)",

            menu_theme: "Theme",
            menu_theme_light: "Light",
            menu_theme_dark: "Dark",
            menu_language: "Language",
            menu_lang_zh: "中文",
            menu_lang_en: "English",
        }
    }

    pub fn zh() -> Self {
        Self {
            drop_hint: "将图片拖放到此处\n或点击选择文件",
            no_file_selected: "未选择文件",
            image_filter: "图片",
            supported_formats: "支持 PNG、JPG、SVG、WebP 格式",

            output: "输出",
            sizes: "尺寸",
            fit_mode: "填充模式",
            background: "背景",
            color: "颜色",
            contain: "包含",
            cover: "覆盖",
            transparent: "透明",
            opaque: "不透明",
            generate_ico: "生成 AppIcon.ico",
            load_image_first: "请先加载一张图片（拖入或点击选择）。",
            select_at_least_one_size: "请至少选择一个尺寸。",
            error_loading_image: "加载图片出错：",
            saved_prefix: "已保存 ",
            error_prefix: "错误：",

            output_directory: "输出目录",
            targets: "目标",
            generate_assets: "生成资源 PNG",
            select_at_least_one_target: "请至少选择一个目标。",
            error_creating_prefix: "创建 ",
            error_creating_middle: " 出错：",
            error_writing_prefix: "写入 ",
            error_writing_middle: " 出错：",
            saved_assets_prefix: "已保存 ",
            saved_assets_middle: " 个资源 PNG 到 ",
            saved_assets_suffix: "/（AppIcon.ico 未更改）",

            menu_theme: "主题",
            menu_theme_light: "浅色",
            menu_theme_dark: "深色",
            menu_language: "语言",
            menu_lang_zh: "中文",
            menu_lang_en: "English",
        }
    }

    pub fn for_language_id(id: &str) -> Self {
        match id {
            "zh-CN" => Self::zh(),
            _ => Self::en(),
        }
    }
}

/// Runtime i18n state, stored as a GPUI global singleton.
pub struct I18nManager {
    pub language_id: String,
    strings: Arc<I18nStrings>,
}

impl Global for I18nManager {}

impl I18nManager {
    /// Initialize the global with the given language id ("en-US" or "zh-CN").
    pub fn init(cx: &mut App, language_id: &str) {
        let id = normalize_id(language_id);
        let strings = Arc::new(I18nStrings::for_language_id(&id));
        cx.set_global(Self {
            language_id: id,
            strings,
        });
    }

    /// Switch language. Returns `true` if the language actually changed.
    pub fn set_language(&mut self, language_id: &str) -> bool {
        let id = normalize_id(language_id);
        if self.language_id == id {
            return false;
        }
        self.strings = Arc::new(I18nStrings::for_language_id(&id));
        self.language_id = id;
        true
    }

    pub fn strings(&self) -> &Arc<I18nStrings> {
        &self.strings
    }
}

/// Detect the best language id from the system locale.
pub fn detect_system_language() -> &'static str {
    if let Some(locale) = sys_locale::get_locale() {
        let lower = locale.to_lowercase();
        if lower.starts_with("zh") {
            return "zh-CN";
        }
    }
    "en-US"
}

fn normalize_id(id: &str) -> String {
    if id.starts_with("zh") {
        "zh-CN".to_string()
    } else {
        "en-US".to_string()
    }
}
