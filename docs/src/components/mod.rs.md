# src/components/mod.rs

**What this file is:** Module declarations for the components directory. Like an `index.js` barrel file that re-exports everything.

**JavaScript equivalent:**
```javascript
// components/index.js
export * from './note_editor';
export * from './notes_sidebar';
export * from './pdf_viewer';
export * from './sticky_note';
export * from './toolbar';
```

## Line-by-Line Breakdown

```rust
pub mod note_editor;
pub mod notes_sidebar;
pub mod pdf_viewer;
pub mod sticky_note;
pub mod toolbar;
```

Each line declares a public submodule. `pub mod pdf_viewer;` tells Rust:
1. There's a file at `src/components/pdf_viewer.rs`
2. Make it publicly accessible as `crate::components::pdf_viewer`

**Why this file exists:** In Rust, when you have a directory as a module (like `components/`), you need a `mod.rs` file inside it to declare what's in the directory. It's like Rust's way of having an index file. Without this file, Rust wouldn't know about any of the component files.

**`pub`** means other modules can access these. Without `pub`, they'd be private to the `components` module only.
