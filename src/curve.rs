use super::{vector_to_array, AsVector, GraphMaker};
use std::fmt::Write;

/// Generates a curve (aka line-plot) given two arrays (x,y)
///
/// # Notes
///
/// * This struct corresponds to the **plot** function of Matplotlib.
/// * You may plot a Scatter plot by setting line_style = "None"
///
/// # Example
///
/// ```
/// # fn main() -> Result<(), &'static str> {
/// // import
/// use plotpy::{Curve, Plot};
/// use russell_lab::Vector;
/// use std::path::Path;
///
/// // directory to save figures
/// const OUT_DIR: &str = "/tmp/plotpy/doc_tests";
///
/// // generate (x,y) points
/// let x = Vector::linspace(-1.0, 1.0, 21);
/// let y = x.get_mapped(|v| 1.0 / (1.0 + f64::exp(-5.0 * v)));
///
/// // configure curve
/// let mut curve = Curve::new();
/// curve.set_label("logistic function")
///     .set_line_alpha(0.8)
///     .set_line_color("#5f9cd8")
///     .set_line_style("-")
///     .set_line_width(5.0)
///     .set_marker_color("#eeea83")
///     .set_marker_every(5)
///     .set_marker_line_color("#da98d1")
///     .set_marker_line_width(2.5)
///     .set_marker_size(20.0)
///     .set_marker_style("*");
///
/// // draw curve
/// curve.draw(&x, &y);
///
/// // add curve to plot
/// let mut plot = Plot::new();
/// plot.add(&curve)
///     .set_num_ticks_y(11)
///     .grid_labels_legend("x", "y");
///
/// // save figure
/// let path = Path::new(OUT_DIR).join("doc_curve.svg");
/// plot.save(&path)?;
/// # Ok(())
/// # }
/// ```
///
/// ![doc_curve.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/doc_curve.svg)
///
pub struct Curve {
    label: String,             // Name of this curve in the legend
    line_alpha: f64,           // Opacity of lines (0, 1]. A<1e-14 => A=1.0
    line_color: String,        // Color of lines
    line_style: String,        // Style of lines
    line_width: f64,           // Width of lines
    marker_color: String,      // Color of markers
    marker_every: usize,       // Increment of data points to use when drawing markers
    marker_void: bool,         // Draw a void marker (draw edge only)
    marker_line_color: String, // Edge color of markers
    marker_line_width: f64,    // Edge width of markers
    marker_size: f64,          // Size of markers
    marker_style: String,      // Style of markers, e.g., "`o`", "`+`"
    buffer: String,            // buffer
}

impl Curve {
    /// Creates new Curve object
    pub fn new() -> Self {
        Curve {
            label: String::new(),
            line_alpha: 0.0,
            line_color: String::new(),
            line_style: String::new(),
            line_width: 0.0,
            marker_color: String::new(),
            marker_every: 0,
            marker_void: false,
            marker_line_color: String::new(),
            marker_line_width: 0.0,
            marker_size: 0.0,
            marker_style: String::new(),
            buffer: String::new(),
        }
    }

    /// Draws curve
    ///
    /// # Input
    ///
    /// * `x` - abscissa values
    /// * `y` - ordinate values
    ///
    /// # Notes
    ///
    /// * The type `U` of the input array must be a number.
    ///
    pub fn draw<'a, T, U>(&mut self, x: &'a T, y: &'a T)
    where
        T: AsVector<'a, U>,
        U: 'a + std::fmt::Display,
    {
        vector_to_array(&mut self.buffer, "x", x);
        vector_to_array(&mut self.buffer, "y", y);
        let opt = self.options();
        write!(&mut self.buffer, "plt.plot(x,y{})\n", &opt).unwrap();
    }

    /// Sets the name of this curve in the legend
    pub fn set_label(&mut self, label: &str) -> &mut Self {
        self.label = String::from(label);
        self
    }

    /// Sets the opacity of lines (0, 1]. A<1e-14 => A=1.0
    pub fn set_line_alpha(&mut self, alpha: f64) -> &mut Self {
        self.line_alpha = alpha;
        self
    }

    /// Sets the color of lines
    pub fn set_line_color(&mut self, color: &str) -> &mut Self {
        self.line_color = String::from(color);
        self
    }

    /// Sets the style of lines
    ///
    /// Options:
    ///
    /// * "`-`", `:`", "`--`", "`-.`", or "`None`"
    /// * As defined in <https://matplotlib.org/stable/gallery/lines_bars_and_markers/linestyles.html>
    pub fn set_line_style(&mut self, style: &str) -> &mut Self {
        self.line_style = String::from(style);
        self
    }

    /// Sets the width of lines
    pub fn set_line_width(&mut self, width: f64) -> &mut Self {
        self.line_width = width;
        self
    }

    /// Sets the color of markers
    pub fn set_marker_color(&mut self, color: &str) -> &mut Self {
        self.marker_color = String::from(color);
        self
    }

    /// Sets the increment of data points to use when drawing markers
    pub fn set_marker_every(&mut self, every: usize) -> &mut Self {
        self.marker_every = every;
        self
    }

    /// Sets the option to draw a void marker (draw edge only)
    pub fn set_marker_void(&mut self, flag: bool) -> &mut Self {
        self.marker_void = flag;
        self
    }

    /// Sets the edge color of markers
    pub fn set_marker_line_color(&mut self, color: &str) -> &mut Self {
        self.marker_line_color = String::from(color);
        self
    }

