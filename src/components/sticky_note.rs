use leptos::prelude::*;

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
    let note_id_delete = note.id.clone();
    let note_id_edit = note.id.clone();
    let color = note.color.css_color().to_string();

    let left = move || note.x_position * canvas_width.get();
    let top = move || note.y_position * canvas_height.get();

    let on_click = move |ev: web_sys::MouseEvent| {
        ev.stop_propagation();
        set_editing_note.set(Some(note_id_edit.clone()));
    };

    let on_delete = move |ev: web_sys::MouseEvent| {
        ev.stop_propagation();
        let id = note_id_delete.clone();
        set_notes.update(|notes| notes.retain(|n| n.id != id));
        set_editing_note.set(None);
    };

    let content = note.content.clone();
    let note_id_for_input = note_id.clone();

    let on_input = move |ev: web_sys::Event| {
        let value = event_target_value(&ev);
        let id = note_id_for_input.clone();
        set_notes.update(|notes| {
            if let Some(n) = notes.iter_mut().find(|n| n.id == id) {
                n.content = value;
                n.updated_at = js_sys::Date::new_0()
                    .to_iso_string()
                    .as_string()
                    .unwrap_or_default();
            }
        });
    };

    view! {
        <div
            class="sticky-note"
            style=move || format!(
                "left:{}px;top:{}px;background-color:{};",
                left(), top(), color
            )
            on:click=on_click
        >
            <div class="sticky-note-header">
                <button class="sticky-note-delete" on:click=on_delete title="Delete note">
                    {"\u{2715}"}
                </button>
            </div>
            {move || if is_editing.get() {
                let c = content.clone();
                view! {
                    <textarea
                        class="sticky-note-input"
                        prop:value=c
                        on:input=on_input.clone()
                        placeholder="Type your note..."
                    />
                }.into_any()
            } else {
                let c = content.clone();
                view! {
                    <div class="sticky-note-content">
                        {if c.is_empty() { "Click to edit...".to_string() } else { c }}
                    </div>
                }.into_any()
            }}
        </div>
    }
}
