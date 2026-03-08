# styles/notes.css

**What this file is:** Styles for sticky notes, the note editor modal, and the notes sidebar. The largest CSS file.

## Key Sections

### Sticky notes (lines 1-83)
```css
.sticky-note {
    position: absolute;      /* positioned relative to notes-overlay */
    width: 180px;
    min-height: 80px;
    resize: both;            /* user can resize via bottom-right corner */
    z-index: 10;
}
```
- `position: absolute` - placed at exact pixel coordinates on the PDF
- `.dragging` class reduces opacity and increases z-index during drag
- `.sticky-note-header` has `cursor: grab` for the drag handle
- `.sticky-note-input` textarea is transparent background to look like part of the note

### Note editor modal (lines 85-181)
```css
.note-editor-overlay {
    position: fixed;
    top: 0; left: 0; right: 0; bottom: 0;
    background: var(--overlay-bg);
    z-index: 1000;
}
.note-editor {
    width: 400px;
    border-radius: 12px;
    gap: 16px;
}
```
Standard modal pattern: fixed overlay covering screen, centered content card. The overlay captures clicks to close the modal.

### Color buttons (lines 133-157)
```css
.color-btn {
    width: 32px; height: 32px;
    border-radius: 50%;       /* circular */
}
.color-btn.selected {
    border-color: var(--text-primary);  /* ring around selected color */
}
```
Circular colored buttons used in both the editor and sidebar.

### Notes sidebar (lines 183-332)
```css
.notes-sidebar {
    width: 300px;
    min-width: 300px;
    border-left: 1px solid var(--border-color);
    overflow-y: auto;
    transition: width 0.2s;
}
.notes-sidebar.expanded {
    width: 480px;
}
```
Fixed-width sidebar on the right side. `.expanded` class makes it wider with a smooth transition.

Each `.sidebar-note` has a colored left border matching the note's color and hover effects.
