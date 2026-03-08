# src/components/notes_sidebar.rs

**What this file is:** The side panel showing all notes with filtering capabilities. Includes color picker for new notes, color/category filters, and an expandable note list with inline editing.

**React equivalent:**
```jsx
function NotesSidebar({ notes, setNotes, currentPage, setCurrentPage, ... }) {
  const [filterColor, setFilterColor] = useState(null);
  const [filterCategory, setFilterCategory] = useState('');

  const filteredNotes = useMemo(() =>
    notes.filter(n => (!filterColor || n.color === filterColor) && ...)
  , [notes, filterColor, filterCategory]);

  return (
    <div className="sidebar">
      <div className="color-picker">...</div>
      <div className="filters">...</div>
      <div className="note-list">
        {filteredNotes.map(note => <SidebarNote key={note.id} ... />)}
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

### Lines 5-17: Date formatting helper
```rust
fn format_date(iso: &str) -> String {
    if iso.len() < 16 { return iso.to_string(); }
    let hours = &iso[11..13];
    let minutes = &iso[14..16];
    let day = &iso[8..10];
    let month = &iso[5..7];
    let year = &iso[0..4];
    format!("{}_{}_{}_{}_{}",hours, minutes, day, month, year)
}
```

Manual ISO date string parsing using string slicing. `&iso[11..13]` extracts characters at positions 11-12 (like `iso.substring(11, 13)` in JavaScript). Formats as `HH_MM_DD_MM_YYYY`.

### Lines 19-28: Component props
```rust
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
```

### Lines 29-33: Local sidebar state
```rust
let (filter_color, set_filter_color) = signal::<Option<NoteColor>>(None);
let (filter_category, set_filter_category) = signal(String::new());
let (editing_sidebar_note, set_editing_sidebar_note) = signal::<Option<String>>(None);
let (edit_text, set_edit_text) = signal(String::new());
let (expanded, set_expanded) = signal(false);
```

- `filter_color: Option<NoteColor>` - `None` means "show all colors"
- `editing_sidebar_note` - Tracks which note is being edited *in the sidebar* (separate from the main editor)
- `expanded` - Whether the sidebar is in wide mode

### Lines 35-61: Filtering logic
```rust
let filtered_notes = move || {
    let notes = notes.get();
    let color_filter = filter_color.get();
    let cat_filter = filter_category.get();

    notes.into_iter()
        .filter(|n| {
            if let Some(ref c) = color_filter {
                if n.color != *c { return false; }
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
```

This is a derived/computed value (like `useMemo` in React). It:
1. Reads all three signals (notes, filter_color, filter_category) - so it re-runs when any changes
2. Filters by color if a color filter is set
3. Filters by category using case-insensitive substring matching
4. Returns the filtered vec

**`if let Some(ref c) = color_filter`** - `ref` borrows the inner value instead of moving it out of the Option.

**`match &n.category { Some(cat) => ..., None => return false }`** - If filtering by category and the note has no category, exclude it.

### Lines 67-227: Template (large, broken into sections)

#### Color picker for new notes (lines 80-97)
```rust
{NoteColor::all().iter().map(|color| {
    let c = color.clone();
    let c2 = color.clone();
    view! {
        <button
            class="color-btn"
            class:selected=move || selected_color.get() == c2
            style=format!("background-color:{}", color.css_color())
            on:click=move |_| set_selected_color.set(c.clone())
        />
    }
}).collect_view()}
```
Maps over all 5 colors, rendering a button for each. `class:selected` adds the "selected" class when this color matches the current selection.

#### Filter by color (lines 99-123)
Same pattern but with an "All" button that sets filter to `None`.

#### Filter by category (lines 125-134)
```rust
<input
    type="text"
    prop:value=move || filter_category.get()
    on:input=move |ev| set_filter_category.set(event_target_value(&ev))
    placeholder="Type to filter..."
/>
```
Standard controlled input pattern.

#### Note list with inline editing (lines 136-225)
```rust
{move || {
    let notes = filtered_notes();
    if notes.is_empty() {
        view! { <p>"No notes found."</p> }.into_any()
    } else {
        notes.into_iter().map(|note| {
            // ... render each note card
        }).collect_view().into_any()
    }
}}
```

Each note card shows:
- Page number and formatted date
- Delete button
- Content area that:
  - **Single click** → navigates to that note's page and opens the editor
  - **Double click** → enables inline editing in the sidebar itself

```rust
on:dblclick=move |ev: web_sys::MouseEvent| {
    ev.stop_propagation();
    set_edit_text.set(content.clone());
    set_editing_sidebar_note.set(Some(note_id_dbl.clone()));
}
on:click=move |_| {
    set_current_page.set(page);
    set_editing_note.set(Some(note_id_click.clone()));
}
```

The inline editing uses a `<textarea>` that saves on blur:
```rust
on:blur=move |_| {
    let final_text = edit_text.get_untracked();
    set_notes.update(|notes| {
        if let Some(n) = notes.iter_mut().find(|n| n.id == id) {
            n.content = final_text;
        }
    });
    set_editing_sidebar_note.set(None);
}
```

#### Expand/collapse button
```rust
<button on:click=move |_| set_expanded.update(|v| *v = !*v)>
    {move || if expanded.get() { "\u{25B6}" } else { "\u{25C0}" }}
</button>
```
Toggles between 300px and 480px sidebar width (controlled by CSS class `.expanded`).
