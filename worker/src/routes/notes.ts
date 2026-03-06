import { getNotes, createNote, updateNote, deleteNote, type Note } from "../db";

export async function handleNotes(
  request: Request,
  db: D1Database,
  path: string
): Promise<Response> {
  const url = new URL(request.url);
  const method = request.method;

  // GET /api/notes?pdf_hash=X&page=N
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

  // POST /api/notes
  if (method === "POST" && path === "/api/notes") {
    const body = await request.json<Note>();
    if (!body.id || !body.pdf_hash) {
      return jsonResponse({ error: "id and pdf_hash are required" }, 400);
    }
    const note = await createNote(db, body);
    return jsonResponse(note, 201);
  }

  // PUT /api/notes/:id
  const putMatch = path.match(/^\/api\/notes\/([^/]+)$/);
  if (method === "PUT" && putMatch) {
    const id = putMatch[1];
    const body = await request.json<Partial<Note>>();
    const note = await updateNote(db, id, body);
    if (!note) {
      return jsonResponse({ error: "Note not found" }, 404);
    }
    return jsonResponse(note);
  }

  // DELETE /api/notes/:id
  const deleteMatch = path.match(/^\/api\/notes\/([^/]+)$/);
  if (method === "DELETE" && deleteMatch) {
    const id = deleteMatch[1];
    const deleted = await deleteNote(db, id);
    if (!deleted) {
      return jsonResponse({ error: "Note not found" }, 404);
    }
    return jsonResponse({ success: true });
  }

  return jsonResponse({ error: "Not found" }, 404);
}

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
