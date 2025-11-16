use super::{GraphMaker, StrError};
use crate::conversions::{matrix_to_array, vector_to_array};
use crate::{AsMatrix, AsVector};
use num_traits::Num;
use std::fmt::Write;

/// Defines the poly-curve code
///
/// Reference: [Matplotlib](https://matplotlib.org/stable/api/path_api.html)
#[derive(Clone, Copy, Debug)]
pub enum PolyCode {
    /// Move to coordinate (first point)
    ///
    /// Matplotlib: Pick up the pen and move to the given vertex.
    MoveTo,

    /// Segment (next point, need 2 points)
    ///
    /// Matplotlib: Draw a line from the current position to the given vertex.
    LineTo,

    /// Quadratic Bezier (next point, need 3 control points with the first and last points on the curve)
    ///
    /// Matplotlib: Draw a quadratic Bezier curve from the current position, with the given control point, to the given end point.
    Curve3,

    /// Cubic Bezier (next point, need 4 control points with the first and last points on the curve)
    ///
    /// Matplotlib: Draw a cubic Bezier curve from the current position, with the given control points, to the given end point.
    Curve4,
}

/// Implements functions to draw 2D and 3D features, including poly-lines and Bezier curves
///
/// # Examples
///
/// ## Drawing functions with polyline set by an array
///
/// ```
/// use plotpy::{Canvas, Plot};
///
/// fn main() -> Result<(), &'static str> {
///     // canvas object and common options
///     let mut canvas = Canvas::new();
///     canvas.set_line_width(3.0).set_edge_color("#cd0000").set_face_color("#eeea83");
///
///     // draw arc
///     canvas.draw_arc(0.5, 0.5, 0.4, 195.0, -15.0);
///
///     // draw arrow
///     canvas.set_arrow_scale(50.0).set_arrow_style("fancy");
///     canvas.draw_arrow(0.4, 0.3, 0.6, 0.5);
///
///     // draw circle
///     canvas.set_face_color("None").set_edge_color("#1f9c25").set_line_width(6.0);
///     canvas.draw_circle(0.5, 0.5, 0.5);
///
///     // draw polyline
///     canvas.set_line_width(3.0).set_edge_color("blue");
///     let a = 0.2;
///     let c = f64::sqrt(3.0) / 2.0;
///     let p = &[[0.1, 0.5], [0.1 + a, 0.5], [0.1 + a / 2.0, 0.5 + a * c]];
///     let q = &[[0.9, 0.5], [0.9 - a, 0.5], [0.9 - a / 2.0, 0.5 + a * c]];
///     canvas.draw_polyline(p, true);
///     canvas.draw_polyline(q, false);
///
///     // add canvas to plot
///     let mut plot = Plot::new();
///     plot.set_hide_axes(true)
///         .set_equal_axes(true)
///         .set_range(-0.05, 1.05, -0.05, 1.05)
///         .add(&canvas);
///
///     // save figure
///     plot.save("/tmp/plotpy/doc_tests/doc_canvas.svg")?;
///     Ok(())
/// }
/// ```
///
/// ![doc_canvas.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/doc_canvas.svg)
///
/// ## Cubic Bezier and use of begin/end functions
///
/// ```
/// use plotpy::{Canvas, Plot, PolyCode, StrError};
///
/// fn main() -> Result<(), StrError> {
///     // codes
///     let data = [
///         (3.0, 0.0, PolyCode::MoveTo),
///         (1.0, 1.5, PolyCode::Curve4),
///         (0.0, 4.0, PolyCode::Curve4),
///         (2.5, 3.9, PolyCode::Curve4),
///         (3.0, 3.8, PolyCode::LineTo),
///         (3.5, 3.9, PolyCode::LineTo),
///         (6.0, 4.0, PolyCode::Curve4),
///         (5.0, 1.5, PolyCode::Curve4),
///         (3.0, 0.0, PolyCode::Curve4),
///     ];
///
///     // polycurve
///     let mut canvas = Canvas::new();
///     canvas.set_face_color("#f88989").set_edge_color("red");
///     canvas.polycurve_begin();
///     for (x, y, code) in data {
///         canvas.polycurve_add(x, y, code);
///     }
///     canvas.polycurve_end(true);
///
///     // add canvas to plot
///     let mut plot = Plot::new();
///     plot.add(&canvas);
///
///     // save figure
///     plot.set_range(1.0, 5.0, 0.0, 4.0)
///         .set_frame_borders(false)
///         .set_hide_axes(true)
///         .set_equal_axes(true)
///         .set_show_errors(true);
///     plot.save("/tmp/plotpy/doc_tests/doc_canvas_polycurve.svg")?;
///     Ok(())
/// }
/// ```
///
/// ![doc_canvas_polycurve.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/doc_canvas_polycurve.svg)
///
/// See also integration tests in the [tests directory](https://github.com/cpmech/plotpy/tree/main/tests)
pub struct Canvas {
    // features
    edge_color: String,  // Edge color (shared)
    face_color: String,  // Face color (shared)
    line_width: f64,     // Line width of edge (shared)
    line_style: String,  // Style of lines (shared)
    arrow_scale: f64,    // Arrow scale
    arrow_style: String, // Arrow style

    // text
    text_color: String,            // Text color
    text_align_horizontal: String, // Horizontal alignment
    text_align_vertical: String,   // Vertical alignment
    text_fontsize: f64,            // Font size
    text_rotation: f64,            // Text rotation

    // alternative text
    alt_text_color: String,            // Text color
    alt_text_align_horizontal: String, // Horizontal alignment
    alt_text_align_vertical: String,   // Vertical alignment
    alt_text_fontsize: f64,            // Font size
    alt_text_rotation: f64,            // Text rotation

    // options
    stop_clip: bool, // Stop clipping features within margins
    shading: bool,   // Shading for 3D surfaces (currently used only in draw_triangles_3d). Default = true

    // buffer
    buffer: String, // buffer
}

impl Canvas {
    /// Creates a new Canvas object
    pub fn new() -> Self {
        Canvas {
            // features
            edge_color: "#427ce5".to_string(),
            face_color: String::new(),
            line_width: 0.0,
            line_style: String::new(),
            arrow_scale: 0.0,
            arrow_style: String::new(),
            // text
            text_color: "#343434".to_string(),
            text_align_horizontal: "center".to_string(),
            text_align_vertical: "center".to_string(),
            text_fontsize: 10.0,
            text_rotation: 0.0,
            // alternative text
            alt_text_color: "#a81414".to_string(),
            alt_text_align_horizontal: String::new(),
            alt_text_align_vertical: String::new(),
            alt_text_fontsize: 8.0,
            alt_text_rotation: 45.0,
            // options
            stop_clip: false,
            shading: true,
            // buffer
            buffer: String::new(),
        }
    }

