# src/models.rs

**What this file is:** Data type definitions. Like TypeScript interfaces/types. Defines the shape of a `Note` and the `NoteColor` enum.

**TypeScript equivalent:**
```typescript
interface Note {
  id: string;
  pdf_hash: string;
  page_number: number;
  x_position: number;
  y_position: number;
  content: string;
  color: NoteColor;
  category: string | null;
  created_at: string;
  updated_at: string;
}

type NoteColor = 'yellow' | 'blue' | 'green' | 'pink' | 'orange';
```

## Line-by-Line Breakdown

### Lines 1: Import serde
```rust
use serde::{Deserialize, Serialize};
```
Imports the serialization traits. These let you convert structs to/from JSON.

### Lines 3-15: Note struct
```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub pdf_hash: String,
    pub page_number: u32,
    pub x_position: f64,
    pub y_position: f64,
    pub content: String,
    pub color: NoteColor,
    pub category: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
```

**`#[derive(...)]`** - Auto-generates implementations of traits (interfaces). Think of it like TypeScript automatically implementing methods:
- `Clone` → Can be copied with `.clone()` (like spreading an object `{...note}`)
- `Debug` → Can be printed for debugging (`console.log` equivalent)
- `PartialEq` → Can be compared with `==`
- `Serialize` → Can be converted TO JSON
- `Deserialize` → Can be parsed FROM JSON

**`pub struct Note`** - Defines a struct (like a TypeScript interface, but concrete). `pub` means it's exported.

**`pub id: String`** - `pub` on each field means the field is accessible from outside the module. Without `pub`, fields are private by default (a key difference from JavaScript).

**Type mappings:**
| Rust | TypeScript | Notes |
|------|-----------|-------|
| `String` | `string` | Heap-allocated string |
| `u32` | `number` | Unsigned 32-bit integer |
| `f64` | `number` | 64-bit float (same as JS number) |
| `Option<String>` | `string \| null` | May or may not have a value |

### Lines 17-25: NoteColor enum
```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NoteColor {
    Yellow,
    Blue,
    Green,
    Pink,
    Orange,
}
```

**`enum`** - A type that can be one of several variants. Like a TypeScript union type but more powerful.

**`#[serde(rename_all = "lowercase")]`** - When serializing to JSON, convert variant names to lowercase. So `NoteColor::Yellow` becomes `"yellow"` in JSON, not `"Yellow"`.

### Lines 27-56: NoteColor methods
```rust
impl NoteColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            NoteColor::Yellow => "yellow",
            ...
        }
    }
```

**`impl NoteColor`** - "Implement methods for NoteColor". Like adding methods to a class.

**`&self`** - A reference to the instance. Like `this` in JavaScript.

**`match`** - Pattern matching. Like a `switch` statement but exhaustive (Rust forces you to handle every variant).

**`&'static str`** - A string reference that lives forever. These string literals (`"yellow"`) are baked into the binary.

**`css_color(&self)`** - Returns the hex color code for each variant. Used in the UI to set background colors.

**`all()`** - Returns a static slice (fixed-size array reference) of all variants. Used when rendering color picker buttons.

### Lines 59-63: Display trait
```rust
impl std::fmt::Display for NoteColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
```

Implementing `Display` lets you convert `NoteColor` to a string with `.to_string()` or use it in format strings. Like overriding `toString()` in JavaScript.
