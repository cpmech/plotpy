use super::*;
use std::fmt::Write;

/// Generates scatter plot given two arrays (x,y)
///
/// # Example
///
/// ```
/// use plotpy::*;
/// let x = &[1.0, 2.0, 3.0, 4.0, 5.0];
/// let y = &[1.0, 4.0, 9.0, 16.0, 25.0];
/// let mut plot = Plot::new();
/// let mut scatter = Scatter::new();
/// scatter.marker_style = "*".to_string();
/// scatter.draw(x, y);
/// plot.add(&scatter);
/// ```
pub struct Scatter {
    /// Opacity of markers (0, 1]
    pub marker_alpha: f64,

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

impl Scatter {
    /// Creates new Scatter object
    pub fn new() -> Self {
        Scatter {
            marker_alpha: 0.0,
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

    /// Draw scatter graph
    ///
    /// # Input
    ///
    /// * `x` - abscissa values
    /// * `y` - ordinate values
    ///
    /// # Notes
    ///
    /// * The type `T` of the input matrices must be a number.
    ///
    pub fn draw<T>(&mut self, x: &[T], y: &[T])
    where
        T: std::fmt::Display,
    {
        vector_to_array(&mut self.buffer, "x", x);
        vector_to_array(&mut self.buffer, "y", y);
        let opt = self.options();
        write!(&mut self.buffer, "plt.scatter(x,y{})\n", &opt).unwrap();
    }

    pub(crate) fn options(&self) -> String {
        let mut opt = String::new();
        if self.marker_alpha > 0.0 {
            write!(&mut opt, ",alpha={}", self.marker_alpha).unwrap();
        }
        if self.marker_color != "" {
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

impl GraphMaker for Scatter {
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
        let scatter = Scatter::new();
        assert_eq!(scatter.marker_alpha, 0.0);
        assert_eq!(scatter.marker_color, String::new());
        assert_eq!(scatter.marker_every, 0);
        assert_eq!(scatter.marker_void, false);
        assert_eq!(scatter.marker_line_color, String::new());
        assert_eq!(scatter.marker_line_width, 0.0);
        assert_eq!(scatter.marker_size, 0.0);
        assert_eq!(scatter.marker_style, String::new());
        assert_eq!(scatter.buffer.len(), 0);
    }

    #[test]
    fn options_works() {
        let mut scatter = Scatter::new();
        scatter.marker_alpha = 0.5;
        scatter.marker_color = "#4c4deb".to_string();
        scatter.marker_every = 2;
        scatter.marker_void = false;
        scatter.marker_line_color = "blue".to_string();
        scatter.marker_line_width = 1.5;
        scatter.marker_size = 8.0;
        scatter.marker_style = "o".to_string();
        let opt = scatter.options();
        assert_eq!(
            opt,
            ",alpha=0.5\
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
        let mut scatter = Scatter::new();
        scatter.draw(x, y);
        let b: &str = "x=np.array([1,2,3,4,5,],dtype=float)\n\
                       y=np.array([1,4,9,16,25,],dtype=float)\n\
                       plt.scatter(x,y)\n";
        assert_eq!(scatter.buffer, b);
    }
}