    /// Sets the edge width of markers
    pub fn set_marker_line_width(&mut self, width: f64) -> &mut Self {
        self.marker_line_width = width;
        self
    }

    /// Sets the size of markers
    pub fn set_marker_size(&mut self, size: f64) -> &mut Self {
        self.marker_size = size;
        self
    }

    /// Sets the style of markers
    ///
    /// Examples:
    ///
    /// * "`o`", "`+`"
    /// * As defined in <https://matplotlib.org/stable/api/markers_api.html>
    pub fn set_marker_style(&mut self, style: &str) -> &mut Self {
        self.marker_style = String::from(style);
        self
    }

    /// Returns options for curve
    fn options(&self) -> String {
        // fix color if marker is void
        let line_color = if self.marker_void && self.line_color == "" {
            "red"
        } else {
            &self.line_color
        };

        // output
        let mut opt = String::new();

        // label
        if self.label != "" {
            write!(&mut opt, ",label='{}'", self.label).unwrap();
        }

        // lines
        if self.line_alpha > 0.0 {
            write!(&mut opt, ",alpha={}", self.line_alpha).unwrap();
        }
        if line_color != "" {
            write!(&mut opt, ",color='{}'", line_color).unwrap();
        }
        if self.line_style != "" {
            write!(&mut opt, ",linestyle='{}'", self.line_style).unwrap();
        }
        if self.line_width > 0.0 {
            write!(&mut opt, ",linewidth={}", self.line_width).unwrap();
        }

        // markers
        if !self.marker_void && self.marker_color != "" {
            write!(&mut opt, ",markerfacecolor='{}'", self.marker_color).unwrap();
        }
        if self.marker_every > 0 {
            write!(&mut opt, ",markevery={}", self.marker_every).unwrap();
        }
        if self.marker_void {
            write!(&mut opt, ",markerfacecolor='none'").unwrap();
        }
        if self.marker_line_color != "" {
            write!(&mut opt, ",markeredgecolor='{}'", self.marker_line_color).unwrap();
        }
        if self.marker_line_width > 0.0 {
            write!(&mut opt, ",markeredgewidth={}", self.marker_line_width).unwrap();
        }
        if self.marker_size > 0.0 {
            write!(&mut opt, ",markersize={}", self.marker_size).unwrap();
        }
        if self.marker_style != "" {
            write!(&mut opt, ",marker='{}'", self.marker_style).unwrap();
        }

        opt
    }
}

impl GraphMaker for Curve {
    fn get_buffer<'a>(&'a self) -> &'a String {
        &self.buffer
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Curve;
    use russell_lab::Vector;

    #[test]
    fn new_works() {
        let curve = Curve::new();
        assert_eq!(curve.label.len(), 0);
        assert_eq!(curve.line_alpha, 0.0);
        assert_eq!(curve.line_color.len(), 0);
        assert_eq!(curve.line_style.len(), 0);
        assert_eq!(curve.line_width, 0.0);
        assert_eq!(curve.marker_color.len(), 0);
        assert_eq!(curve.marker_every, 0);
        assert_eq!(curve.marker_void, false);
        assert_eq!(curve.marker_line_color.len(), 0);
        assert_eq!(curve.marker_line_width, 0.0);
        assert_eq!(curve.marker_size, 0.0);
        assert_eq!(curve.marker_style.len(), 0);
        assert_eq!(curve.buffer.len(), 0);
    }

    #[test]
    fn options_works() {
        let mut curve = Curve::new();
        curve
            .set_label("my-curve")
            .set_line_alpha(0.7)
            .set_line_color("#b33434")
            .set_line_style("-")
            .set_line_width(3.0)
            .set_marker_color("#4c4deb")
            .set_marker_every(2)
            .set_marker_void(false)
            .set_marker_line_color("blue")
            .set_marker_line_width(1.5)
            .set_marker_size(8.0)
            .set_marker_style("o");
        let options = curve.options();
        assert_eq!(
            options,
            ",label='my-curve'\
             ,alpha=0.7\
             ,color='#b33434'\
             ,linestyle='-'\
             ,linewidth=3\
             ,markerfacecolor='#4c4deb'\
             ,markevery=2\
             ,markeredgecolor='blue'\
             ,markeredgewidth=1.5\
             ,markersize=8\
             ,marker='o'"
        );
    }

    #[test]
    fn draw_works() {
        let x = &[1.0, 2.0, 3.0, 4.0, 5.0];
        let y = &[1.0, 4.0, 9.0, 16.0, 25.0];
        let mut curve = Curve::new();
        curve.set_label("the-curve");
        curve.draw(x, y);
        let b: &str = "x=np.array([1,2,3,4,5,],dtype=float)\n\
                       y=np.array([1,4,9,16,25,],dtype=float)\n\
                       plt.plot(x,y,label='the-curve')\n";
        assert_eq!(curve.buffer, b);
    }

    #[test]
    fn draw_with_vector_works() {
        let x = Vector::from(&[1.0, 2.0, 3.0, 4.0, 5.0]);
        let y = Vector::from(&[1.0, 4.0, 9.0, 16.0, 25.0]);
        let mut curve = Curve::new();
        curve.set_label("the-curve");
        curve.draw(&x, &y);
        let b: &str = "x=np.array([1,2,3,4,5,],dtype=float)\n\
                       y=np.array([1,4,9,16,25,],dtype=float)\n\
                       plt.plot(x,y,label='the-curve')\n";
        assert_eq!(curve.buffer, b);
    }
}
