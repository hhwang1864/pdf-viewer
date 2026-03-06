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

export async function getNotes(
  db: D1Database,
  pdfHash: string,
  page?: number
): Promise<Note[]> {
  let query = "SELECT * FROM notes WHERE pdf_hash = ?";
  const params: (string | number)[] = [pdfHash];

  if (page !== undefined) {
    query += " AND page_number = ?";
    params.push(page);
  }

  query += " ORDER BY created_at DESC";

  const result = await db
    .prepare(query)
    .bind(...params)
    .all<Note>();

  return result.results;
}

export async function createNote(
  db: D1Database,
  note: Note
): Promise<Note> {
  await db
    .prepare(
      `INSERT INTO notes (id, pdf_hash, page_number, x_position, y_position, content, color, category, created_at, updated_at)
       VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`
    )
    .bind(
      note.id,
      note.pdf_hash,
      note.page_number,
      note.x_position,
      note.y_position,
      note.content,
      note.color,
      note.category,
      note.created_at,
      note.updated_at
    )
    .run();

  return note;
}

export async function updateNote(
  db: D1Database,
  id: string,
  updates: Partial<Note>
): Promise<Note | null> {
  const fields: string[] = [];
  const values: (string | number | null)[] = [];

  if (updates.content !== undefined) {
    fields.push("content = ?");
    values.push(updates.content);
  }
  if (updates.color !== undefined) {
    fields.push("color = ?");
    values.push(updates.color);
  }
  if (updates.category !== undefined) {
    fields.push("category = ?");
    values.push(updates.category);
  }
  if (updates.x_position !== undefined) {
    fields.push("x_position = ?");
    values.push(updates.x_position);
  }
  if (updates.y_position !== undefined) {
    fields.push("y_position = ?");
    values.push(updates.y_position);
  }

  fields.push("updated_at = ?");
  values.push(new Date().toISOString());
  values.push(id);

  await db
    .prepare(`UPDATE notes SET ${fields.join(", ")} WHERE id = ?`)
    .bind(...values)
    .run();

  const result = await db
    .prepare("SELECT * FROM notes WHERE id = ?")
    .bind(id)
    .first<Note>();

  return result;
}

export async function deleteNote(
  db: D1Database,
  id: string
): Promise<boolean> {
  const result = await db
    .prepare("DELETE FROM notes WHERE id = ?")
    .bind(id)
    .run();

  return result.meta.changes > 0;
}
