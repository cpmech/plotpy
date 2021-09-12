use super::*;
use std::fmt::Write;

/// Draw polygonal shapes
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
        let mut opt = String::new();
        if self.edge_color != "" {
            write!(&mut opt, ",edgecolor='{}'", self.edge_color).unwrap();
        }
        if self.face_color != "" {
            write!(&mut opt, ",facecolor='{}'", self.face_color).unwrap();
        }
        opt
    }
}

impl GraphMaker for Shapes {
    fn get_buffer<'a>(&'a self) -> &'a String {
        &self.buffer
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_works() {
        let shapes = Shapes::new();
        assert_eq!(shapes.edge_color, "");
    }

    #[test]
    fn options_works() {
        let mut shapes = Shapes::new();
        shapes.edge_color = "red".to_string();
        let opt = shapes.options();
        assert_eq!(opt, ",edgecolor='red'");
    }
}
