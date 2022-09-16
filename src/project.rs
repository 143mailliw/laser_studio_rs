use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct GraphicalPoint {
  r: u8,
  g: u8,
  b: u8,
  active: bool
}

impl Default for GraphicalPoint {
    fn default() -> Self {
        Self { r: 255, g: 255, b: 255, active: false }
    }
}

pub struct GraphicalData {
    pub last_update: u64,
    pub width: u16,
    pub height: u16,
    pub points: Vec<GraphicalPoint>
}

impl Default for GraphicalData {
    fn default() -> Self {
        Self { 
            last_update: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time went backwards")
                .as_secs(),
            width: 20,
            height: 20,
            points: vec![GraphicalPoint::default(); 400]
        }
    }
}

pub struct TextData {
    pub last_update: u64,
    pub content: String
}

impl Default for TextData {
    fn default() -> Self {
        Self {
            last_update: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time went backwards")
                .as_secs(),
            content: " 
                # This is some test content.
                ".to_string()
        }
    }
}

pub struct Project {
    pub last_update: u64,
    pub text_data: TextData,
    pub graphical_data: GraphicalData
}


impl Default for Project {
    fn default() -> Self {
        Self {
            last_update: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time went backwards")
                .as_secs(),
            text_data: TextData::default(),
            graphical_data: GraphicalData::default()
        }
    }
}
