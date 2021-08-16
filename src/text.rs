use super::*;

/// Creates text to be added to a plot
pub struct Text {
    pub alignment_horizontal: String, // e.g., 'center'
    pub alignment_vertical: String,   // e.g., 'center'
    pub rotation: f64,                // text rotation
    pub font_size: f64,               // font size

    // buffer
    pub(crate) buffer: String,
}

impl Text {
    pub fn new() -> Self {
        Text {
            alignment_horizontal: String::new(),
            alignment_vertical: String::new(),
            rotation: 0.0,
            font_size: 0.0,
            buffer: String::new(),
        }
    }

    pub(crate) fn options(&self) -> String {
        let mut options = String::new();
        if self.alignment_horizontal != "" {
            options.push_str(&format!(",ha='{}'", self.alignment_horizontal));
        }
        if self.alignment_vertical != "" {
            options.push_str(&format!(",va='{}'", self.alignment_vertical));
        }
        if self.rotation > 0.0 {
            options.push_str(&format!(",rotation={}", self.rotation));
        }
        if self.font_size > 0.0 {
            options.push_str(&format!(",fontsize={}", self.font_size));
        }
        options
    }
}

impl GraphMaker for Text {
    fn get_buffer<'a>(&'a self) -> &'a String {
        &self.buffer
    }
}
