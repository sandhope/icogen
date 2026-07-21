//! Shared GPUI UI building blocks for the `icogen` family of graphical tools.
//!
//! Styling lives here and *only* here: both GUIs import `card`, `section_label`,
//! `style_button` and `style_pill` from this crate, so editing a style in one
//! place keeps the two front-ends visually in sync.

pub mod assets;
pub mod color;
pub mod components;
pub mod i18n;
pub mod logo;
pub mod settings;
pub mod theme;
pub mod toolbar;
pub mod window_state;
