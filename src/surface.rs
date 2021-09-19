use super::*;
use std::fmt::Write;

/// Generates a 3D a surface (or wireframe, or both)
///
/// # Example
///
/// ```
/// # fn main() -> Result<(), &'static str> {
/// // import
/// use plotpy::{Surface, Plot};
/// use russell_lab::Matrix;
/// use std::path::Path;
///
/// // directory to save figures
/// const OUT_DIR: &str = "/tmp/plotpy/doc_tests";
///
/// // generate (x,y,z) matrices
/// let n = 21;
/// let mut x = Matrix::new(n, n);
/// let mut y = Matrix::new(n, n);
/// let mut z = Matrix::new(n, n);
/// let (min, max) = (-2.0, 2.0);
/// let d = (max - min) / ((n - 1) as f64);
/// for i in 0..n {
///     let v = min + (i as f64) * d;
///     for j in 0..n {
///         let u = min + (j as f64) * d;
///         x[i][j] = u;
///         y[i][j] = v;
///         z[i][j] = u * u - v * v;
///     }
/// }
///
/// // configure and draw surface + wireframe
/// let mut surface = Surface::new();
/// surface.set_colormap_name("seismic")
///     .set_with_colorbar(true)
///     .set_with_wireframe(true)
///     .set_line_width(0.3);
///
/// // draw surface + wireframe
/// surface.draw(&x, &y, &z);
///
/// // add surface to plot
/// let mut plot = Plot::new();
/// plot.add(&surface);
/// plot.camera(20.0, 35.0); // must be after add surface
///
/// // save figure
/// let path = Path::new(OUT_DIR).join("doc_surface.svg");
/// plot.title("horse saddle equation");
/// plot.save(&path)?;
/// # Ok(())
/// # }
/// ```
///
/// ![doc_surface.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/doc_surface.svg)
///
pub struct Surface {
    row_stride: i32,          // Row stride
    col_stride: i32,          // Column stride
    with_surface: bool,       // Generates a surface
    with_wireframe: bool,     // Generates a wireframe
    colormap_index: i32,      // Colormap index
    colormap_name: String,    // Colormap name
    with_colorbar: bool,      // Draw a colorbar
    colorbar_label: String,   // Colorbar label
    number_format_cb: String, // Number format for labels in colorbar
    line_color: String,       // Color of wireframe lines
    line_style: String,       // Style of wireframe line
    line_width: f64,          // Width of wireframe line
    buffer: String,           // buffer
}

