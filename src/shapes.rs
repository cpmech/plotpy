use super::{GraphMaker, StrError};
use crate::AsMatrix;
use std::fmt::Write;

/// Defines the poly-curve code
#[derive(Clone, Debug)]
pub enum PcCode {
    /// Automatically selects between MoveTo and LineTo
    Auto,

    /// Segment
    LineTo,

    /// Quadratic Bezier
    Curve3,

    /// Cubic Bezier
    Curve4,
}

/// Draw polygonal shapes
///
/// # Example
///
/// ```
/// # fn main() -> Result<(), &'static str> {
/// // import
/// use plotpy::{Plot, Shapes};
/// use std::path::Path;
///
/// // directory to save figures
/// const OUT_DIR: &str = "/tmp/plotpy/doc_tests";
///
/// // shapes object and common options
/// let mut shapes = Shapes::new();
/// shapes.set_line_width(3.0).set_edge_color("#cd0000").set_face_color("#eeea83");
///
/// // draw arc
/// shapes.draw_arc(0.5, 0.5, 0.4, 195.0, -15.0);
///
/// // draw arrow
/// shapes.set_arrow_scale(50.0).set_arrow_style("fancy");
/// shapes.draw_arrow(0.4, 0.3, 0.6, 0.5);
///
/// // draw circle
/// shapes.set_face_color("None").set_edge_color("#1f9c25").set_line_width(6.0);
/// shapes.draw_circle(0.5, 0.5, 0.5);
///
/// // draw polyline
/// shapes.set_line_width(3.0).set_edge_color("blue");
/// let a = 0.2;
/// let c = f64::sqrt(3.0) / 2.0;
/// let p = &[[0.1, 0.5], [0.1 + a, 0.5], [0.1 + a / 2.0, 0.5 + a * c]];
/// let q = &[[0.9, 0.5], [0.9 - a, 0.5], [0.9 - a / 2.0, 0.5 + a * c]];
/// shapes.draw_polyline(p, true);
/// shapes.draw_polyline(q, false);
///
/// // add shapes to plot
/// let mut plot = Plot::new();
/// plot.set_hide_axes(true)
///     .set_equal_axes(true)
///     .set_range(-0.05, 1.05, -0.05, 1.05)
///     .add(&shapes);
///
/// // save figure
/// let path = Path::new(OUT_DIR).join("doc_shapes.svg");
/// plot.save(&path)?;
/// # Ok(())
/// # }
/// ```
///
/// ![doc_shapes.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/doc_shapes.svg)
///
pub struct Shapes {
    // shapes
    edge_color: String,  // Edge color (shared)
    face_color: String,  // Face color (shared)
    line_width: f64,     // Line width of edge (shared)
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

    // buffer
    buffer: String, // buffer
}

impl Shapes {
    pub fn new() -> Self {
        Shapes {
            // shapes
            edge_color: "#427ce5".to_string(),
            face_color: String::new(),
            line_width: 0.0,
            arrow_scale: 0.0,
            arrow_style: String::new(),
            // text
            text_color: "#a81414".to_string(),
            text_align_horizontal: String::new(),
            text_align_vertical: String::new(),
            text_fontsize: 8.0,
            text_rotation: 45.0,
            // alternative text
            alt_text_color: "#343434".to_string(),
            alt_text_align_horizontal: "center".to_string(),
            alt_text_align_vertical: "center".to_string(),
            alt_text_fontsize: 10.0,
            alt_text_rotation: 0.0,
            // buffer
            buffer: String::new(),
        }
    }

    /// Draws arc (2D only)
    pub fn draw_arc<T>(&mut self, xc: T, yc: T, r: T, ini_angle: T, fin_angle: T)
    where
        T: std::fmt::Display,
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
        T: std::fmt::Display,
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
        T: std::fmt::Display,
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

