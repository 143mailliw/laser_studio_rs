use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TextData {
    pub content: String,
    pub size_x: u8,
    pub size_y: u8,
}

impl Default for TextData {
    fn default() -> Self {
        Self {
            content: " 
                # This is some test content.
                "
            .to_string(),
            size_y: 20,
            size_x: 20,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Project {
    pub version: u16,
    pub text_data: TextData,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            version: 2,
            text_data: TextData::default(),
        }
    }
}
