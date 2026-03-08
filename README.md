# PDF Viewer

A web-based PDF viewer with sticky notes, built with **Rust + Leptos** (compiled to WebAssembly) and **pdf.js** for rendering. Notes are persisted via a **Cloudflare Workers** API backed by **D1** (SQLite).

## How It Works (The Big Picture)

If you're coming from React/SolidJS, here's the mental model:

```
React/SolidJS world          Rust/Leptos world
─────────────────            ─────────────────
JSX                    →     view! { } macro (looks like HTML)
useState()             →     signal() → (getter, setter)
props                  →     #[component] function params
useEffect()            →     Effect::new()
fetch()                →     gloo_net::http::Request
npm/bun                →     cargo (Rust package manager)
node_modules/          →     target/ (compiled dependencies)
package.json           →     Cargo.toml
```

### How `index.html` Gets Compiled

This is the key thing that's different from React/SolidJS. Here's the build pipeline:

```
┌─────────────────────────────────────────────────────────────┐
│  1. You run: trunk build (or trunk serve for dev)           │
│                                                             │
│  2. Trunk reads index.html and finds these special tags:    │
│     <link data-trunk rel="css" href="styles/main.css" />    │
│     <link data-trunk rel="rust" data-wasm-opt="z" />        │
│                                                             │
│  3. For the CSS links (data-trunk rel="css"):               │
│     → Copies CSS files into dist/ with hashed filenames     │
│     → Injects <link> tags into the output HTML              │
│                                                             │
│  4. For the Rust link (data-trunk rel="rust"):              │
│     → Runs cargo build --target wasm32-unknown-unknown      │
│     → Compiles ALL your Rust code (src/) into a .wasm file  │
│     → Runs wasm-opt to shrink it (the "z" = smallest size)  │
│     → Runs wasm-bindgen to generate JS glue code            │
│     → Injects <script> + <link rel="modulepreload"> tags    │
│                                                             │
│  5. Output goes to dist/ folder:                            │
│     dist/                                                   │
│       index.html          (processed version)               │
│       pdf-viewer-xxxxx.js (wasm-bindgen glue)               │
│       pdf-viewer-xxxxx_bg.wasm (your compiled Rust)         │
│       main-xxxxx.css      (hashed CSS files)                │
│       ...                                                   │
│                                                             │
│  6. The browser loads index.html → loads the JS glue →      │
│     loads the .wasm file → calls main() in main.rs →        │
│     mounts your App component to <body>                     │
└─────────────────────────────────────────────────────────────┘
```

**In short:** `trunk` is like Vite/webpack for Rust WASM apps. It sees `data-trunk` attributes in your HTML, compiles your Rust to WASM, bundles CSS, and outputs a ready-to-deploy `dist/` folder.

## Project Structure

```
pdf-viewer/
├── index.html                 # Entry HTML (Trunk processes this)
├── Cargo.toml                 # Rust dependencies (like package.json)
├── Trunk.toml                 # Trunk build config (like vite.config.js)
├── wrangler.jsonc             # Cloudflare deployment config
├── package.json               # Just has wrangler as a dependency
│
├── src/                       # Rust frontend code (compiles to WASM)
│   ├── main.rs                # Entry point - mounts App to DOM
│   ├── app.rs                 # Root component - state + file upload
│   ├── models.rs              # Data types (Note, NoteColor)
│   ├── api.rs                 # HTTP client for notes CRUD
│   ├── js_bindings.rs         # Bridge between Rust and JavaScript
│   ├── theme.rs               # Light/Dark theme with localStorage
│   └── components/
│       ├── mod.rs             # Module declarations
│       ├── toolbar.rs         # Top bar: nav, zoom, save, theme
│       ├── pdf_viewer.rs      # PDF canvas + notes overlay
│       ├── sticky_note.rs     # Draggable sticky note widget
│       ├── note_editor.rs     # Modal editor for notes
│       └── notes_sidebar.rs   # Side panel with note list + filters
│
├── styles/                    # CSS files
│   ├── main.css               # Global styles + upload area
│   ├── themes.css             # CSS variables for light/dark
│   ├── toolbar.css            # Toolbar styles
│   ├── notes.css              # Sticky notes + sidebar styles
│   └── pdf.css                # PDF canvas + text layer styles
│
└── worker/                    # Cloudflare Worker (backend API)
    └── src/
        ├── index.ts           # Worker entry + CORS + routing
        ├── db.ts              # D1 database queries
        └── routes/notes.ts    # REST endpoints for notes
```

## Detailed File Documentation

Each file has its own README in the `docs/` folder with line-by-line explanations:

- **Build & Config:** [index.html](docs/index.html.md) | [Cargo.toml](docs/Cargo.toml.md) | [Trunk.toml](docs/Trunk.toml.md)
- **Rust Source:** [main.rs](docs/src/main.rs.md) | [app.rs](docs/src/app.rs.md) | [models.rs](docs/src/models.rs.md) | [api.rs](docs/src/api.rs.md) | [js_bindings.rs](docs/src/js_bindings.rs.md) | [theme.rs](docs/src/theme.rs.md)
- **Components:** [mod.rs](docs/src/components/mod.rs.md) | [toolbar.rs](docs/src/components/toolbar.rs.md) | [pdf_viewer.rs](docs/src/components/pdf_viewer.rs.md) | [sticky_note.rs](docs/src/components/sticky_note.rs.md) | [note_editor.rs](docs/src/components/note_editor.rs.md) | [notes_sidebar.rs](docs/src/components/notes_sidebar.rs.md)
- **Styles:** [main.css](docs/styles/main.css.md) | [themes.css](docs/styles/themes.css.md) | [toolbar.css](docs/styles/toolbar.css.md) | [notes.css](docs/styles/notes.css.md) | [pdf.css](docs/styles/pdf.css.md)
- **Worker API:** [index.ts](docs/worker/src/index.ts.md) | [db.ts](docs/worker/src/db.ts.md) | [notes.ts](docs/worker/src/routes/notes.ts.md)

## Quick Start

```bash
# Install Rust + wasm target
rustup target add wasm32-unknown-unknown

# Install Trunk (the build tool)
cargo install trunk

# Dev server with hot reload
trunk serve

# Production build
trunk build --release

# Deploy to Cloudflare
wrangler deploy
```

## Key Concepts for React/SolidJS Developers

### Signals = State
```rust
// React: const [count, setCount] = useState(0)
// Solid:  const [count, setCount] = createSignal(0)
// Leptos:
let (count, set_count) = signal(0);
```

### Components = Functions with #[component]
```rust
// React: function MyComponent({ name }) { return <div>{name}</div> }
// Leptos:
#[component]
fn MyComponent(name: ReadSignal<String>) -> impl IntoView {
    view! { <div>{move || name.get()}</div> }
}
```

### Why `move ||` everywhere?
In Leptos (like SolidJS), reactive values need closures to track dependencies. `move || value.get()` creates a closure that re-runs when `value` changes. This is similar to how SolidJS uses `() => value()` - same concept, Rust syntax.

### Ownership & Cloning
You'll see `.clone()` a lot. Rust doesn't let you use the same variable in two places (ownership). When you need a value in two closures, you clone it first. Think of it like creating a copy so each closure gets its own.