    /// Draws arc (2D only)
    pub fn draw_arc<T>(&mut self, xc: T, yc: T, r: T, ini_angle: T, fin_angle: T)
    where
        T: std::fmt::Display + Num,
    {
        let opt = self.options_shared();
        write!(
            &mut self.buffer,
            "p=pat.Arc(({},{}),2*{},2*{},theta1={},theta2={},angle=0{})\n\
             plt.gca().add_patch(p)\n",
            xc, yc, r, r, ini_angle, fin_angle, &opt
        )
        .unwrap();
    }

    /// Draws arrow (2D only)
    pub fn draw_arrow<T>(&mut self, xi: T, yi: T, xf: T, yf: T)
    where
        T: std::fmt::Display + Num,
    {
        let opt_shared = self.options_shared();
        let opt_arrow = self.options_arrow();
        write!(
            &mut self.buffer,
            "p=pat.FancyArrowPatch(({},{}),({},{})\
                    ,shrinkA=0,shrinkB=0\
                    ,path_effects=[pff.Stroke(joinstyle='miter')]\
                    {}{})\n\
             plt.gca().add_patch(p)\n",
            xi, yi, xf, yf, &opt_shared, &&opt_arrow,
        )
        .unwrap();
    }

    /// Draws circle (2D only)
    pub fn draw_circle<T>(&mut self, xc: T, yc: T, r: T)
    where
        T: std::fmt::Display + Num,
    {
        let opt = self.options_shared();
        write!(
            &mut self.buffer,
            "p=pat.Circle(({},{}),{}{})\n\
             plt.gca().add_patch(p)\n",
            xc, yc, r, &opt
        )
        .unwrap();
    }

