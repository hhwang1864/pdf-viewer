# styles/themes.css

**What this file is:** CSS custom properties (variables) for light and dark themes. The theme system works by changing `data-theme` attribute on the root element, which activates different variable sets.

## How It Works

```css
:root, [data-theme="light"] {
    --bg-primary: #ffffff;
    --text-primary: #111827;
    ...
}

[data-theme="dark"] {
    --bg-primary: #1f2937;
    --text-primary: #f9fafb;
    ...
}
```

1. `:root` sets the defaults (light theme)
2. `[data-theme="light"]` also sets light values (explicit)
3. `[data-theme="dark"]` overrides everything for dark mode
4. All other CSS files use `var(--bg-primary)` etc.
5. When `theme.rs` calls `el.set_attribute("data-theme", "dark")`, all variables switch instantly

**Variables defined:**
| Variable | Purpose | Light | Dark |
|----------|---------|-------|------|
| `--bg-primary` | Card/panel backgrounds | white | dark gray |
| `--bg-secondary` | Page background | light gray | near-black |
| `--bg-tertiary` | Hover states, inputs | medium gray | medium-dark gray |
| `--text-primary` | Main text | near-black | near-white |
| `--text-secondary` | Dimmer text | medium gray | light gray |
| `--text-muted` | Hint text | light gray | medium gray |
| `--border-color` | Borders | light gray | medium gray |
| `--toolbar-bg/border` | Toolbar | white | dark gray |
| `--shadow/shadow-lg` | Box shadows | subtle | stronger |
| `--btn-hover/active` | Button states | grays | darker grays |
| `--overlay-bg` | Modal backdrop | 30% black | 60% black |
| `--sidebar-bg` | Sidebar | white | dark gray |
| `--input-bg/border` | Form inputs | white | dark gray |
