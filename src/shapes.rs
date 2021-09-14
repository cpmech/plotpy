use super::*;
use std::fmt::Write;

/// Draw polygonal shapes
pub struct Shapes {
    /// Edge color (shared)
    pub edge_color: String,

    /// Face color (shared)
    pub face_color: String,

    /// Line width of edge (shared)
    pub line_width: f64,

    /// Closed polygonal (shared)
    pub closed: bool,

    /// Arrow scale
    pub arrow_scale: f64,

    /// Arrow style
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
    ///
    /// As defined in <https://matplotlib.org/stable/api/_as_gen/matplotlib.patches.FancyArrowPatch.html>
    pub arrow_style: String,

    // buffer
    pub(crate) buffer: String,
}

impl Shapes {
    pub fn new() -> Self {
        Shapes {
            edge_color: String::new(),
            face_color: String::new(),
            line_width: 0.0,
            closed: false,
            arrow_scale: 0.0,
            arrow_style: String::new(),
            buffer: String::new(),
        }
    }

    /// Draws arrow
    pub fn arrow(&mut self, xi: f64, yi: f64, xf: f64, yf: f64) {
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

    /// Returns shared options
    pub(crate) fn options_shared(&self) -> String {
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
    pub(crate) fn options_arrow(&self) -> String {
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
        assert_eq!(shapes.edge_color, "");
    }

    #[test]
    fn options_shared_works() {
        let mut shapes = Shapes::new();
        shapes.edge_color = "red".to_string();
        shapes.face_color = "blue".to_string();
        shapes.line_width = 2.5;
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
        shapes.arrow_scale = 25.0;
        shapes.arrow_style = "fancy".to_string();
        let opt = shapes.options_arrow();
        assert_eq!(
            opt,
            ",mutation_scale=25\
             ,arrowstyle='fancy'"
        );
    }

    #[test]
    fn arrow_woks() {
        let mut shapes = Shapes::new();
        shapes.arrow(0.0, 0.0, 1.0, 1.0);
        let b: &str =
            "p=pat.FancyArrowPatch((0,0),(1,1),shrinkA=0,shrinkB=0,path_effects=[pff.Stroke(joinstyle='miter')])\n\
             plt.gca().add_patch(p)\n";
        assert_eq!(shapes.buffer, b);
    }
}
