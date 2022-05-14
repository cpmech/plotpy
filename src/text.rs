use super::GraphMaker;
use std::fmt::Write;

/// Creates text to be added to a plot
///
/// # Example
///
/// ```
/// # fn main() -> Result<(), &'static str> {
/// // import
/// use plotpy::{Plot, Text};
/// use std::path::Path;
///
/// // directory to save figures
/// const OUT_DIR: &str = "/tmp/plotpy/doc_tests";
///
/// // configure and draw text
/// let mut text = Text::new();
/// text.set_color("#cd0000")
///     .set_align_horizontal("center")
///     .set_align_vertical("center")
///     .set_fontsize(30.0)
///     .set_rotation(45.0);
/// text.draw(0.0, 0.0, "Hello World!");
///
/// // add text to plot
/// let mut plot = Plot::new();
/// plot.add(&text)
///     .set_range(-1.0, 1.0, -1.0, 1.0)
///     .set_hide_axes(true);
///
/// // save figure
/// let path = Path::new(OUT_DIR).join("doc_text.svg");
/// plot.save(&path)?;
/// # Ok(())
/// # }
/// ```
///
/// ![doc_text.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/doc_text.svg)
///
pub struct Text {
    color: String,            // Color
    align_horizontal: String, // Horizontal alignment
    align_vertical: String,   // Vertical alignment
    fontsize: f64,            // Font size
    rotation: f64,            // Text rotation
    buffer: String,           // buffer
}

impl Text {
    /// Creates a new Text object
    pub fn new() -> Self {
        Text {
            color: String::new(),
            align_horizontal: String::new(),
            align_vertical: String::new(),
            fontsize: 0.0,
            rotation: 0.0,
            buffer: String::new(),
        }
    }

    /// Draws text
    pub fn draw(&mut self, x: f64, y: f64, message: &str) {
        let opt = self.options();
        write!(&mut self.buffer, "plt.text({},{},'{}'{})\n", x, y, message, &opt).unwrap();
    }

    /// Draws text in 3D plot
    pub fn draw_3d(&mut self, x: f64, y: f64, z: f64, message: &str) {
        let opt = self.options();
        write!(
            &mut self.buffer,
            "maybeCreateAX3D()\n\
             AX3D.text({},{},{},'{}'{})\n",
            x, y, z, message, &opt
        )
        .unwrap();
    }

    /// Sets the text color
    pub fn set_color(&mut self, color: &str) -> &mut Self {
        self.color = String::from(color);
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

    /// Returns options for text
    fn options(&self) -> String {
        let mut opt = String::new();
        if self.color != "" {
            write!(&mut opt, ",color='{}'", self.color).unwrap();
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
}

impl GraphMaker for Text {
    fn get_buffer<'a>(&'a self) -> &'a String {
        &self.buffer
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Text;

    #[test]
    fn new_works() {
        let text = Text::new();
        assert_eq!(text.color.len(), 0);
        assert_eq!(text.align_horizontal.len(), 0);
        assert_eq!(text.align_vertical.len(), 0);
        assert_eq!(text.fontsize, 0.0);
        assert_eq!(text.rotation, 0.0);
        assert_eq!(text.buffer.len(), 0);
    }

    #[test]
    fn options_works() {
        let mut text = Text::new();
        text.set_color("red")
            .set_align_horizontal("center")
            .set_align_vertical("center")
            .set_fontsize(8.0)
            .set_rotation(45.0);
        let opt = text.options();
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
    fn draw_works() {
        let mut text = Text::new();
        text.draw(1.2, 3.4, &"message".to_string());
        let b: &str = "plt.text(1.2,3.4,'message')\n";
        assert_eq!(text.buffer, b);
    }

    #[test]
    fn draw_3d_works() {
        let mut text = Text::new();
        text.draw_3d(1.2, 3.4, 5.6, &"message".to_string());
        let b: &str = "maybeCreateAX3D()\n\
                       AX3D.text(1.2,3.4,5.6,'message')\n";
        assert_eq!(text.buffer, b);
    }
}
