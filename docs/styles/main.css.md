# styles/main.css

**What this file is:** Global styles and the file upload area. Sets up the base layout (flexbox full-height app) and resets.

## Key Sections

### Reset (lines 1-5)
```css
* { margin: 0; padding: 0; box-sizing: border-box; }
```
Standard CSS reset. `box-sizing: border-box` makes padding included in width calculations.

### Body (lines 7-12)
```css
html, body {
    height: 100%;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    background: var(--bg-secondary);
    color: var(--text-primary);
}
```
Uses CSS custom properties (`var(--bg-secondary)`) defined in `themes.css`. These change based on `[data-theme]`.

### App layout (lines 14-25)
```css
.app {
    display: flex;
    flex-direction: column;
    height: 100vh;
    overflow: hidden;
}
.main-content {
    display: flex;
    flex: 1;
    overflow: hidden;
}
```
The app is a vertical flex container: toolbar on top, main-content fills the rest. `main-content` is a horizontal flex: PDF area + sidebar side by side.

### Upload area (lines 27-107)
The initial screen before a PDF is loaded. A centered box with dashed border containing:
- "Choose File" button (styled label hiding the actual `<input type="file">`)
- "or" text
- URL input with submit button