    /// Draws triangles (2D only)
    ///
    /// Using <https://matplotlib.org/stable/api/_as_gen/matplotlib.pyplot.triplot.html>
    pub fn draw_triangles<'a, T, U, C>(&mut self, xx: &'a T, yy: &'a T, connectivity: &'a C) -> &mut Self
    where
        T: AsVector<'a, U>,
        U: 'a + std::fmt::Display + Num,
        C: AsMatrix<'a, usize>,
    {
        vector_to_array(&mut self.buffer, "xx", xx);
        vector_to_array(&mut self.buffer, "yy", yy);
        matrix_to_array(&mut self.buffer, "triangles", connectivity);
        let opt = self.options_triangles();
        write!(&mut self.buffer, "plt.triplot(xx,yy,triangles{})\n", &opt).unwrap();
        self
    }

    /// Draws triangles (3D only)
    ///
    /// Using <https://matplotlib.org/stable/api/_as_gen/mpl_toolkits.mplot3d.axes3d.Axes3D.plot_trisurf.html#mpl_toolkits.mplot3d.axes3d.Axes3D.plot_trisurf>
    ///
    /// Note: There is no way to set shading and facecolor at the same time.
    pub fn draw_triangles_3d<'a, T, U, C>(&mut self, xx: &'a T, yy: &'a T, zz: &'a T, connectivity: &'a C) -> &mut Self
    where
        T: AsVector<'a, U>,
        U: 'a + std::fmt::Display + Num,
        C: AsMatrix<'a, usize>,
    {
        // write arrays
        vector_to_array(&mut self.buffer, "xx", xx);
        vector_to_array(&mut self.buffer, "yy", yy);
        vector_to_array(&mut self.buffer, "zz", zz);
        matrix_to_array(&mut self.buffer, "triangles", connectivity);

        // Issue when setting facecolor directly:
        //
        // In matplotlib 3.6+, passing facecolors as a parameter directly to plot_trisurf() doesn't work.
        // The solution is:
        // * Create the surface first with shade=False
        // * Then use set_facecolor() to apply the colors after the surface is created
        //
        // Also, there is no way to set shading and facecolor at the same time.

        // get options without facecolor
        let opt = self.options_triangles_3d();

        // write Python command
        let shade = if self.shading { "True" } else { "False" };
        write!(
            &mut self.buffer,
            "poly_collection=ax3d().plot_trisurf(xx,yy,zz,triangles=triangles,shade={}{})\n",
            shade, &opt
        )
        .unwrap();

        // set facecolor if specified
        if self.face_color != "" {
            write!(
                &mut self.buffer,
                "colors=np.array(['{}']*len(triangles))\n\
                poly_collection.set_facecolor(colors)\n",
                self.face_color
            )
            .unwrap();
        }

        // done
        self
    }

    /// Begins drawing a polycurve (straight segments, quadratic Bezier, and cubic Bezier) (2D only)
    ///
    /// # Warning
    ///
    /// You must call [Canvas::polycurve_add] next, followed by [Canvas::polycurve_end] when finishing adding points.
    /// Otherwise, Python/Matplotlib will fail.
    pub fn polycurve_begin(&mut self) -> &mut Self {
        write!(&mut self.buffer, "dat=[",).unwrap();
        self
    }

    /// Adds point to a polycurve (straight segments, quadratic Bezier, and cubic Bezier) (2D only)
    ///
    /// # Warning
    ///
    /// You must call [Canvas::polycurve_begin] first, otherwise Python/Matplotlib will fail.
    /// Afterwards, you must call [Canvas::polycurve_end] when finishing adding points.
    pub fn polycurve_add<T>(&mut self, x: T, y: T, code: PolyCode) -> &mut Self
    where
        T: std::fmt::Display + Num,
    {
        let keyword = match code {
            PolyCode::MoveTo => "MOVETO",
            PolyCode::LineTo => "LINETO",
            PolyCode::Curve3 => "CURVE3",
            PolyCode::Curve4 => "CURVE4",
        };
        write!(&mut self.buffer, "[pth.Path.{},({},{})],", keyword, x, y).unwrap();
        self
    }

    /// Ends drawing a polycurve (straight segments, quadratic Bezier, and cubic Bezier) (2D only)
    ///
    /// # Warning
    ///
    /// This function must be the last one called after [Canvas::polycurve_begin] and [Canvas::polycurve_add].
    /// Otherwise, Python/Matplotlib will fail.
    pub fn polycurve_end(&mut self, closed: bool) -> &mut Self {
        if closed {
            write!(&mut self.buffer, "[pth.Path.CLOSEPOLY,(None,None)]").unwrap();
        }
        let opt = self.options_shared();
        write!(
            &mut self.buffer,
            "]\n\
            cmd,pts=zip(*dat)\n\
            h=pth.Path(pts,cmd)\n\
            p=pat.PathPatch(h{})\n\
            plt.gca().add_patch(p)\n",
            &opt
        )
        .unwrap();
        self
    }

    /// Draws polyline with straight segments, quadratic Bezier, or cubic Bezier (2D only)
    ///
    /// **Note:** The first and last commands are ignored.
    pub fn draw_polycurve<'a, T, U>(&mut self, points: &'a T, codes: &[PolyCode], closed: bool) -> Result<(), StrError>
    where
        T: AsMatrix<'a, U>,
        U: 'a + std::fmt::Display,
    {
        let (npoint, ndim) = points.size();
        if npoint < 3 {
            return Err("npoint must be ≥ 3");
        }
        if ndim != 2 {
            return Err("ndim must be equal to 2");
        }
        if codes.len() != npoint {
            return Err("codes.len() must be equal to npoint");
        }
        write!(
            &mut self.buffer,
            "dat=[[pth.Path.MOVETO,({},{})]",
            points.at(0, 0),
            points.at(0, 1)
        )
        .unwrap();
        for i in 1..npoint {
            let keyword = match codes[i] {
                PolyCode::MoveTo => "MOVETO",
                PolyCode::LineTo => "LINETO",
                PolyCode::Curve3 => "CURVE3",
                PolyCode::Curve4 => "CURVE4",
            };
            write!(
                &mut self.buffer,
                ",[pth.Path.{},({},{})]",
                keyword,
                points.at(i, 0),
                points.at(i, 1)
            )
            .unwrap();
        }
        if closed {
            write!(&mut self.buffer, ",[pth.Path.CLOSEPOLY,(None,None)]").unwrap();
        }
        let opt = self.options_shared();
        write!(
            &mut self.buffer,
            "]\n\
            cmd,pts=zip(*dat)\n\
            h=pth.Path(pts,cmd)\n\
            p=pat.PathPatch(h{})\n\
            plt.gca().add_patch(p)\n",
            &opt
        )
        .unwrap();
        Ok(())
    }

    /// Begins adding points to a 3D polyline
    ///
    /// # Warning
    ///
    /// This function must be followed by [Canvas::polyline_3d_add] and [Canvas::polyline_3d_end],
    /// otherwise Python/Matplotlib will fail
    pub fn polyline_3d_begin(&mut self) -> &mut Self {
        write!(&mut self.buffer, "xyz=np.array([").unwrap();
        self
    }

    /// Adds point to a 3D polyline
    ///
    /// # Warning
    ///
    /// This function must be called after [Canvas::polyline_3d_begin] and must be followed by [Canvas::polyline_3d_end],
    /// otherwise Python/Matplotlib will fail.
    pub fn polyline_3d_add<T>(&mut self, x: T, y: T, z: T) -> &mut Self
    where
        T: std::fmt::Display + Num,
    {
        write!(&mut self.buffer, "[{},{},{}],", x, y, z).unwrap();
        self
    }

    /// Ends adding points to a 3D polyline
    ///
    /// # Warning
    ///
    /// This function must be called after [Canvas::polyline_3d_begin] and [Canvas::polyline_3d_add],
    /// otherwise Python/Matplotlib will fail.
    pub fn polyline_3d_end(&mut self) -> &mut Self {
        let opt = self.options_line_3d();
        write!(
            &mut self.buffer,
            "])\nax3d().plot(xyz[:,0],xyz[:,1],xyz[:,2]{})\n",
            &opt
        )
        .unwrap();
        self
    }

    /// Draws polyline (2D or 3D)
    pub fn draw_polyline<'a, T, U>(&mut self, points: &'a T, closed: bool)
    where
        T: AsMatrix<'a, U>,
        U: 'a + std::fmt::Display + Num,
    {
        let (npoint, ndim) = points.size();
        if npoint < 2 {
            return;
        }
        if ndim == 2 {
            write!(
                &mut self.buffer,
                "dat=[[pth.Path.MOVETO,({},{})]",
                points.at(0, 0),
                points.at(0, 1)
            )
            .unwrap();
            for i in 1..npoint {
                write!(
                    &mut self.buffer,
                    ",[pth.Path.LINETO,({},{})]",
                    points.at(i, 0),
                    points.at(i, 1)
                )
                .unwrap();
            }
            if closed {
                write!(&mut self.buffer, ",[pth.Path.CLOSEPOLY,(None,None)]").unwrap();
            }
            let opt = self.options_shared();
            write!(
                &mut self.buffer,
                "]\n\
                cmd,pts=zip(*dat)\n\
                h=pth.Path(pts,cmd)\n\
                p=pat.PathPatch(h{})\n\
                plt.gca().add_patch(p)\n",
                &opt
            )
            .unwrap();
        }
        if ndim == 3 {
            self.polyline_3d_begin();
            for i in 0..npoint {
                self.polyline_3d_add(points.at(i, 0), points.at(i, 1), points.at(i, 2));
            }
            if closed && npoint > 2 {
                self.polyline_3d_add(points.at(0, 0), points.at(0, 1), points.at(0, 2));
            }
            self.polyline_3d_end();
        }
    }

    /// Draws a rectangle
    pub fn draw_rectangle<T>(&mut self, x: T, y: T, width: T, height: T) -> &mut Self
    where
        T: std::fmt::Display + Num,
    {
        let opt = self.options_shared();
        write!(
            &mut self.buffer,
            "p=pat.Rectangle(({},{}),{},{}{})\n\
             plt.gca().add_patch(p)\n",
            x, y, width, height, &opt
        )
        .unwrap();
        self
    }

    /// Draws a text in a 2D graph
    pub fn draw_text<T>(&mut self, x: T, y: T, label: &str) -> &mut Self
    where
        T: std::fmt::Display + Num,
    {
        self.text(2, &[x, y, T::zero()], label, false);
        self
    }

    /// Draws an alternative text in a 2D graph
    pub fn draw_alt_text<T>(&mut self, x: T, y: T, label: &str) -> &mut Self
    where
        T: std::fmt::Display + Num,
    {
        self.text(2, &[x, y, T::zero()], label, true);
        self
    }

    /// Draws a 2D or 3D grid
    ///
    /// # Input
    ///
    /// * `xmin, xmax` -- min and max coordinates (len = 2 or 3 == ndim)
    /// * `ndiv` -- number of divisions along each dimension (len = 2 or 3 == ndim)
    ///
    /// **Note:** See the `set_text_...` and `set_alt_text_...` functions to configure
    /// the cell and point labels, respectively.
    pub fn draw_grid(
        &mut self,
        xmin: &[f64],
        xmax: &[f64],
        ndiv: &[usize],
        with_point_ids: bool,
        with_cell_ids: bool,
    ) -> Result<(), StrError> {
        // check input
        let ndim = ndiv.len();
        if ndim < 2 || ndim > 3 {
            return Err("len(ndiv) == ndim must be 2 or 3");
        }
        if xmin.len() != ndim {
            return Err("size of xmin must equal ndim == len(ndiv)");
        }
        if xmax.len() != ndim {
            return Err("size of xmax must equal ndim == len(ndiv)");
        }

        // compute delta
        let mut npoint = [1; 3];
        let mut delta = [0.0; 3];
        for i in 0..ndim {
            npoint[i] = ndiv[i] + 1;
            delta[i] = xmax[i] - xmin[i];
            if delta[i] <= 0.0 {
                return Err("xmax must be greater than xmin");
            }
            delta[i] /= ndiv[i] as f64;
        }

        // auxiliary points
        let mut a = [0.0; 3];
        let mut b = [0.0; 3];

        // loop over lines
        if ndim == 2 {
            write!(&mut self.buffer, "dat=[\n").unwrap();
        }
        let opt = self.options_shared();
        let mut id_point = 0;
        for k in 0..npoint[2] {
            if ndim == 3 {
                a[2] = xmin[2] + delta[2] * (k as f64);
                b[2] = a[2];
            }

            // vertical lines
            a[1] = xmin[1];
            b[1] = xmax[1];
            for i in 0..npoint[0] {
                a[0] = xmin[0] + delta[0] * (i as f64);
                b[0] = a[0];
                self.line(ndim, &a, &b);
            }

            // horizontal lines
            a[0] = xmin[0];
            b[0] = xmax[0];
            for j in 0..npoint[1] {
                a[1] = xmin[1] + delta[1] * (j as f64);
                b[1] = a[1];
                self.line(ndim, &a, &b);
            }

            // add patch
            if ndim == 2 {
                write!(
                    &mut self.buffer,
                    "]\n\
                    cmd,pts=zip(*dat)\n\
                    h=pth.Path(pts,cmd)\n\
                    p=pat.PathPatch(h{})\n\
                    plt.gca().add_patch(p)\n",
                    &opt
                )
                .unwrap();
            }

            // labels
            if with_point_ids {
                for j in 0..npoint[1] {
                    a[1] = xmin[1] + delta[1] * (j as f64);
                    for i in 0..npoint[0] {
                        a[0] = xmin[0] + delta[0] * (i as f64);
                        let txt = format!("{}", id_point);
                        self.text(ndim, &a, &txt, true);
                        id_point += 1;
                    }
                }
            }
        }

        // cell ids
        if with_cell_ids {
            let mut id_cell = 0;
            let nz = if ndim == 2 { 1 } else { ndiv[2] };
            for k in 0..nz {
                if ndim == 3 {
                    a[2] = xmin[2] + delta[2] * (k as f64);
                    b[2] = a[2] + delta[2] / 2.0;
                }
                for j in 0..ndiv[1] {
                    a[1] = xmin[1] + delta[1] * (j as f64);
                    b[1] = a[1] + delta[1] / 2.0;
                    for i in 0..ndiv[0] {
                        a[0] = xmin[0] + delta[0] * (i as f64);
                        b[0] = a[0] + delta[0] / 2.0;
                        let txt = format!("{}", id_cell);
                        self.text(ndim, &b, &txt, false);
                        id_cell += 1;
                    }
                }
            }
        }

        // z-lines
        if ndim == 3 {
            a[2] = xmin[2];
            b[2] = xmax[2];
            for j in 0..npoint[1] {
                a[1] = xmin[1] + delta[1] * (j as f64);
                b[1] = a[1];
                for i in 0..npoint[0] {
                    a[0] = xmin[0] + delta[0] * (i as f64);
                    b[0] = a[0];
                    self.line(ndim, &a, &b);
                }
            }
        }

        // adjust limits
        self.limits(ndim, xmin, xmax);

        // done
        Ok(())
    }

    /// Sets the edge color (shared among features)
    pub fn set_edge_color(&mut self, color: &str) -> &mut Self {
        self.edge_color = String::from(color);
        self
    }

    /// Sets the face color (shared among features)
    pub fn set_face_color(&mut self, color: &str) -> &mut Self {
        self.face_color = String::from(color);
        self
    }

    /// Sets the line width of edge (shared among features)
    pub fn set_line_width(&mut self, width: f64) -> &mut Self {
        self.line_width = width;
        self
    }

    /// Sets the line width of edge (shared among features)
    ///
    /// Options:
    ///
    /// * "`-`", `:`", "`--`", "`-.`", or "`None`"
    /// * As defined in <https://matplotlib.org/stable/gallery/lines_bars_and_markers/linestyles.html>
    pub fn set_line_style(&mut self, style: &str) -> &mut Self {
        self.line_style = String::from(style);
        self
    }

    /// Sets the arrow scale
    pub fn set_arrow_scale(&mut self, scale: f64) -> &mut Self {
        self.arrow_scale = scale;
        self
    }

    /// Sets the arrow style
    ///
    /// Options:
    ///
    /// * "`-`"      -- Curve         : None
    /// * "`->`"     -- CurveB        : head_length=0.4,head_width=0.2
    /// * "`-[`"     -- BracketB      : widthB=1.0,lengthB=0.2,angleB=None
    /// * "`-|>`"    -- CurveFilledB  : head_length=0.4,head_width=0.2
    /// * "`<-`"     -- CurveA        : head_length=0.4,head_width=0.2
    /// * "`<->`"    -- CurveAB       : head_length=0.4,head_width=0.2
    /// * "`<|-`"    -- CurveFilledA  : head_length=0.4,head_width=0.2
    /// * "`<|-|>`"  -- CurveFilledAB : head_length=0.4,head_width=0.2
    /// * "`]-`"     -- BracketA      : widthA=1.0,lengthA=0.2,angleA=None
    /// * "`]-[`"    -- BracketAB     : widthA=1.0,lengthA=0.2,angleA=None,widthB=1.0,lengthB=0.2,angleB=None
    /// * "`fancy`"  -- Fancy         : head_length=0.4,head_width=0.4,tail_width=0.4
    /// * "`simple`" -- Simple        : head_length=0.5,head_width=0.5,tail_width=0.2
    /// * "`wedge`"  -- Wedge         : tail_width=0.3,shrink_factor=0.5
    /// * "`|-|`"    -- BarAB         : widthA=1.0,angleA=None,widthB=1.0,angleB=None
    /// * As defined in <https://matplotlib.org/stable/api/_as_gen/matplotlib.patches.FancyArrowPatch.html>
    pub fn set_arrow_style(&mut self, style: &str) -> &mut Self {
        self.arrow_style = String::from(style);
        self
    }

    /// Sets the text color
    pub fn set_text_color(&mut self, color: &str) -> &mut Self {
        self.text_color = String::from(color);
        self
    }

    /// Sets the text horizontal alignment
    ///
    /// Options: "center", "left", "right"
    pub fn set_text_align_horizontal(&mut self, option: &str) -> &mut Self {
        self.text_align_horizontal = String::from(option);
        self
    }

    /// Sets the text vertical alignment
    ///
    /// Options: "center", "top", "bottom", "baseline", "center_baseline"
    pub fn set_text_align_vertical(&mut self, option: &str) -> &mut Self {
        self.text_align_vertical = String::from(option);
        self
    }

    /// Sets the text font size
    pub fn set_text_fontsize(&mut self, fontsize: f64) -> &mut Self {
        self.text_fontsize = fontsize;
        self
    }

    /// Sets the text rotation
    pub fn set_text_rotation(&mut self, rotation: f64) -> &mut Self {
        self.text_rotation = rotation;
        self
    }

    /// Sets the alternative text color
    pub fn set_alt_text_color(&mut self, color: &str) -> &mut Self {
        self.alt_text_color = String::from(color);
        self
    }

    /// Sets the alternative text horizontal alignment
    ///
    /// Options: "center", "left", "right"
    pub fn set_alt_text_align_horizontal(&mut self, option: &str) -> &mut Self {
        self.alt_text_align_horizontal = String::from(option);
        self
    }

    /// Sets the alternative text vertical alignment
    ///
    /// Options: "center", "top", "bottom", "baseline", "center_baseline"
    pub fn set_alt_text_align_vertical(&mut self, option: &str) -> &mut Self {
        self.alt_text_align_vertical = String::from(option);
        self
    }

    /// Sets the alternative text font size
    pub fn set_alt_text_fontsize(&mut self, fontsize: f64) -> &mut Self {
        self.alt_text_fontsize = fontsize;
        self
    }

    /// Sets the alternative text rotation
    pub fn set_alt_text_rotation(&mut self, rotation: f64) -> &mut Self {
        self.alt_text_rotation = rotation;
        self
    }

    /// Sets the flag to stop clipping features within margins
    pub fn set_stop_clip(&mut self, flag: bool) -> &mut Self {
        self.stop_clip = flag;
        self
    }

    /// Sets shading for 3D surfaces (currently used only in draw_triangles_3d)
    ///
    /// Note: Shading is disabled if facecolor is non-empty.
    ///
    /// Default = true
    pub fn set_shading(&mut self, flag: bool) -> &mut Self {
        self.shading = flag;
        self
    }

    /// Returns options for triangles (2D only)
    fn options_triangles(&self) -> String {
        let mut opt = String::new();
        if self.edge_color != "" {
            write!(&mut opt, ",color='{}'", self.edge_color).unwrap();
        }
        if self.line_width > 0.0 {
            write!(&mut opt, ",linewidth={}", self.line_width).unwrap();
        }
        if self.line_style != "" {
            write!(&mut opt, ",linestyle='{}'", self.line_style).unwrap();
        }
        if self.stop_clip {
            write!(&mut opt, ",clip_on=False").unwrap();
        }
        opt
    }

    /// Returns shared options
    fn options_triangles_3d(&self) -> String {
        let mut opt = String::new();
        if self.edge_color != "" {
            write!(&mut opt, ",edgecolor='{}'", self.edge_color).unwrap();
        }
        if self.line_width > 0.0 {
            write!(&mut opt, ",linewidth={}", self.line_width).unwrap();
        }
        if self.line_style != "" {
            write!(&mut opt, ",linestyle='{}'", self.line_style).unwrap();
        }
        if self.stop_clip {
            write!(&mut opt, ",clip_on=False").unwrap();
        }
        opt
    }

    /// Returns shared options
    fn options_shared(&self) -> String {
        let mut opt = String::new();
        if self.edge_color != "" {
            write!(&mut opt, ",edgecolor='{}'", self.edge_color).unwrap();
        }
        if self.face_color != "" {
            write!(&mut opt, ",facecolor='{}'", self.face_color).unwrap();
        }
        if self.line_width > 0.0 {
            write!(&mut opt, ",linewidth={}", self.line_width).unwrap();
        }
        if self.line_style != "" {
            write!(&mut opt, ",linestyle='{}'", self.line_style).unwrap();
        }
        if self.stop_clip {
            write!(&mut opt, ",clip_on=False").unwrap();
        }
        opt
    }

    /// Returns options for arrows
    fn options_arrow(&self) -> String {
        let mut opt = String::new();
        if self.arrow_scale > 0.0 {
            write!(&mut opt, ",mutation_scale={}", self.arrow_scale).unwrap();
        }
        if self.arrow_style != "" {
            write!(&mut opt, ",arrowstyle='{}'", self.arrow_style).unwrap();
        }
        opt
    }

    /// Returns options for text
    fn options_text(&self) -> String {
        let mut opt = String::new();
        if self.text_color != "" {
            write!(&mut opt, ",color='{}'", self.text_color).unwrap();
        }
        if self.text_align_horizontal != "" {
            write!(&mut opt, ",ha='{}'", self.text_align_horizontal).unwrap();
        }
        if self.text_align_vertical != "" {
            write!(&mut opt, ",va='{}'", self.text_align_vertical).unwrap();
        }
        if self.text_fontsize > 0.0 {
            write!(&mut opt, ",fontsize={}", self.text_fontsize).unwrap();
        }
        if self.text_rotation > 0.0 {
            write!(&mut opt, ",rotation={}", self.text_rotation).unwrap();
        }
        opt
    }

    /// Returns options for alternative text
    fn options_alt_text(&self) -> String {
        let mut opt = String::new();
        if self.alt_text_color != "" {
            write!(&mut opt, ",color='{}'", self.alt_text_color).unwrap();
        }
        if self.alt_text_align_horizontal != "" {
            write!(&mut opt, ",ha='{}'", self.alt_text_align_horizontal).unwrap();
        }
        if self.alt_text_align_vertical != "" {
            write!(&mut opt, ",va='{}'", self.alt_text_align_vertical).unwrap();
        }
        if self.alt_text_fontsize > 0.0 {
            write!(&mut opt, ",fontsize={}", self.alt_text_fontsize).unwrap();
        }
        if self.alt_text_rotation > 0.0 {
            write!(&mut opt, ",rotation={}", self.alt_text_rotation).unwrap();
        }
        opt
    }

    /// Returns options for 3D line
    fn options_line_3d(&self) -> String {
        let mut opt = String::new();
        if self.edge_color != "" {
            write!(&mut opt, ",color='{}'", self.edge_color).unwrap();
        }
        if self.line_width > 0.0 {
            write!(&mut opt, ",linewidth={}", self.line_width).unwrap();
        }
        if self.line_style != "" {
            write!(&mut opt, ",linestyle='{}'", self.line_style).unwrap();
        }
        opt
    }

    /// Draws 2D or 3D line
    fn line<T>(&mut self, ndim: usize, a: &[T; 3], b: &[T; 3])
    where
        T: std::fmt::Display,
    {
        if ndim == 2 {
            write!(
                &mut self.buffer,
                "    [pth.Path.MOVETO,({},{})],[pth.Path.LINETO,({},{})],\n",
                a[0], a[1], b[0], b[1]
            )
            .unwrap();
        } else {
            let opt = self.options_line_3d();
            write!(
                &mut self.buffer,
                "ax3d().plot([{},{}],[{},{}],[{},{}]{})\n",
                a[0], b[0], a[1], b[1], a[2], b[2], opt,
            )
            .unwrap();
        }
    }

    /// Draws 2D or 3D text
    fn text<T>(&mut self, ndim: usize, a: &[T; 3], txt: &str, alternative: bool)
    where
        T: std::fmt::Display,
    {
        let opt = if alternative {
            self.options_alt_text()
        } else {
            self.options_text()
        };
        if ndim == 2 {
            write!(&mut self.buffer, "plt.text({},{},'{}'{})\n", a[0], a[1], txt, &opt).unwrap();
        } else {
            write!(
                &mut self.buffer,
                "ax3d().text({},{},{},'{}'{})\n",
                a[0], a[1], a[2], txt, &opt
            )
            .unwrap();
        }
    }

    /// Adjust 2D or 3D limits
    fn limits(&mut self, ndim: usize, xmin: &[f64], xmax: &[f64]) {
        const FACTOR: f64 = 0.1;
        let mut gap = [0.0; 3];
        for i in 0..ndim {
            gap[i] = (xmax[i] - xmin[i]) * FACTOR;
        }
        if ndim == 2 {
            write!(
                &mut self.buffer,
                "plt.axis([{},{},{},{}])\n",
                xmin[0] - gap[0],
                xmax[0] + gap[0],
                xmin[1] - gap[1],
                xmax[1] + gap[1]
            )
            .unwrap();
        } else {
            write!(
                &mut self.buffer,
                "ax3d().set_xlim3d({},{})\n\
                 ax3d().set_ylim3d({},{})\n\
                 ax3d().set_zlim3d({},{})\n",
                xmin[0] - gap[0],
                xmax[0] + gap[0],
                xmin[1] - gap[1],
                xmax[1] + gap[1],
                xmin[2] - gap[2],
                xmax[2] + gap[2]
            )
            .unwrap();
        }
    }
}

