use super::GraphMaker;
use std::fmt::Write;

/// Implements the capability to add inset Axes to existing Axes.
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
    pub fn add(&mut self, graph: &dyn GraphMaker) -> &mut Self {
        let buf0 = graph.get_buffer();
        let buf1 = buf0.replace("plt.gca()", "zoom");
        let buf2 = buf1.replace("plt.imshow", "zoom.imshow");
        self.buffer.push_str(&buf2);
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

    /// Sets the limits of axes
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
    pub fn set_axes_visibility(&mut self, visible: bool) -> &mut Self {
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
}
