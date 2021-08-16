use super::*;

pub struct Shapes {
    pub edge_color: String, // edge color
    pub face_color: String, // face color
    pub is_closed: bool,    // closed shape
    pub scale: f64,         // scale
    pub style: String,      // style

    // buffer
    pub(crate) buffer: String,
}

impl Shapes {
    pub fn new() -> Self {
        Shapes {
            edge_color: String::new(),
            face_color: String::new(),
            is_closed: false,
            scale: 0.0,
            style: String::new(),
            buffer: String::new(),
        }
    }

    pub(crate) fn options(&self) -> String {
        let mut options = String::new();
        if self.edge_color != "" {
            options.push_str(&format!(",edgecolor='{}'", self.edge_color));
        }
        if self.face_color != "" {
            options.push_str(&format!(",facecolor='{}'", self.face_color));
        }
        options
    }
}

impl GraphMaker for Shapes {
    fn get_buffer<'a>(&'a self) -> &'a String {
        &self.buffer
    }
}