impl GraphMaker for Canvas {
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
    use super::Canvas;
    use crate::{GraphMaker, PolyCode};

    #[test]
    fn derive_works() {
        let code = PolyCode::Curve3;
        let clone = code.clone();
        let correct = "Curve3";
        assert_eq!(format!("{:?}", code), correct);
        assert_eq!(format!("{:?}", clone), correct);
    }

    #[test]
    fn new_works() {
        let canvas = Canvas::new();
        assert_eq!(canvas.edge_color.len(), 7);
        assert_eq!(canvas.face_color.len(), 0);
        assert_eq!(canvas.line_width, 0.0);
        assert_eq!(canvas.line_style.len(), 0);
        assert_eq!(canvas.arrow_scale, 0.0);
        assert_eq!(canvas.arrow_style.len(), 0);
        assert_eq!(canvas.buffer.len(), 0);
    }

    #[test]
    fn options_shared_works() {
        let mut canvas = Canvas::new();
        canvas
            .set_edge_color("red")
            .set_face_color("blue")
            .set_line_width(2.5)
            .set_line_style("--")
            .set_stop_clip(true);
        let opt = canvas.options_shared();
        assert_eq!(
            opt,
            ",edgecolor='red'\
             ,facecolor='blue'\
             ,linewidth=2.5\
             ,linestyle='--'\
             ,clip_on=False"
        );
    }

