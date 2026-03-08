# index.html

**What this file is:** The entry point HTML file that Trunk processes during build. It's NOT a normal HTML file - it has special `data-trunk` attributes that tell Trunk what to do.

**React equivalent:** Like `public/index.html` in Create React App, but with build instructions embedded.

## Line-by-Line Breakdown

### Lines 1-5: Standard HTML boilerplate
```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
```
Nothing special here - standard HTML5 document setup.

### Lines 6: Page title
```html
    <title>PDF Viewer</title>
```

### Lines 7-11: CSS imports with `data-trunk`
```html
    <link data-trunk rel="css" href="styles/main.css" />
    <link data-trunk rel="css" href="styles/themes.css" />
    <link data-trunk rel="css" href="styles/toolbar.css" />
    <link data-trunk rel="css" href="styles/notes.css" />
    <link data-trunk rel="css" href="styles/pdf.css" />
```

**Key concept:** `data-trunk` is a special attribute. When Trunk builds the project:
- It reads these CSS files from the `styles/` folder
- Copies them to `dist/` with hashed filenames (e.g., `main-a1b2c3.css`)
- Replaces these lines with normal `<link rel="stylesheet">` tags pointing to the hashed files

This is like how webpack/Vite handles CSS imports - cache busting via filename hashing.

### Lines 12-98: The pdf.js Bridge (JavaScript)
```html
<script type="module">
    import * as pdfjsLib from "https://cdnjs.cloudflare.com/ajax/libs/pdf.js/4.9.155/pdf.min.mjs";
```

This loads **pdf.js** from a CDN. pdf.js is Mozilla's PDF rendering library (it's what Firefox uses to display PDFs).

```javascript
    pdfjsLib.GlobalWorkerOptions.workerSrc =
        "https://cdnjs.cloudflare.com/ajax/libs/pdf.js/4.9.155/pdf.worker.min.mjs";
```
PDF parsing is CPU-heavy, so pdf.js runs it in a Web Worker (background thread). This tells it where to find that worker script.

```javascript
    let currentPdf = null;  // stores the loaded PDF document
    let currentPage = null; // stores the current page being viewed
```

#### `window.pdfBridge` - The JavaScript/Rust Bridge

This is the crucial piece. Since Rust/WASM can't directly use pdf.js (a JavaScript library), we create a `window.pdfBridge` object that Rust can call through FFI (Foreign Function Interface).

Think of it as an API layer:

```
Rust code → calls window.pdfBridge.loadPdfFromData() → runs JavaScript → uses pdf.js
```

**`loadPdfFromUrl(url)`** (line 22-25): Loads a PDF from a URL. Returns the number of pages.

**`loadPdfFromData(data)`** (line 27-30): Loads a PDF from binary data (Uint8Array from file upload). Returns the number of pages.

**`renderPage(pageNum, canvasId, scale)`** (line 32-83): The big one. It:
1. Gets the requested page from the PDF
2. Creates a viewport at the given scale (zoom level)
3. Finds the `<canvas>` element by ID
4. Renders the PDF page onto that canvas
5. Also renders a text layer (invisible text on top) for text selection
6. Returns `{ width, height }` of the rendered page

**`getNumPages()`** (line 85-87): Returns total page count.

**`isLoaded()`** (line 89-91): Returns whether a PDF is currently loaded.

**`computeHash(data)`** (line 93-97): Creates a SHA-256 hash of the PDF binary data. This hash is used as a unique identifier to associate notes with a specific PDF file.

### Line 100: The Rust/WASM Link
```html
    <link data-trunk rel="rust" data-wasm-opt="z" />
```

**This is the magic line.** It tells Trunk:
1. Compile the Rust project (finds `Cargo.toml` automatically)
2. Target: `wasm32-unknown-unknown` (WebAssembly)
3. Run `wasm-bindgen` to generate JS glue code
4. Run `wasm-opt` with optimization level "z" (optimize for smallest size)
5. Inject the resulting `<script>` tags into the HTML

`data-wasm-opt="z"` means "optimize the WASM binary for minimum size" (like `-Os` in C/C++).

### Lines 101-104: Empty body
```html
<body>
</body>
```

The `<body>` is empty because Leptos will mount the app into it at runtime via `mount_to_body(App)` in `main.rs`. Just like React's `<div id="root"></div>` but Leptos uses the entire `<body>`.
