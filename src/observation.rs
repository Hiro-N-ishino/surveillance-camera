use chrono::{DateTime, Local};

pub struct Observation {
    pub file_name: String,
    pub captured_at: DateTime<Local>,
    pub temperature: f32,
    pub humidity: f32,
}