    #[test]
    fn options_arrow_works() {
        let mut canvas = Canvas::new();
        canvas.set_arrow_scale(25.0).set_arrow_style("fancy");
        let opt = canvas.options_arrow();
        assert_eq!(
            opt,
            ",mutation_scale=25\
             ,arrowstyle='fancy'"
        );
    }

    #[test]
    fn options_text_works() {
        let mut canvas = Canvas::new();
        canvas
            .set_text_color("red")
            .set_text_align_horizontal("center")
            .set_text_align_vertical("center")
            .set_text_fontsize(8.0)
            .set_text_rotation(45.0);
        let opt = canvas.options_text();
        assert_eq!(
            opt,
            ",color='red'\
             ,ha='center'\
             ,va='center'\
             ,fontsize=8\
             ,rotation=45"
        );
    }

    #[test]
    fn options_alt_text_works() {
        let mut canvas = Canvas::new();
        canvas
            .set_alt_text_color("blue")
            .set_alt_text_align_horizontal("right")
            .set_alt_text_align_vertical("bottom")
            .set_alt_text_fontsize(10.0)
            .set_alt_text_rotation(30.0);
        let opt = canvas.options_alt_text();
        assert_eq!(
            opt,
            ",color='blue'\
             ,ha='right'\
             ,va='bottom'\
             ,fontsize=10\
             ,rotation=30"
        );
    }

