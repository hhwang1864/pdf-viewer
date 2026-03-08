# src/app.rs

**What this file is:** The root component of the application. Manages all top-level state and renders the main layout. Like `App.tsx` in React.

**React equivalent concept:**
```jsx
function App() {
  const [currentPage, setCurrentPage] = useState(1);
  const [notes, setNotes] = useState([]);
  // ... more state
  return (
    <div className="app">
      <Toolbar ... />
      <PdfViewer ... />
      <Sidebar ... />
    </div>
  );
}
```

## Line-by-Line Breakdown

### Lines 1-11: Imports
```rust
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
```

- `leptos::prelude::*` → All Leptos essentials (signals, view macro, component macro, etc.)
- `wasm_bindgen::JsCast` → Trait for casting between JavaScript types. Like TypeScript type assertions but at runtime. When you get a `JsValue` and know it's an `HtmlInputElement`, you use `.dyn_into::<HtmlInputElement>()` to cast it.
- `spawn_local` → Spawns an async task on the browser's event loop. Since Rust's `.await` doesn't work directly in event handlers, you wrap async code in `spawn_local(async move { ... })`. Like starting a Promise without awaiting it at the call site.

```rust
use crate::components::notes_sidebar::NotesSidebar;
use crate::components::pdf_viewer::PdfViewer;
use crate::components::toolbar::Toolbar;
use crate::api;
use crate::js_bindings;
use crate::models::{Note, NoteColor};
use crate::theme::{apply_theme, load_theme};
```

`crate::` means "from the root of this project". These import components, the API module, JS bindings, and data types.

### Lines 13-14: Component definition
```rust
#[component]
pub fn App() -> impl IntoView {
```

**`#[component]`** is a Leptos macro that transforms this function into a proper component. It:
- Generates the component registration code
- Enables the `view!` macro to work
- Similar to how React class components needed `extends React.Component` but now it's just a decorator-like attribute

**`pub fn App()`** - A public function named `App`. Components in Leptos are just functions.

**`-> impl IntoView`** - The return type. `impl IntoView` means "returns something that can be rendered as HTML". In React, you return `JSX.Element`. In Leptos, you return `impl IntoView`.

### Lines 15-28: State signals
```rust
let initial_theme = load_theme();
apply_theme(initial_theme);

let (current_page, set_current_page) = signal(1u32);
let (total_pages, set_total_pages) = signal(0u32);
let (zoom, set_zoom) = signal(1.0f64);
let (theme, set_theme) = signal(initial_theme);
let (notes, set_notes) = signal(Vec::<Note>::new());
let (pdf_hash, set_pdf_hash) = signal(String::new());
let (editing_note, set_editing_note) = signal::<Option<String>>(None);
let (sidebar_open, set_sidebar_open) = signal(false);
let (selected_color, set_selected_color) = signal(NoteColor::Yellow);
let (pdf_loaded, set_pdf_loaded) = signal(false);
let (note_mode, set_note_mode) = signal(false);
```

Each `signal()` creates a reactive state pair - exactly like `createSignal()` in SolidJS or `useState()` in React.

- `signal(1u32)` → initial value `1`, type is `u32` (unsigned 32-bit integer). The `u32` suffix tells Rust the type.
- `signal(1.0f64)` → `1.0` as a 64-bit float (like JavaScript's `number`)
- `signal(Vec::<Note>::new())` → empty vector (array) of Notes. `Vec<Note>` = `Note[]` in TypeScript.
- `signal::<Option<String>>(None)` → `Option<String>` means "maybe a String, maybe nothing". `None` = `null`. Like `string | null` in TypeScript.

**Key difference from React:** These signals are "fine-grained reactive" (like SolidJS). Only the specific DOM nodes that read a signal update when it changes - NOT the whole component. The `App` function runs once, not on every render.

### Lines 30-92: File upload handler (`on_file_select`)
```rust
let on_file_select = move |ev: web_sys::Event| {
```
This is a closure (anonymous function) that handles file input changes. `move` means it takes ownership of captured variables.

**The flow:**
1. Get the `<input>` element from the event target
2. Read the selected file using `FileReader` API (same as JavaScript's `FileReader`)
3. When file is loaded (`onload`):
   - Compute SHA-256 hash of the file (via `js_bindings::compute_hash`)
   - Load the PDF via `js_bindings::load_pdf_from_data`
   - Fetch any saved notes from the API for this PDF hash
4. Set all the state signals

**Why `spawn_local`?** (line 46) - The hash computation and PDF loading are `async` (they return Promises on the JS side). Rust closures used as event handlers can't be `async` directly, so `spawn_local` creates an async context.

**Why `onload.forget()`?** (line 89) - In Rust, when a `Closure` goes out of scope, it gets freed. But we need this closure to live as long as the FileReader needs it. `.forget()` tells Rust "don't free this memory ever". It's a small memory leak, but acceptable for event handlers that exist for the app's lifetime.

### Lines 94-123: URL submit handler (`on_url_submit`)
Similar to file upload but simpler - takes a URL string, loads the PDF directly via `js_bindings::load_pdf_from_url`, then fetches saved notes.

**`ev.prevent_default()`** - Same as JavaScript - prevents the form from submitting and reloading the page.

### Lines 130-211: The view! macro (Template)
```rust
view! {
    <div class="app" data-theme=move || theme.get().as_str() on:click=on_app_click>
        <Toolbar
            current_page=current_page
            ...
        />
        ...
    </div>
}
```

The `view!` macro is Leptos's JSX equivalent. It looks like HTML but:

- **`on:click=handler`** → event binding (React: `onClick={handler}`)
- **`move || theme.get().as_str()`** → reactive attribute. The `move ||` closure re-runs when `theme` changes, updating the attribute. Like `() => theme()` in SolidJS.
- **`<Toolbar current_page=current_page />`** → Component with props. Each prop is a signal passed down.
- **`{move || if condition { view!{...}.into_any() } else { view!{...}.into_any() }}`** → conditional rendering. `.into_any()` is needed because Rust requires both branches to return the same type.

### Props Passing Pattern
```rust
<Toolbar
    current_page=current_page      // ReadSignal<u32> - child can read
    set_current_page=set_current_page  // WriteSignal<u32> - child can write
/>
```

This is like React's pattern of passing both state and setState down as props. The child component receives the signal getter (to read) and setter (to write).
