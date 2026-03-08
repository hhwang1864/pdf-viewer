# styles/pdf.css

**What this file is:** Styles for the PDF display area, canvas, text layer, and notes overlay.

## Key Sections

### PDF area (lines 1-7)
```css
.pdf-area {
    flex: 1;          /* fills remaining horizontal space */
    overflow: auto;   /* scrollable when PDF is larger than viewport */
    padding: 20px;
    position: relative;
}
```
The main scrollable container for the PDF. Takes all space not used by the sidebar.

### Canvas wrapper (lines 9-15)
```css
.pdf-canvas-wrapper {
    position: relative;
    box-shadow: var(--shadow-lg);
    background: white;
    margin: 0 auto;        /* centered horizontally */
    width: fit-content;
}
```
Wraps the canvas, text layer, and notes overlay. White background gives the PDF a "paper" look with a shadow.

### Notes overlay (lines 21-33)
```css
.notes-overlay {
    position: absolute;
    top: 0; left: 0;
    width: 100%; height: 100%;
    pointer-events: none;      /* clicks pass through to canvas/text-layer */
    z-index: 10;
}
.notes-overlay > * {
    pointer-events: auto;      /* but sticky notes ARE clickable */
}
```
Positioned over the canvas. `pointer-events: none` lets you click through the overlay to select text, while `> *` re-enables events on the actual note elements.

### Note mode (lines 35-41)
```css
.pdf-area.note-mode { cursor: crosshair; }
.pdf-area.note-mode .text-layer { pointer-events: none; }
```
When note placement mode is active, cursor changes to crosshair and text selection is disabled (so clicks go to the pdf-area for note placement instead of text selection).

### Text layer (lines 43-67)
```css
.text-layer {
    position: absolute;
    top: 0; left: 0;
    z-index: 5;
}
.text-layer > span {
    position: absolute;
    color: transparent;     /* text is invisible but selectable */
    cursor: text;
    user-select: text;
}
```
pdf.js renders invisible text spans positioned exactly over the PDF text on the canvas. This enables text selection - the user sees the canvas rendering but selects the invisible text layer on top.

The `::selection` pseudo-element adds a blue highlight when text is selected.
