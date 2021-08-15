pub struct CurveStyle {
    // lines
    pub line_color: String, // color
    pub line_alpha: f64,    // alpha (0, 1]. A<1e-14 => A=1.0
    pub line_style: String, // style
    pub line_width: f64,    // width

    // markers
    pub marker_type: String,       // type
    pub marker_img: String,        // image filename
    pub marker_color: String,      // color
    pub marker_alpha: f64,         // alpha (0, 1]
    pub marker_size: f64,          // size for images. set marker_size=0 to use the image width
    pub marker_every: i32,         // mark-every
    pub marker_line_color: String, // edge color
    pub marker_line_width: f64,    // edge width
    pub marker_line_style: String, // edge style
    pub marker_is_void: bool,      // void marker (draw edge only)
}

impl CurveStyle {
    pub fn new() -> Self {
        CurveStyle {
            // lines
            line_color: "#b33434".to_string(),
            line_alpha: 0.7,
            line_style: "-".to_string(),
            line_width: 3.0,

            // markers
            marker_type: "o".to_string(),
            marker_img: "".to_string(),
            marker_color: "#4c4deb".to_string(),
            marker_alpha: 1.0,
            marker_size: 0.0,
            marker_every: 0,
            marker_line_color: "#ffffff".to_string(),
            marker_line_width: 2.0,
            marker_line_style: "none".to_string(),
            marker_is_void: false,
        }
    }

    pub(crate) fn args_for_plot(&self) -> String {
        // lines
        let mut args = String::new();
        let line_color = if self.marker_is_void && self.line_color == "" {
            "red"
        } else {
            &self.line_color
        };
        if line_color != "" {
            args.push_str(&format!(",color='{}'", line_color));
        }
        if self.line_alpha > 0.0 {
            args.push_str(&format!(",alpha={}", self.line_alpha));
        }
        if self.line_style != "" {
            args.push_str(&format!(",linestyle='{}'", self.line_style));
        }
        if self.line_width > 0.0 {
            args.push_str(&format!(",linewidth={}", self.line_width));
        }

        // markers
        if self.marker_type != "" {
            args.push_str(&format!(",marker='{}'", self.marker_type));
        }

        // done
        args
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_curve_style_works() {
        let style = CurveStyle::new();
        assert_eq!(style.line_color, "#b33434");
    }

    #[test]
    fn args_for_plot_works() {
        let style = CurveStyle::new();
        let args = style.args_for_plot();
        assert_eq!(
            args,
            ",color='#b33434',alpha=0.7,linestyle='-',linewidth=3,marker='o'"
        );
    }
}
