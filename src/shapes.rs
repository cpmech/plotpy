use super::*;
use std::fmt::Write;

/// Draw polygonal shapes
///
/// # Example
///
/// ```
/// # fn main() -> Result<(), &'static str> {
/// // import
/// use plotpy::*;
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
/// plot.hide_axes();
/// plot.set_equal_axes();
/// plot.set_range(-0.05, 1.05, -0.05, 1.05);
/// plot.add(&shapes);
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
    edge_color: String,  // Edge color (shared)
    face_color: String,  // Face color (shared)
    line_width: f64,     // Line width of edge (shared)
    arrow_scale: f64,    // Arrow scale
    arrow_style: String, // Arrow style
    buffer: String,      // buffer
}

impl Shapes {
    pub fn new() -> Self {
        Shapes {
            edge_color: String::new(),
            face_color: String::new(),
            line_width: 0.0,
            arrow_scale: 0.0,
            arrow_style: String::new(),
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
        write!(&mut self.buffer, "]\n").unwrap();
        write!(&mut self.buffer, "cmd,pts=zip(*dat)\n").unwrap();
        write!(&mut self.buffer, "h=pth.Path(pts,cmd)\n").unwrap();
        write!(&mut self.buffer, "p=pat.PathPatch(h{})\n", &opt).unwrap();
        write!(&mut self.buffer, "plt.gca().add_patch(p)\n").unwrap();
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
        assert_eq!(shapes.edge_color.len(), 0);
        assert_eq!(shapes.face_color.len(), 0);
        assert_eq!(shapes.line_width, 0.0);
        assert_eq!(shapes.arrow_scale, 0.0);
        assert_eq!(shapes.arrow_style.len(), 0);
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
    fn arc_works() {
        let mut shapes = Shapes::new();
        shapes.draw_arc(0.0, 0.0, 1.0, 30.0, 60.0);
        let b: &str = "p=pat.Arc((0,0),2*1,2*1,theta1=30,theta2=60,angle=0)\n\
                       plt.gca().add_patch(p)\n";
        assert_eq!(shapes.buffer, b);
    }

    #[test]
    fn arrow_woks() {
        let mut shapes = Shapes::new();
        shapes.draw_arrow(0.0, 0.0, 1.0, 1.0);
        let b: &str =
            "p=pat.FancyArrowPatch((0,0),(1,1),shrinkA=0,shrinkB=0,path_effects=[pff.Stroke(joinstyle='miter')])\n\
             plt.gca().add_patch(p)\n";
        assert_eq!(shapes.buffer, b);
    }

    #[test]
    fn circle_works() {
        let mut shapes = Shapes::new();
        shapes.draw_circle(0.0, 0.0, 1.0);
        let b: &str = "p=pat.Circle((0,0),1)\n\
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
                       p=pat.PathPatch(h)\n\
                       plt.gca().add_patch(p)\n";
        assert_eq!(shapes.buffer, b);
    }
}
