use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub pdf_hash: String,
    pub page_number: u32,
    pub x_position: f64,
    pub y_position: f64,
    pub content: String,
    pub color: NoteColor,
    pub category: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NoteColor {
    Yellow,
    Blue,
    Green,
    Pink,
    Orange,
}

impl NoteColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            NoteColor::Yellow => "yellow",
            NoteColor::Blue => "blue",
            NoteColor::Green => "green",
            NoteColor::Pink => "pink",
            NoteColor::Orange => "orange",
        }
    }

    pub fn css_color(&self) -> &'static str {
        match self {
            NoteColor::Yellow => "#fef08a",
            NoteColor::Blue => "#93c5fd",
            NoteColor::Green => "#86efac",
            NoteColor::Pink => "#f9a8d4",
            NoteColor::Orange => "#fdba74",
        }
    }

    pub fn all() -> &'static [NoteColor] {
        &[
            NoteColor::Yellow,
            NoteColor::Blue,
            NoteColor::Green,
            NoteColor::Pink,
            NoteColor::Orange,
        ]
    }
}

impl std::fmt::Display for NoteColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
