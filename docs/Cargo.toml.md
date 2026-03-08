# Cargo.toml

**What this file is:** The Rust project manifest - equivalent to `package.json` in Node.js. It lists the project name, version, and all dependencies.

## Line-by-Line Breakdown

### Lines 1-4: Package metadata
```toml
[package]
name = "pdf-viewer"
version = "0.1.0"
edition = "2024"
```
- `name`: The crate (package) name
- `edition`: Rust edition year. Rust releases new editions every 3 years with syntax improvements. `2024` is the latest.

### Lines 6-7: Leptos (UI Framework)
```toml
[dependencies]
leptos = { version = "0.7", features = ["csr"] }
```
**Leptos** is the UI framework - like React or SolidJS but for Rust.
- `features = ["csr"]` means **Client-Side Rendering**. Leptos also supports SSR (server-side rendering) but this project runs entirely in the browser.
- Leptos is very similar to SolidJS - fine-grained reactivity, signals, not a virtual DOM.

### Lines 8-9: WASM Bindings
```toml
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
```
- `wasm-bindgen`: The bridge between Rust and JavaScript. Lets Rust call JS functions and vice versa.
- `wasm-bindgen-futures`: Converts JS Promises into Rust async/await. When Rust calls an `async` JS function, this makes it work with Rust's `async`.

### Lines 10-30: web-sys (Browser APIs)
```toml
web-sys = { version = "0.3", features = [
    "Window", "Document", "Element", "HtmlElement",
    "HtmlCanvasElement", "HtmlInputElement", ...
] }
```
`web-sys` provides Rust bindings to browser Web APIs. In JavaScript you just use `document.getElementById()` freely, but in Rust you need to explicitly opt-in to each API you use via `features`.

Each feature corresponds to a Web API interface:
- `Window` → `window` object
- `Document` → `document` object
- `HtmlCanvasElement` → `<canvas>` element methods
- `MouseEvent` → mouse event properties (clientX, clientY, etc.)
- `FileReader` → FileReader API for reading uploaded files
- `Blob`, `File`, `FileList` → File upload handling

### Line 31: js-sys
```toml
js-sys = "0.3"
```
Rust bindings to JavaScript built-in objects like `Array`, `Date`, `Uint8Array`, `ArrayBuffer`, `Function`, `Promise`. Used when you need to work with JS types directly.

### Lines 32-34: Serialization
```toml
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde-wasm-bindgen = "0.6"
```
- `serde`: Rust's serialization framework. `derive` lets you add `#[derive(Serialize, Deserialize)]` to structs.
- `serde_json`: JSON serialization/deserialization.
- `serde-wasm-bindgen`: Converts between Rust types and `JsValue` (JavaScript values).

Think of serde like `JSON.parse()` / `JSON.stringify()` but type-safe. When you `#[derive(Serialize)]` on a struct, Rust auto-generates the conversion code.

### Lines 35-36: HTTP & Storage
```toml
gloo-net = { version = "0.6", features = ["http"] }
gloo-storage = "0.3"
```
- `gloo-net`: HTTP client for WASM. Like `fetch()` but with a Rust API. Used in `api.rs` for the notes CRUD.
- `gloo-storage`: `localStorage` wrapper. Used in `theme.rs` to persist the theme choice.

### Line 37: Error Handling
```toml
console_error_panic_hook = "0.1"
```
When Rust panics (crashes), by default WASM just shows a cryptic error. This crate hooks into the panic handler and prints a useful stack trace to the browser console. Essential for debugging.

### Line 38: UUID Generation
```toml
uuid = { version = "1", features = ["v4", "js"] }
```
Generates unique IDs for notes. `v4` = random UUIDs, `js` = use the browser's `crypto.getRandomValues()` as the random source (since WASM doesn't have OS-level randomness).
