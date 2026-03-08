# worker/src/db.ts

**What this file is:** Database access layer. Contains functions that run SQL queries against Cloudflare D1 (serverless SQLite). Like a repository/DAO pattern.

## Line-by-Line Breakdown

### Lines 1-12: Note interface
```typescript
export interface Note {
  id: string;
  pdf_hash: string;
  page_number: number;
  x_position: number;
  y_position: number;
  content: string;
  color: string;
  category: string | null;
  created_at: string;
  updated_at: string;
}
```
TypeScript type matching the `notes` table schema and the Rust `Note` struct. This is the shared data shape between frontend and backend.

### Lines 14-35: `getNotes` - Fetch notes
```typescript
export async function getNotes(
  db: D1Database, pdfHash: string, page?: number
): Promise<Note[]> {
  let query = "SELECT * FROM notes WHERE pdf_hash = ?";
  const params: (string | number)[] = [pdfHash];

  if (page !== undefined) {
    query += " AND page_number = ?";
    params.push(page);
  }
  query += " ORDER BY created_at DESC";

  const result = await db.prepare(query).bind(...params).all<Note>();
  return result.results;
}
```

**D1 query pattern:**
1. `db.prepare(query)` - Create a prepared statement (prevents SQL injection)
2. `.bind(...params)` - Bind parameter values to `?` placeholders
3. `.all<Note>()` - Execute and return all rows typed as `Note`

The `?` placeholders are parameterized queries - the database engine handles escaping, preventing SQL injection attacks.

### Lines 37-61: `createNote` - Insert a note
```typescript
await db.prepare(
  `INSERT INTO notes (id, pdf_hash, ...) VALUES (?, ?, ...)`
).bind(note.id, note.pdf_hash, ...).run();
```
- `.run()` instead of `.all()` because INSERT doesn't return rows
- All 10 columns are inserted

### Lines 63-107: `updateNote` - Partial update
```typescript
export async function updateNote(
  db: D1Database, id: string, updates: Partial<Note>
): Promise<Note | null> {
  const fields: string[] = [];
  const values: (string | number | null)[] = [];

  if (updates.content !== undefined) {
    fields.push("content = ?");
    values.push(updates.content);
  }
  // ... more fields
```

**Dynamic SQL builder:** Only updates fields that are provided in the request body. `Partial<Note>` means all fields are optional.

The pattern builds a query like `UPDATE notes SET content = ?, color = ? WHERE id = ?` dynamically based on which fields are present.

After updating, it re-fetches the note to return the current state:
```typescript
const result = await db.prepare("SELECT * FROM notes WHERE id = ?")
  .bind(id).first<Note>();
```
`.first<Note>()` returns a single row (or null).

### Lines 109-119: `deleteNote`
```typescript
const result = await db.prepare("DELETE FROM notes WHERE id = ?")
  .bind(id).run();
return result.meta.changes > 0;
```
Returns `true` if a row was actually deleted (`changes > 0`), `false` if the ID didn't exist.
