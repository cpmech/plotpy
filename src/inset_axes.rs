use super::GraphMaker;
use std::fmt::Write;

/// Implements the capability to add inset Axes to existing Axes.
///
/// # Warning
///
/// **WARNING:** If the range of axes has been modified in [crate::Plot], e.g. by `plot.set_range(...)`,
/// then the inset must be added after the range has been set. Otherwise, the inset will not be displayed correctly.
/// Specifically the connector lines will not be drawn if the inset is added before `set_range`.
///
/// For example, below is the correct procedure:
///
/// ```
/// use plotpy::{Plot, InsetAxes};
///
/// let mut inset = InsetAxes::new();
/// inset.draw(0.5, 0.5, 0.4, 0.3);
///
/// let mut plot = Plot::new();
/// plot.set_range(0.0, 10.0, 0.0, 10.0)
///     .add(&inset); // IMPORTANT: add inset after setting the range
/// ```
pub struct InsetAxes {
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
    extra_for_axes: String,
    extra_for_indicator: String,
    indicator_line_style: String,
    indicator_line_color: String,
    indicator_line_width: f64,
    indicator_hatch: String,
    indicator_alpha: Option<f64>,
    axes_visible: bool,
    title: String,
    buffer: String,
}

impl InsetAxes {
    /// Creates a new `InsetAxes` object with an empty buffer.
    ///
    /// # Returns
    ///
    /// A new instance of `InsetAxes`.
    ///
    /// # Warning
    ///
    /// **WARNING:** If the range of axes has been modified in [crate::Plot], e.g. by `plot.set_range(...)`,
    /// then the inset must be added after the range has been set. Otherwise, the inset will not be displayed correctly.
    /// Specifically the connector lines will not be drawn if the inset is added before `set_range`.
    ///
    /// For example, below is the correct procedure:
    ///
    /// ```
    /// use plotpy::{InsetAxes, Plot};
    /// let mut inset = InsetAxes::new();
    /// let mut plot = Plot::new();
    /// plot.set_range(0.0, 10.0, 0.0, 10.0)
    ///     .add(&inset); // IMPORTANT: add inset after setting the range
    /// ```
    pub fn new() -> Self {
        Self {
            xmin: 0.0,
            xmax: 1.0,
            ymin: 0.0,
            ymax: 1.0,
            extra_for_axes: String::new(),
            extra_for_indicator: String::new(),
            indicator_line_style: String::new(),
            indicator_line_color: String::new(),
            indicator_line_width: 0.0,
            indicator_hatch: String::new(),
            indicator_alpha: None,
            axes_visible: false,
            title: String::new(),
            buffer: String::new(),
        }
    }

    /// Sets the line style for the indicator (e.g. "--", ":", "-.")
    pub fn set_indicator_line_style(&mut self, style: &str) -> &mut Self {
        self.indicator_line_style = style.to_string();
        self
    }

    /// Sets the line color for the indicator (e.g. "red", "#FF0000")
    pub fn set_indicator_line_color(&mut self, color: &str) -> &mut Self {
        self.indicator_line_color = color.to_string();
        self
    }

    /// Sets the line width for the indicator
    pub fn set_indicator_line_width(&mut self, width: f64) -> &mut Self {
        self.indicator_line_width = width;
        self
    }

    /// Sets the alpha (opacity) for the indicator
    pub fn set_indicator_alpha(&mut self, alpha: f64) -> &mut Self {
        self.indicator_alpha = Some(alpha);
        self
    }

    /// Sets the hatch pattern for the indicator (e.g. "/", "\\", "|", "-", "+", "x", "o", "O", ".", "*")
    ///
    /// Common hatch patterns include:                                                                                 
    ///
    /// * "/" - diagonal hatching                                                                                     
    /// * "\" - back diagonal hatching                                                                                
    /// * "|" - vertical hatching                                                                                     
    /// * "-" - horizontal hatching                                                                                   
    /// * "+" - crossed hatching                                                                                      
    /// * "x" - crossed diagonal hatching                                                                             
    /// * "o" - small circle hatching                                                                                 
    /// * "O" - large circle hatching                                                                                 
    /// * "." - dot hatching                                                                                          
    /// * "*" - star hatching  
    ///
    /// [See options in ](https://matplotlib.org/stable/api/_as_gen/matplotlib.axes.Axes.indicate_inset.html#matplotlib.axes.Axes.indicate_inset)
    ///
    /// [See Matplotlib's documentation for more hatch patterns](https://matplotlib.org/stable/gallery/shapes_and_collections/hatch_demo.html)
    pub fn set_indicator_hatch(&mut self, hatch: &str) -> &mut Self {
        self.indicator_hatch = hatch.to_string();
        self
    }

