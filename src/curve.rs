use super::*;
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
/// use plotpy::*;
/// use std::path::Path;
///
/// // directory to save figures
/// const OUT_DIR: &str = "/tmp/plotpy/doc_tests";
///
/// // generate (x,y) points
/// let x = &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
/// let y = &[1.0, 4.0, 9.0, 16.0, 25.0, 36.0, 49.0, 64.0];
///
/// // configure and draw curve
/// let mut curve = Curve::new();
/// curve.label = "parabolic".to_string();
/// curve.line_alpha = 0.95;
/// curve.line_color = "#5f9cd8".to_string();
/// curve.line_style = "-".to_string();
/// curve.line_width = 5.0;
/// curve.marker_color = "#eeea83".to_string();
/// curve.marker_every = 1;
/// curve.marker_line_color = "#da98d1".to_string();
/// curve.marker_line_width = 2.5;
/// curve.marker_size = 20.0;
/// curve.marker_style = "*".to_string();
/// curve.draw(x, y);
///
/// // add curve to plot
/// let mut plot = Plot::new();
/// plot.add(&curve);
/// plot.grid_and_labels("x", "y");
///
/// // save figure
/// let path = Path::new(OUT_DIR).join("doc_curve.svg");
/// plot.save(&path)?;
/// # Ok(())
/// # }
/// ```
pub struct Curve {
    /// Label; name of this curve in the legend
    pub label: String,

    /// Opacity of lines (0, 1]. A<1e-14 => A=1.0
    pub line_alpha: f64,

    /// Color of lines
    pub line_color: String,

    /// Style of lines
    ///
    /// Options: "`-`", `:`", "`--`", "`-.`", or "`None`"
    ///
    /// As defined in <https://matplotlib.org/stable/gallery/lines_bars_and_markers/linestyles.html>
    pub line_style: String,

    /// Width of lines
    pub line_width: f64,

    /// Color of markers
    pub marker_color: String,

    /// Increment of data points to use when drawing markers
    pub marker_every: i32,

    /// Draw a void marker (draw edge only)
    pub marker_void: bool,

    /// Edge color of markers
    pub marker_line_color: String,

    /// Edge width of markers
    pub marker_line_width: f64,

    /// Size of markers
    pub marker_size: f64,

    /// Style of markers, e.g., "`o`", "`+`"
    ///
    /// As defined in <https://matplotlib.org/stable/api/markers_api.html>
    pub marker_style: String,

    // buffer
    pub(crate) buffer: String,
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
    /// * The type `T` of the input array must be a number.
    ///
    pub fn draw<T>(&mut self, x: &[T], y: &[T])
    where
        T: std::fmt::Display,
    {
        vector_to_array(&mut self.buffer, "x", x);
        vector_to_array(&mut self.buffer, "y", y);
        let opt = self.options();
        write!(&mut self.buffer, "plt.plot(x,y{})\n", &opt).unwrap();
    }

    /// Returns options for curve
    pub(crate) fn options(&self) -> String {
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
    use super::*;

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
        curve.label = "my-curve".to_string();
        curve.line_alpha = 0.7;
        curve.line_color = "#b33434".to_string();
        curve.line_style = "-".to_string();
        curve.line_width = 3.0;
        curve.marker_color = "#4c4deb".to_string();
        curve.marker_every = 2;
        curve.marker_void = false;
        curve.marker_line_color = "blue".to_string();
        curve.marker_line_width = 1.5;
        curve.marker_size = 8.0;
        curve.marker_style = "o".to_string();
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
        curve.label = "the-curve".to_string();
        curve.draw(x, y);
        let b: &str = "x=np.array([1,2,3,4,5,],dtype=float)\n\
                       y=np.array([1,4,9,16,25,],dtype=float)\n\
                       plt.plot(x,y,label='the-curve')\n";
        assert_eq!(curve.buffer, b);
    }
}
