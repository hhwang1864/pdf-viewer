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
    canvas_width: ReadSignal<f64>,
    canvas_height: ReadSignal<f64>,
) -> impl IntoView {
    let note_id = note.id.clone();
    let note_id_click = note.id.clone();
    let note_id_delete = note.id.clone();
    let color = note.color.css_color().to_string();

    // Local signals for position so dragging doesn't trigger parent re-render
    let (x_pos, set_x_pos) = signal(note.x_position);
    let (y_pos, set_y_pos) = signal(note.y_position);

    // Local signal for content so typing doesn't trigger parent re-render
    let (local_content, set_local_content) = signal(note.content.clone());

    // Dragging state
    let (dragging, set_dragging) = signal(false);
    let (drag_offset_x, set_drag_offset_x) = signal(0.0f64);
    let (drag_offset_y, set_drag_offset_y) = signal(0.0f64);

    let left = move || x_pos.get() * canvas_width.get();
    let top = move || y_pos.get() * canvas_height.get();

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
        let value = event_target_value(&ev);
        set_local_content.set(value);
    };

    // Store note_id in a signal so blur closure is Copy-friendly
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

    // Drag: mousedown on header
    let note_id_drag = note_id.clone();
    let on_mousedown = move |ev: web_sys::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();
        set_dragging.set(true);

        let cw = canvas_width.get_untracked();
        let ch = canvas_height.get_untracked();
        let current_left = x_pos.get_untracked() * cw;
        let current_top = y_pos.get_untracked() * ch;
        set_drag_offset_x.set(ev.client_x() as f64 - current_left);
        set_drag_offset_y.set(ev.client_y() as f64 - current_top);

        let note_id_up = note_id_drag.clone();

        let on_mousemove = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |ev: web_sys::MouseEvent| {
            let cw = canvas_width.get_untracked();
            let ch = canvas_height.get_untracked();
            if cw > 0.0 && ch > 0.0 {
                // Get canvas wrapper position
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                if let Some(wrapper) = document.query_selector(".pdf-canvas-wrapper").ok().flatten() {
                    let rect = wrapper.get_bounding_client_rect();
                    let px = ev.client_x() as f64 - drag_offset_x.get_untracked();
                    let py = ev.client_y() as f64 - drag_offset_y.get_untracked();
                    let rel_x = (px - rect.left()) / cw;
                    let rel_y = (py - rect.top()) / ch;
                    let rel_x = rel_x.clamp(0.0, 1.0);
                    let rel_y = rel_y.clamp(0.0, 1.0);
                    set_x_pos.set(rel_x);
                    set_y_pos.set(rel_y);
                }
            }
        });

        let on_mouseup = Closure::<dyn FnMut(web_sys::MouseEvent)>::new({
            let note_id_up = note_id_up.clone();
            move |_ev: web_sys::MouseEvent| {
                set_dragging.set(false);
                // Sync position back to parent
                let id = note_id_up.clone();
                let x = x_pos.get_untracked();
                let y = y_pos.get_untracked();
                set_notes.update(|notes| {
                    if let Some(n) = notes.iter_mut().find(|n| n.id == id) {
                        n.x_position = x;
                        n.y_position = y;
                    }
                });
            }
        });

        let window = web_sys::window().unwrap();
        let _ = window.add_event_listener_with_callback("mousemove", on_mousemove.as_ref().unchecked_ref());
        let move_ref = on_mousemove.as_ref().unchecked_ref::<js_sys::Function>().clone();
        let up_ref_for_cleanup = on_mouseup.as_ref().unchecked_ref::<js_sys::Function>().clone();
        let window2 = window.clone();

        let cleanup = Closure::<dyn FnMut(web_sys::MouseEvent)>::once(move |_ev: web_sys::MouseEvent| {
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
            let _ = window2.remove_event_listener_with_callback("mouseup", &up_ref_for_cleanup);
        });

        let _ = window.add_event_listener_with_callback("mouseup", cleanup.as_ref().unchecked_ref());

        on_mousemove.forget();
        on_mouseup.forget();
        cleanup.forget();
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
                left(), top(), color
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
