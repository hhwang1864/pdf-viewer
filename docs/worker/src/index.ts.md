# worker/src/index.ts

**What this file is:** The Cloudflare Worker entry point. This is the backend API server. Like an Express.js app but running on Cloudflare's edge network.

**Express equivalent:**
```javascript
const app = express();
app.use(cors());
app.use('/api/notes', notesRouter);
app.listen(3000);
```

## Line-by-Line Breakdown

### Line 1: Import
```typescript
import { handleNotes } from "./routes/notes";
```
Import the notes route handler.

### Lines 3-5: Environment interface
```typescript
interface Env {
  DB: D1Database;
}
```
Cloudflare Workers use "bindings" to access services. `DB` is a D1 database binding configured in `wrangler.jsonc`. The `Env` interface tells TypeScript what bindings are available.

**D1** is Cloudflare's serverless SQLite database. It runs at the edge (close to users) and is accessed through the Worker.

### Lines 7-31: Request handler
```typescript
export default {
  async fetch(request: Request, env: Env): Promise<Response> {
```

Cloudflare Workers export a `fetch` handler (not `listen()` like Express). Every HTTP request calls this function.

### Lines 9-17: CORS preflight
```typescript
if (request.method === "OPTIONS") {
  return new Response(null, {
    headers: {
      "Access-Control-Allow-Origin": "*",
      "Access-Control-Allow-Methods": "GET, POST, PUT, DELETE, OPTIONS",
      "Access-Control-Allow-Headers": "Content-Type",
    },
  });
}
```
Handles CORS preflight requests. The browser sends an OPTIONS request before cross-origin requests. `"*"` allows any origin (the frontend is on a different domain than the API).

### Lines 20-25: Routing
```typescript
const url = new URL(request.url);
const path = url.pathname;

if (path.startsWith("/api/notes")) {
  return handleNotes(request, env.DB, path);
}
```
Simple manual routing. If the path starts with `/api/notes`, delegate to the notes handler. No router library needed for this small API.

### Lines 27-30: 404 fallback
```typescript
return new Response(JSON.stringify({ error: "Not found" }), {
  status: 404,
  headers: { "Content-Type": "application/json" },
});
```
Any unmatched route returns 404 JSON.