    #[test]
    fn options_line_3d_works() {
        let mut canvas = Canvas::new();
        canvas.set_edge_color("red");
        let opt = canvas.options_line_3d();
        assert_eq!(opt, ",color='red'");

        let mut canvas = Canvas::new();
        canvas.set_edge_color("red").set_line_width(5.0).set_line_style(":");
        let opt = canvas.options_line_3d();
        assert_eq!(opt, ",color='red',linewidth=5,linestyle=':'");
    }

    #[test]
    fn line_works() {
        let mut canvas = Canvas::new();
        let a = [0.0; 3];
        let b = [0.0; 3];
        canvas.line(2, &a, &b);
        canvas.line(3, &a, &b);
        assert_eq!(
            canvas.buffer,
            "\x20\x20\x20\x20[pth.Path.MOVETO,(0,0)],[pth.Path.LINETO,(0,0)],\n\
             ax3d().plot([0,0],[0,0],[0,0],color='#427ce5')\n"
        );
        canvas.clear_buffer();
        assert_eq!(canvas.buffer, "");
    }

    #[test]
    fn text_works() {
        let mut canvas = Canvas::new();
        let a = [0.0; 3];
        canvas.text(2, &a, "hello", true);
        canvas.text(3, &a, "hello", false);
        assert_eq!(
            canvas.buffer,
            "plt.text(0,0,'hello',color='#a81414',fontsize=8,rotation=45)\n\
             ax3d().text(0,0,0,'hello',color='#343434',ha='center',va='center',fontsize=10)\n"
        );
    }

    #[test]
    fn limits_works() {
        let mut canvas = Canvas::new();
        let xmin = [0.0; 3];
        let xmax = [0.0; 3];
        canvas.limits(2, &xmin, &xmax);
        canvas.limits(3, &xmin, &xmax);
        assert_eq!(
            canvas.buffer,
            "plt.axis([0,0,0,0])\n\
            ax3d().set_xlim3d(0,0)\n\
            ax3d().set_ylim3d(0,0)\n\
            ax3d().set_zlim3d(0,0)\n"
        );
    }

