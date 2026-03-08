# src/components/note_editor.rs

**What this file is:** A modal dialog for editing note details - content, color, and category. Appears as an overlay when clicking a sticky note.

**React equivalent:**
```jsx
function NoteEditor({ note, setNotes, setEditingNote }) {
  return (
    <div className="overlay" onClick={() => setEditingNote(null)}>
      <div className="editor" onClick={e => e.stopPropagation()}>
        <textarea defaultValue={note.content} onChange={handleContent} />
        <div className="colors">
          {colors.map(c => <button onClick={() => handleColor(c)} />)}
        </div>
        <input placeholder="Category" onChange={handleCategory} />
        <button onClick={() => setEditingNote(null)}>Done</button>
      </div>
    </div>
  );
}
```

## Line-by-Line Breakdown

### Lines 1-3: Imports
```rust
use leptos::prelude::*;
use crate::models::{Note, NoteColor};
```

### Lines 5-10: Component props
```rust
#[component]
pub fn NoteEditor(
    note: Note,
    set_notes: WriteSignal<Vec<Note>>,
    set_editing_note: WriteSignal<Option<String>>,
) -> impl IntoView {
```

Takes the note being edited, plus setters for the notes array and editing state.

### Lines 11-15: Clone IDs for closures
```rust
let note_id = note.id.clone();
let note_id_color = note.id.clone();
let note_id_cat = note.id.clone();
let content = note.content.clone();
let category = note.category.clone().unwrap_or_default();
```
Each handler closure needs its own copy of the note ID. `.unwrap_or_default()` converts `None` to `""` (empty string is the default for `String`).

### Lines 17-25: Content change handler
```rust
let on_content_change = move |ev: web_sys::Event| {
    let value = event_target_value(&ev);
    let id = note_id.clone();
    set_notes.update(|notes| {
        if let Some(n) = notes.iter_mut().find(|n| n.id == id) {
            n.content = value;
        }
    });
};
```

**Pattern: updating a specific item in an array.**
1. `set_notes.update(|notes| { ... })` - Get mutable access to the Vec
2. `notes.iter_mut().find(|n| n.id == id)` - Find the note by ID
3. `n.content = value` - Mutate it directly

In React you'd do: `setNotes(prev => prev.map(n => n.id === id ? {...n, content: value} : n))`. Rust lets you mutate in place because the signal tracks the whole Vec.

### Lines 27-34: Color change handler
```rust
let on_color_change = move |color: NoteColor| {
    let id = note_id_color.clone();
    set_notes.update(|notes| {
        if let Some(n) = notes.iter_mut().find(|n| n.id == id) {
            n.color = color;
        }
    });
};
```
Same pattern - find note, update color.

### Lines 36-44: Category change handler
```rust
n.category = if value.is_empty() { None } else { Some(value) };
```
Converts empty string back to `None`. This is the Rust way of handling "optional string" - not an empty string, but actually `None` (null).

### Lines 46-48: Close handler
```rust
let on_close = move |_| {
    set_editing_note.set(None);
};
```

### Lines 50-87: Template
```rust
view! {
    <div class="note-editor-overlay" on:click=on_close>
        <div class="note-editor" on:click=move |ev: web_sys::MouseEvent| ev.stop_propagation()>
```

- The overlay covers the entire screen. Clicking it closes the editor.
- `stop_propagation()` on the inner div prevents clicks inside the editor from closing it (same pattern as React modal overlays).

```rust
            {NoteColor::all().iter().map(|color| {
                let c = color.clone();
                let handler = on_color_change.clone();
                view! {
                    <button
                        style=format!("background-color:{}", color.css_color())
                        on:click=move |_| handler(c.clone())
                    />
                }
            }).collect_view()}
```

Maps over all colors to create buttons. Note `on_color_change.clone()` - closures in Rust need to be cloned when used in multiple iterations of a loop.
