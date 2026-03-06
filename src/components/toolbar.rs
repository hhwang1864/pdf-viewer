use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::api;
use crate::models::Note;
use crate::theme::{Theme, apply_theme};

#[component]
pub fn Toolbar(
    current_page: ReadSignal<u32>,
    total_pages: ReadSignal<u32>,
    zoom: ReadSignal<f64>,
    theme: ReadSignal<Theme>,
    notes: ReadSignal<Vec<Note>>,
    set_current_page: WriteSignal<u32>,
    set_zoom: WriteSignal<f64>,
    set_theme: WriteSignal<Theme>,
    sidebar_open: ReadSignal<bool>,
    set_sidebar_open: WriteSignal<bool>,
) -> impl IntoView {
    let (saving, set_saving) = signal(false);
    let (save_status, set_save_status) = signal(String::new());

    let on_prev = move |_| {
        set_current_page.update(|p| {
            if *p > 1 {
                *p -= 1;
            }
        });
    };

    let on_next = move |_| {
        let total = total_pages.get();
        set_current_page.update(|p| {
            if *p < total {
                *p += 1;
            }
        });
    };

    let on_page_input = move |ev: web_sys::Event| {
        let target = event_target::<web_sys::HtmlInputElement>(&ev);
        if let Ok(val) = target.value().parse::<u32>() {
            let total = total_pages.get();
            if val >= 1 && val <= total {
                set_current_page.set(val);
            }
        }
    };

    let on_slider_input = move |ev: web_sys::Event| {
        let target = event_target::<web_sys::HtmlInputElement>(&ev);
        if let Ok(val) = target.value().parse::<u32>() {
            set_current_page.set(val);
        }
    };

    let on_zoom_change = move |ev: web_sys::Event| {
        let target = event_target::<web_sys::HtmlSelectElement>(&ev);
        if let Ok(val) = target.value().parse::<f64>() {
            set_zoom.set(val);
        }
    };

    let zoom_in = move |_| {
        let current = zoom.get();
        let levels = [0.5, 0.75, 1.0, 1.25, 1.5, 2.0];
        if let Some(next) = levels.iter().find(|&&l| l > current) {
            set_zoom.set(*next);
        }
    };

    let zoom_out = move |_| {
        let current = zoom.get();
        let levels = [0.5, 0.75, 1.0, 1.25, 1.5, 2.0];
        if let Some(prev) = levels.iter().rev().find(|&&l| l < current) {
            set_zoom.set(*prev);
        }
    };

    let toggle_theme = move |_| {
        let new_theme = theme.get().toggle();
        set_theme.set(new_theme);
        apply_theme(new_theme);
    };

    let toggle_fullscreen = move |_| {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if document.fullscreen_element().is_some() {
                    let _ = document.exit_fullscreen();
                } else if let Some(el) = document.document_element() {
                    let _ = el.request_fullscreen();
                }
            }
        }
    };

    let toggle_sidebar = move |_| {
        set_sidebar_open.update(|v| *v = !*v);
    };

    let on_save = move |_| {
        let all_notes = notes.get_untracked();
        if all_notes.is_empty() {
            set_save_status.set("Nothing to save".into());
            return;
        }
        set_saving.set(true);
        set_save_status.set("Saving...".into());

        spawn_local(async move {
            let mut errors = 0;
            for note in &all_notes {
                if let Err(_) = api::create_note(note).await {
                    errors += 1;
                }
            }
            set_saving.set(false);
            if errors == 0 {
                set_save_status.set("Saved!".into());
            } else {
                set_save_status.set(format!("{} failed", errors));
            }
        });
    };

    let zoom_levels: Vec<f64> = vec![0.5, 0.75, 1.0, 1.25, 1.5, 2.0];
    let zoom_options = zoom_levels
        .iter()
        .map(|&z| {
            let label = format!("{}%", (z * 100.0) as u32);
            let val = z.to_string();
            view! {
                <option value={val}>{label}</option>
            }
        })
        .collect_view();

    view! {
        <div class="toolbar">
            <div class="toolbar-group nav-group">
                <button class="toolbar-btn" on:click=on_prev title="Previous Page">
                    {"\u{25C0}"}
                </button>
                <input
                    type="range"
                    class="page-slider"
                    prop:value=move || current_page.get().to_string()
                    on:input=on_slider_input
                    min="1"
                    max=move || total_pages.get().to_string()
                    step="1"
                />
                <input
                    type="number"
                    class="page-input"
                    prop:value=move || current_page.get().to_string()
                    on:change=on_page_input
                    min="1"
                    max=move || total_pages.get().to_string()
                />
                <span class="page-count">
                    {"/ "}{move || total_pages.get()}
                </span>
                <button class="toolbar-btn" on:click=on_next title="Next Page">
                    {"\u{25B6}"}
                </button>
            </div>

            <div class="toolbar-group">
                <button class="toolbar-btn" on:click=zoom_out title="Zoom Out">
                    {"\u{2212}"}
                </button>
                <select
                    class="zoom-select"
                    on:change=on_zoom_change
                    prop:value=move || zoom.get().to_string()
                >
                    {zoom_options}
                </select>
                <button class="toolbar-btn" on:click=zoom_in title="Zoom In">
                    {"+"}
                </button>
            </div>

            <div class="toolbar-group">
                <button
                    class="toolbar-btn save-btn"
                    on:click=on_save
                    disabled=move || saving.get()
                    title="Save notes to cloud"
                >
                    {move || if saving.get() { "..." } else { "\u{1F4BE}" }}
                </button>
                <span class="save-status">{move || save_status.get()}</span>
                <button class="toolbar-btn" on:click=toggle_theme title="Toggle Theme">
                    {move || if theme.get() == Theme::Light { "\u{1F319}" } else { "\u{2600}\u{FE0F}" }}
                </button>
                <button class="toolbar-btn" on:click=toggle_fullscreen title="Fullscreen">
                    {"\u{26F6}"}
                </button>
                <button
                    class="toolbar-btn"
                    class:active=move || sidebar_open.get()
                    on:click=toggle_sidebar
                    title="Notes Sidebar"
                >
                    {"\u{1F4DD}"}
                </button>
            </div>
        </div>
    }
}
