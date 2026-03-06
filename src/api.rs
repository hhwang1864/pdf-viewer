use crate::models::Note;
use gloo_net::http::Request;

const API_BASE: &str = "https://pdf-viewer-api.hhojin1864.workers.dev/api";

pub async fn fetch_notes(pdf_hash: &str, page: Option<u32>) -> Result<Vec<Note>, String> {
    let mut url = format!("{}/notes?pdf_hash={}", API_BASE, pdf_hash);
    if let Some(p) = page {
        url.push_str(&format!("&page={}", p));
    }

    let resp = Request::get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch notes: {}", e))?;

    if resp.ok() {
        resp.json::<Vec<Note>>()
            .await
            .map_err(|e| format!("Failed to parse notes: {}", e))
    } else {
        Err(format!("API error: {}", resp.status()))
    }
}

pub async fn create_note(note: &Note) -> Result<Note, String> {
    let resp = Request::post(&format!("{}/notes", API_BASE))
        .json(note)
        .map_err(|e| format!("Failed to serialize: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Failed to create note: {}", e))?;

    if resp.ok() {
        resp.json::<Note>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    } else {
        Err(format!("API error: {}", resp.status()))
    }
}

pub async fn update_note(note: &Note) -> Result<Note, String> {
    let resp = Request::put(&format!("{}/notes/{}", API_BASE, note.id))
        .json(note)
        .map_err(|e| format!("Failed to serialize: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Failed to update note: {}", e))?;

    if resp.ok() {
        resp.json::<Note>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    } else {
        Err(format!("API error: {}", resp.status()))
    }
}

pub async fn delete_note(id: &str) -> Result<(), String> {
    let resp = Request::delete(&format!("{}/notes/{}", API_BASE, id))
        .send()
        .await
        .map_err(|e| format!("Failed to delete note: {}", e))?;

    if resp.ok() {
        Ok(())
    } else {
        Err(format!("API error: {}", resp.status()))
    }
}
