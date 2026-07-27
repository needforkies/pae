use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct EditorSettings {
    pub font: String,
    pub size: i32,
    pub dark_mode: bool,
}

impl Default for EditorSettings {
    fn default() -> Self {
        Self {
            font: "Monospace".to_string(),
            size: 14,
            dark_mode: true,
        }
    }
}