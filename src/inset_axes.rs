use super::GraphMaker;
use std::fmt::Write;

/// Implements the capability to add inset Axes to existing Axes.
pub struct InsetAxes {
    handle: String,           // Python's variable name
    xlim: Option<(f64, f64)>, // range for x
    ylim: Option<(f64, f64)>, // range for y
    extra: String,            // extra commands (comma separated)
    buffer: String,           // buffer
}

impl InsetAxes {
    /// Creates a new `InsetAxes` object with an empty buffer.
    ///
    /// # Arguments
    ///
    /// * `handle` - The name of the **Python variable** that will hold the inset Axes.
    ///
    /// # Returns
    ///
    /// A new instance of `InsetAxes`.
    pub fn new(handle: &str) -> Self {
        Self {
            handle: handle.to_string(),
            xlim: None,
            ylim: None,
            extra: String::new(),
            buffer: String::new(),
        }
    }

    /// Draws the `InsetAxes` onto the current Axes.
    ///
    /// This function generates a command to create an inset Axes within the current Axes.
    /// The command is stored in the buffer.
    ///
    /// **Note::** The `handle` is the name of the **Python variable** that will hold the inset Axes.
    ///
    /// # Arguments
    ///
    /// * `x0` - The x-coordinate of the lower-left corner of the inset Axes.
    /// * `y0` - The y-coordinate of the lower-left corner of the inset Axes.
    /// * `width` - The width of the inset Axes.
    /// * `height` - The height of the inset Axes.
    ///
    /// The bounds are `(x0, y0, width, height)` where `x0`, `y0` are the lower-left corner of the inset Axes,
    /// and `width`, `height` are the width and height of the inset Axes.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `InsetAxes` instance, allowing for method chaining.
    pub fn draw(&mut self, x0: f64, y0: f64, width: f64, height: f64) -> &mut Self {
        let opt = self.options();
        write!(
            &mut self.buffer,
            "{} = plt.gca().inset_axes([{},{},{},{}]{})\n",
            self.handle, x0, y0, width, height, opt
        )
        .unwrap();
        self
    }

    /// Sets the x-range for the inset Axes.
    ///
    /// # Arguments
    ///
    /// * `xmin` - The minimum x-value.
    /// * `xmax` - The maximum x-value.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `InsetAxes` instance, allowing for method chaining.
    pub fn set_xlim(&mut self, xmin: f64, xmax: f64) -> &mut Self {
        self.xlim = Some((xmin, xmax));
        self
    }

    /// Sets the y-range for the inset Axes.
    ///
    /// # Arguments
    ///
    /// * `ymin` - The minimum y-value.
    /// * `ymax` - The maximum y-value.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `InsetAxes` instance, allowing for method chaining.
    pub fn set_ylim(&mut self, ymin: f64, ymax: f64) -> &mut Self {
        self.ylim = Some((ymin, ymax));
        self
    }

    /// Sets extra Matplotlib commands (comma separated).
    ///
    /// # Arguments
    ///
    /// * `extra` - A string containing extra Matplotlib commands, separated by commas.
    ///
    /// **Important:** The extra commands must be comma separated.
    ///
    /// [See Matplotlib's documentation for extra parameters](https://matplotlib.org/stable/api/_as_gen/matplotlib.axes.Axes.inset_axes.html#matplotlib.axes.Axes.inset_axes)
    ///
    /// # Returns
    ///
    /// A mutable reference to the `InsetAxes` instance, allowing for method chaining.
    pub fn set_extra(&mut self, extra: &str) -> &mut Self {
        self.extra = extra.to_string();
        self
    }

    /// Returns options for the `InsetAxes`.
    ///
    /// This function generates a string containing the options for the `InsetAxes`,
    /// including the x-range, y-range, and any extra commands.
    ///
    /// # Returns
    ///
    /// A string containing the options for the `InsetAxes`.
    fn options(&self) -> String {
        let mut opt = String::new();

        if let Some((xmin, xmax)) = self.xlim {
            write!(&mut opt, ",xlim=({},{})", xmin, xmax).unwrap();
        }

        if let Some((ymin, ymax)) = self.ylim {
            write!(&mut opt, ",ylim=({},{})", ymin, ymax).unwrap();
        }

        if !self.extra.is_empty() {
            write!(&mut opt, ",{}", self.extra).unwrap();
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
    fn new_works() {
        let inset = InsetAxes::new("zoom");
        assert_eq!(inset.handle, "zoom");
        assert_eq!(inset.get_buffer(), "");
    }

    #[test]
    fn draw_works() {
        let mut inset = InsetAxes::new("zoom");
        inset.draw(0.1, 0.1, 0.4, 0.4);
        assert_eq!(inset.get_buffer(), "zoom = plt.gca().inset_axes([0.1,0.1,0.4,0.4])\n");
    }

    #[test]
    fn set_xlim_works() {
        let mut inset = InsetAxes::new("zoom");
        inset.set_xlim(8.0, 9.0);
        assert_eq!(inset.options(), ",xlim=(8,9)");
    }

    #[test]
    fn set_ylim_works() {
        let mut inset = InsetAxes::new("zoom");
        inset.set_ylim(3.0, 5.0);
        assert_eq!(inset.options(), ",ylim=(3,5)");
    }

    #[test]
    fn set_extra_works() {
        let mut inset = InsetAxes::new("zoom");
        inset.set_extra("aspect='equal'");
        assert_eq!(inset.options(), ",aspect='equal'");
    }

    #[test]
    fn options_combined_works() {
        let mut inset = InsetAxes::new("zoom");
        inset.set_xlim(1.0, 2.0).set_ylim(3.0, 4.0).set_extra("aspect='equal'");
        assert_eq!(inset.options(), ",xlim=(1,2),ylim=(3,4),aspect='equal'");
    }

    #[test]
    fn clear_buffer_works() {
        let mut inset = InsetAxes::new("zoom");
        inset.draw(0.1, 0.1, 0.4, 0.4);
        inset.clear_buffer();
        assert_eq!(inset.get_buffer(), "");
    }
}
