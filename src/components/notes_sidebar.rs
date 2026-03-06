use leptos::prelude::*;

use crate::models::{Note, NoteColor};

#[component]
pub fn NotesSidebar(
    notes: ReadSignal<Vec<Note>>,
    set_notes: WriteSignal<Vec<Note>>,
    current_page: ReadSignal<u32>,
    set_current_page: WriteSignal<u32>,
    set_editing_note: WriteSignal<Option<String>>,
    selected_color: ReadSignal<NoteColor>,
    set_selected_color: WriteSignal<NoteColor>,
) -> impl IntoView {
    let (filter_color, set_filter_color) = signal::<Option<NoteColor>>(None);
    let (filter_category, set_filter_category) = signal(String::new());

    let filtered_notes = move || {
        let notes = notes.get();
        let color_filter = filter_color.get();
        let cat_filter = filter_category.get();

        notes
            .into_iter()
            .filter(|n| {
                if let Some(ref c) = color_filter {
                    if n.color != *c {
                        return false;
                    }
                }
                if !cat_filter.is_empty() {
                    match &n.category {
                        Some(cat) => {
                            if !cat.to_lowercase().contains(&cat_filter.to_lowercase()) {
                                return false;
                            }
                        }
                        None => return false,
                    }
                }
                true
            })
            .collect::<Vec<_>>()
    };

    let on_color_filter = move |color: Option<NoteColor>| {
        set_filter_color.set(color);
    };

    view! {
        <div class="notes-sidebar">
            <h3>"Notes"</h3>

            <div class="sidebar-section">
                <label>"New note color:"</label>
                <div class="color-picker">
                    {NoteColor::all().iter().map(|color| {
                        let c = color.clone();
                        let c2 = color.clone();
                        view! {
                            <button
                                class="color-btn"
                                class:selected=move || selected_color.get() == c2
                                style=format!("background-color:{}", color.css_color())
                                on:click=move |_| set_selected_color.set(c.clone())
                                title=color.as_str()
                            />
                        }
                    }).collect_view()}
                </div>
            </div>

            <div class="sidebar-section">
                <label>"Filter by color:"</label>
                <div class="color-picker">
                    <button
                        class="color-btn all-colors"
                        class:selected=move || filter_color.get().is_none()
                        on:click=move |_| on_color_filter(None)
                        title="All"
                    >"All"</button>
                    {NoteColor::all().iter().map(|color| {
                        let c = color.clone();
                        let c2 = color.clone();
                        let handler = on_color_filter.clone();
                        view! {
                            <button
                                class="color-btn"
                                class:selected=move || filter_color.get().as_ref() == Some(&c2)
                                style=format!("background-color:{}", color.css_color())
                                on:click=move |_| handler(Some(c.clone()))
                                title=color.as_str()
                            />
                        }
                    }).collect_view()}
                </div>
            </div>

            <div class="sidebar-section">
                <label>"Filter by category:"</label>
                <input
                    type="text"
                    class="category-filter"
                    prop:value=move || filter_category.get()
                    on:input=move |ev| set_filter_category.set(event_target_value(&ev))
                    placeholder="Type to filter..."
                />
            </div>

            <div class="notes-list">
                {move || {
                    let notes = filtered_notes();
                    if notes.is_empty() {
                        view! { <p class="no-notes">"No notes found."</p> }.into_any()
                    } else {
                        notes.into_iter().map(|note| {
                            let note_id_nav = note.id.clone();
                            let note_id_del = note.id.clone();
                            let page = note.page_number;
                            let color = note.color.css_color().to_string();
                            let content = if note.content.is_empty() {
                                "(empty)".to_string()
                            } else if note.content.len() > 50 {
                                format!("{}...", &note.content[..50])
                            } else {
                                note.content.clone()
                            };
                            let category = note.category.clone().unwrap_or_default();

                            view! {
                                <div
                                    class="sidebar-note"
                                    style=format!("border-left: 4px solid {}", color)
                                    on:click=move |_| {
                                        set_current_page.set(page);
                                        set_editing_note.set(Some(note_id_nav.clone()));
                                    }
                                >
                                    <div class="sidebar-note-header">
                                        <span class="sidebar-note-page">"Page " {page}</span>
                                        {if !category.is_empty() {
                                            view! { <span class="sidebar-note-category">{category.clone()}</span> }.into_any()
                                        } else {
                                            view! { <span /> }.into_any()
                                        }}
                                        <button
                                            class="sidebar-note-delete"
                                            on:click=move |ev: web_sys::MouseEvent| {
                                                ev.stop_propagation();
                                                let id = note_id_del.clone();
                                                set_notes.update(|n| n.retain(|note| note.id != id));
                                            }
                                            title="Delete"
                                        >{"\u{2715}"}</button>
                                    </div>
                                    <div class="sidebar-note-content">{content}</div>
                                </div>
                            }
                        }).collect_view().into_any()
                    }
                }}
            </div>
        </div>
    }
}