    /// Draws polyline with straight segments, quadratic Bezier, or cubic Bezier (2D only)
    ///
    /// **Note:** The first and last commands are ignored.
    pub fn draw_polycurve<T>(&mut self, x: &[T], y: &[T], codes: &[PcCode], closed: bool) -> Result<(), StrError>
    where
        T: std::fmt::Display,
    {
        let npoint = x.len();
        if y.len() != npoint || codes.len() != npoint {
            return Err("x, y, and codes must have the same lengths");
        }
        if npoint < 3 {
            return Err("npoint must be ≥ 3");
        }
        write!(&mut self.buffer, "dat=[[pth.Path.MOVETO,({},{})]", x[0], y[0]).unwrap();
        for i in 1..npoint {
            let keyword = match codes[i] {
                PcCode::Auto => "LINETO",
                PcCode::LineTo => "LINETO",
                PcCode::Curve3 => "CURVE3",
                PcCode::Curve4 => "CURVE4",
            };
            write!(&mut self.buffer, ",[pth.Path.{},({},{})]", keyword, x[i], y[i]).unwrap();
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

    /// Draws polyline (2D or 3D)
    pub fn draw_polyline<'a, T, U>(&mut self, points: &'a T, closed: bool)
    where
        T: AsMatrix<'a, U>,
        U: 'a + std::fmt::Display,
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
            write!(&mut self.buffer, "maybeCreateAX3D()\n").unwrap();
            let opt = self.options_line_3d();
            let mut xx = format!("xx=[{}", points.at(0, 0));
            let mut yy = format!("yy=[{}", points.at(0, 1));
            let mut zz = format!("zz=[{}", points.at(0, 2));
            for i in 1..npoint {
                write!(&mut xx, ",{}", points.at(i, 0)).unwrap();
                write!(&mut yy, ",{}", points.at(i, 1)).unwrap();
                write!(&mut zz, ",{}", points.at(i, 2)).unwrap();
            }
            if closed && npoint > 2 {
                write!(&mut xx, ",{}", points.at(0, 0)).unwrap();
                write!(&mut yy, ",{}", points.at(0, 1)).unwrap();
                write!(&mut zz, ",{}", points.at(0, 2)).unwrap();
            }
            write!(&mut self.buffer, "{}]\n", xx).unwrap();
            write!(&mut self.buffer, "{}]\n", yy).unwrap();
            write!(&mut self.buffer, "{}]\n", zz).unwrap();
            write!(&mut self.buffer, "AX3D.plot(xx,yy,zz{})\n", opt).unwrap();
        }
    }

    /// Draws a 2D or 3D grid
    ///
    /// # Input
    ///
    /// * `xmin, xmax` -- min and max coordinates (len = 2 or 3 == ndim)
    /// * `ndiv` -- number of divisions along each dimension (len = 2 or 3 == ndim)
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
        } else {
            write!(&mut self.buffer, "maybeCreateAX3D()\n").unwrap();
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
                        self.text(ndim, &a, &txt, false);
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
                        self.text(ndim, &b, &txt, true);
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

    /// Sets the edge color (shared among shapes)
    pub fn set_edge_color(&mut self, color: &str) -> &mut Self {
        self.edge_color = String::from(color);
        self
    }

    /// Sets the face color (shared among shapes)
    pub fn set_face_color(&mut self, color: &str) -> &mut Self {
        self.face_color = String::from(color);
        self
    }

    /// Sets the line width of edge (shared among shapes)
    pub fn set_line_width(&mut self, width: f64) -> &mut Self {
        self.line_width = width;
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
        opt
    }

    /// Draws 2D or 3D line
    fn line(&mut self, ndim: usize, a: &[f64; 3], b: &[f64; 3]) {
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
                "AX3D.plot([{},{}],[{},{}],[{},{}]{})\n",
                a[0], b[0], a[1], b[1], a[2], b[2], opt,
            )
            .unwrap();
        }
    }

    /// Draws 2D or 3D text
    fn text(&mut self, ndim: usize, a: &[f64; 3], txt: &str, alternative: bool) {
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
                "AX3D.text({},{},{},'{}'{})\n",
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
                "AX3D.set_xlim3d({},{})\n\
                 AX3D.set_ylim3d({},{})\n\
                 AX3D.set_zlim3d({},{})\n",
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

impl GraphMaker for Shapes {
    fn get_buffer<'a>(&'a self) -> &'a String {
        &self.buffer
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::PcCode;

    use super::{Shapes, StrError};

    #[test]
    fn new_works() {
        let shapes = Shapes::new();
        assert_eq!(shapes.edge_color.len(), 7);
        assert_eq!(shapes.face_color.len(), 0);
        assert_eq!(shapes.line_width, 0.0);
        assert_eq!(shapes.arrow_scale, 0.0);
        assert_eq!(shapes.arrow_style.len(), 0);
        assert_eq!(shapes.text_color.len(), 7);
        assert_eq!(shapes.text_align_horizontal.len(), 0);
        assert_eq!(shapes.text_align_vertical.len(), 0);
        assert_eq!(shapes.text_fontsize, 8.0);
        assert_eq!(shapes.text_rotation, 45.0);
        assert_eq!(shapes.buffer.len(), 0);
    }

    #[test]
    fn options_shared_works() {
        let mut shapes = Shapes::new();
        shapes.set_edge_color("red").set_face_color("blue").set_line_width(2.5);
        let opt = shapes.options_shared();
        assert_eq!(
            opt,
            ",edgecolor='red'\
             ,facecolor='blue'\
             ,linewidth=2.5"
        );
    }

    #[test]
    fn options_arrow_works() {
        let mut shapes = Shapes::new();
        shapes.set_arrow_scale(25.0).set_arrow_style("fancy");
        let opt = shapes.options_arrow();
        assert_eq!(
            opt,
            ",mutation_scale=25\
             ,arrowstyle='fancy'"
        );
    }

    #[test]
    fn options_text_works() {
        let mut shapes = Shapes::new();
        shapes
            .set_text_color("red")
            .set_text_align_horizontal("center")
            .set_text_align_vertical("center")
            .set_text_fontsize(8.0)
            .set_text_rotation(45.0);
        let opt = shapes.options_text();
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
        let mut shapes = Shapes::new();
        shapes
            .set_alt_text_color("blue")
            .set_alt_text_align_horizontal("right")
            .set_alt_text_align_vertical("bottom")
            .set_alt_text_fontsize(10.0)
            .set_alt_text_rotation(30.0);
        let opt = shapes.options_alt_text();
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
        let mut shapes = Shapes::new();
        shapes.set_edge_color("red");
        let opt = shapes.options_line_3d();
        assert_eq!(opt, ",color='red'");
    }

    #[test]
    fn line_works() {
        let mut shapes = Shapes::new();
        let a = [0.0; 3];
        let b = [0.0; 3];
        shapes.line(2, &a, &b);
        shapes.line(3, &a, &b);
        assert_eq!(
            shapes.buffer,
            "\x20\x20\x20\x20[pth.Path.MOVETO,(0,0)],[pth.Path.LINETO,(0,0)],\n\
             AX3D.plot([0,0],[0,0],[0,0],color='#427ce5')\n"
        );
    }

    #[test]
    fn text_works() {
        let mut shapes = Shapes::new();
        let a = [0.0; 3];
        shapes.text(2, &a, "hello", false);
        shapes.text(3, &a, "hello", true);
        assert_eq!(
            shapes.buffer,
            "plt.text(0,0,'hello',color='#a81414',fontsize=8,rotation=45)\n\
             AX3D.text(0,0,0,'hello',color='#343434',ha='center',va='center',fontsize=10)\n"
        );
    }

    #[test]
    fn limits_works() {
        let mut shapes = Shapes::new();
        let xmin = [0.0; 3];
        let xmax = [0.0; 3];
        shapes.limits(2, &xmin, &xmax);
        shapes.limits(3, &xmin, &xmax);
        assert_eq!(
            shapes.buffer,
            "plt.axis([0,0,0,0])\n\
            AX3D.set_xlim3d(0,0)\n\
            AX3D.set_ylim3d(0,0)\n\
            AX3D.set_zlim3d(0,0)\n"
        );
    }

    #[test]
    fn arc_works() {
        let mut shapes = Shapes::new();
        shapes.draw_arc(0.0, 0.0, 1.0, 30.0, 60.0);
        let b: &str = "p=pat.Arc((0,0),2*1,2*1,theta1=30,theta2=60,angle=0,edgecolor='#427ce5')\n\
                       plt.gca().add_patch(p)\n";
        assert_eq!(shapes.buffer, b);
    }

    #[test]
    fn arrow_woks() {
        let mut shapes = Shapes::new();
        shapes.draw_arrow(0.0, 0.0, 1.0, 1.0);
        let b: &str =
            "p=pat.FancyArrowPatch((0,0),(1,1),shrinkA=0,shrinkB=0,path_effects=[pff.Stroke(joinstyle='miter')],edgecolor='#427ce5')\n\
             plt.gca().add_patch(p)\n";
        assert_eq!(shapes.buffer, b);
    }

    #[test]
    fn circle_works() {
        let mut shapes = Shapes::new();
        shapes.draw_circle(0.0, 0.0, 1.0);
        let b: &str = "p=pat.Circle((0,0),1,edgecolor='#427ce5')\n\
                       plt.gca().add_patch(p)\n";
        assert_eq!(shapes.buffer, b);
    }

    #[test]
    fn polycurve_capture_errors() {
        let mut shapes = Shapes::new();
        assert_eq!(
            shapes.draw_polycurve(&[0], &[0, 1], &[PcCode::Auto], true).err(),
            Some("x, y, and codes must have the same lengths")
        );
        assert_eq!(
            shapes
                .draw_polycurve(&[0], &[0], &[PcCode::Auto, PcCode::Auto], true)
                .err(),
            Some("x, y, and codes must have the same lengths")
        );
        assert_eq!(
            shapes
                .draw_polycurve(&[0, 0], &[0, 0], &[PcCode::Auto, PcCode::Auto], true)
                .err(),
            Some("npoint must be ≥ 3")
        );
    }

    #[test]
    fn polycurve_works() -> Result<(), StrError> {
        let mut shapes = Shapes::new();
        let x = &[0, 1, 1];
        let y = &[0, 0, 1];
        let codes = &[PcCode::Auto, PcCode::Curve3, PcCode::Curve3];
        shapes.draw_polycurve(x, y, codes, true)?;
        let b: &str = "dat=[[pth.Path.MOVETO,(0,0)],[pth.Path.CURVE3,(1,0)],[pth.Path.CURVE3,(1,1)],[pth.Path.CLOSEPOLY,(None,None)]]\n\
                       cmd,pts=zip(*dat)\n\
                       h=pth.Path(pts,cmd)\n\
                       p=pat.PathPatch(h,edgecolor='#427ce5')\n\
                       plt.gca().add_patch(p)\n";
        assert_eq!(shapes.buffer, b);
        Ok(())
    }

    #[test]
    fn polyline_works_2d() {
        let mut shapes = Shapes::new();
        let points = &[[1.0, 1.0], [2.0, 1.0], [1.5, 1.866]];
        shapes.draw_polyline(points, true);
        let b: &str = "dat=[[pth.Path.MOVETO,(1,1)],[pth.Path.LINETO,(2,1)],[pth.Path.LINETO,(1.5,1.866)],[pth.Path.CLOSEPOLY,(None,None)]]\n\
                       cmd,pts=zip(*dat)\n\
                       h=pth.Path(pts,cmd)\n\
                       p=pat.PathPatch(h,edgecolor='#427ce5')\n\
                       plt.gca().add_patch(p)\n";
        assert_eq!(shapes.buffer, b);
    }

    #[test]
    fn polyline_works_3d() {
        let mut nothing = Shapes::new();
        nothing.draw_polyline(&[[0.0, 0.0]], true);
        assert_eq!(nothing.buffer, "");

        #[rustfmt::skip]
        let points = &[
            [2.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 3.0],
            [2.0, 1.0, 3.0],
        ];

        let mut open = Shapes::new();
        open.draw_polyline(points, false);
        let b: &str = "maybeCreateAX3D()\n\
            xx=[2,0,0,2]\n\
            yy=[1,1,1,1]\n\
            zz=[0,0,3,3]\n\
            AX3D.plot(xx,yy,zz,color='#427ce5')\n";
        assert_eq!(open.buffer, b);

        let mut closed = Shapes::new();
        closed.draw_polyline(points, true);
        let b: &str = "maybeCreateAX3D()\n\
            xx=[2,0,0,2,2]\n\
            yy=[1,1,1,1,1]\n\
            zz=[0,0,3,3,0]\n\
            AX3D.plot(xx,yy,zz,color='#427ce5')\n";
        assert_eq!(closed.buffer, b);

        #[rustfmt::skip]
        let points = &[
            [2.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
        ];

        let mut closed_few_points = Shapes::new();
        closed_few_points.draw_polyline(points, true);
        let b: &str = "maybeCreateAX3D()\n\
            xx=[2,0]\n\
            yy=[1,1]\n\
            zz=[0,0]\n\
            AX3D.plot(xx,yy,zz,color='#427ce5')\n";
        assert_eq!(closed_few_points.buffer, b);
    }

    #[test]
    fn grid_fails_on_wrong_input() {
        let mut shapes = Shapes::new();
        let res = shapes.draw_grid(&[0.0, 0.0], &[1.0, 1.0], &[1], true, false);
        assert_eq!(res, Err("len(ndiv) == ndim must be 2 or 3"));
        let res = shapes.draw_grid(&[0.0], &[1.0, 1.0], &[1, 1], true, false);
        assert_eq!(res, Err("size of xmin must equal ndim == len(ndiv)"));
        let res = shapes.draw_grid(&[0.0, 0.0], &[1.0], &[1, 1], true, false);
        assert_eq!(res, Err("size of xmax must equal ndim == len(ndiv)"));
        let res = shapes.draw_grid(&[0.0, 0.0], &[0.0, 1.0], &[1, 1], true, false);
        assert_eq!(res, Err("xmax must be greater than xmin"));
    }

    #[test]
    fn grid_no_ids_works() -> Result<(), StrError> {
        let mut shapes = Shapes::new();
        shapes.draw_grid(&[0.0, 0.0], &[1.0, 1.0], &[1, 1], false, false)?;
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
        assert_eq!(shapes.buffer, b);
        Ok(())
    }

    #[test]
    fn grid_2d_works() -> Result<(), StrError> {
        let mut shapes = Shapes::new();
        shapes.draw_grid(&[0.0, 0.0], &[1.0, 1.0], &[1, 1], true, true)?;
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
        assert_eq!(shapes.buffer, b);
        Ok(())
    }

    #[test]
    fn grid_3d_works() -> Result<(), StrError> {
        let mut shapes = Shapes::new();
        shapes.draw_grid(&[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0], &[1, 1, 1], true, true)?;
        let b: &str = "maybeCreateAX3D()\n\
                       AX3D.plot([0,0],[0,1],[0,0],color='#427ce5')\n\
                       AX3D.plot([1,1],[0,1],[0,0],color='#427ce5')\n\
                       AX3D.plot([0,1],[0,0],[0,0],color='#427ce5')\n\
                       AX3D.plot([0,1],[1,1],[0,0],color='#427ce5')\n\
                       AX3D.text(0,0,0,'0',color='#a81414',fontsize=8,rotation=45)\n\
                       AX3D.text(1,0,0,'1',color='#a81414',fontsize=8,rotation=45)\n\
                       AX3D.text(0,1,0,'2',color='#a81414',fontsize=8,rotation=45)\n\
                       AX3D.text(1,1,0,'3',color='#a81414',fontsize=8,rotation=45)\n\
                       AX3D.plot([0,0],[0,1],[1,1],color='#427ce5')\n\
                       AX3D.plot([1,1],[0,1],[1,1],color='#427ce5')\n\
                       AX3D.plot([0,1],[0,0],[1,1],color='#427ce5')\n\
                       AX3D.plot([0,1],[1,1],[1,1],color='#427ce5')\n\
                       AX3D.text(0,0,1,'4',color='#a81414',fontsize=8,rotation=45)\n\
                       AX3D.text(1,0,1,'5',color='#a81414',fontsize=8,rotation=45)\n\
                       AX3D.text(0,1,1,'6',color='#a81414',fontsize=8,rotation=45)\n\
                       AX3D.text(1,1,1,'7',color='#a81414',fontsize=8,rotation=45)\n\
                       AX3D.text(0.5,0.5,0.5,'0',color='#343434',ha='center',va='center',fontsize=10)\n\
                       AX3D.plot([0,0],[0,0],[0,1],color='#427ce5')\n\
                       AX3D.plot([1,1],[0,0],[0,1],color='#427ce5')\n\
                       AX3D.plot([0,0],[1,1],[0,1],color='#427ce5')\n\
                       AX3D.plot([1,1],[1,1],[0,1],color='#427ce5')\n\
                       AX3D.set_xlim3d(-0.1,1.1)\n\
                       AX3D.set_ylim3d(-0.1,1.1)\n\
                       AX3D.set_zlim3d(-0.1,1.1)\n";
        assert_eq!(shapes.buffer, b);
        Ok(())
    }
}