    /// Adds new graph entity
    ///
    /// # Warning
    ///
    /// **WARNING:** If the range of axes has been modified in [crate::Plot], e.g. by `plot.set_range(...)`,
    /// then the inset must be added after the range has been set. Otherwise, the inset will not be displayed correctly.
    /// Specifically the connector lines will not be drawn if the inset is added before `set_range`.
    ///
    /// For example, below is the correct procedure:
    ///
    /// ```
    /// use plotpy::{InsetAxes, Plot};
    /// let mut inset = InsetAxes::new();
    /// let mut plot = Plot::new();
    /// plot.set_range(0.0, 10.0, 0.0, 10.0)
    ///     .add(&inset); // IMPORTANT: add inset after setting the range
    /// ```
    pub fn add(&mut self, graph: &dyn GraphMaker) -> &mut Self {
        let buf = graph
            .get_buffer()
            .replace("plt.gca()", "zoom")
            .replace("plt.barh", "zoom.barh")
            .replace("plt.bar", "zoom.bar")
            .replace("plt.imshow", "zoom.imshow")
            .replace("plt.plot", "zoom.plot");
        self.buffer.push_str(&buf);
        self
    }

    /// Draws the inset Axes.
    ///
    /// Example of normalized coordinates: `(0.5, 0.5, 0.4, 0.3)`.
    ///
    /// # Arguments
    ///
    /// * `u0` -- The normalized (0 to 1) horizontal figure coordinate of the lower-left corner of the inset Axes.
    /// * `v0` -- The normalized (0 to 1) vertical figure coordinate of the lower-left corner of the inset Axes.
    /// * `width` -- The width of the inset Axes.
    /// * `height` -- The height of the inset Axes.
    ///
    /// # Warning
    ///
    /// **WARNING:** If the range of axes has been modified in [crate::Plot], e.g. by `plot.set_range(...)`,
    /// then the inset must be added after the range has been set. Otherwise, the inset will not be displayed correctly.
    /// Specifically the connector lines will not be drawn if the inset is added before `set_range`.
    ///
    /// For example, below is the correct procedure:
    ///
    /// ```
    /// use plotpy::{Curve, InsetAxes, Plot, StrError};
    ///
    /// fn main() -> Result<(), StrError> {
    ///     // draw curve
    ///     let mut curve = Curve::new();
    ///     curve.draw(&[0.0, 1.0, 2.0], &[0.0, 1.0, 4.0]);
    ///
    ///     // allocate inset and add curve to it
    ///     let mut inset = InsetAxes::new();
    ///     inset
    ///         .add(&curve) // add curve to inset
    ///         .set_range(0.5, 1.5, 0.5, 1.5) // set the range of the inset
    ///         .draw(0.5, 0.5, 0.4, 0.3);
    ///
    ///     // add curve and inset to plot
    ///     let mut plot = Plot::new();
    ///     plot.add(&curve)
    ///         .set_range(0.0, 5.0, 0.0, 5.0)
    ///         .add(&inset) // IMPORTANT: add inset after setting the range
    ///         .save("/tmp/plotpy/doc_tests/doc_inset_axes_add.svg")
    /// }
    /// ```
    ///
    /// ![doc_inset_axes_add.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/doc_inset_axes_add.svg)
    pub fn draw(&mut self, u0: f64, v0: f64, width: f64, height: f64) {
        let opt1 = self.options_for_axes();
        let opt2 = self.options_for_indicator();
        self.buffer.insert_str(
            0,
            &format!(
                "zoom=plt.gca().inset_axes([{},{},{},{}],xlim=({},{}),ylim=({},{}){})\n",
                u0, v0, width, height, self.xmin, self.xmax, self.ymin, self.ymax, opt1,
            ),
        );
        if !self.axes_visible {
            write!(&mut self.buffer, "zoom.set_xticks([])\nzoom.set_yticks([])\n").unwrap();
        }
        if !self.title.is_empty() {
            write!(&mut self.buffer, "zoom.set_title(r'{}')\n", self.title).unwrap();
        }
        write!(&mut self.buffer, "plt.gca().indicate_inset_zoom(zoom{})\n", opt2,).unwrap();
    }

    /// Sets the limits of axes in the inset.
    pub fn set_range(&mut self, xmin: f64, xmax: f64, ymin: f64, ymax: f64) -> &mut Self {
        self.xmin = xmin;
        self.xmax = xmax;
        self.ymin = ymin;
        self.ymax = ymax;
        self
    }

    /// Sets extra Matplotlib commands for the inset Axes (comma separated).
    ///
    /// [See Matplotlib's documentation for extra parameters](<https://matplotlib.org/stable/api/_as_gen/matplotlib.axes.Axes.html#matplotlib.axes.Axes>)
    pub fn set_extra_for_axes(&mut self, extra: &str) -> &mut Self {
        self.extra_for_axes = extra.to_string();
        self
    }

