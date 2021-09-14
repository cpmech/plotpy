use super::*;
use std::fmt::Write;

/// Generates a 3D a surface (or wireframe, or both)
pub struct Surface {
    /// Row stride
    pub row_stride: i32,

    /// Column stride
    pub col_stride: i32,

    /// Generates a surface
    pub surface: bool,

    /// Generates a wireframe
    pub wireframe: bool,

    /// Colormap index
    /// * 0 -- bwr
    /// * 1 -- RdBu
    /// * 2 -- hsv
    /// * 3 -- jet
    /// * 4 -- terrain
    /// * 5 -- pink
    /// * 6 -- Greys
    /// * `>`6 -- starts over from 0
    pub colormap_index: i32,

    /// Colormap name as defined in <https://matplotlib.org/stable/tutorials/colors/colormaps.html>
    ///
    /// Will use `colormap_index` instead if `colormap_name` is empty.
    pub colormap_name: String,

    /// Draw a colorbar
    pub colorbar: bool,

    /// Colorbar label
    pub colorbar_label: String,

    /// Number format for the labels in lines contour (e.g. "%.2f")
    pub colorbar_number_format: String,

    /// Color of wireframe lines
    pub line_color: String,

    /// Style of wireframe line
    ///
    /// Options: "`-`", "`:`", "`--`", "`-.`"
    pub line_style: String,

    /// Width of wireframe line
    pub line_width: f64,

    // buffer
    pub(crate) buffer: String,
}

impl Surface {
    /// Creates a new Surface object
    pub fn new() -> Self {
        Surface {
            row_stride: 0,
            col_stride: 0,
            surface: true,
            wireframe: false,
            colormap_index: 0,
            colormap_name: String::new(),
            colorbar: false,
            colorbar_label: String::new(),
            colorbar_number_format: String::new(),
            line_color: "black".to_string(),
            line_style: String::new(),
            line_width: 0.0,
            buffer: String::new(),
        }
    }

    /// Draws a surface, or wireframe, or both
    ///
    /// # Input
    ///
    /// * `x` -- matrix with x values
    /// * `y` -- matrix with y values
    /// * `z` -- matrix with z values
    ///
    /// # Flags
    ///
    /// The following flags control what features are not to be drawn:
    ///
    /// * `surface` -- draws surface
    /// * `wireframe` -- draws wireframe
    ///
    /// # Notes
    ///
    /// * The type `T` of the input matrices must be a number.
    ///
    pub fn draw<T>(&mut self, x: &Vec<Vec<T>>, y: &Vec<Vec<T>>, z: &Vec<Vec<T>>)
    where
        T: std::fmt::Display,
    {
        matrix_to_array(&mut self.buffer, "x", x);
        matrix_to_array(&mut self.buffer, "y", y);
        matrix_to_array(&mut self.buffer, "z", z);
        write!(&mut self.buffer, "maybeCreateAX3D()\n").unwrap();
        if self.surface {
            let opt_surface = self.options_surface();
            write!(&mut self.buffer, "sf=AX3D.plot_surface(x,y,z{})\n", &opt_surface).unwrap();
        }
        if self.wireframe {
            let opt_wireframe = self.options_wireframe();
            write!(&mut self.buffer, "AX3D.plot_wireframe(x,y,z{})\n", &opt_wireframe).unwrap();
        }
        if self.colorbar {
            let opt_colorbar = self.options_colorbar();
            write!(&mut self.buffer, "cb=plt.colorbar(sf{})\n", &opt_colorbar).unwrap();
            if self.colorbar_label != "" {
                write!(&mut self.buffer, "cb.ax.set_ylabel(r'{}')\n", self.colorbar_label).unwrap();
            }
        }
    }

