# WebAssembly (WASM) — What It Is, How It Works, and Why Not Just Use JavaScript

## What Is WebAssembly?

WebAssembly (WASM) is a **binary instruction format** that runs in the browser alongside JavaScript. Think of it as a second language the browser speaks — JavaScript is the first, WASM is the second.

```
Traditional web app:
  Browser → runs JavaScript

WASM web app:
  Browser → runs JavaScript AND WebAssembly side by side
```

WASM is **not** a replacement for JavaScript. It's a **compilation target** — you write code in another language (Rust, C++, Go, etc.), and a compiler turns it into `.wasm` bytecode that the browser can execute.

```
You write Rust code (.rs files)
       ↓
Rust compiler (rustc) compiles to .wasm binary
       ↓
Browser downloads and runs the .wasm file
```

---

## How WASM Actually Runs in the Browser

### Step 1: The .wasm file is just bytecode

When you run `trunk build`, the Rust compiler produces a `.wasm` file. This is a compact binary — not human-readable, not JavaScript. It's closer to machine code than to a scripting language.

```
your Rust code → compiler → pdf-viewer_bg.wasm (binary bytecode)
```

### Step 2: JavaScript loads and instantiates the WASM module

The browser can't just run a `.wasm` file on its own. JavaScript must load it:

```javascript
// This is what wasm-bindgen generates (simplified):
const wasmModule = await WebAssembly.instantiateStreaming(
  fetch('pdf-viewer_bg.wasm'),
  importObject  // JS functions that WASM can call
);

// Now call the Rust main() function
wasmModule.instance.exports.main();
```

Trunk and wasm-bindgen generate this glue code automatically — you never write it yourself.

### Step 3: WASM runs in the same thread as JavaScript

WASM code runs on the **main thread**, just like JavaScript. It shares the same event loop. When your Rust code calls `set_current_page.set(5)`, it updates the DOM through JavaScript interop — the DOM is still a JavaScript API.

```
┌─────────────────────────────────────────────┐
│                 Browser                      │
│                                              │
│  ┌──────────────┐    ┌───────────────────┐   │
│  │  JavaScript   │◄──►│   WebAssembly     │   │
│  │  Engine (V8)  │    │   Runtime         │   │
│  │               │    │                   │   │
│  │  - pdf.js     │    │  - Your Rust code │   │
│  │  - DOM APIs   │    │  - Leptos         │   │
│  │  - Events     │    │  - State mgmt     │   │
│  │  - glue code  │    │  - Component logic│   │
│  └──────┬───────┘    └───────────────────┘   │
│         │                                     │
│         ▼                                     │
│  ┌──────────────┐                             │
│  │     DOM       │  (only JS can touch this)  │
│  └──────────────┘                             │
└─────────────────────────────────────────────┘
```

**Key point:** WASM cannot directly access the DOM. Every DOM operation goes through JavaScript. When Leptos updates the UI, it calls JavaScript DOM APIs under the hood via `web-sys` and `wasm-bindgen`.

### Step 4: Communication between WASM and JS

This is where `wasm-bindgen` comes in. It creates a bridge:

```
Rust side (WASM)                    JavaScript side
──────────────────                  ────────────────
js_bindings::render_page(1, ...)
       ↓
  wasm-bindgen glue code
       ↓
                          →    window.pdfBridge.renderPage(1, ...)
                          →    pdf.js renders to <canvas>
                          ←    returns { width: 800, height: 600 }
       ↓
  wasm-bindgen converts JsValue to Rust struct
       ↓
Rust gets PageDimensions { width: 800.0, height: 600.0 }
```

Every time Rust calls a JavaScript function (or vice versa), there's a small overhead for crossing this bridge — converting types between the two worlds.

---

## How This Project Uses WASM

In this PDF viewer, the split is:

| Layer | Technology | Why |
|-------|-----------|-----|
| **PDF rendering** | JavaScript (pdf.js) | pdf.js is a mature JS library, no Rust equivalent |
| **UI framework** | Rust (Leptos → WASM) | Leptos provides React-like components in Rust |
| **State management** | Rust (Leptos signals) | Fine-grained reactivity, type-safe |
| **DOM manipulation** | Rust via web-sys → JS | Leptos calls DOM APIs through wasm-bindgen |
| **API calls** | Rust (gloo-net → fetch) | HTTP requests from WASM via browser's fetch API |
| **Backend** | TypeScript (Cloudflare Worker) | Runs on server, not WASM |

The `index.html` file is where these two worlds meet. The `<script>` block defines JavaScript functions (`window.pdfBridge`), and `js_bindings.rs` declares them so Rust can call them.

---

## WASM vs JavaScript: The Real Differences

### 1. Performance

**WASM is faster for computation, not for DOM updates.**

```
CPU-heavy tasks (parsing, crypto, image processing, physics):
  WASM: ████████████████████ ~1.2x native speed
  JS:   ████████████         ~3-10x slower than native

DOM updates (adding elements, changing text, event handling):
  WASM: ████████████████     must go through JS bridge
  JS:   ████████████████████ direct access, no bridge overhead

Startup time:
  WASM: ████████████         download .wasm + compile
  JS:   ████████████████████ parse and JIT compile (faster for small apps)
```

