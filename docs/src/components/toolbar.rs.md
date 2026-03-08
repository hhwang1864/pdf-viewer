# src/components/toolbar.rs

**What this file is:** The toolbar component at the top of the app. Contains page navigation, zoom controls, save button, theme toggle, fullscreen, and sidebar toggle.

**React equivalent:**
```jsx
function Toolbar({ currentPage, setCurrentPage, zoom, setZoom, theme, setTheme, ... }) {
  return (
    <div className="toolbar">
      <button onClick={() => setCurrentPage(p => p - 1)}>Prev</button>
      <input type="range" ... />
      {/* zoom controls, theme toggle, etc. */}
    </div>
  );
}
```

## Line-by-Line Breakdown

### Lines 1-6: Imports
```rust
use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::api;
use crate::models::Note;
use crate::theme::{Theme, apply_theme};
```

### Lines 8-22: Component with props
```rust
#[component]
pub fn Toolbar(
    current_page: ReadSignal<u32>,
    total_pages: ReadSignal<u32>,
    zoom: ReadSignal<f64>,
    theme: ReadSignal<Theme>,
    notes: ReadSignal<Vec<Note>>,
    set_current_page: WriteSignal<u32>,
    set_zoom: WriteSignal<f64>,
    set_theme: WriteSignal<Theme>,
    sidebar_open: ReadSignal<bool>,
    set_sidebar_open: WriteSignal<bool>,
    note_mode: ReadSignal<bool>,
    set_note_mode: WriteSignal<bool>,
) -> impl IntoView {
```

**Props in Leptos** are just function parameters. The `#[component]` macro turns them into proper props.

**`ReadSignal<u32>`** - Can only read the value (like passing just the getter)
**`WriteSignal<u32>`** - Can only write/update the value (like passing just the setter)

This is a nice pattern: the parent controls which components can read vs write each piece of state.

### Lines 23-24: Local state
```rust
let (saving, set_saving) = signal(false);
let (save_status, set_save_status) = signal(String::new());
```
Local signals only used within this component. Not passed as props.

### Lines 26-41: Page navigation
```rust
let on_prev = move |_| {
    set_current_page.update(|p| {
        if *p > 1 { *p -= 1; }
    });
};
```

- `move |_|` - Closure that ignores its argument (`_` = don't care about the event)
- `.update(|p| ...)` - Takes a mutable reference to the current value. `*p` dereferences it. This is like `setCurrentPage(prev => prev > 1 ? prev - 1 : prev)` in React.
- `*p -= 1` - Decrement the dereferenced value. The `*` is needed because `p` is a reference (`&mut u32`).

### Lines 43-58: Input handlers
```rust
let on_page_input = move |ev: web_sys::Event| {
    let target = event_target::<web_sys::HtmlInputElement>(&ev);
    if let Ok(val) = target.value().parse::<u32>() {
```

- `event_target::<web_sys::HtmlInputElement>(&ev)` - Leptos helper to get the typed event target. Like `ev.target as HTMLInputElement` in TypeScript.
- `target.value().parse::<u32>()` - Get the input value (String) and try to parse it as a u32. Returns `Result<u32, ParseError>`.
- `if let Ok(val)` - If parsing succeeded, use the value. If user typed "abc", this silently ignores it.

### Lines 67-81: Zoom controls
```rust
let zoom_in = move |_| {
    let current = zoom.get();
    let levels = [0.5, 0.75, 1.0, 1.25, 1.5, 2.0];
    if let Some(next) = levels.iter().find(|&&l| l > current) {
        set_zoom.set(*next);
    }
};
```

Finds the next zoom level greater than the current one. `levels.iter().find()` is like `levels.find()` in JavaScript. The `**&&l**` is double-dereferencing (the iterator yields references, and `find` gives a reference to that reference).

### Lines 83-87: Theme toggle
```rust
let toggle_theme = move |_| {
    let new_theme = theme.get().toggle();
    set_theme.set(new_theme);
    apply_theme(new_theme);
};
```
Gets current theme, toggles it, updates the signal, and applies to DOM.

### Lines 89-99: Fullscreen toggle
```rust
let toggle_fullscreen = move |_| {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if document.fullscreen_element().is_some() {
                let _ = document.exit_fullscreen();
            } else if let Some(el) = document.document_element() {
                let _ = el.request_fullscreen();
            }
        }
    }
};
```
Uses the Fullscreen API. All the `if let Some` are null checks (Rust requires explicit handling of possibly-null values).

### Lines 109-132: Save handler
```rust
let on_save = move |_| {
    let all_notes = notes.get_untracked();
    ...
    spawn_local(async move {
        let mut errors = 0;
        for note in &all_notes {
            if let Err(_) = api::create_note(note).await {
                errors += 1;
            }
        }
```

- `get_untracked()` - Read the signal value WITHOUT creating a reactive dependency. Used in event handlers where you want the current value but don't want re-renders.
- `spawn_local(async move { ... })` - Start an async task. Each note is saved sequentially with `for ... in`.
- `if let Err(_)` - Check if the API call failed, count errors.

### Lines 134-144: Zoom dropdown options
```rust
let zoom_levels: Vec<f64> = vec![0.5, 0.75, 1.0, 1.25, 1.5, 2.0];
let zoom_options = zoom_levels
    .iter()
    .map(|&z| { ... })
    .collect_view();
```

- `.iter().map().collect_view()` - Iterate, transform each into a `view!`, and collect into a renderable list. Like `.map()` in JSX.
- `collect_view()` is a Leptos method that collects an iterator of views into a single view fragment.

### Lines 146-227: The template
```rust
view! {
    <div class="toolbar">
        <button class="toolbar-btn" on:click=on_prev title="Previous Page">
            {"\u{25C0}"}  // Unicode: ◀
        </button>
```

- `{"\u{25C0}"}` - Unicode character. Rust uses `\u{XXXX}` syntax (JS uses `\uXXXX`).
- `prop:value=move || current_page.get().to_string()` - Reactive property binding. `prop:` prefix sets the DOM property (not the HTML attribute). This matters for `<input>` elements where the `value` property and attribute diverge.
- `class:active=move || note_mode.get()` - Conditional CSS class. Adds "active" class when `note_mode` is true. Like `className={noteMode ? 'active' : ''}` in React.
