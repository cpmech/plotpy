use super::*;
use std::fmt::Write;

/// Creates text to be added to a plot
pub struct Text {
    pub alignment_horizontal: String, // e.g., 'center'
    pub alignment_vertical: String,   // e.g., 'center'
    pub rotation: f64,                // text rotation
    pub font_size: f64,               // font size

    // buffer
    pub(crate) buffer: String,
}

impl Text {
    pub fn new() -> Self {
        Text {
            alignment_horizontal: String::new(),
            alignment_vertical: String::new(),
            rotation: 0.0,
            font_size: 0.0,
            buffer: String::new(),
        }
    }

    pub fn draw(&mut self, x: f64, y: f64, message: &String) {
        let opt = self.options();
        write!(&mut self.buffer, "plt.text({},{},{}{})", x, y, message, &opt).unwrap();
    }

    pub(crate) fn options(&self) -> String {
        let mut opt = String::new();
        if self.alignment_horizontal != "" {
            write!(&mut opt, ",ha='{}'", self.alignment_horizontal).unwrap();
        }
        if self.alignment_vertical != "" {
            write!(&mut opt, ",va='{}'", self.alignment_vertical).unwrap();
        }
        if self.rotation > 0.0 {
            write!(&mut opt, ",rotation={}", self.rotation).unwrap();
        }
        if self.font_size > 0.0 {
            write!(&mut opt, ",fontsize={}", self.font_size).unwrap();
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
        assert_eq!(text.alignment_horizontal, "");
    }

    #[test]
    fn options_works() {
        let mut text = Text::new();
        text.alignment_horizontal = "center".to_string();
        let opt = text.options();
        assert_eq!(opt, ",ha='center'");
    }
}
