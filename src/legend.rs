use super::*;

pub struct Legend {
    pub show_frame: bool,      // show frame around legend
    pub length_indicator: f64, // length of legend's indicator line
    pub location: String,      // e.g., "right", "center left"
    pub number_columns: i32,   // number of columns
    pub coordinates: Vec<f64>, // normalized coordinates to put legend outsize
    pub is_outside: bool,      // put legend outside

    // buffer
    pub(crate) buffer: String,
}

impl Legend {
    pub fn new() -> Self {
        Legend {
            show_frame: true,
            length_indicator: 3.0,
            location: "best".to_string(),
            number_columns: 1,
            coordinates: vec![0.0, 1.02, 1.0, 0.102],
            is_outside: false,
            buffer: String::new(),
        }
    }

    pub(crate) fn options() -> String {
        let options = String::new();
        options
    }
}

impl GraphMaker for Legend {
    fn get_buffer<'a>(&'a self) -> &'a String {
        &self.buffer
    }
}
