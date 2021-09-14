use super::*;
use std::fmt::Write;

/// Generates a 3D graph such as a surface, wireframe, or a 3D curve
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

    pub fn draw_surface(&mut self, x: &Vec<Vec<f64>>, y: &Vec<Vec<f64>>, z: &Vec<Vec<f64>>) {
        matrix_to_array(&mut self.buffer, "x", x);
        matrix_to_array(&mut self.buffer, "y", y);
        matrix_to_array(&mut self.buffer, "z", z);
        let opt = self.options();
        write!(&mut self.buffer, "AX3D.plot_surface(x,y,z{})\n", &opt).unwrap();
    }

    pub(crate) fn options(&self) -> String {
        let mut opt = String::new();
        if self.row_stride > 0 {
            write!(&mut opt, ",rstride={}", self.row_stride).unwrap();
        }
        if self.col_stride > 0 {
            write!(&mut opt, ",cstride={}", self.col_stride).unwrap();
        }
        opt
    }
}

impl GraphMaker for Graph3d {
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
        let graph3d = Graph3d::new();
        assert_eq!(graph3d.row_stride, 0);
    }

    #[test]
    fn options_works() {
        let mut graph3d = Graph3d::new();
        graph3d.row_stride = 3;
        graph3d.col_stride = 4;
        let opt = graph3d.options();
        assert_eq!(opt, ",rstride=3,cstride=4");
    }
}
