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
}