**Why WASM is faster for computation:**
- WASM bytecode is pre-compiled — the browser just needs to translate it to machine code, not parse and optimize source code
- Static typing means no runtime type checks (JS engine spends time guessing types)
- Predictable memory layout (no garbage collector pauses)
- Integer arithmetic is native (JS converts everything to float64)

**Why WASM isn't always faster in practice:**
- DOM access goes through JavaScript anyway (there's overhead crossing the bridge)
- Small apps don't have enough computation to benefit
- The `.wasm` file is an extra download (though it's typically smaller than equivalent JS)

### 2. Type Safety

This is the biggest practical difference for developers.

**JavaScript/TypeScript:**
```typescript
// TypeScript catches some bugs at compile time, but:
const page: number = "hello" as any;  // TypeScript allows this escape hatch
const notes: Note[] = JSON.parse(response);  // runtime type is unknown
// If the API returns unexpected data, you get runtime errors
```

**Rust:**
```rust
// Rust catches these at compile time — the program won't build:
let page: u32 = "hello";  // ERROR: expected u32, found &str
let notes: Vec<Note> = serde_json::from_str(&response)?;  // ERROR if structure doesn't match
// If types don't match, the compiler tells you before the code ever runs
```

Rust's compiler is extremely strict. If your code compiles, entire categories of bugs are impossible:
- No null pointer exceptions (`Option<T>` forces you to handle the `None` case)
- No undefined is not a function
- No accessing properties that don't exist
- No type coercion surprises (`"5" + 3 = "53"` can't happen)
- No data races in concurrent code

### 3. Memory Management

**JavaScript:** Garbage collected. The JS engine periodically scans memory, finds objects no longer referenced, and frees them. You don't think about memory, but GC pauses can cause stutters.

```javascript
// JS: just create objects, GC handles cleanup
function processNotes(notes) {
  const filtered = notes.filter(n => n.page === 1);
  const mapped = filtered.map(n => n.content);
  return mapped;
  // filtered and mapped are freed "eventually" by GC
}
```

**Rust (WASM):** No garbage collector. Memory is freed deterministically when variables go out of scope. This is why you see ownership and borrowing rules:

```rust
// Rust: memory is freed at the closing brace
fn process_notes(notes: Vec<Note>) -> Vec<String> {
    let filtered: Vec<&Note> = notes.iter()
        .filter(|n| n.page_number == 1)
        .collect();  // allocated here
    let mapped: Vec<String> = filtered.iter()
        .map(|n| n.content.clone())
        .collect();  // allocated here
    mapped
    // `filtered` is freed here (end of scope)
    // `notes` is freed here (end of scope)
    // `mapped` is returned to caller (ownership transferred)
}
```

**Why this matters for WASM:** No GC means no GC pauses. The app's memory usage is predictable. But it also means you (the programmer) must satisfy the borrow checker — that's why you see `.clone()` everywhere and why closures need `move`.

### 4. Bundle Size

```
Typical React app (create-react-app):
  JS bundle: 150-500 KB (gzipped)

Typical Leptos WASM app:
  .wasm file: 100-400 KB (gzipped)
  JS glue:    ~5 KB

This project specifically:
  .wasm file: ~200-300 KB (with wasm-opt "z" optimization)
  + pdf.js from CDN: ~400 KB (not in your bundle)
```

WASM binaries are compact because they're bytecode, not text. But there's additional overhead from Rust's standard library being compiled in.

### 5. Developer Experience

**JavaScript:**
```
+ Write code → see result instantly (hot reload in ms)
+ Huge ecosystem (npm has 2M+ packages)
+ Every web developer knows it
+ Direct DOM access, no bridge overhead
+ Easy debugging in browser DevTools
- Runtime errors (production crashes)
- "undefined is not a function"
- Implicit type coercion bugs
```

**Rust + WASM:**
```
+ If it compiles, it (usually) works correctly
+ No runtime type errors, no null exceptions
+ Better performance for heavy computation
+ Fearless refactoring (compiler catches breakage)
- Slower compile times (30s-2min for full build)
- Smaller ecosystem for web
- Steeper learning curve (ownership, lifetimes, borrow checker)
- Debugging is harder (WASM stack traces are less readable)
- DOM access has bridge overhead
- Need .clone() everywhere for closures
```

### 6. When to Use WASM vs JavaScript

**Use WASM (Rust/C++/Go) when:**
- You need heavy computation (image/video processing, crypto, physics engines, data parsing)
- You have existing non-JS code to port to the web (games, CAD software, codecs)
- You want compile-time safety guarantees for a large, complex app
- You're building performance-critical tools (IDE, database query engine)
- You want to share code between web and native (same Rust code, different targets)

**Use JavaScript when:**
- You're building a typical web app (CRUD, dashboards, forms)
- You need fast iteration speed and hot reload
- Your team knows JS but not Rust/C++
- DOM-heavy interactions (animations, drag-and-drop) are the bottleneck
- You need the npm ecosystem

**This project uses both:** Rust/WASM for the app framework (Leptos components, state management, type safety) and JavaScript for what it's best at (pdf.js rendering, browser APIs).

---

## The Build Pipeline in Detail

Here's exactly what happens when you run `trunk build`:

```
1. Trunk reads Trunk.toml → finds target: "index.html"

2. Trunk reads index.html → finds two types of data-trunk links:
   a. <link data-trunk rel="css" ...>  → CSS files to bundle
   b. <link data-trunk rel="rust" ...> → Rust project to compile

3. CSS Processing:
   styles/main.css    →  dist/main-a1b2c3d4.css
   styles/themes.css  →  dist/themes-e5f6g7h8.css
   (filenames get content hashes for cache busting)

4. Rust Compilation (the big step):
   a. cargo build --target wasm32-unknown-unknown --release
      - Compiles ALL .rs files into a single .wasm binary
      - Resolves all dependencies from Cargo.toml
      - Runs the Rust compiler (rustc) + LLVM backend
      - Output: target/wasm32-unknown-unknown/release/pdf_viewer.wasm

   b. wasm-bindgen pdf_viewer.wasm --out-dir dist
      - Reads #[wasm_bindgen] annotations in your code
      - Generates JavaScript glue code (pdf-viewer.js)
      - Generates TypeScript declarations (pdf-viewer.d.ts)
      - Modifies the .wasm to work with the glue code

   c. wasm-opt -Oz pdf_viewer_bg.wasm
      - Optimizes the .wasm binary for smallest size
      - Removes dead code, optimizes instruction sequences
      - Can shrink the binary by 10-30%

5. HTML Processing:
   - Replaces <link data-trunk rel="css"> with real <link> tags
   - Replaces <link data-trunk rel="rust"> with:
     <link rel="preload" href="pdf-viewer-xxx_bg.wasm" as="fetch">
     <script type="module">
       import init from './pdf-viewer-xxx.js';
       init('./pdf-viewer-xxx_bg.wasm');
     </script>
   - Outputs processed HTML to dist/index.html

6. Final dist/ folder:
   dist/
   ├── index.html                    (processed)
   ├── pdf-viewer-a1b2c3d4.js        (wasm-bindgen glue)
   ├── pdf-viewer-a1b2c3d4_bg.wasm   (compiled Rust)
   ├── main-e5f6g7h8.css             (bundled CSS)
   ├── themes-i9j0k1l2.css
   ├── toolbar-m3n4o5p6.css
   ├── notes-q7r8s9t0.css
   └── pdf-u1v2w3x4.css
```

### What `wasm32-unknown-unknown` means

This is the Rust **compilation target triple**:

```
wasm32    - Architecture: WebAssembly (32-bit address space)
unknown   - Vendor: unknown (not Apple, Microsoft, etc.)
unknown   - OS: unknown (WASM runs in a sandbox, not an OS)
```

Compare with other targets:
```
x86_64-unknown-linux-gnu    → Linux desktop binary
aarch64-apple-darwin        → macOS on Apple Silicon
x86_64-pc-windows-msvc      → Windows binary
wasm32-unknown-unknown      → WebAssembly (browser)
```

Same Rust source code, different compilation target → different output format.

---

## WASM Security Model

WASM runs in a **sandbox** — the same security sandbox as JavaScript:

```
What WASM CAN do:
  ✓ Computation (math, string processing, data structures)
  ✓ Call JavaScript functions (via wasm-bindgen)
  ✓ Access browser APIs (via web-sys → JavaScript)
  ✓ Make HTTP requests (same-origin policy applies)
  ✓ Use Web Workers (run in background threads)

What WASM CANNOT do:
  ✗ Access the file system directly
  ✗ Access raw memory outside its sandbox
  ✗ Make system calls
  ✗ Access other browser tabs
  ✗ Bypass CORS restrictions
  ✗ Access hardware directly (GPU, USB, etc.)
```

WASM is not less safe than JavaScript — it has the exact same restrictions. The `.wasm` binary runs inside a memory-safe sandbox provided by the browser.

---

## Common Misconceptions

### "WASM replaces JavaScript"
No. WASM **complements** JavaScript. It can't access the DOM directly, can't handle events directly, and can't replace JavaScript for most web tasks. Think of it as a co-processor.

### "WASM is always faster"
Not for DOM-heavy apps. Every DOM call from WASM goes through a JS bridge. If your app is mostly UI updates, WASM adds overhead. WASM shines for computation-heavy work that doesn't touch the DOM.

### "WASM is only for C/C++"
Rust, Go, C#, Kotlin, Swift, Zig, and many other languages compile to WASM. Rust has the best WASM ecosystem currently (wasm-bindgen, wasm-pack, Trunk, Leptos, Yew, etc.).

### "WASM files are huge"
A typical WASM app is 100-400 KB gzipped — comparable to a React app. With `wasm-opt`, dead code elimination, and Rust's lack of a runtime, binaries stay small.

### "You need to learn a whole new language"
True for Rust, but frameworks like Leptos intentionally mirror React/SolidJS patterns. If you know React, the concepts transfer — signals are state, components are functions, view! is JSX. The new part is Rust's ownership model, not the web patterns.
