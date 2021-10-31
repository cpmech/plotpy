use super::GraphMaker;
use std::fmt::Write;

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
/// let p = vec![vec![0.1, 0.5], vec![0.1 + a, 0.5], vec![0.1 + a / 2.0, 0.5 + a * c]];
/// let q = vec![vec![0.9, 0.5], vec![0.9 - a, 0.5], vec![0.9 - a / 2.0, 0.5 + a * c]];
/// shapes.draw_polyline(&p, true);
/// shapes.draw_polyline(&q, false);
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
    text_color: String,       // Text color
    align_horizontal: String, // Horizontal alignment
    align_vertical: String,   // Vertical alignment
    fontsize: f64,            // Font size
    rotation: f64,            // Text rotation

    // buffer
    buffer: String, // buffer
}

impl Shapes {
    pub fn new() -> Self {
        Shapes {
            edge_color: "#427ce5".to_string(),
            face_color: String::new(),
            line_width: 0.0,
            arrow_scale: 0.0,
            arrow_style: String::new(),
            text_color: "#a81414".to_string(),
            align_horizontal: String::new(),
            align_vertical: String::new(),
            fontsize: 8.0,
            rotation: 45.0,
            buffer: String::new(),
        }
    }

    /// Draws arc
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

    /// Draws arrow
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

    /// Draws circle
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

    /// Draws polyline
    pub fn draw_polyline<T>(&mut self, points: &Vec<Vec<T>>, closed: bool)
    where
        T: std::fmt::Display,
    {
        let mut first = true;
        for p in points {
            if first {
                write!(&mut self.buffer, "dat=[[pth.Path.MOVETO,({},{})]", p[0], p[1]).unwrap();
            } else {
                write!(&mut self.buffer, ",[pth.Path.LINETO,({},{})]", p[0], p[1]).unwrap();
            }
            first = false;
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
        with_ids: bool,
    ) -> Result<(), &'static str> {
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
        let mut id = 0;
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
            if with_ids {
                for j in 0..npoint[1] {
                    a[1] = xmin[1] + delta[1] * (j as f64);
                    for i in 0..npoint[0] {
                        a[0] = xmin[0] + delta[0] * (i as f64);
                        let txt = format!("{}", id);
                        self.text(ndim, &a, &txt);
                        id += 1;
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

    /// Sets the horizontal alignment
    ///
    /// Options: "center", "left", "right"
    pub fn set_align_horizontal(&mut self, option: &str) -> &mut Self {
        self.align_horizontal = String::from(option);
        self
    }

    /// Sets the vertical alignment
    ///
    /// Options: "center", "top", "bottom", "baseline", "center_baseline"
    pub fn set_align_vertical(&mut self, option: &str) -> &mut Self {
        self.align_vertical = String::from(option);
        self
    }

    /// Sets the font size
    pub fn set_fontsize(&mut self, fontsize: f64) -> &mut Self {
        self.fontsize = fontsize;
        self
    }

    /// Sets the text rotation
    pub fn set_rotation(&mut self, rotation: f64) -> &mut Self {
        self.rotation = rotation;
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
        if self.align_horizontal != "" {
            write!(&mut opt, ",ha='{}'", self.align_horizontal).unwrap();
        }
        if self.align_vertical != "" {
            write!(&mut opt, ",va='{}'", self.align_vertical).unwrap();
        }
        if self.fontsize > 0.0 {
            write!(&mut opt, ",fontsize={}", self.fontsize).unwrap();
        }
        if self.rotation > 0.0 {
            write!(&mut opt, ",rotation={}", self.rotation).unwrap();
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
    fn text(&mut self, ndim: usize, a: &[f64; 3], txt: &str) {
        let opt_text = self.options_text();
        if ndim == 2 {
            write!(&mut self.buffer, "plt.text({},{},'{}'{})\n", a[0], a[1], txt, &opt_text).unwrap();
        } else {
            write!(
                &mut self.buffer,
                "AX3D.text({},{},{},'{}'{})\n",
                a[0], a[1], a[2], txt, &opt_text
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
    use super::Shapes;

    #[test]
    fn new_works() {
        let shapes = Shapes::new();
        assert_eq!(shapes.edge_color.len(), 7);
        assert_eq!(shapes.face_color.len(), 0);
        assert_eq!(shapes.line_width, 0.0);
        assert_eq!(shapes.arrow_scale, 0.0);
        assert_eq!(shapes.arrow_style.len(), 0);
        assert_eq!(shapes.text_color.len(), 7);
        assert_eq!(shapes.align_horizontal.len(), 0);
        assert_eq!(shapes.align_vertical.len(), 0);
        assert_eq!(shapes.fontsize, 8.0);
        assert_eq!(shapes.rotation, 45.0);
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
            .set_align_horizontal("center")
            .set_align_vertical("center")
            .set_fontsize(8.0)
            .set_rotation(45.0);
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
        shapes.text(2, &a, "hello");
        shapes.text(3, &a, "hello");
        assert_eq!(
            shapes.buffer,
            "plt.text(0,0,'hello',color='#a81414',fontsize=8,rotation=45)\n\
             AX3D.text(0,0,0,'hello',color='#a81414',fontsize=8,rotation=45)\n"
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
    fn polyline_works() {
        let mut shapes = Shapes::new();
        let points = vec![vec![1.0, 1.0], vec![2.0, 1.0], vec![1.5, 1.866]];
        shapes.draw_polyline(&points, true);
        let b: &str = "dat=[[pth.Path.MOVETO,(1,1)],[pth.Path.LINETO,(2,1)],[pth.Path.LINETO,(1.5,1.866)],[pth.Path.CLOSEPOLY,(None,None)]]\n\
                       cmd,pts=zip(*dat)\n\
                       h=pth.Path(pts,cmd)\n\
                       p=pat.PathPatch(h,edgecolor='#427ce5')\n\
                       plt.gca().add_patch(p)\n";
        assert_eq!(shapes.buffer, b);
    }

    #[test]
    fn grid_fails_on_wrong_input() {
        let mut shapes = Shapes::new();
        let res = shapes.draw_grid(&[0.0, 0.0], &[1.0, 1.0], &[1], true);
        assert_eq!(res, Err("len(ndiv) == ndim must be 2 or 3"));
        let res = shapes.draw_grid(&[0.0], &[1.0, 1.0], &[1, 1], true);
        assert_eq!(res, Err("size of xmin must equal ndim == len(ndiv)"));
        let res = shapes.draw_grid(&[0.0, 0.0], &[1.0], &[1, 1], true);
        assert_eq!(res, Err("size of xmax must equal ndim == len(ndiv)"));
        let res = shapes.draw_grid(&[0.0, 0.0], &[0.0, 1.0], &[1, 1], true);
        assert_eq!(res, Err("xmax must be greater than xmin"));
    }

    #[test]
    fn grid_no_ids_works() -> Result<(), &'static str> {
        let mut shapes = Shapes::new();
        shapes.draw_grid(&[0.0, 0.0], &[1.0, 1.0], &[1, 1], false)?;
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
    fn grid_2d_works() -> Result<(), &'static str> {
        let mut shapes = Shapes::new();
        shapes.draw_grid(&[0.0, 0.0], &[1.0, 1.0], &[1, 1], true)?;
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
                      plt.axis([-0.1,1.1,-0.1,1.1])\n";
        assert_eq!(shapes.buffer, b);
        Ok(())
    }

    #[test]
    fn grid_3d_works() -> Result<(), &'static str> {
        let mut shapes = Shapes::new();
        shapes.draw_grid(&[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0], &[1, 1, 1], true)?;
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