impl Surface {
    /// Creates a new Surface object
    pub fn new() -> Self {
        Surface {
            row_stride: 0,
            col_stride: 0,
            with_surface: true,
            with_wireframe: false,
            colormap_index: 0,
            colormap_name: String::new(),
            with_colorbar: false,
            colorbar_label: String::new(),
            number_format_cb: String::new(),
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
    /// * The type `U` of the input matrices must be a number.
    ///
    pub fn draw<'a, T, U>(&mut self, x: &'a T, y: &'a T, z: &'a T)
    where
        T: AsMatrix<'a, U>,
        U: 'a + std::fmt::Display,
    {
        matrix_to_array(&mut self.buffer, "x", x);
        matrix_to_array(&mut self.buffer, "y", y);
        matrix_to_array(&mut self.buffer, "z", z);
        write!(&mut self.buffer, "maybeCreateAX3D()\n").unwrap();
        if self.with_surface {
            let opt_surface = self.options_surface();
            write!(&mut self.buffer, "sf=AX3D.plot_surface(x,y,z{})\n", &opt_surface).unwrap();
        }
        if self.with_wireframe {
            let opt_wireframe = self.options_wireframe();
            write!(&mut self.buffer, "AX3D.plot_wireframe(x,y,z{})\n", &opt_wireframe).unwrap();
        }
        if self.with_colorbar {
            let opt_colorbar = self.options_colorbar();
            write!(&mut self.buffer, "cb=plt.colorbar(sf{})\n", &opt_colorbar).unwrap();
            if self.colorbar_label != "" {
                write!(&mut self.buffer, "cb.ax.set_ylabel(r'{}')\n", self.colorbar_label).unwrap();
            }
        }
    }

    /// Sets the row stride
    pub fn set_row_stride(&mut self, value: i32) -> &mut Self {
        self.row_stride = value;
        self
    }

    /// Sets the column stride
    pub fn set_col_stride(&mut self, value: i32) -> &mut Self {
        self.col_stride = value;
        self
    }

    /// Sets option to generate surface
    pub fn set_with_surface(&mut self, flag: bool) -> &mut Self {
        self.with_surface = flag;
        self
    }

    /// Sets option to generate wireframe
    pub fn set_with_wireframe(&mut self, flag: bool) -> &mut Self {
        self.with_wireframe = flag;
        self
    }

    /// Sets the colormap index
    ///
    /// Options:
    ///
    /// * 0 -- bwr
    /// * 1 -- RdBu
    /// * 2 -- hsv
    /// * 3 -- jet
    /// * 4 -- terrain
    /// * 5 -- pink
    /// * 6 -- Greys
    /// * `>`6 -- starts over from 0
    pub fn set_colormap_index(&mut self, index: i32) -> &mut Self {
        self.colormap_index = index;
        self.colormap_name = String::new();
        self
    }

    /// Sets the colormap name
    ///
    /// Options:
    ///
    /// * `bwr`
    /// * `RdBu`
    /// * `hsv`
    /// * `jet`
    /// * `terrain`
    /// * `pink`
    /// * `Greys`
    /// * see more here <https://matplotlib.org/stable/tutorials/colors/colormaps.html>
    pub fn set_colormap_name(&mut self, name: &str) -> &mut Self {
        self.colormap_name = String::from(name);
        self
    }

    /// Sets option to draw a colorbar
    pub fn set_with_colorbar(&mut self, flag: bool) -> &mut Self {
        self.with_colorbar = flag;
        self
    }

    /// Sets the colorbar label
    pub fn set_colorbar_label(&mut self, label: &str) -> &mut Self {
        self.colorbar_label = String::from(label);
        self
    }

    /// Sets the number format for the labels in the colorbar (cb)
    pub fn set_number_format_cb(&mut self, format: &str) -> &mut Self {
        self.number_format_cb = String::from(format);
        self
    }

    /// Sets the color of wireframe lines
    pub fn set_line_color(&mut self, color: &str) -> &mut Self {
        self.line_color = String::from(color);
        self
    }

    /// Sets the style of wireframe line
    ///
    /// Options:
    ///
    /// * "`-`", "`:`", "`--`", "`-.`"
    pub fn set_line_style(&mut self, style: &str) -> &mut Self {
        self.line_style = String::from(style);
        self
    }

    /// Sets the width of wireframe line
    pub fn set_line_width(&mut self, width: f64) -> &mut Self {
        self.line_width = width;
        self
    }

    /// Returns options for surface
    fn options_surface(&self) -> String {
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
    fn options_wireframe(&self) -> String {
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
    fn options_colorbar(&self) -> String {
        let mut opt = String::new();
        if self.number_format_cb != "" {
            write!(&mut opt, ",format='{}'", self.number_format_cb).unwrap();
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
    use russell_lab::Matrix;

    #[test]
    fn new_works() {
        let surface = Surface::new();
        assert_eq!(surface.row_stride, 0);
        assert_eq!(surface.col_stride, 0);
        assert_eq!(surface.with_surface, true);
        assert_eq!(surface.with_wireframe, false);
        assert_eq!(surface.colormap_index, 0);
        assert_eq!(surface.colormap_name.len(), 0);
        assert_eq!(surface.with_colorbar, false);
        assert_eq!(surface.colorbar_label.len(), 0);
        assert_eq!(surface.number_format_cb.len(), 0);
        assert_eq!(surface.line_color, "black".to_string());
        assert_eq!(surface.line_style.len(), 0);
        assert_eq!(surface.line_width, 0.0);
        assert_eq!(surface.buffer.len(), 0);
    }

    #[test]
    fn options_surface_works() {
        let mut surface = Surface::new();
        surface.set_row_stride(3).set_col_stride(4);
        let opt = surface.options_surface();
        assert_eq!(opt, ",rstride=3,cstride=4,cmap=getColormap(0)");

        surface.set_colormap_name("Pastel1");
        let opt = surface.options_surface();
        assert_eq!(opt, ",rstride=3,cstride=4,cmap=plt.get_cmap('Pastel1')");

        surface.set_colormap_index(3);
        let opt = surface.options_surface();
        assert_eq!(opt, ",rstride=3,cstride=4,cmap=getColormap(3)");

        surface.set_colormap_name("turbo");
        let opt = surface.options_surface();
        assert_eq!(opt, ",rstride=3,cstride=4,cmap=plt.get_cmap('turbo')");
    }

    #[test]
    fn options_wireframe_works() {
        let mut surface = Surface::new();
        surface
            .set_row_stride(3)
            .set_col_stride(4)
            .set_line_color("red")
            .set_line_style("--")
            .set_line_width(2.5);
        let opt = surface.options_wireframe();
        assert_eq!(opt, ",rstride=3,cstride=4,color='red',linestyle='--',linewidth=2.5");
    }

    #[test]
    fn options_colorbar_works() {
        let mut surface = Surface::new();
        surface.set_number_format_cb("%.3f");
        let opt = surface.options_colorbar();
        assert_eq!(opt, ",format='%.3f'");
    }

    #[test]
    fn draw_works() {
        let mut surface = Surface::new();
        surface
            .set_with_wireframe(true)
            .set_with_colorbar(true)
            .set_colorbar_label("temperature");
        let x = vec![vec![-0.5, 0.0, 0.5], vec![-0.5, 0.0, 0.5], vec![-0.5, 0.0, 0.5]];
        let y = vec![vec![-0.5, -0.5, -0.5], vec![0.0, 0.0, 0.0], vec![0.5, 0.5, 0.5]];
        let z = vec![vec![0.50, 0.25, 0.50], vec![0.25, 0.00, 0.25], vec![0.50, 0.25, 0.50]];
        surface.draw(&x, &y, &z);
        let b: &str = "x=np.array([[-0.5,0,0.5,],[-0.5,0,0.5,],[-0.5,0,0.5,],],dtype=float)\n\
                       y=np.array([[-0.5,-0.5,-0.5,],[0,0,0,],[0.5,0.5,0.5,],],dtype=float)\n\
                       z=np.array([[0.5,0.25,0.5,],[0.25,0,0.25,],[0.5,0.25,0.5,],],dtype=float)\n\
                       maybeCreateAX3D()\n\
                       sf=AX3D.plot_surface(x,y,z,cmap=getColormap(0))\n\
                       AX3D.plot_wireframe(x,y,z,color='black')\n\
                       cb=plt.colorbar(sf)\n\
                       cb.ax.set_ylabel(r'temperature')\n";
        assert_eq!(surface.buffer, b);
    }

    #[test]
    fn draw_with_matrix_works() {
        let mut surface = Surface::new();
        let x = Matrix::from(&[[-0.5, 0.0, 0.5], [-0.5, 0.0, 0.5], [-0.5, 0.0, 0.5]]);
        let y = Matrix::from(&[[-0.5, -0.5, -0.5], [0.0, 0.0, 0.0], [0.5, 0.5, 0.5]]);
        let z = Matrix::from(&[[0.50, 0.25, 0.50], [0.25, 0.00, 0.25], [0.50, 0.25, 0.50]]);
        surface.draw(&x, &y, &z);
        let b: &str = "x=np.array([[-0.5,0,0.5,],[-0.5,0,0.5,],[-0.5,0,0.5,],],dtype=float)\n\
                       y=np.array([[-0.5,-0.5,-0.5,],[0,0,0,],[0.5,0.5,0.5,],],dtype=float)\n\
                       z=np.array([[0.5,0.25,0.5,],[0.25,0,0.25,],[0.5,0.25,0.5,],],dtype=float)\n\
                       maybeCreateAX3D()\n\
                       sf=AX3D.plot_surface(x,y,z,cmap=getColormap(0))\n";
        assert_eq!(surface.buffer, b);
    }
}
