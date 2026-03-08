# src/components/sticky_note.rs

**What this file is:** A draggable sticky note widget. Each note appears on the PDF at a specific position and can be dragged, edited inline, and deleted.

This is the most complex component because it implements mouse-based drag-and-drop manually (no library). If you've done drag-and-drop in vanilla JS, the pattern is familiar.

**React equivalent concept:**
```jsx
function StickyNote({ note, isEditing, setEditingNote, setNotes }) {
  const [pos, setPos] = useState({ x: note.x, y: note.y });
  const [dragging, setDragging] = useState(false);

  const handleMouseDown = (e) => {
    // Start drag: add mousemove/mouseup listeners to window
  };

  return (
    <div style={{ left: pos.x, top: pos.y }} onClick={() => setEditingNote(note.id)}>
      <div className="header" onMouseDown={handleMouseDown}>drag handle</div>
      {isEditing ? <textarea ... /> : <div>{note.content}</div>}
    </div>
  );
}
```

## Line-by-Line Breakdown

### Lines 1-5: Imports
```rust
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::models::Note;
```

### Lines 8-13: Component props
```rust
#[component]
pub fn StickyNote(
    note: Note,                              // The note data (owned, not a signal)
    is_editing: Signal<bool>,                // Whether this note is being edited
    set_editing_note: WriteSignal<Option<String>>,  // Set which note is editing
    set_notes: WriteSignal<Vec<Note>>,       // Modify the notes array
) -> impl IntoView {
```

**`note: Note`** - Takes ownership of the Note value (not a signal). This means the note data is a snapshot - if you need to update it, you go through `set_notes`.

**`is_editing: Signal<bool>`** - A derived signal (computed from `editing_note.get() == Some(this_note_id)`). `Signal<bool>` can be either a read signal or a derived signal.

### Lines 14-27: Setup variables
```rust
let note_id = note.id.clone();
let note_id_click = note.id.clone();
let note_id_delete = note.id.clone();
let color = note.color.css_color().to_string();
```

**Why so many `.clone()`?** Each closure below needs its own copy of `note_id` because Rust's ownership rules prevent sharing. When a closure uses `move`, it takes ownership of the captured variable. So if two closures both need `note_id`, you need two copies.

Think of it like:
```javascript
// JS doesn't need this because it uses reference counting
const noteId = note.id;  // shared freely

// Rust equivalent - each closure gets its own copy
const noteIdForClick = note.id.clone();
const noteIdForDelete = note.id.clone();
```

```rust
let (x_pos, set_x_pos) = signal(note.x_position);
let (y_pos, set_y_pos) = signal(note.y_position);
let (local_content, set_local_content) = signal(note.content.clone());
let (dragging, set_dragging) = signal(false);
let (drag_offset_x, set_drag_offset_x) = signal(0.0f64);
let (drag_offset_y, set_drag_offset_y) = signal(0.0f64);
```

Local state for position (changes during drag), content (for inline editing), and drag state.

### Lines 29-34: Click handler
```rust
let on_click = move |ev: web_sys::MouseEvent| {
    ev.stop_propagation();  // Don't trigger pdf-area click (which would create a new note)
    if !dragging.get_untracked() {
        set_editing_note.set(Some(note_id_click.clone()));
    }
};
```
Only opens editor if not currently dragging (prevents opening editor at end of drag).

### Lines 36-40: Delete handler
```rust
let on_delete = move |ev: web_sys::MouseEvent| {
    ev.stop_propagation();
    let id = note_id_delete.clone();
    set_notes.update(|notes| notes.retain(|n| n.id != id));
    set_editing_note.set(None);
};
```
- `.retain(|n| n.id != id)` - Keep only notes whose ID doesn't match. Like `.filter()` in JavaScript. This removes the note from the array.

### Lines 43-56: Inline editing
```rust
let on_input = move |ev: web_sys::Event| {
    set_local_content.set(event_target_value(&ev));
};

let on_blur = move |_ev: web_sys::FocusEvent| {
    let id = note_id_stored.get_value();
    let content = local_content.get_untracked();
    set_notes.update(|notes| {
        if let Some(n) = notes.iter_mut().find(|n| n.id == id) {
            n.content = content;
        }
    });
};
```

- Typing updates `local_content` (local state) immediately
- On blur (focus loss), syncs the local content back to the shared notes array
- `notes.iter_mut().find(|n| n.id == id)` - Find the note in the array by ID and get a mutable reference
- **`StoredValue::new(note_id.clone())`** (line 47) - A non-reactive storage. Unlike signals, it doesn't trigger reactivity when read. Used when you just need to store a value for later use in closures.

### Lines 60-145: Drag implementation (the complex part)

```rust
let on_mousedown = move |ev: web_sys::MouseEvent| {
    ev.prevent_default();
    ev.stop_propagation();
    set_dragging.set(true);
```

On mousedown on the header:
1. Calculate offset between mouse position and note position (so note doesn't jump to cursor)
2. Create a `mousemove` closure that updates position
3. Create a `mouseup` closure that finalizes the drag
4. Attach both to `window` (not the note element - so dragging works even if cursor leaves the note)

```rust
    let on_mousemove = Closure::<dyn FnMut(web_sys::MouseEvent)>::new(
        move |ev: web_sys::MouseEvent| {
            // Calculate new position relative to .pdf-area
            set_x_pos.set(px);
            set_y_pos.set(py);
        },
    );
```

**`Closure::<dyn FnMut(...)>::new(...)`** - Creates a Rust closure that JavaScript can call. This is the bridge: Rust creates the function, wraps it so JS can use it as an event listener.

```rust
    let _ = window.add_event_listener_with_callback("mousemove", ...);
```
Adds the mousemove listener to `window` (standard drag pattern).

```rust
    let cleanup = Closure::once(move |_ev| {
        set_dragging.set(false);
        // Update note position in the array
        set_notes.update(|notes| { ... });
        // Remove mousemove listener
        window.remove_event_listener_with_callback("mousemove", &move_ref);
    });
```

**`Closure::once`** - A closure that can only be called once. Perfect for the mouseup handler (you only release once per drag).

```rust
    on_mousemove.forget();
    cleanup.forget();
    cleanup_wrapper.forget();
```

**`.forget()`** - Prevents Rust from freeing these closures. They need to live as long as the event listeners are attached. This leaks memory but is the standard pattern for WASM event listeners. The leak is tiny (a few bytes per drag operation).

### Lines 156-187: Template
```rust
view! {
    <div class="sticky-note"
        class:dragging=move || dragging.get()
        style=move || format!("left:{}px;top:{}px;background-color:{};",
            x_pos.get(), y_pos.get(), color)
    >
        <div class="sticky-note-header" on:mousedown=on_mousedown>
            <span class="sticky-note-drag">{"\u{2630}"}</span>  // ☰ hamburger icon
            <button on:click=on_delete>{"\u{2715}"}</button>    // ✕ close icon
        </div>
        <Show when=move || is_editing.get()
            fallback=move || view! { <div class="sticky-note-content">{display_content}</div> }
        >
            <textarea ... />
        </Show>
    </div>
}
```

**`<Show when=... fallback=...>`** - Leptos's conditional rendering component. Like `{condition ? <A /> : <B />}` in React but more explicit. When `is_editing` is true, shows textarea; otherwise shows static content.

**`style=move || format!(...)`** - Reactive inline style. Position updates during drag cause the style to reactively update, moving the note on screen.
