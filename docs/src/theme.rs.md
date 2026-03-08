# src/theme.rs

**What this file is:** Theme management - light/dark mode with localStorage persistence. Like a theme context/hook in React.

**React equivalent:**
```jsx
const ThemeContext = createContext();
function useTheme() {
  const [theme, setTheme] = useState(localStorage.getItem('theme') || 'light');
  useEffect(() => { localStorage.setItem('theme', theme) }, [theme]);
  return [theme, setTheme];
}
```

## Line-by-Line Breakdown

### Lines 1-2: Imports
```rust
use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;
```
- `gloo_storage` - Wrapper around browser's `localStorage` API. `Storage` is a trait (interface) that `LocalStorage` implements.

### Line 4: Storage key
```rust
const THEME_KEY: &str = "pdf-viewer-theme";
```
The localStorage key. `&str` = string literal.

### Lines 6-11: Theme enum
```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}
```

A simple enum with two variants. `Copy` trait means it can be implicitly copied (no need for `.clone()`). Small types like this are cheap to copy.

### Lines 13-26: Theme methods
```rust
impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }

    pub fn toggle(&self) -> Theme {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }
}
```

- `as_str()` - Convert to string (for HTML `data-theme` attribute)
- `toggle()` - Return the opposite theme. Note it returns a NEW Theme value, doesn't mutate `self`. Rust encourages this functional style.

### Lines 28-34: Load from localStorage
```rust
pub fn load_theme() -> Theme {
    let stored: Result<String, _> = LocalStorage::get(THEME_KEY);
    match stored.as_deref() {
        Ok("dark") => Theme::Dark,
        _ => Theme::Light,
    }
}
```

- `LocalStorage::get(THEME_KEY)` - Like `localStorage.getItem('pdf-viewer-theme')` but returns a `Result` instead of possibly `null`
- `stored.as_deref()` - Converts `Result<String, _>` to `Result<&str, _>` so we can pattern match against string literals
- `Ok("dark")` - If we got a value and it's "dark", use dark theme
- `_` - Wildcard: anything else (error, missing, "light", etc.) defaults to Light

### Lines 36-38: Save to localStorage
```rust
pub fn save_theme(theme: Theme) {
    let _ = LocalStorage::set(THEME_KEY, theme.as_str());
}
```
`let _ =` means "I know this returns a Result but I'm ignoring it". The save might fail (e.g., localStorage full) but we don't care.

### Lines 40-47: Apply theme to DOM
```rust
pub fn apply_theme(theme: Theme) {
    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
        if let Some(el) = document.document_element() {
            let _ = el.set_attribute("data-theme", theme.as_str());
        }
    }
    save_theme(theme);
}
```

This does: `document.documentElement.setAttribute('data-theme', 'light')` (or 'dark').

**Why so many `if let Some`?** In Rust, browser APIs return `Option` (might be `None`) because technically `window` might not exist (e.g., in server-side contexts). Each `.and_then()` and `if let Some` is a null check. In JavaScript you'd just write `document.documentElement.setAttribute(...)` and hope it doesn't crash.

The `data-theme` attribute triggers CSS variable switching in `themes.css`:
```css
[data-theme="dark"] { --bg-primary: #1f2937; ... }
```
