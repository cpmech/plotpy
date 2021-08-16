use super::*;

pub struct Graph3d {
    pub row_stride: i32, // row stride
    pub col_stride: i32, // column stride
    pub surface: bool,   // generate surface
    pub wireframe: bool, // generate wireframe

    // buffer
    pub(crate) buffer: String,
}

impl Graph3d {
    pub fn new() -> Self {
        Graph3d {
            row_stride: 0,
            col_stride: 0,
            surface: false,
            wireframe: false,
            buffer: String::new(),
        }
    }

    pub(crate) fn options(&self) -> String {
        let mut options = String::new();
        if self.row_stride > 0 {
            options.push_str(&format!(",rstride={}", self.row_stride));
        }
        if self.col_stride > 0 {
            options.push_str(&format!(",cstride={}", self.col_stride));
        }
        options
    }
}

impl GraphMaker for Graph3d {
    fn get_buffer<'a>(&'a self) -> &'a String {
        &self.buffer
    }
}