    /// Returns options for surface
    pub(crate) fn options_surface(&self) -> String {
        let mut opt = String::new();
        if self.row_stride > 0 {
            write!(&mut opt, ",rstride={}", self.row_stride).unwrap();
        }
        if self.col_stride > 0 {
            write!(&mut opt, ",cstride={}", self.col_stride).unwrap();
        }
        if self.colormap_name != "" {
            write!(&mut opt, ",cmap=plt.get_cmap('{}')", self.colormap_name).unwrap();
        } else {
            write!(&mut opt, ",cmap=getColormap({})", self.colormap_index).unwrap();
        }
        opt
    }

    /// Returns options for wireframe
    pub(crate) fn options_wireframe(&self) -> String {
        let mut opt = String::new();
        if self.row_stride > 0 {
            write!(&mut opt, ",rstride={}", self.row_stride).unwrap();
        }
        if self.col_stride > 0 {
            write!(&mut opt, ",cstride={}", self.col_stride).unwrap();
        }
        if self.line_color != "" {
            write!(&mut opt, ",color='{}'", self.line_color).unwrap();
        }
        if self.line_style != "" {
            write!(&mut opt, ",linestyle='{}'", self.line_style).unwrap();
        }
        if self.line_width > 0.0 {
            write!(&mut opt, ",linewidth={}", self.line_width).unwrap();
        }
        opt
    }

    /// Returns options for colorbar
    pub(crate) fn options_colorbar(&self) -> String {
        let mut opt = String::new();
        if self.colorbar_number_format != "" {
            write!(&mut opt, ",format='{}'", self.colorbar_number_format).unwrap();
        }
        opt
    }
}

impl GraphMaker for Surface {
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
        let graph3d = Surface::new();
        assert_eq!(graph3d.row_stride, 0);
        assert_eq!(graph3d.col_stride, 0);
        assert_eq!(graph3d.surface, true);
        assert_eq!(graph3d.wireframe, false);
        assert_eq!(graph3d.colormap_index, 0);
        assert_eq!(graph3d.colormap_name, "".to_string());
        assert_eq!(graph3d.colorbar, false);
        assert_eq!(graph3d.colorbar_label, "".to_string());
        assert_eq!(graph3d.colorbar_number_format, "".to_string());
        assert_eq!(graph3d.line_color, "black".to_string());
        assert_eq!(graph3d.line_style, "".to_string());
        assert_eq!(graph3d.line_width, 0.0);
        assert_eq!(graph3d.buffer.len(), 0);
    }

    #[test]
    fn options_works() {
        let mut graph3d = Surface::new();
        graph3d.row_stride = 3;
        graph3d.col_stride = 4;
        let opt = graph3d.options_surface();
        assert_eq!(opt, ",rstride=3,cstride=4");
    }

    #[test]
    fn draw_works() {
        let mut surface = Surface::new();
        surface.wireframe = true;
        surface.colorbar = true;
        let x = vec![vec![-0.5, 0.0, 0.5], vec![-0.5, 0.0, 0.5], vec![-0.5, 0.0, 0.5]];
        let y = vec![vec![-0.5, -0.5, -0.5], vec![0.0, 0.0, 0.0], vec![0.5, 0.5, 0.5]];
        let z = vec![vec![0.50, 0.25, 0.50], vec![0.25, 0.00, 0.25], vec![0.50, 0.25, 0.50]];
        surface.draw(&x, &y, &z);
        let b: &str = "x=np.array([[-0.5,0,0.5,],[-0.5,0,0.5,],[-0.5,0,0.5,],],dtype=float)\n\
                       y=np.array([[-0.5,-0.5,-0.5,],[0,0,0,],[0.5,0.5,0.5,],],dtype=float)\n\
                       z=np.array([[0.5,0.25,0.5,],[0.25,0,0.25,],[0.5,0.25,0.5,],],dtype=float)\n\
                       maybeCreateAX3D()\n\
                       sf=AX3D.plot_surface(x,y,z)\n\
                       AX3D.plot_wireframe(x,y,z)\n\
                       cb=plt.colorbar(sf)\n";
        assert_eq!(surface.buffer, b);
    }
}
