use super::{matrix_to_array, AsMatrix, GraphMaker, StrError};
use crate::quote_marker;
use std::fmt::Write;

/// Generates a 3D a surface (or wireframe, or both)
///
/// # Example
///
/// ```
/// use plotpy::{generate3d, Plot, StrError, Surface};
///
/// fn main() -> Result<(), StrError> {
///     // generate (x,y,z) matrices
///     let n = 21;
///     let (x, y, z) = generate3d(-2.0, 2.0, -2.0, 2.0, n, n, |x, y| x * x - y * y);
///
///     // configure and draw surface + wireframe
///     let mut surface = Surface::new();
///     surface.set_colormap_name("seismic")
///         .set_with_colorbar(true)
///         .set_with_wireframe(true)
///         .set_wire_line_width(0.3);
///
///     // draw surface + wireframe
///     surface.draw(&x, &y, &z);
///
///     // add surface to plot
///     let mut plot = Plot::new();
///     plot.add(&surface)
///         .set_title("horse saddle equation") // must be after add surface
///         .set_camera(20.0, 35.0); // must be after add surface
///
///     // save figure
///     plot.save("/tmp/plotpy/doc_tests/doc_surface.svg")?;
///     Ok(())
/// }
/// ```
///
/// ![doc_surface.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/doc_surface.svg)
///
/// See also integration tests in the [tests directory](https://github.com/cpmech/plotpy/tree/main/tests)
///
/// Output from some integration tests:
///
/// ![integ_surface_wireframe.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/integ_surface_wireframe.svg)
pub struct Surface {
    row_stride: usize,        // Row stride
    col_stride: usize,        // Column stride
    with_surface: bool,       // Generates a surface
    with_wireframe: bool,     // Generates a wireframe
    with_points: bool,        // Generates (a scatter of) points on the surface
    colormap_name: String,    // Colormap name
    with_colorbar: bool,      // Draw a colorbar
    colorbar_label: String,   // Colorbar label
    number_format_cb: String, // Number format for labels in colorbar
    surf_color: String,       // Const color of surface (when not using colormap)
    wire_line_color: String,  // Color of wireframe lines
    wire_line_style: String,  // Style of wireframe line
    wire_line_width: f64,     // Width of wireframe line
    point_color: String,      // Color of markers (scatter)
    point_void: bool,         // Draws a void marker (edge only)
    point_line_color: String, // Edge color of markers
    point_line_width: f64,    // Edge width of markers
    point_size: f64,          // Size of markers
    point_style: String,      // Style of markers, e.g., "`o`", "`+`"
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
            with_points: false,
            colormap_name: "bwr".to_string(),
            with_colorbar: false,
            colorbar_label: String::new(),
            number_format_cb: String::new(),
            surf_color: String::new(),
            wire_line_color: "black".to_string(),
            wire_line_style: String::new(),
            wire_line_width: 0.0,
            point_color: String::new(),
            point_void: false,
            point_line_color: String::new(),
            point_line_width: 0.0,
            point_size: 0.0,
            point_style: String::new(),
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
        if self.with_surface {
            let opt_surface = self.options_surface();
            write!(&mut self.buffer, "sf=ax3d().plot_surface(x,y,z{})\n", &opt_surface).unwrap();
        }
        if self.with_wireframe {
            let opt_wireframe = self.options_wireframe();
            write!(&mut self.buffer, "ax3d().plot_wireframe(x,y,z{})\n", &opt_wireframe).unwrap();
        }
        if self.with_points {
            let opt_points = self.options_points();
            write!(&mut self.buffer, "ax3d().scatter(x,y,z{})\n", &opt_points).unwrap();
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
    pub fn set_row_stride(&mut self, value: usize) -> &mut Self {
        self.row_stride = value;
        self
    }

    /// Sets the column stride
    pub fn set_col_stride(&mut self, value: usize) -> &mut Self {
        self.col_stride = value;
        self
    }

    /// Sets option to generate surface
    pub fn set_with_surface(&mut self, flag: bool) -> &mut Self {
        self.with_surface = flag;
        self
    }

    /// Enables the drawing of a wireframe representing the surface
    pub fn set_with_wireframe(&mut self, flag: bool) -> &mut Self {
        self.with_wireframe = flag;
        self
    }

    /// Enables the drawing of (a scatter of) points representing the surface
    pub fn set_with_points(&mut self, flag: bool) -> &mut Self {
        self.with_points = flag;
        self
    }

    // -- surface --------------------------------------------------------------------------------

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
    pub fn set_colormap_index(&mut self, index: usize) -> &mut Self {
        const CMAP: [&str; 7] = ["bwr", "RdBu", "hsv", "jet", "terrain", "pink", "Greys"];
        self.colormap_name = CMAP[index % 7].to_string();
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

    /// Sets a constant color for the surface (disables colormap)
    pub fn set_surf_color(&mut self, color: &str) -> &mut Self {
        self.surf_color = String::from(color);
        self
    }

    // -- wireframe ------------------------------------------------------------------------------

    /// Sets the color of wireframe lines
    pub fn set_wire_line_color(&mut self, color: &str) -> &mut Self {
        self.wire_line_color = String::from(color);
        self
    }

    /// Sets the style of wireframe line
    ///
    /// Options:
    ///
    /// * "`-`", "`:`", "`--`", "`-.`"
    pub fn set_wire_line_style(&mut self, style: &str) -> &mut Self {
        self.wire_line_style = String::from(style);
        self
    }

    /// Sets the width of wireframe line
    pub fn set_wire_line_width(&mut self, width: f64) -> &mut Self {
        self.wire_line_width = width;
        self
    }

    // -- scatter --------------------------------------------------------------------------------

    /// Sets the color of point markers
    pub fn set_point_color(&mut self, color: &str) -> &mut Self {
        self.point_color = String::from(color);
        self.point_void = false;
        self
    }

    /// Sets the option to draw a void point marker (edge only)
    pub fn set_point_void(&mut self, flag: bool) -> &mut Self {
        self.point_void = flag;
        self
    }

    /// Sets the edge color of point markers
    pub fn set_point_line_color(&mut self, color: &str) -> &mut Self {
        self.point_line_color = String::from(color);
        self
    }

    /// Sets the edge width of point markers
    pub fn set_point_line_width(&mut self, width: f64) -> &mut Self {
        self.point_line_width = width;
        self
    }

    /// Sets the size of point markers
    pub fn set_point_size(&mut self, size: f64) -> &mut Self {
        self.point_size = size;
        self
    }

    /// Sets the style of point markers
    ///
    /// Examples:
    ///
    /// * "`o`", "`+`"
    /// * As defined in <https://matplotlib.org/stable/api/markers_api.html>
    pub fn set_point_style(&mut self, style: &str) -> &mut Self {
        self.point_style = String::from(style);
        self
    }

    // -- options --------------------------------------------------------------------------------

    /// Returns options for surface
    fn options_surface(&self) -> String {
        let mut opt = String::new();
        if self.row_stride > 0 {
            write!(&mut opt, ",rstride={}", self.row_stride).unwrap();
        }
        if self.col_stride > 0 {
            write!(&mut opt, ",cstride={}", self.col_stride).unwrap();
        }
        if self.surf_color != "" {
            write!(&mut opt, ",color='{}'", self.surf_color).unwrap();
        } else {
            if self.colormap_name != "" {
                write!(&mut opt, ",cmap=plt.get_cmap('{}')", self.colormap_name).unwrap();
            }
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
        if self.wire_line_color != "" {
            write!(&mut opt, ",color='{}'", self.wire_line_color).unwrap();
        }
        if self.wire_line_style != "" {
            write!(&mut opt, ",linestyle='{}'", self.wire_line_style).unwrap();
        }
        if self.wire_line_width > 0.0 {
            write!(&mut opt, ",linewidth={}", self.wire_line_width).unwrap();
        }
        opt
    }

    /// Returns options for points
    fn options_points(&self) -> String {
        let mut opt = String::new();
        if self.row_stride > 0 {
            write!(&mut opt, ",rstride={}", self.row_stride).unwrap();
        }
        if self.col_stride > 0 {
            write!(&mut opt, ",cstride={}", self.col_stride).unwrap();
        }
        if self.point_line_width > 0.0 {
            write!(&mut opt, ",linewidths={}", self.point_line_width).unwrap();
        }
        if self.point_size > 0.0 {
            write!(&mut opt, ",s={}", self.point_size).unwrap();
        }
        if self.point_style != "" {
            write!(&mut opt, ",marker={}", quote_marker(&self.point_style)).unwrap();
        }
        // note: unlike contour and surface, scatter requires setting 'c=z'
        if self.point_void {
            let lc = if self.point_line_color == "" {
                "black"
            } else {
                self.point_line_color.as_str()
            };
            write!(&mut opt, ",color='none',edgecolor='{}'", lc).unwrap();
        } else if self.point_color != "" {
            write!(&mut opt, ",color='{}'", self.point_color).unwrap();
            if self.point_line_color != "" {
                write!(&mut opt, ",edgecolor='{}'", self.point_line_color).unwrap();
            }
        } else if self.colormap_name != "" {
            write!(&mut opt, ",c=z,cmap=plt.get_cmap('{}')", self.colormap_name).unwrap();
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

    /// Creates a triad aligned to an axis passing through a and b
    pub(super) fn aligned_system(a: &[f64], b: &[f64]) -> Result<(Vec<f64>, Vec<f64>, Vec<f64>), StrError> {
        // vector aligned with the axis
        let n = vec![b[0] - a[0], b[1] - a[1], b[2] - a[2]];
        let n_dot_n = n[0] * n[0] + n[1] * n[1] + n[2] * n[2];
        if n_dot_n <= f64::EPSILON {
            return Err("a-to-b segment is too short");
        }

        // arbitrary vector not parallel to n
        let x = if f64::abs(n[1]) <= f64::EPSILON && f64::abs(n[2]) <= f64::EPSILON {
            vec![n[0], n[1] + 1.0, n[2]] // parallel to x => distort along y
        } else {
            vec![n[0] + 1.0, n[1], n[2]] // distort along x
        };

        // orthogonal projection of x onto the axis
        // q = x - p = x - n * (x⋅n)/(n⋅n)
        let x_dot_n = x[0] * n[0] + x[1] * n[1] + x[2] * n[2];
        let q = vec![
            x[0] - n[0] * x_dot_n / n_dot_n,
            x[1] - n[1] * x_dot_n / n_dot_n,
            x[2] - n[2] * x_dot_n / n_dot_n,
        ];

        // local system aligned with the axis (parallel to n)
        let norm_n = f64::sqrt(n_dot_n);
        let norm_q = f64::sqrt(q[0] * q[0] + q[1] * q[1] + q[2] * q[2]);
        let e0 = vec![n[0] / norm_n, n[1] / norm_n, n[2] / norm_n];
        let e1 = vec![q[0] / norm_q, q[1] / norm_q, q[2] / norm_q];
        let e2 = vec![
            e0[1] * e1[2] - e0[2] * e1[1],
            e0[2] * e1[0] - e0[0] * e1[2],
            e0[0] * e1[1] - e0[1] * e1[0],
        ];
        Ok((e0, e1, e2))
    }
}

impl GraphMaker for Surface {
    fn get_buffer<'a>(&'a self) -> &'a String {
        &self.buffer
    }
    fn clear_buffer(&mut self) {
        self.buffer.clear();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Surface;
    use crate::GraphMaker;

    #[test]
    fn new_works() {
        let surface = Surface::new();
        assert_eq!(surface.row_stride, 0);
        assert_eq!(surface.col_stride, 0);
        assert_eq!(surface.with_surface, true);
        assert_eq!(surface.with_wireframe, false);
        assert_eq!(surface.colormap_name, "bwr".to_string());
        assert_eq!(surface.with_colorbar, false);
        assert_eq!(surface.colorbar_label.len(), 0);
        assert_eq!(surface.number_format_cb.len(), 0);
        assert_eq!(surface.wire_line_color, "black".to_string());
        assert_eq!(surface.wire_line_style.len(), 0);
        assert_eq!(surface.wire_line_width, 0.0);
        assert_eq!(surface.buffer.len(), 0);
    }

    #[test]
    fn options_surface_works() {
        let mut surface = Surface::new();
        surface.set_row_stride(3).set_col_stride(4);
        let opt = surface.options_surface();
        assert_eq!(opt, ",rstride=3,cstride=4,cmap=plt.get_cmap('bwr')");

        surface.set_colormap_name("Pastel1");
        let opt = surface.options_surface();
        assert_eq!(opt, ",rstride=3,cstride=4,cmap=plt.get_cmap('Pastel1')");

        surface.set_colormap_index(3);
        let opt = surface.options_surface();
        assert_eq!(opt, ",rstride=3,cstride=4,cmap=plt.get_cmap('jet')");

        surface.set_colormap_name("turbo");
        let opt = surface.options_surface();
        assert_eq!(opt, ",rstride=3,cstride=4,cmap=plt.get_cmap('turbo')");

        surface.set_surf_color("blue");
        let opt = surface.options_surface();
        assert_eq!(opt, ",rstride=3,cstride=4,color='blue'");
    }

    #[test]
    fn options_wireframe_works() {
        let mut surface = Surface::new();
        surface
            .set_row_stride(3)
            .set_col_stride(4)
            .set_wire_line_color("red")
            .set_wire_line_style("--")
            .set_wire_line_width(2.5);
        let opt = surface.options_wireframe();
        assert_eq!(opt, ",rstride=3,cstride=4,color='red',linestyle='--',linewidth=2.5");
    }

    #[test]
    fn options_points_works() {
        let mut surface = Surface::new();
        surface.set_row_stride(3).set_col_stride(4);
        let opt = surface.options_points();
        assert_eq!(opt, ",rstride=3,cstride=4,c=z,cmap=plt.get_cmap('bwr')");

        surface.set_colormap_name("Pastel1");
        let opt = surface.options_points();
        assert_eq!(opt, ",rstride=3,cstride=4,c=z,cmap=plt.get_cmap('Pastel1')");

        surface.set_colormap_index(3);
        let opt = surface.options_points();
        assert_eq!(opt, ",rstride=3,cstride=4,c=z,cmap=plt.get_cmap('jet')");

        surface.set_colormap_name("turbo");
        let opt = surface.options_points();
        assert_eq!(opt, ",rstride=3,cstride=4,c=z,cmap=plt.get_cmap('turbo')");

        let mut surface = Surface::new();
        surface
            .set_point_color("blue")
            .set_point_line_color("red")
            .set_point_size(100.0)
            .set_point_style("*")
            .set_point_line_width(3.0);
        let opt = surface.options_points();
        assert_eq!(opt, ",linewidths=3,s=100,marker='*',color='blue',edgecolor='red'");

        surface.set_point_void(true);
        let opt = surface.options_points();
        assert_eq!(opt, ",linewidths=3,s=100,marker='*',color='none',edgecolor='red'");

        surface.set_point_void(true).set_point_line_color("");
        let opt = surface.options_points();
        assert_eq!(opt, ",linewidths=3,s=100,marker='*',color='none',edgecolor='black'");
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
                       sf=ax3d().plot_surface(x,y,z,cmap=plt.get_cmap('bwr'))\n\
                       ax3d().plot_wireframe(x,y,z,color='black')\n\
                       cb=plt.colorbar(sf)\n\
                       cb.ax.set_ylabel(r'temperature')\n";
        assert_eq!(surface.buffer, b);
        surface.clear_buffer();
        assert_eq!(surface.buffer, "");
    }

    #[test]
    fn aligned_system_fails_on_wrong_input() {
        let res = Surface::aligned_system(&[0.0, 0.0, 0.0], &[0.0, 0.0, 0.0]);
        assert_eq!(res.err(), Some("a-to-b segment is too short"));
    }

    fn approx_eq(a: f64, b: f64, tol: f64) {
        let diff = f64::abs(a - b);
        if diff > tol {
            panic!("numbers are not approximately equal. diff = {:?}", diff);
        }
    }

    #[test]
    #[should_panic(expected = "numbers are not approximately equal. diff = 1.0")]
    fn approx_eq_captures_errors() {
        approx_eq(1.0, 2.0, 1e-15);
    }

    #[test]
    fn aligned_system_works() {
        let (e0, e1, e2) = Surface::aligned_system(&[-1.0, 0.0, 0.0], &[8.0, 0.0, 0.0]).unwrap();
        assert_eq!(e0, &[1.0, 0.0, 0.0]);
        assert_eq!(e1, &[0.0, 1.0, 0.0]);
        assert_eq!(e2, &[0.0, 0.0, 1.0]);

        let (e0, e1, e2) = Surface::aligned_system(&[0.0, -3.0, 0.0], &[0.0, 3.0, 0.0]).unwrap();
        assert_eq!(e0, &[0.0, 1.0, 0.0]);
        assert_eq!(e1, &[1.0, 0.0, 0.0]);
        assert_eq!(e2, &[0.0, 0.0, -1.0]);

        let (e0, e1, e2) = Surface::aligned_system(&[0.0, 10.0, 0.0], &[0.0, 3.0, 0.0]).unwrap();
        assert_eq!(e0, &[0.0, -1.0, 0.0]);
        assert_eq!(e1, &[1.0, 0.0, 0.0]);
        assert_eq!(e2, &[0.0, 0.0, 1.0]);

        let (e0, e1, e2) = Surface::aligned_system(&[0.0, 0.0, 80.0], &[0.0, 0.0, 7770.0]).unwrap();
        assert_eq!(e0, &[0.0, 0.0, 1.0]);
        assert_eq!(e1, &[1.0, 0.0, 0.0]);
        assert_eq!(e2, &[0.0, 1.0, 0.0]);

        let (m, n, l) = (3.0, 4.0, 5.0);
        let (e0, e1, e2) = Surface::aligned_system(&[2.0, -7.0, 5.0], &[2.0 + m, -7.0 + n, 5.0]).unwrap();
        let correct0 = &[m / l, n / l, 0.0];
        let correct1 = &[n / l, -m / l, 0.0];
        let correct2 = &[0.0, 0.0, -1.0];
        for i in 0..3 {
            approx_eq(e0[i], correct0[i], 1e-15);
            approx_eq(e1[i], correct1[i], 1e-15);
            approx_eq(e2[i], correct2[i], 1e-15);
        }

        let s = f64::sqrt(2.0) / 2.0;
        let (e0, e1, e2) = Surface::aligned_system(&[0.0, 0.0, 1.0], &[1.0, 0.0, 2.0]).unwrap();
        let correct0 = &[s, 0.0, s];
        let correct1 = &[s, 0.0, -s];
        let correct2 = &[0.0, 1.0, 0.0];
        for i in 0..3 {
            approx_eq(e0[i], correct0[i], 1e-15);
            approx_eq(e1[i], correct1[i], 1e-15);
            approx_eq(e2[i], correct2[i], 1e-15);
        }

        let (c, d, e) = (1.0 / f64::sqrt(3.0), 1.0 / f64::sqrt(6.0), 1.0 / f64::sqrt(2.0));
        let (e0, e1, e2) = Surface::aligned_system(&[3.0, 4.0, 5.0], &[13.0, 14.0, 15.0]).unwrap();
        let correct0 = &[c, c, c];
        let correct1 = &[2.0 * d, -d, -d];
        let correct2 = &[0.0, e, -e];
        for i in 0..3 {
            approx_eq(e0[i], correct0[i], 1e-15);
            approx_eq(e1[i], correct1[i], 1e-15);
            approx_eq(e2[i], correct2[i], 1e-15);
        }
    }
}
