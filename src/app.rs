use leptos::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

use crate::components::notes_sidebar::NotesSidebar;
use crate::components::pdf_viewer::PdfViewer;
use crate::components::toolbar::Toolbar;
use crate::js_bindings;
use crate::models::{Note, NoteColor};
use crate::theme::{apply_theme, load_theme};

#[component]
pub fn App() -> impl IntoView {
    let initial_theme = load_theme();
    apply_theme(initial_theme);

    let (current_page, set_current_page) = signal(1u32);
    let (total_pages, set_total_pages) = signal(0u32);
    let (zoom, set_zoom) = signal(1.0f64);
    let (theme, set_theme) = signal(initial_theme);
    let (notes, set_notes) = signal(Vec::<Note>::new());
    let (pdf_hash, set_pdf_hash) = signal(String::new());
    let (editing_note, set_editing_note) = signal::<Option<String>>(None);
    let (sidebar_open, set_sidebar_open) = signal(false);
    let (selected_color, set_selected_color) = signal(NoteColor::Yellow);
    let (pdf_loaded, set_pdf_loaded) = signal(false);

    let on_file_select = move |ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let input = target.dyn_into::<web_sys::HtmlInputElement>().unwrap();

        if let Some(files) = input.files() {
            if let Some(file) = files.get(0) {
                let reader =
                    web_sys::FileReader::new().expect("Failed to create FileReader");
                let reader_clone = reader.clone();

                let onload = wasm_bindgen::closure::Closure::wrap(Box::new(
                    move |_ev: web_sys::ProgressEvent| {
                        let result = reader_clone.result().unwrap();
                        let array_buffer = result.dyn_into::<js_sys::ArrayBuffer>().unwrap();
                        let uint8_array = js_sys::Uint8Array::new(&array_buffer);

                        spawn_local(async move {
                            // Compute hash
                            match js_bindings::compute_hash(&uint8_array).await {
                                Ok(hash_val) => {
                                    if let Some(hash) = hash_val.as_string() {
                                        set_pdf_hash.set(hash);
                                    }
                                }
                                Err(e) => web_sys::console::error_1(&e),
                            }

                            // Load PDF
                            match js_bindings::load_pdf_from_data(&uint8_array).await {
                                Ok(num_pages) => {
                                    let pages =
                                        num_pages.as_f64().unwrap_or(0.0) as u32;
                                    set_total_pages.set(pages);
                                    set_current_page.set(1);
                                    set_pdf_loaded.set(true);
                                    set_notes.set(Vec::new());
                                }
                                Err(e) => {
                                    web_sys::console::error_1(&e);
                                }
                            }
                        });
                    },
                )
                    as Box<dyn FnMut(web_sys::ProgressEvent)>);

                reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                let _ = reader.read_as_array_buffer(&file);
                onload.forget();
            }
        }
    };

    let on_url_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let form = ev.target().unwrap();
        let form_el = form.dyn_into::<web_sys::HtmlElement>().unwrap();
        if let Ok(Some(input_el)) = form_el.query_selector("input[name=pdf-url]") {
            let input = input_el.dyn_into::<web_sys::HtmlInputElement>().unwrap();
            let url = input.value();
            if !url.is_empty() {
                spawn_local(async move {
                    match js_bindings::load_pdf_from_url(&url).await {
                        Ok(num_pages) => {
                            let pages = num_pages.as_f64().unwrap_or(0.0) as u32;
                            set_total_pages.set(pages);
                            set_current_page.set(1);
                            set_pdf_loaded.set(true);
                            set_pdf_hash.set(format!("url:{}", url));
                            set_notes.set(Vec::new());
                        }
                        Err(e) => web_sys::console::error_1(&e),
                    }
                });
            }
        }
    };

    // Click outside notes overlay to deselect
    let on_app_click = move |_ev: web_sys::MouseEvent| {
        // handled by note editor overlay
    };

    view! {
        <div class="app" data-theme=move || theme.get().as_str() on:click=on_app_click>
            <Toolbar
                current_page=current_page
                total_pages=total_pages
                zoom=zoom
                theme=theme
                set_current_page=set_current_page
                set_zoom=set_zoom
                set_theme=set_theme
                sidebar_open=sidebar_open
                set_sidebar_open=set_sidebar_open
            />

            <div class="main-content">
                <div class="pdf-area">
                    {move || if !pdf_loaded.get() {
                        view! {
                            <div class="upload-area">
                                <h2>"Open a PDF"</h2>
                                <div class="upload-options">
                                    <div class="upload-file">
                                        <label for="file-input" class="file-label">"Choose File"</label>
                                        <input
                                            id="file-input"
                                            type="file"
                                            accept=".pdf"
                                            on:change=on_file_select.clone()
                                        />
                                    </div>
                                    <span class="upload-or">"or"</span>
                                    <form class="upload-url" on:submit=on_url_submit>
                                        <input
                                            type="url"
                                            name="pdf-url"
                                            placeholder="Enter PDF URL..."
                                            class="url-input"
                                        />
                                        <button type="submit" class="url-submit">"Load"</button>
                                    </form>
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <PdfViewer
                                current_page=current_page
                                zoom=zoom
                                notes=notes
                                set_notes=set_notes
                                pdf_hash=pdf_hash
                                editing_note=editing_note
                                set_editing_note=set_editing_note
                                selected_color=selected_color
                            />
                        }.into_any()
                    }}
                </div>

                {move || if sidebar_open.get() {
                    view! {
                        <NotesSidebar
                            notes=notes
                            set_notes=set_notes
                            current_page=current_page
                            set_current_page=set_current_page
                            set_editing_note=set_editing_note
                            selected_color=selected_color
                            set_selected_color=set_selected_color
                        />
                    }.into_any()
                } else {
                    view! { <div /> }.into_any()
                }}
            </div>
        </div>
    }
}
