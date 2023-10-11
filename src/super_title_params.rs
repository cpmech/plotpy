use std::fmt::Write;

/// Holds parameters for the SuperTitle
pub struct SuperTitleParams {
    /// The x location of the text in figure coordinates (default = 0.5)
    x: Option<f64>,

    /// The y location of the text in figure coordinates (default = 0.98)
    y: Option<f64>,

    /// The horizontal alignment of the text relative to (x, y) (default = "center")
    ///
    /// Options: "center", "left", "right"
    align_horizontal: String,

    /// The vertical alignment of the text relative to (x, y) (default = "top")
    ///
    /// Options: "top", "center", "bottom", "baseline"
    align_vertical: String,

    /// The font size of the text
    fontsize: f64,

    /// The font weight of the text
    fontweight: f64,
}

impl SuperTitleParams {
    /// Allocates a new instance
    pub fn new() -> Self {
        SuperTitleParams {
            x: None,
            y: None,
            align_horizontal: String::new(),
            align_vertical: String::new(),
            fontsize: 0.0,
            fontweight: 0.0,
        }
    }

    /// Sets the x location of the text in figure coordinates (default = 0.5)
    pub fn set_x(&mut self, value: f64) -> &mut Self {
        self.x = Some(value);
        self
    }

    /// Sets the y location of the text in figure coordinates (default = 0.98)
    pub fn set_y(&mut self, value: f64) -> &mut Self {
        self.y = Some(value);
        self
    }

    /// Sets the horizontal alignment of the text relative to (x, y) (default = "center")
    ///
    /// Options: "center", "left", "right"
    pub fn set_align_horizontal(&mut self, value: &str) -> &mut Self {
        self.align_horizontal = String::from(value);
        self
    }

    /// Sets the vertical alignment of the text relative to (x, y) (default = "top")
    ///
    /// Options: "top", "center", "bottom", "baseline"
    pub fn set_align_vertical(&mut self, value: &str) -> &mut Self {
        self.align_vertical = String::from(value);
        self
    }

    /// Sets the font size of the text
    pub fn set_fontsize(&mut self, value: f64) -> &mut Self {
        self.fontsize = value;
        self
    }

    /// Sets the font weight of the text
    pub fn set_fontweight(&mut self, value: f64) -> &mut Self {
        self.fontweight = value;
        self
    }

    /// Returns options for SuperTitle
    pub(crate) fn options(&self) -> String {
        let mut opt = String::new();
        if let Some(v) = self.x {
            write!(&mut opt, ",x={}", v).unwrap()
        }
        if let Some(v) = self.y {
            write!(&mut opt, ",y={}", v).unwrap()
        }
        if self.align_horizontal != "" {
            write!(&mut opt, ",ha='{}'", self.align_horizontal).unwrap()
        }
        if self.align_vertical != "" {
            write!(&mut opt, ",va='{}'", self.align_vertical).unwrap()
        }
        if self.fontsize > 0.0 {
            write!(&mut opt, ",fontsize={}", self.fontsize).unwrap()
        }
        if self.fontweight > 0.0 {
            write!(&mut opt, ",fontweight={}", self.fontweight).unwrap()
        }
        opt
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::SuperTitleParams;

    #[test]
    fn new_works() {
        let params = SuperTitleParams::new();
        assert_eq!(params.x, None);
        assert_eq!(params.y, None);
        assert_eq!(params.align_horizontal.len(), 0);
        assert_eq!(params.align_vertical.len(), 0);
        assert_eq!(params.fontsize, 0.0);
        assert_eq!(params.fontweight, 0.0);
    }

    #[test]
    fn options_works() {
        let mut params = SuperTitleParams::new();
        params
            .set_x(0.6)
            .set_y(0.8)
            .set_align_horizontal("center")
            .set_align_vertical("center")
            .set_fontsize(8.0)
            .set_fontweight(10.0);
        let opt = params.options();
        assert_eq!(
            opt,
            ",x=0.6\
             ,y=0.8\
             ,ha='center'\
             ,va='center'\
             ,fontsize=8\
             ,fontweight=10"
        );
    }
}
