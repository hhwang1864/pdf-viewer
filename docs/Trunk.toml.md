# Trunk.toml

**What this file is:** Configuration for Trunk, the build tool. Like `vite.config.js` or `webpack.config.js`.

## Line-by-Line Breakdown

```toml
[build]
target = "index.html"
```
Tells Trunk which HTML file to process. Trunk reads this file, finds `data-trunk` attributes, and processes them.

```toml
[watch]
watch = ["src", "styles"]
```
During `trunk serve` (dev mode), Trunk watches these directories for changes and auto-rebuilds. Like hot-reload in Vite. When you change a `.rs` file in `src/` or a `.css` file in `styles/`, Trunk recompiles and refreshes the browser.

```toml
[serve]
addresses = ["127.0.0.1"]
port = 8080
```
Dev server settings. `trunk serve` starts a local server at `http://127.0.0.1:8080`. Like `npm run dev` starting Vite on a port.
