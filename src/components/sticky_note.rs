use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::models::Note;

#[component]
pub fn StickyNote(
    note: Note,
    is_editing: Signal<bool>,
    set_editing_note: WriteSignal<Option<String>>,
    set_notes: WriteSignal<Vec<Note>>,
) -> impl IntoView {
    let note_id = note.id.clone();
    let note_id_click = note.id.clone();
    let note_id_delete = note.id.clone();
    let color = note.color.css_color().to_string();

    // Pixel positions directly
    let (x_pos, set_x_pos) = signal(note.x_position);
    let (y_pos, set_y_pos) = signal(note.y_position);

    let (local_content, set_local_content) = signal(note.content.clone());

    let (dragging, set_dragging) = signal(false);
    let (drag_offset_x, set_drag_offset_x) = signal(0.0f64);
    let (drag_offset_y, set_drag_offset_y) = signal(0.0f64);

    let on_click = move |ev: web_sys::MouseEvent| {
        ev.stop_propagation();
        if !dragging.get_untracked() {
            set_editing_note.set(Some(note_id_click.clone()));
        }
    };

    let on_delete = move |ev: web_sys::MouseEvent| {
        ev.stop_propagation();
        let id = note_id_delete.clone();
        set_notes.update(|notes| notes.retain(|n| n.id != id));
        set_editing_note.set(None);
    };

    let on_input = move |ev: web_sys::Event| {
        set_local_content.set(event_target_value(&ev));
    };

    let note_id_stored = StoredValue::new(note_id.clone());
    let on_blur = move |_ev: web_sys::FocusEvent| {
        let id = note_id_stored.get_value();
        let content = local_content.get_untracked();
        set_notes.update(|notes| {
            if let Some(n) = notes.iter_mut().find(|n| n.id == id) {
                n.content = content;
            }
        });
    };

    // Drag via mousedown on header
    let note_id_drag = note_id.clone();
    let on_mousedown = move |ev: web_sys::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();
        set_dragging.set(true);

        // Calculate offset between mouse and note position so it doesn't jump
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        if let Some(area) = document.query_selector(".pdf-area").ok().flatten() {
            let area_el: web_sys::HtmlElement = area.dyn_into().unwrap();
            let rect = area_el.get_bounding_client_rect();
            let mouse_x = ev.client_x() as f64 - rect.left() + area_el.scroll_left() as f64;
            let mouse_y = ev.client_y() as f64 - rect.top() + area_el.scroll_top() as f64;
            set_drag_offset_x.set(mouse_x - x_pos.get_untracked());
            set_drag_offset_y.set(mouse_y - y_pos.get_untracked());
        }

        let note_id_up = note_id_drag.clone();

        let on_mousemove = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(
            move |ev: web_sys::MouseEvent| {
                // Get scroll offset of .pdf-area
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                if let Some(area) = document.query_selector(".pdf-area").ok().flatten() {
                    let area_el: web_sys::HtmlElement = area.dyn_into().unwrap();
                    let rect = area_el.get_bounding_client_rect();
                    let px = ev.client_x() as f64 - rect.left() + area_el.scroll_left() as f64 - drag_offset_x.get_untracked();
                    let py = ev.client_y() as f64 - rect.top() + area_el.scroll_top() as f64 - drag_offset_y.get_untracked();
                    let px = px.max(0.0);
                    let py = py.max(0.0);
                    set_x_pos.set(px);
                    set_y_pos.set(py);
                }
            },
        );

        let window = web_sys::window().unwrap();
        let _ = window.add_event_listener_with_callback(
            "mousemove",
            on_mousemove.as_ref().unchecked_ref(),
        );
        let move_ref = on_mousemove
            .as_ref()
            .unchecked_ref::<js_sys::Function>()
            .clone();
        let window2 = window.clone();

        let cleanup =
            Closure::<dyn FnMut(web_sys::MouseEvent)>::once(move |_ev: web_sys::MouseEvent| {
                set_dragging.set(false);
                let id = note_id_up.clone();
                let x = x_pos.get_untracked();
                let y = y_pos.get_untracked();
                set_notes.update(|notes| {
                    if let Some(n) = notes.iter_mut().find(|n| n.id == id) {
                        n.x_position = x;
                        n.y_position = y;
                    }
                });
                let _ = window2.remove_event_listener_with_callback("mousemove", &move_ref);
            });

        let up_ref = cleanup
            .as_ref()
            .unchecked_ref::<js_sys::Function>()
            .clone();
        let window3 = window.clone();
        // Self-removing mouseup listener
        let cleanup_wrapper =
            Closure::<dyn FnMut(web_sys::MouseEvent)>::once(move |ev: web_sys::MouseEvent| {
                // Call the actual cleanup
                let _ = up_ref.call1(&wasm_bindgen::JsValue::NULL, &ev);
                let _ =
                    window3.remove_event_listener_with_callback("mouseup", &up_ref);
            });

        let _ = window.add_event_listener_with_callback(
            "mouseup",
            cleanup_wrapper.as_ref().unchecked_ref(),
        );

        on_mousemove.forget();
        cleanup.forget();
        cleanup_wrapper.forget();
    };

    let display_content = move || {
        let c = local_content.get();
        if c.is_empty() {
            "Click to edit...".to_string()
        } else {
            c
        }
    };

    view! {
        <div
            class="sticky-note"
            class:dragging=move || dragging.get()
            style=move || format!(
                "left:{}px;top:{}px;background-color:{};",
                x_pos.get(), y_pos.get(), color
            )
            on:click=on_click
        >
            <div class="sticky-note-header" on:mousedown=on_mousedown>
                <span class="sticky-note-drag">{"\u{2630}"}</span>
                <button class="sticky-note-delete" on:click=on_delete title="Delete note">
                    {"\u{2715}"}
                </button>
            </div>
            <Show when=move || is_editing.get()
                fallback=move || view! {
                    <div class="sticky-note-content">{display_content}</div>
                }
            >
                <textarea
                    class="sticky-note-input"
                    prop:value=move || local_content.get()
                    on:input=on_input
                    on:blur=on_blur
                    on:click=move |ev: web_sys::MouseEvent| ev.stop_propagation()
                    placeholder="Type your note..."
                />
            </Show>
        </div>
    }
}