    /// Sets extra Matplotlib commands for the indicator (comma separated).
    ///
    /// [See Matplotlib's documentation for extra parameters](https://matplotlib.org/stable/api/_as_gen/matplotlib.axes.Axes.indicate_inset.html#matplotlib.axes.Axes.indicate_inset)
    pub fn set_extra_for_indicator(&mut self, extra: &str) -> &mut Self {
        self.extra_for_indicator = extra.to_string();
        self
    }

    /// Sets the visibility of the axes ticks
    ///
    /// # Arguments
    ///
    /// * `visible` - If true, shows the axes ticks. If false, hides them.
    pub fn set_visibility(&mut self, visible: bool) -> &mut Self {
        self.axes_visible = visible;
        self
    }

    /// Sets the title of the inset axes
    pub fn set_title(&mut self, title: &str) -> &mut Self {
        self.title = title.to_string();
        self
    }

    /// Returns options for the inset Axes
    fn options_for_axes(&self) -> String {
        let mut opt = String::new();
        if !self.extra_for_axes.is_empty() {
            write!(&mut opt, ",{}", self.extra_for_axes).unwrap();
        }
        opt
    }

    /// Returns options for the indicator
    fn options_for_indicator(&self) -> String {
        let mut opt = String::new();
        if !self.indicator_line_style.is_empty() {
            write!(&mut opt, ",linestyle='{}'", self.indicator_line_style).unwrap();
        }
        if !self.indicator_line_color.is_empty() {
            write!(&mut opt, ",edgecolor='{}'", self.indicator_line_color).unwrap();
        }
        if self.indicator_line_width > 0.0 {
            write!(&mut opt, ",linewidth={}", self.indicator_line_width).unwrap();
        }
        if !self.indicator_hatch.is_empty() {
            write!(&mut opt, ",hatch='{}'", self.indicator_hatch).unwrap();
        }
        if let Some(alpha) = self.indicator_alpha {
            write!(&mut opt, ",alpha={}", alpha).unwrap();
        }
        if !self.extra_for_indicator.is_empty() {
            write!(&mut opt, ",{}", self.extra_for_indicator).unwrap();
        }
        opt
    }
}

impl GraphMaker for InsetAxes {
    /// Returns a reference to the buffer containing the generated commands.
    ///
    /// # Returns
    ///
    /// A reference to the buffer as a `String`.
    fn get_buffer<'a>(&'a self) -> &'a String {
        &self.buffer
    }

    /// Clears the buffer, removing all stored commands.
    fn clear_buffer(&mut self) {
        self.buffer.clear();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::InsetAxes;
    use crate::GraphMaker;

    #[test]
    fn test_new() {
        let inset = InsetAxes::new();
        assert_eq!(inset.xmin, 0.0);
        assert_eq!(inset.xmax, 1.0);
        assert_eq!(inset.ymin, 0.0);
        assert_eq!(inset.ymax, 1.0);
        assert!(inset.buffer.is_empty());
    }

    #[test]
    fn test_set_range() {
        let mut inset = InsetAxes::new();
        inset.set_range(-1.0, 2.0, -3.0, 4.0);
        assert_eq!(inset.xmin, -1.0);
        assert_eq!(inset.xmax, 2.0);
        assert_eq!(inset.ymin, -3.0);
        assert_eq!(inset.ymax, 4.0);
    }

    #[test]
    fn test_set_title() {
        let mut inset = InsetAxes::new();
        inset.set_title("Test Title");
        assert_eq!(inset.title, "Test Title");
    }

    #[test]
    fn test_set_visibility() {
        let mut inset = InsetAxes::new();
        inset.set_visibility(true);
        assert!(inset.axes_visible);
        inset.set_visibility(false);
        assert!(!inset.axes_visible);
    }

    #[test]
    fn test_indicator_options() {
        let mut inset = InsetAxes::new();
        inset
            .set_indicator_line_style("--")
            .set_indicator_line_color("red")
            .set_indicator_line_width(2.0)
            .set_indicator_hatch("/")
            .set_indicator_alpha(0.5);

        let options = inset.options_for_indicator();
        assert!(options.contains("linestyle='--'"));
        assert!(options.contains("edgecolor='red'"));
        assert!(options.contains("linewidth=2"));
        assert!(options.contains("hatch='/'"));
        assert!(options.contains("alpha=0.5"));
    }

    #[test]
    fn test_draw_basic() {
        let mut inset = InsetAxes::new();
        inset.draw(0.5, 0.5, 0.4, 0.3);
        let buffer = inset.get_buffer();
        assert!(buffer.contains("zoom=plt.gca().inset_axes([0.5,0.5,0.4,0.3]"));
        assert!(buffer.contains("plt.gca().indicate_inset_zoom(zoom"));
    }

    #[test]
    fn test_clear_buffer() {
        let mut inset = InsetAxes::new();
        inset.draw(0.5, 0.5, 0.4, 0.3);
        assert!(!inset.buffer.is_empty());
        inset.clear_buffer();
        assert!(inset.buffer.is_empty());
    }
}
