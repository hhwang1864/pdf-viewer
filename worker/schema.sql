CREATE TABLE IF NOT EXISTS notes (
  id TEXT PRIMARY KEY,
  pdf_hash TEXT NOT NULL,
  page_number INTEGER NOT NULL,
  x_position REAL NOT NULL,
  y_position REAL NOT NULL,
  content TEXT NOT NULL DEFAULT '',
  color TEXT NOT NULL DEFAULT 'yellow',
  category TEXT DEFAULT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_notes_pdf_page ON notes(pdf_hash, page_number);
