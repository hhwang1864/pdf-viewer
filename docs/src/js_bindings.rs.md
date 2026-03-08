# src/js_bindings.rs

**What this file is:** The Foreign Function Interface (FFI) between Rust and JavaScript. It declares Rust function signatures that map to the `window.pdfBridge` object defined in `index.html`.

This is the most unique file if you're coming from JavaScript - there's no equivalent concept in React/SolidJS because you're already in JavaScript. Here, Rust needs to explicitly declare "these JavaScript functions exist and here's their type signature."

**Mental model:** Think of it as writing TypeScript type declarations (`.d.ts` files) for JavaScript functions, but for Rust.

## Line-by-Line Breakdown

### Line 1: Import
```rust
use wasm_bindgen::prelude::*;
```
Imports `wasm_bindgen` macros and types. This crate is what makes Rust-JavaScript interop possible.

### Lines 3-4: FFI block
```rust
#[wasm_bindgen]
extern "C" {
```

**`#[wasm_bindgen]`** - Macro that generates the glue code between Rust and JS.

**`extern "C"`** - Declares that the following functions are implemented externally (in JavaScript, not Rust). The `"C"` is the calling convention. This block is saying "these functions exist somewhere outside Rust - trust me."

### Lines 5-6: loadPdfFromUrl
```rust
    #[wasm_bindgen(js_namespace = ["window", "pdfBridge"], js_name = loadPdfFromUrl, catch)]
    pub async fn load_pdf_from_url(url: &str) -> Result<JsValue, JsValue>;
```

This single declaration does a lot:

- **`js_namespace = ["window", "pdfBridge"]`** → The function lives at `window.pdfBridge` in JavaScript
- **`js_name = loadPdfFromUrl`** → The JavaScript function is called `loadPdfFromUrl` (camelCase). The Rust function uses `load_pdf_from_url` (snake_case) - Rust convention.
- **`catch`** → If the JavaScript function throws an error, catch it and return `Err(JsValue)` instead of panicking. Without `catch`, a JS exception would crash the WASM module.
- **`pub async fn`** → The JS function returns a Promise, so Rust sees it as an async function
- **`-> Result<JsValue, JsValue>`** → Returns either Ok(value) or Err(error). `JsValue` is a generic JavaScript value (could be a number, string, object, etc.)

**What happens at runtime:**
```
Rust: load_pdf_from_url("https://example.com/doc.pdf").await
  ↓ wasm_bindgen glue code
JS: window.pdfBridge.loadPdfFromUrl("https://example.com/doc.pdf")
  ↓ returns Promise
Rust: gets Result<JsValue, JsValue>
```

### Lines 8-9: loadPdfFromData
```rust
    #[wasm_bindgen(js_namespace = ["window", "pdfBridge"], js_name = loadPdfFromData, catch)]
    pub async fn load_pdf_from_data(data: &js_sys::Uint8Array) -> Result<JsValue, JsValue>;
```
Same pattern. Takes a `Uint8Array` reference (the raw PDF bytes) and returns the page count.

### Lines 11-16: renderPage
```rust
    pub async fn render_page(
        page_num: u32,
        canvas_id: &str,
        scale: f64,
    ) -> Result<JsValue, JsValue>;
```
Renders a PDF page to a canvas. The `JsValue` returned is `{ width, height }` which gets deserialized in `pdf_viewer.rs`.

### Lines 18-19: getNumPages (synchronous)
```rust
    #[wasm_bindgen(js_namespace = ["window", "pdfBridge"], js_name = getNumPages)]
    pub fn get_num_pages() -> u32;
```
No `async`, no `catch` - this is a simple synchronous function that returns a number. It doesn't need error handling because it just reads `currentPdf.numPages`.

### Lines 21-22: isLoaded (synchronous)
```rust
    pub fn is_loaded() -> bool;
```
Returns whether a PDF is loaded. Simple boolean check.

### Lines 24-25: computeHash
```rust
    pub async fn compute_hash(data: &js_sys::Uint8Array) -> Result<JsValue, JsValue>;
```
Calls `crypto.subtle.digest("SHA-256", data)` on the JavaScript side and returns the hex hash string.

## How the Bridge Works (Full Picture)

```
index.html defines:                    js_bindings.rs declares:
─────────────────                      ────────────────────────
window.pdfBridge = {                   extern "C" {
  loadPdfFromUrl(url) {...}     ←→       pub async fn load_pdf_from_url(url)
  loadPdfFromData(data) {...}   ←→       pub async fn load_pdf_from_data(data)
  renderPage(num, id, s) {...}  ←→       pub async fn render_page(num, id, s)
  getNumPages() {...}           ←→       pub fn get_num_pages()
  isLoaded() {...}              ←→       pub fn is_loaded()
  computeHash(data) {...}       ←→       pub async fn compute_hash(data)
}                                      }
```

The `wasm_bindgen` macro generates JavaScript glue code that connects these two sides at compile time.
