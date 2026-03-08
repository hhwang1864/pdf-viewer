# styles/toolbar.css

**What this file is:** Styles for the top toolbar - navigation, zoom controls, and action buttons.

## Key Sections

### Toolbar layout (lines 1-12)
```css
.toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 16px;
    z-index: 100;
    flex-wrap: wrap;
}
```
Horizontal flex with three groups spread across the width. `flex-wrap` allows wrapping on narrow screens.

### Toolbar buttons (lines 25-58)
```css
.toolbar-btn {
    width: 36px; height: 36px;
    border: 1px solid var(--border-color);
    border-radius: 6px;
}
.toolbar-btn.active {
    background: #3b82f6; color: white;
}
```
Square buttons with rounded corners. `.active` class highlights buttons in blue (used for note mode toggle and sidebar toggle).

### Page slider (lines 76-108)
Custom-styled range input (the page navigation slider). Uses `-webkit-appearance: none` to remove browser defaults and applies a blue circular thumb.

### Page input (lines 110-130)
Small number input for typing a page number directly. Hides the default spin buttons.

### Zoom select (lines 132-140)
Dropdown for zoom level selection. Styled to match the toolbar aesthetic.
