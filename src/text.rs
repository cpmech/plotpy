use super::GraphMaker;
use std::fmt::Write;

/// Creates text to be added to a plot
///
/// # Example
///
/// ```
/// use plotpy::{Plot, Text, StrError};
/// use std::path::Path;
///
/// fn main() -> Result<(), StrError> {
///     // configure text
///     let mut text = Text::new();
///     text.set_color("purple")
///         .set_align_horizontal("center")
///         .set_align_vertical("center")
///         .set_fontsize(30.0)
///         .set_rotation(45.0)
///         .set_bbox(true)
///         .set_bbox_facecolor("pink")
///         .set_bbox_edgecolor("black")
///         .set_bbox_alpha(0.3)
///         .set_bbox_style("roundtooth,pad=0.3,tooth_size=0.2");
///
///     // draw text
///     text.draw_3d(0.5, 0.5, 0.5, "Hello World!");
///
///     // add text to plot
///     let mut plot = Plot::new();
///     plot.add(&text);
///
///     // save figure
///     plot.save("/tmp/plotpy/doc_tests/doc_text.svg")?;
///     Ok(())
/// }
/// ```
///
/// ![doc_text.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/doc_text.svg)
pub struct Text {
    // text
    color: String,            // Color
    align_horizontal: String, // Horizontal alignment
    align_vertical: String,   // Vertical alignment
    fontsize: f64,            // Font size
    rotation: f64,            // Text rotation

    // bounding box
    bbox: bool,             // Use bounding box
    bbox_facecolor: String, // Facecolor of bounding box
    bbox_edgecolor: String, // Edgecolor of bounding box
    bbox_alpha: f64,        // Alpha of bounding box
    bbox_style: String,     // Style of bounding box; example "round,pad=0.2"

    // buffer
    buffer: String,
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
            bbox: false,
            bbox_facecolor: String::new(),
            bbox_edgecolor: String::new(),
            bbox_alpha: 1.0,
            bbox_style: String::new(),
            buffer: String::new(),
        }
    }

    /// Draws text
    pub fn draw(&mut self, x: f64, y: f64, message: &str) {
        let opt = self.options();
        write!(&mut self.buffer, "t=plt.text({},{},'{}'{})\n", x, y, message, &opt).unwrap();
        if self.bbox {
            let opt_bbox = self.options_bbox();
            write!(&mut self.buffer, "t.set_bbox(dict({}))\n", opt_bbox).unwrap();
        }
    }

    /// Draws text in 3D plot
    pub fn draw_3d(&mut self, x: f64, y: f64, z: f64, message: &str) {
        let opt = self.options();
        write!(
            &mut self.buffer,
            "maybeCreateAX3D()\n\
             t=AX3D.text({},{},{},'{}'{})\n",
            x, y, z, message, &opt
        )
        .unwrap();
        if self.bbox {
            let opt_bbox = self.options_bbox();
            write!(&mut self.buffer, "t.set_bbox(dict({}))\n", opt_bbox).unwrap();
        }
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

    /// Sets use bounding box flag
    pub fn set_bbox(&mut self, flag: bool) -> &mut Self {
        self.bbox = flag;
        self
    }

    /// Sets facecolor of bounding box
    pub fn set_bbox_facecolor(&mut self, color: &str) -> &mut Self {
        self.bbox_facecolor = String::from(color);
        self
    }

    /// Sets edgecolor of bounding box
    pub fn set_bbox_edgecolor(&mut self, color: &str) -> &mut Self {
        self.bbox_edgecolor = String::from(color);
        self
    }

    /// Sets alpha of bounding box
    pub fn set_bbox_alpha(&mut self, value: f64) -> &mut Self {
        self.bbox_alpha = value;
        self
    }

    /// Sets style of bounding box; example
    ///
    /// Examples:
    ///
    /// * "square,pad=0.3"
    /// * "circle,pad=0.3"
    /// * "larrow,pad=0.3"
    /// * "rarrow,pad=0.3"
    /// * "darrow,pad=0.3"
    /// * "round,pad=0.3,rounding_size=0.15"
    /// * "round4,pad=0.3,rounding_size=0.2"
    /// * "sawtooth,pad=0.3,tooth_size=0.1"
    /// * "roundtooth,pad=0.3,tooth_size=0.2"
    ///
    /// See [matplotlib](https://matplotlib.org/stable/api/_as_gen/matplotlib.patches.BoxStyle.html)
    pub fn set_bbox_style(&mut self, style: &str) -> &mut Self {
        self.bbox_style = String::from(style);
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

    /// Returns options for bounding box
    fn options_bbox(&self) -> String {
        let mut opt = String::new();
        if self.bbox_facecolor != "" {
            write!(&mut opt, "facecolor='{}',", self.bbox_facecolor).unwrap();
        }
        if self.bbox_edgecolor != "" {
            write!(&mut opt, "edgecolor='{}',", self.bbox_edgecolor).unwrap();
        }
        write!(&mut opt, "alpha={},", self.bbox_alpha).unwrap();
        if self.bbox_style != "" {
            write!(&mut opt, "boxstyle='{}',", self.bbox_style).unwrap();
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
    fn options_box_works() {
        let mut text = Text::new();
        text.set_bbox(true)
            .set_bbox_facecolor("pink")
            .set_bbox_edgecolor("black")
            .set_bbox_alpha(0.3)
            .set_bbox_style("round,pad=0.4");
        assert_eq!(text.bbox, true);
        let opt = text.options_bbox();
        assert_eq!(
            opt,
            "facecolor='pink',\
             edgecolor='black',\
             alpha=0.3,\
             boxstyle='round,pad=0.4',"
        );
    }

    #[test]
    fn draw_works() {
        let mut text = Text::new();
        text.draw(1.2, 3.4, &"message".to_string());
        let b: &str = "t=plt.text(1.2,3.4,'message')\n";
        assert_eq!(text.buffer, b);
    }

    #[test]
    fn draw_3d_works() {
        let mut text = Text::new();
        text.draw_3d(1.2, 3.4, 5.6, &"message".to_string());
        let b: &str = "maybeCreateAX3D()\n\
                       t=AX3D.text(1.2,3.4,5.6,'message')\n";
        assert_eq!(text.buffer, b);
    }
}