    #[test]
    fn arc_works() {
        let mut canvas = Canvas::new();
        canvas.draw_arc(0.0, 0.0, 1.0, 30.0, 60.0);
        let b: &str = "p=pat.Arc((0,0),2*1,2*1,theta1=30,theta2=60,angle=0,edgecolor='#427ce5')\n\
                       plt.gca().add_patch(p)\n";
        assert_eq!(canvas.buffer, b);
    }

    #[test]
    fn arrow_woks() {
        let mut canvas = Canvas::new();
        canvas.draw_arrow(0.0, 0.0, 1.0, 1.0);
        let b: &str =
            "p=pat.FancyArrowPatch((0,0),(1,1),shrinkA=0,shrinkB=0,path_effects=[pff.Stroke(joinstyle='miter')],edgecolor='#427ce5')\n\
             plt.gca().add_patch(p)\n";
        assert_eq!(canvas.buffer, b);
    }

    #[test]
    fn circle_works() {
        let mut canvas = Canvas::new();
        canvas.draw_circle(0.0, 0.0, 1.0);
        let b: &str = "p=pat.Circle((0,0),1,edgecolor='#427ce5')\n\
                       plt.gca().add_patch(p)\n";
        assert_eq!(canvas.buffer, b);
    }

    #[test]
    fn polycurve_methods_work() {
        // note the following sequence of codes won't work in Matplotlib because Curve3 and Curve4 are wrong
        let mut canvas = Canvas::new();
        canvas.polycurve_begin();
        assert_eq!(canvas.buffer, "dat=[");
        canvas.polycurve_add(0, 0, PolyCode::MoveTo);
        assert_eq!(canvas.buffer, "dat=[[pth.Path.MOVETO,(0,0)],");
        canvas.polycurve_add(1, 0, PolyCode::LineTo);
        assert_eq!(canvas.buffer, "dat=[[pth.Path.MOVETO,(0,0)],[pth.Path.LINETO,(1,0)],");
        canvas.polycurve_add(2, 0, PolyCode::Curve3);
        assert_eq!(
            canvas.buffer,
            "dat=[[pth.Path.MOVETO,(0,0)],[pth.Path.LINETO,(1,0)],[pth.Path.CURVE3,(2,0)],"
        );
        canvas.polycurve_add(3, 0, PolyCode::Curve4);
        assert_eq!(
            canvas.buffer,
            "dat=[[pth.Path.MOVETO,(0,0)],[pth.Path.LINETO,(1,0)],[pth.Path.CURVE3,(2,0)],[pth.Path.CURVE4,(3,0)],"
        );
        canvas.polycurve_end(true);
        assert_eq!(
            canvas.buffer,
            "dat=[[pth.Path.MOVETO,(0,0)],[pth.Path.LINETO,(1,0)],[pth.Path.CURVE3,(2,0)],[pth.Path.CURVE4,(3,0)],[pth.Path.CLOSEPOLY,(None,None)]]\n\
            cmd,pts=zip(*dat)\n\
            h=pth.Path(pts,cmd)\n\
            p=pat.PathPatch(h,edgecolor='#427ce5')\n\
            plt.gca().add_patch(p)\n"
        );
    }

    #[test]
    fn polycurve_capture_errors() {
        let mut canvas = Canvas::new();
        assert_eq!(
            canvas.draw_polycurve(&[[0, 0]], &[PolyCode::MoveTo], true).err(),
            Some("npoint must be ≥ 3")
        );
        assert_eq!(
            canvas
                .draw_polycurve(
                    &[[0], [0], [0]],
                    &[PolyCode::MoveTo, PolyCode::LineTo, PolyCode::LineTo],
                    true
                )
                .err(),
            Some("ndim must be equal to 2")
        );
        assert_eq!(
            canvas
                .draw_polycurve(&[[0, 0], [0, 0], [0, 0]], &[PolyCode::MoveTo], true)
                .err(),
            Some("codes.len() must be equal to npoint")
        );
    }

    #[test]
    fn polycurve_works() {
        let mut canvas = Canvas::new();
        let points = &[[0, 0], [1, 0], [1, 1]];
        let codes = &[PolyCode::MoveTo, PolyCode::Curve3, PolyCode::Curve3];
        canvas.draw_polycurve(points, codes, true).unwrap();
        let b: &str = "dat=[[pth.Path.MOVETO,(0,0)],[pth.Path.CURVE3,(1,0)],[pth.Path.CURVE3,(1,1)],[pth.Path.CLOSEPOLY,(None,None)]]\n\
                       cmd,pts=zip(*dat)\n\
                       h=pth.Path(pts,cmd)\n\
                       p=pat.PathPatch(h,edgecolor='#427ce5')\n\
                       plt.gca().add_patch(p)\n";
        assert_eq!(canvas.buffer, b);
    }

    #[test]
    fn polyline_works_2d() {
        let mut canvas = Canvas::new();
        let points = &[[1.0, 1.0], [2.0, 1.0], [1.5, 1.866]];
        canvas.draw_polyline(points, true);
        let b: &str = "dat=[[pth.Path.MOVETO,(1,1)],[pth.Path.LINETO,(2,1)],[pth.Path.LINETO,(1.5,1.866)],[pth.Path.CLOSEPOLY,(None,None)]]\n\
                       cmd,pts=zip(*dat)\n\
                       h=pth.Path(pts,cmd)\n\
                       p=pat.PathPatch(h,edgecolor='#427ce5')\n\
                       plt.gca().add_patch(p)\n";
        assert_eq!(canvas.buffer, b);
    }

    #[test]
    fn polyline_3d_methods_work() {
        let mut canvas = Canvas::new();
        canvas
            .polyline_3d_begin()
            .polyline_3d_add(1, 2, 3)
            .polyline_3d_add(4, 5, 6)
            .polyline_3d_end();
        let b: &str = "\
            xyz=np.array([[1,2,3],[4,5,6],])\n\
            ax3d().plot(xyz[:,0],xyz[:,1],xyz[:,2],color='#427ce5')\n";
        assert_eq!(canvas.buffer, b);
    }

