use leptos::prelude::*;

use crate::models::{Note, NoteColor};

#[component]
pub fn NoteEditor(
    note: Note,
    set_notes: WriteSignal<Vec<Note>>,
    set_editing_note: WriteSignal<Option<String>>,
) -> impl IntoView {
    let note_id = note.id.clone();
    let note_id_color = note.id.clone();
    let note_id_cat = note.id.clone();
    let content = note.content.clone();
    let category = note.category.clone().unwrap_or_default();

    let on_content_change = move |ev: web_sys::Event| {
        let value = event_target_value(&ev);
        let id = note_id.clone();
        set_notes.update(|notes| {
            if let Some(n) = notes.iter_mut().find(|n| n.id == id) {
                n.content = value;
            }
        });
    };

    let on_color_change = move |color: NoteColor| {
        let id = note_id_color.clone();
        set_notes.update(|notes| {
            if let Some(n) = notes.iter_mut().find(|n| n.id == id) {
                n.color = color;
            }
        });
    };

    let on_category_change = move |ev: web_sys::Event| {
        let value = event_target_value(&ev);
        let id = note_id_cat.clone();
        set_notes.update(|notes| {
            if let Some(n) = notes.iter_mut().find(|n| n.id == id) {
                n.category = if value.is_empty() { None } else { Some(value) };
            }
        });
    };

    let on_close = move |_| {
        set_editing_note.set(None);
    };

    view! {
        <div class="note-editor-overlay" on:click=on_close>
            <div class="note-editor" on:click=move |ev: web_sys::MouseEvent| ev.stop_propagation()>
                <h3>"Edit Note"</h3>
                <textarea
                    class="note-editor-textarea"
                    prop:value=content
                    on:input=on_content_change
                    placeholder="Write your note..."
                />
                <div class="note-editor-colors">
                    {NoteColor::all().iter().map(|color| {
                        let c = color.clone();
                        let handler = on_color_change.clone();
                        view! {
                            <button
                                class="color-btn"
                                style=format!("background-color:{}", color.css_color())
                                on:click=move |_| handler(c.clone())
                                title=color.as_str()
                            />
                        }
                    }).collect_view()}
                </div>
                <input
                    type="text"
                    class="note-editor-category"
                    prop:value=category
                    on:input=on_category_change
                    placeholder="Category (optional)"
                />
                <button class="note-editor-close" on:click=move |_| set_editing_note.set(None)>
                    "Done"
                </button>
            </div>
        </div>
    }
}
