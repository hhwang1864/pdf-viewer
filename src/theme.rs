use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;

const THEME_KEY: &str = "pdf-viewer-theme";

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }

    pub fn toggle(&self) -> Theme {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }
}

pub fn load_theme() -> Theme {
    let stored: Result<String, _> = LocalStorage::get(THEME_KEY);
    match stored.as_deref() {
        Ok("dark") => Theme::Dark,
        _ => Theme::Light,
    }
}

pub fn save_theme(theme: Theme) {
    let _ = LocalStorage::set(THEME_KEY, theme.as_str());
}

pub fn apply_theme(theme: Theme) {
    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
        if let Some(el) = document.document_element() {
            let _ = el.set_attribute("data-theme", theme.as_str());
        }
    }
    save_theme(theme);
}
