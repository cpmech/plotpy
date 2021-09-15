use super::*;
use std::fmt::Write;

/// Creates text to be added to a plot
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
/// // configure and draw text
/// let mut text = Text::new();
/// text.color = "#cd0000".to_string();
/// text.align_horizontal = "center".to_string();
/// text.align_vertical = "center".to_string();
/// text.font_size = 30.0;
/// text.rotation = 45.0;
/// text.draw(0.0, 0.0, "Hello World!");
///
/// // add text to plot
/// let mut plot = Plot::new();
/// plot.add(&text);
/// plot.range(-1.0, 1.0, -1.0, 1.0);
/// plot.hide_axes();
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
    /// Color
    pub color: String,

    /// Horizontal alignment: "center", "left", "right"
    pub align_horizontal: String,

    /// Vertical alignment: "center", "top", "bottom", "baseline", "center_baseline"
    pub align_vertical: String,

    /// Font size
    pub font_size: f64,

    /// Text rotation
    pub rotation: f64,

    // buffer
    pub(crate) buffer: String,
}

impl Text {
    /// Creates a new Text object
    pub fn new() -> Self {
        Text {
            color: String::new(),
            align_horizontal: String::new(),
            align_vertical: String::new(),
            font_size: 0.0,
            rotation: 0.0,
            buffer: String::new(),
        }
    }

    /// Draws text
    pub fn draw(&mut self, x: f64, y: f64, message: &str) {
        let opt = self.options();
        write!(&mut self.buffer, "plt.text({},{},'{}'{})\n", x, y, message, &opt).unwrap();
    }

    /// Returns options for text
    pub(crate) fn options(&self) -> String {
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
        if self.font_size > 0.0 {
            write!(&mut opt, ",fontsize={}", self.font_size).unwrap();
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
    use super::*;

    #[test]
    fn new_works() {
        let text = Text::new();
        assert_eq!(text.color.len(), 0);
        assert_eq!(text.align_horizontal.len(), 0);
        assert_eq!(text.align_vertical.len(), 0);
        assert_eq!(text.font_size, 0.0);
        assert_eq!(text.rotation, 0.0);
        assert_eq!(text.buffer.len(), 0);
    }

    #[test]
    fn options_works() {
        let mut text = Text::new();
        text.color = "red".to_string();
        text.align_horizontal = "center".to_string();
        text.align_vertical = "center".to_string();
        text.font_size = 8.0;
        text.rotation = 45.0;
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
}
