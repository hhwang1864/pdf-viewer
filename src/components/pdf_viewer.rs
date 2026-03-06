use leptos::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

use crate::js_bindings;
use crate::models::{Note, NoteColor};

#[component]
pub fn PdfViewer(
    current_page: ReadSignal<u32>,
    zoom: ReadSignal<f64>,
    notes: ReadSignal<Vec<Note>>,
    set_notes: WriteSignal<Vec<Note>>,
    pdf_hash: ReadSignal<String>,
    editing_note: ReadSignal<Option<String>>,
    set_editing_note: WriteSignal<Option<String>>,
    selected_color: ReadSignal<NoteColor>,
) -> impl IntoView {
    let (canvas_width, set_canvas_width) = signal(0.0f64);
    let (canvas_height, set_canvas_height) = signal(0.0f64);

    // Re-render when page or zoom changes
    Effect::new(move || {
        let page = current_page.get();
        let scale = zoom.get();
        if page > 0 && js_bindings::is_loaded() {
            spawn_local(async move {
                match js_bindings::render_page(page, "pdf-canvas", scale).await {
                    Ok(result) => {
                        if let Ok(dims) = serde_wasm_bindgen::from_value::<PageDimensions>(result)
                        {
                            set_canvas_width.set(dims.width);
                            set_canvas_height.set(dims.height);
                        }
                    }
                    Err(e) => {
                        web_sys::console::error_1(&e);
                    }
                }
            });
        }
    });

    let on_canvas_click = move |ev: web_sys::MouseEvent| {
        let hash = pdf_hash.get();
        if hash.is_empty() {
            return;
        }

        // Get click position relative to canvas
        let target = ev.target().unwrap();
        let canvas = target.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let rect = canvas.get_bounding_client_rect();
        let x = (ev.client_x() as f64 - rect.left()) / rect.width();
        let y = (ev.client_y() as f64 - rect.top()) / rect.height();

        let page = current_page.get_untracked();
        let color = selected_color.get_untracked();
        let now = js_sys::Date::new_0().to_iso_string().as_string().unwrap();

        let new_note = Note {
            id: uuid::Uuid::new_v4().to_string(),
            pdf_hash: hash,
            page_number: page,
            x_position: x,
            y_position: y,
            content: String::new(),
            color,
            category: None,
            created_at: now.clone(),
            updated_at: now,
        };

        let note_id = new_note.id.clone();
        set_notes.update(|n| n.push(new_note));
        set_editing_note.set(Some(note_id));
    };

    let page_notes = move || {
        let page = current_page.get();
        notes
            .get()
            .into_iter()
            .filter(move |n| n.page_number == page)
            .collect::<Vec<_>>()
    };

    view! {
        <div class="pdf-container">
            <div class="pdf-canvas-wrapper" style=move || {
                format!("width:{}px;height:{}px", canvas_width.get(), canvas_height.get())
            }>
                <canvas id="pdf-canvas" on:click=on_canvas_click />
                <div class="notes-overlay">
                    {move || page_notes().into_iter().map(|note| {
                        let note_id = note.id.clone();
                        let is_editing = move || editing_note.get().as_deref() == Some(&note_id);
                        view! {
                            <crate::components::sticky_note::StickyNote
                                note=note.clone()
                                is_editing=Signal::derive(is_editing)
                                set_editing_note=set_editing_note
                                set_notes=set_notes
                                canvas_width=canvas_width
                                canvas_height=canvas_height
                            />
                        }
                    }).collect_view()}
                </div>
            </div>
        </div>
    }
}

#[derive(serde::Deserialize)]
struct PageDimensions {
    width: f64,
    height: f64,
}
