# src/main.rs

**What this file is:** The entry point of the Rust application. Like `index.js` or `main.tsx` in a React app.

**React equivalent:**
```jsx
// index.js
import ReactDOM from 'react-dom/client';
import App from './App';
ReactDOM.createRoot(document.getElementById('root')).render(<App />);
```

## Line-by-Line Breakdown

### Lines 1-6: Module declarations
```rust
mod api;
mod app;
mod components;
mod js_bindings;
mod models;
mod theme;
```

`mod` declares that these modules exist. In Rust, every file must be declared as a module somewhere. This is like having an `index.js` that imports all your other files.

- `mod api;` → tells Rust "there's a file called `src/api.rs`, include it"
- `mod components;` → tells Rust "there's a file `src/components/mod.rs`, include it" (directories use `mod.rs`)

**Key difference from JS:** In JavaScript, you import files wherever you need them. In Rust, you declare the module tree once in `main.rs` (or `lib.rs`), and then use `use` statements to bring things into scope.

### Lines 8-9: Imports
```rust
use app::App;
use leptos::prelude::*;
```

- `use app::App;` → Import the `App` component from the `app` module. Like `import { App } from './app'`.
- `use leptos::prelude::*;` → Import everything from Leptos's prelude. The `*` is a wildcard import (like `import * from`). Leptos puts commonly-used items in `prelude` so you don't need to import each one individually.

### Lines 11-14: The main function
```rust
fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
```

**`fn main()`** - The program entry point. When the WASM module loads in the browser, this function runs first.

**`console_error_panic_hook::set_once()`** - Sets up better error messages. Without this, when Rust panics (crashes), you'd see `unreachable` in the console. With this, you get a full stack trace. Think of it like a global error handler.

**`mount_to_body(App)`** - Renders the `App` component and attaches it to `<body>`. This is equivalent to:
```jsx
// React
ReactDOM.createRoot(document.body).render(<App />)
// SolidJS
render(() => <App />, document.body)
```

Note: `App` is passed as a function reference (not called with `App()`). Leptos calls it internally.