    #[test]
    fn polyline_works_3d() {
        let mut nothing = Canvas::new();
        nothing.draw_polyline(&[[0.0, 0.0]], true);
        assert_eq!(nothing.buffer, "");

        #[rustfmt::skip]
        let points = &[
            [2.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 3.0],
            [2.0, 1.0, 3.0],
        ];

        let mut open = Canvas::new();
        open.draw_polyline(points, false);
        let b: &str = "\
            xyz=np.array([[2,1,0],[0,1,0],[0,1,3],[2,1,3],])\n\
            ax3d().plot(xyz[:,0],xyz[:,1],xyz[:,2],color='#427ce5')\n";
        assert_eq!(open.buffer, b);

        let mut closed = Canvas::new();
        closed.draw_polyline(points, true);
        let b: &str = "\
            xyz=np.array([[2,1,0],[0,1,0],[0,1,3],[2,1,3],[2,1,0],])\n\
            ax3d().plot(xyz[:,0],xyz[:,1],xyz[:,2],color='#427ce5')\n";
        assert_eq!(closed.buffer, b);

        #[rustfmt::skip]
        let points = &[
            [2.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
        ];

        let mut closed_few_points = Canvas::new();
        closed_few_points.draw_polyline(points, true);
        let b: &str = "\
            xyz=np.array([[2,1,0],[0,1,0],])\n\
            ax3d().plot(xyz[:,0],xyz[:,1],xyz[:,2],color='#427ce5')\n";
        assert_eq!(closed_few_points.buffer, b);
    }

    #[test]
    fn grid_fails_on_wrong_input() {
        let mut canvas = Canvas::new();
        let res = canvas.draw_grid(&[0.0, 0.0], &[1.0, 1.0], &[1], true, false);
        assert_eq!(res, Err("len(ndiv) == ndim must be 2 or 3"));
        let res = canvas.draw_grid(&[0.0], &[1.0, 1.0], &[1, 1], true, false);
        assert_eq!(res, Err("size of xmin must equal ndim == len(ndiv)"));
        let res = canvas.draw_grid(&[0.0, 0.0], &[1.0], &[1, 1], true, false);
        assert_eq!(res, Err("size of xmax must equal ndim == len(ndiv)"));
        let res = canvas.draw_grid(&[0.0, 0.0], &[0.0, 1.0], &[1, 1], true, false);
        assert_eq!(res, Err("xmax must be greater than xmin"));
    }

    #[test]
    fn grid_no_ids_works() {
        let mut canvas = Canvas::new();
        canvas
            .draw_grid(&[0.0, 0.0], &[1.0, 1.0], &[1, 1], false, false)
            .unwrap();
        let b: &str = "dat=[\n\
                      \x20\x20\x20\x20[pth.Path.MOVETO,(0,0)],[pth.Path.LINETO,(0,1)],\n\
                      \x20\x20\x20\x20[pth.Path.MOVETO,(1,0)],[pth.Path.LINETO,(1,1)],\n\
                      \x20\x20\x20\x20[pth.Path.MOVETO,(0,0)],[pth.Path.LINETO,(1,0)],\n\
                      \x20\x20\x20\x20[pth.Path.MOVETO,(0,1)],[pth.Path.LINETO,(1,1)],\n\
                      ]\n\
                      cmd,pts=zip(*dat)\n\
                      h=pth.Path(pts,cmd)\n\
                      p=pat.PathPatch(h,edgecolor='#427ce5')\n\
                      plt.gca().add_patch(p)\n\
                      plt.axis([-0.1,1.1,-0.1,1.1])\n";
        assert_eq!(canvas.buffer, b);
    }

    #[test]
    fn grid_2d_works() {
        let mut canvas = Canvas::new();
        canvas.draw_grid(&[0.0, 0.0], &[1.0, 1.0], &[1, 1], true, true).unwrap();
        let b: &str = "dat=[\n\
                      \x20\x20\x20\x20[pth.Path.MOVETO,(0,0)],[pth.Path.LINETO,(0,1)],\n\
                      \x20\x20\x20\x20[pth.Path.MOVETO,(1,0)],[pth.Path.LINETO,(1,1)],\n\
                      \x20\x20\x20\x20[pth.Path.MOVETO,(0,0)],[pth.Path.LINETO,(1,0)],\n\
                      \x20\x20\x20\x20[pth.Path.MOVETO,(0,1)],[pth.Path.LINETO,(1,1)],\n\
                      ]\n\
                      cmd,pts=zip(*dat)\n\
                      h=pth.Path(pts,cmd)\n\
                      p=pat.PathPatch(h,edgecolor='#427ce5')\n\
                      plt.gca().add_patch(p)\n\
                      plt.text(0,0,'0',color='#a81414',fontsize=8,rotation=45)\n\
                      plt.text(1,0,'1',color='#a81414',fontsize=8,rotation=45)\n\
                      plt.text(0,1,'2',color='#a81414',fontsize=8,rotation=45)\n\
                      plt.text(1,1,'3',color='#a81414',fontsize=8,rotation=45)\n\
                      plt.text(0.5,0.5,'0',color='#343434',ha='center',va='center',fontsize=10)\n\
                      plt.axis([-0.1,1.1,-0.1,1.1])\n";
        assert_eq!(canvas.buffer, b);
    }

    #[test]
    fn grid_3d_works() {
        let mut canvas = Canvas::new();
        canvas
            .draw_grid(&[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0], &[1, 1, 1], true, true)
            .unwrap();
        let b: &str = "\
                       ax3d().plot([0,0],[0,1],[0,0],color='#427ce5')\n\
                       ax3d().plot([1,1],[0,1],[0,0],color='#427ce5')\n\
                       ax3d().plot([0,1],[0,0],[0,0],color='#427ce5')\n\
                       ax3d().plot([0,1],[1,1],[0,0],color='#427ce5')\n\
                       ax3d().text(0,0,0,'0',color='#a81414',fontsize=8,rotation=45)\n\
                       ax3d().text(1,0,0,'1',color='#a81414',fontsize=8,rotation=45)\n\
                       ax3d().text(0,1,0,'2',color='#a81414',fontsize=8,rotation=45)\n\
                       ax3d().text(1,1,0,'3',color='#a81414',fontsize=8,rotation=45)\n\
                       ax3d().plot([0,0],[0,1],[1,1],color='#427ce5')\n\
                       ax3d().plot([1,1],[0,1],[1,1],color='#427ce5')\n\
                       ax3d().plot([0,1],[0,0],[1,1],color='#427ce5')\n\
                       ax3d().plot([0,1],[1,1],[1,1],color='#427ce5')\n\
                       ax3d().text(0,0,1,'4',color='#a81414',fontsize=8,rotation=45)\n\
                       ax3d().text(1,0,1,'5',color='#a81414',fontsize=8,rotation=45)\n\
                       ax3d().text(0,1,1,'6',color='#a81414',fontsize=8,rotation=45)\n\
                       ax3d().text(1,1,1,'7',color='#a81414',fontsize=8,rotation=45)\n\
                       ax3d().text(0.5,0.5,0.5,'0',color='#343434',ha='center',va='center',fontsize=10)\n\
                       ax3d().plot([0,0],[0,0],[0,1],color='#427ce5')\n\
                       ax3d().plot([1,1],[0,0],[0,1],color='#427ce5')\n\
                       ax3d().plot([0,0],[1,1],[0,1],color='#427ce5')\n\
                       ax3d().plot([1,1],[1,1],[0,1],color='#427ce5')\n\
                       ax3d().set_xlim3d(-0.1,1.1)\n\
                       ax3d().set_ylim3d(-0.1,1.1)\n\
                       ax3d().set_zlim3d(-0.1,1.1)\n";
        assert_eq!(canvas.buffer, b);
    }
}
