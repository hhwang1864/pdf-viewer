# src/components/pdf_viewer.rs

**What this file is:** The main PDF display area. Renders the PDF on a canvas, handles note placement clicks, and overlays sticky notes on top.

**React equivalent:**
```jsx
function PdfViewer({ currentPage, zoom, notes, setNotes, ... }) {
  const [canvasSize, setCanvasSize] = useState({ width: 0, height: 0 });

  useEffect(() => {
    // Re-render PDF when page or zoom changes
    renderPage(currentPage, 'pdf-canvas', zoom).then(setCanvasSize);
  }, [currentPage, zoom]);

  return (
    <div className="pdf-area" onClick={handleNoteClick}>
      <canvas id="pdf-canvas" />
      <div className="notes-overlay">
        {pageNotes.map(note => <StickyNote key={note.id} ... />)}
      </div>
    </div>
  );
}
```

## Line-by-Line Breakdown

### Lines 1-6: Imports
```rust
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use crate::js_bindings;
use crate::models::{Note, NoteColor};
```

### Lines 8-20: Component with props
```rust
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
    note_mode: ReadSignal<bool>,
    set_note_mode: WriteSignal<bool>,
) -> impl IntoView {
```

### Lines 21-22: Canvas dimension state
```rust
let (canvas_width, set_canvas_width) = signal(0.0f64);
let (canvas_height, set_canvas_height) = signal(0.0f64);
```
Tracks the rendered PDF dimensions to size the wrapper div.

### Lines 25-44: Reactive PDF rendering (Effect)
```rust
Effect::new(move || {
    let page = current_page.get();
    let scale = zoom.get();
    if page > 0 && js_bindings::is_loaded() {
        spawn_local(async move {
            match js_bindings::render_page(page, "pdf-canvas", scale).await {
                Ok(result) => {
                    if let Ok(dims) = serde_wasm_bindgen::from_value::<PageDimensions>(result) {
                        set_canvas_width.set(dims.width);
                        set_canvas_height.set(dims.height);
                    }
                }
                Err(e) => { web_sys::console::error_1(&e); }
            }
        });
    }
});
```

**`Effect::new(move || { ... })`** - Like `useEffect` in React or `createEffect` in SolidJS. Runs whenever any signal read inside it changes.

Here, it reads `current_page.get()` and `zoom.get()`, so it re-runs whenever either changes. Every time, it:
1. Calls the JS bridge to render the page at the given zoom
2. Parses the returned `{ width, height }` using `serde_wasm_bindgen::from_value`
3. Updates the canvas dimension signals

**Key difference from React:** No dependency array needed. Leptos (like SolidJS) automatically tracks which signals are read inside the effect.

**`serde_wasm_bindgen::from_value::<PageDimensions>(result)`** - Converts a `JsValue` (the `{ width, height }` object from JS) into a Rust struct. The `::<PageDimensions>` tells it what type to deserialize into.

### Lines 47-90: Note placement handler
```rust
let on_area_click = move |ev: web_sys::MouseEvent| {
    if !note_mode.get_untracked() { return; }
```
Only places notes when note mode is active (the pin button is toggled on).

```rust
    let area = document.query_selector(".pdf-area").ok().flatten().unwrap();
    let area_rect = area.get_bounding_client_rect();
    let area_el: web_sys::HtmlElement = area.dyn_into().unwrap();
    let x = ev.client_x() as f64 - area_rect.left() + area_el.scroll_left() as f64;
    let y = ev.client_y() as f64 - area_rect.top() + area_el.scroll_top() as f64;
```
Calculates the click position relative to the PDF area, accounting for scroll offset. This is the same `getBoundingClientRect()` + scroll math you'd do in JavaScript.

- `as f64` - Type cast. `client_x()` returns `i32`, we need `f64` for the position math.
- `.dyn_into().unwrap()` - Dynamic type cast. Converts a generic `Element` into `HtmlElement`.

```rust
    let new_note = Note {
        id: uuid::Uuid::new_v4().to_string(),
        ...
    };
    set_notes.update(|n| n.push(new_note));
    set_editing_note.set(Some(note_id));
    set_note_mode.set(false);  // Turn off note mode after placing
```
Creates a new note at the click position, adds it to the notes array, opens the editor, and turns off note placement mode.

### Lines 92-99: Page notes filter
```rust
let page_notes = move || {
    let page = current_page.get();
    notes.get()
        .into_iter()
        .filter(move |n| n.page_number == page)
        .collect::<Vec<_>>()
};
```
A derived value (like `useMemo` in React or a computed value in SolidJS). Returns only notes for the current page.

### Lines 101-124: Template
```rust
view! {
    <div class="pdf-area" class:note-mode=move || note_mode.get() on:click=on_area_click>
        <div class="pdf-canvas-wrapper" style=move || {
            format!("width:{}px;height:{}px", canvas_width.get(), canvas_height.get())
        }>
            <canvas id="pdf-canvas" />
            <div id="text-layer" class="text-layer" />
            <div class="notes-overlay">
                {move || page_notes().into_iter().map(|note| {
                    ...
                    view! { <StickyNote note=note.clone() ... /> }
                }).collect_view()}
            </div>
        </div>
    </div>
}
```

The layout structure:
```
.pdf-area (scrollable container, click handler for note placement)
  └── .pdf-canvas-wrapper (sized to PDF dimensions)
      ├── canvas#pdf-canvas (PDF rendered here by pdf.js)
      ├── #text-layer (invisible text for selection)
      └── .notes-overlay (positioned absolute, holds sticky notes)
          └── StickyNote (for each note on current page)
```

### Lines 127-131: PageDimensions struct
```rust
#[derive(serde::Deserialize)]
struct PageDimensions {
    width: f64,
    height: f64,
}
```
A small struct just for deserializing the `{ width, height }` returned by `renderPage()` in JavaScript. `#[derive(serde::Deserialize)]` auto-generates the JSON parsing code.
