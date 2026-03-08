# worker/src/routes/notes.ts

**What this file is:** REST API route handler for notes. Maps HTTP methods and paths to database operations. Like an Express router.

**Express equivalent:**
```javascript
router.get('/notes', async (req, res) => { ... });
router.post('/notes', async (req, res) => { ... });
router.put('/notes/:id', async (req, res) => { ... });
router.delete('/notes/:id', async (req, res) => { ... });
```

## Line-by-Line Breakdown

### Line 1: Import
```typescript
import { getNotes, createNote, updateNote, deleteNote, type Note } from "../db";
```
Import all DB functions and the Note type.

### Lines 3-7: Main handler
```typescript
export async function handleNotes(
  request: Request, db: D1Database, path: string
): Promise<Response> {
```
Single function that handles all `/api/notes` routes by checking method + path.

### Lines 11-21: GET /api/notes
```typescript
if (method === "GET" && path === "/api/notes") {
  const pdfHash = url.searchParams.get("pdf_hash");
  if (!pdfHash) {
    return jsonResponse({ error: "pdf_hash is required" }, 400);
  }
  const page = url.searchParams.get("page");
  const pageNum = page ? parseInt(page, 10) : undefined;
  const notes = await getNotes(db, pdfHash, pageNum);
  return jsonResponse(notes);
}
```
- Requires `pdf_hash` query parameter
- Optional `page` parameter for filtering by page number
- Returns JSON array of notes

### Lines 23-31: POST /api/notes
```typescript
if (method === "POST" && path === "/api/notes") {
  const body = await request.json<Note>();
  if (!body.id || !body.pdf_hash) {
    return jsonResponse({ error: "id and pdf_hash are required" }, 400);
  }
  const note = await createNote(db, body);
  return jsonResponse(note, 201);
}
```
- Parses JSON body as Note
- Validates required fields
- Returns 201 Created with the note

### Lines 33-43: PUT /api/notes/:id
```typescript
const putMatch = path.match(/^\/api\/notes\/([^/]+)$/);
if (method === "PUT" && putMatch) {
  const id = putMatch[1];
  const body = await request.json<Partial<Note>>();
  const note = await updateNote(db, id, body);
```
- Uses regex to extract the note ID from the URL path
- `Partial<Note>` means any subset of fields can be sent
- Returns 404 if note doesn't exist

### Lines 45-54: DELETE /api/notes/:id
```typescript
const deleteMatch = path.match(/^\/api\/notes\/([^/]+)$/);
if (method === "DELETE" && deleteMatch) {
  const id = deleteMatch[1];
  const deleted = await deleteNote(db, id);
```
Same URL pattern as PUT. Returns 404 if note doesn't exist, `{ success: true }` if deleted.

### Lines 59-69: JSON response helper
```typescript
function jsonResponse(data: unknown, status = 200): Response {
  return new Response(JSON.stringify(data), {
    status,
    headers: {
      "Content-Type": "application/json",
      "Access-Control-Allow-Origin": "*",
      "Access-Control-Allow-Methods": "GET, POST, PUT, DELETE, OPTIONS",
      "Access-Control-Allow-Headers": "Content-Type",
    },
  });
}
```
Helper that wraps data in a JSON response with CORS headers. Every response needs CORS headers (not just OPTIONS preflight) so the browser allows the frontend to read the response.

## API Summary

| Method | Path | Body | Response |
|--------|------|------|----------|
| GET | `/api/notes?pdf_hash=X&page=N` | - | `Note[]` |
| POST | `/api/notes` | `Note` | `Note` (201) |
| PUT | `/api/notes/:id` | `Partial<Note>` | `Note` |
| DELETE | `/api/notes/:id` | - | `{ success: true }` |
