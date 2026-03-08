# src/api.rs

**What this file is:** HTTP client module for the notes REST API. Makes fetch requests to the Cloudflare Worker backend. Like an API service file in React (e.g., `api/notes.js`).

**React/JS equivalent:**
```javascript
const API_BASE = 'https://pdf-viewer-api.hhojin1864.workers.dev/api';

export async function fetchNotes(pdfHash, page) {
  const res = await fetch(`${API_BASE}/notes?pdf_hash=${pdfHash}&page=${page}`);
  return res.json();
}
```

## Line-by-Line Breakdown

### Lines 1-2: Imports
```rust
use crate::models::Note;
use gloo_net::http::Request;
```
- `Note` struct from our models
- `gloo_net::http::Request` - HTTP client for WASM. This wraps the browser's `fetch()` API.

### Line 4: API base URL
```rust
const API_BASE: &str = "https://pdf-viewer-api.hhojin1864.workers.dev/api";
```
A compile-time constant. `&str` is a string reference (like `const` in JS - it's baked into the binary, not allocated at runtime).

### Lines 6-24: `fetch_notes`
```rust
pub async fn fetch_notes(pdf_hash: &str, page: Option<u32>) -> Result<Vec<Note>, String> {
```

**`pub async fn`** - Public async function. Same concept as JavaScript `async function`.

**`pdf_hash: &str`** - Takes a string reference (borrowed, not owned). `&str` means "I'm just reading this string, I don't need to own it."

**`page: Option<u32>`** - Optional page number. `Option<u32>` = `number | undefined` in TypeScript.

**`-> Result<Vec<Note>, String>`** - Returns either a success (`Vec<Note>` = array of notes) or an error (`String` message). `Result` is Rust's way of handling errors instead of try/catch.

```rust
let mut url = format!("{}/notes?pdf_hash={}", API_BASE, pdf_hash);
if let Some(p) = page {
    url.push_str(&format!("&page={}", p));
}
```
- `format!()` - String interpolation. Like JS template literals `` `${API_BASE}/notes?pdf_hash=${pdf_hash}` ``
- `if let Some(p) = page` - Pattern matching on Option. If `page` is `Some(value)`, extract the value into `p`. If it's `None`, skip this block. Like `if (page !== undefined)`.
- `mut` - Makes `url` mutable. In Rust, variables are immutable by default.

```rust
let resp = Request::get(&url)
    .send()
    .await
    .map_err(|e| format!("Failed to fetch notes: {}", e))?;
```
- `Request::get(&url)` - Create a GET request (like `fetch(url)`)
- `.send().await` - Send it and await the response
- `.map_err(|e| ...)` - If there's an error, transform it into our error type (String)
- `?` - **The question mark operator.** This is Rust's error propagation. If the result is an error, immediately return it from this function. If it's ok, unwrap the value. Like writing `if (result.isError) return result.error` but in one character.

```rust
if resp.ok() {
    resp.json::<Vec<Note>>()
        .await
        .map_err(|e| format!("Failed to parse notes: {}", e))
} else {
    Err(format!("API error: {}", resp.status()))
}
```
- `resp.ok()` - Check if status is 2xx (same as JS `response.ok`)
- `resp.json::<Vec<Note>>()` - Parse JSON into `Vec<Note>`. The `::<Vec<Note>>` is a "turbofish" - it tells Rust what type to deserialize into. Like `response.json() as Note[]` in TypeScript.

### Lines 26-41: `create_note`
```rust
pub async fn create_note(note: &Note) -> Result<Note, String> {
    let resp = Request::post(&format!("{}/notes", API_BASE))
        .json(note)
        .map_err(|e| format!("Failed to serialize: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Failed to create note: {}", e))?;
```
- `Request::post(url)` - POST request
- `.json(note)` - Serialize `note` as JSON body (like `fetch(url, { body: JSON.stringify(note) })`)
- Two `?` operators - any error along the chain returns early

### Lines 43-58: `update_note`
Same pattern but uses `Request::put()` and includes the note ID in the URL.

### Lines 60-71: `delete_note`
```rust
pub async fn delete_note(id: &str) -> Result<(), String> {
```
Returns `Result<(), String>` - the `()` is Rust's "void" or "unit" type. Success but no meaningful return value. Like a function returning `void` in TypeScript.
