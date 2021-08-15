use super::CurveStyle;

pub struct Curve<'a> {
    pub label: String,     // curve name or connection pair such as 'SF -> LA'
    pub style: CurveStyle, // line and marker arguments
    pub x: &'a [f64],      // x-coordinates
    pub y: &'a [f64],      // y-coordinates
}

impl<'a> Curve<'a> {
    pub fn new(x: &'a [f64], y: &'a [f64]) -> Self {
        Curve {
            label: "".to_string(),
            style: CurveStyle::new(),
            x,
            y,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_curve_works() {
        let x = &[0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
        let y = &[0.0, 1.0, 4.0, 9.0, 16.0, 25.0];
        let curve = Curve::new(x, y);
        assert_eq!(curve.label, "");
        assert_eq!(curve.style.line_color, "#b33434");
        assert_eq!(curve.x, x);
        assert_eq!(curve.y, y);
    }
}
