use super::{vector_to_array, GraphMaker};
use std::fmt::Write;

pub struct SlopeIndicator {
    flipped: bool,      // indicator is flipped
    log_x: bool,        // x-axis is logarithm
    log_y: bool,        // y-axis is logarithm
    color: String,      // Color of lines and text
    line_style: String, // Style of lines
    line_width: f64,    // Width of lines
    offset_v: f64,      // vertical offset in points
    no_text: bool,      // do not draw text
    fontsize: f64,      // text font size
    txt_slope: String,  // use fixed text for slope value
    txt_offset_h: f64,  // horizontal offset for text in points
    txt_offset_v: f64,  // vertical offset for text in points
    buffer: String,     // buffer
}

impl SlopeIndicator {
    /// Creates a new SlopeIndicator object
    pub fn new() -> Self {
        SlopeIndicator {
            flipped: false,
            log_x: false,
            log_y: false,
            color: "#3f3f3f".to_string(),
            line_style: String::new(),
            line_width: 0.0,
            offset_v: 5.0,
            no_text: false,
            fontsize: 0.0,
            txt_slope: String::new(),
            txt_offset_h: 3.0,
            txt_offset_v: 3.0,
            buffer: String::new(),
        }
    }

    /// Draws an indicator of line slope
    pub fn draw(&mut self, slope: f64, x_center: f64, y_center: f64, x_length: f64) {
        // draw lines
        let m = slope;
        let l = x_length / 2.0;
        let (mut xc, mut yc) = (x_center, y_center);
        let mut x = [xc - l, xc + l, xc + l, xc - l];
        let mut y = [yc - m * l, yc - m * l, yc + m * l, yc - m * l];
        if self.flipped {
            x[1] = xc - l;
            y[1] = yc + m * l;
        }
        if self.log_x {
            for i in 0..4 {
                x[i] = f64::powf(10.0, x[i]);
            }
        }
        if self.log_y {
            for i in 0..4 {
                y[i] = f64::powf(10.0, y[i]);
            }
        }
        vector_to_array(&mut self.buffer, "x", &x);
        vector_to_array(&mut self.buffer, "y", &y);
        let tf = self.transform(slope);
        let opt = self.options();
        write!(&mut self.buffer, "{}plt.plot(x,y{})\n", &tf, &opt).unwrap();

        // skip text
        if self.no_text {
            return;
        }

        // coordinates for labels
        let mut xm = xc - l;
        let mut xp = xc + l;
        let mut ym = yc + m * l;
        let mut yp = yc + m * l;
        let mut yr = yc - m * l;
        let mut ys = yc - m * l;
        if self.log_x {
            xc = f64::powf(10.0, xc);
            xm = f64::powf(10.0, xm);
            xp = f64::powf(10.0, xp);
        }
        if self.log_y {
            yc = f64::powf(10.0, yc);
            ym = f64::powf(10.0, ym);
            yp = f64::powf(10.0, yp);
            yr = f64::powf(10.0, yr);
            ys = f64::powf(10.0, ys);
        }

        // slope text
        let mut text = String::new();
        if self.txt_slope == "" {
            write!(&mut text, "'{}'", f64::abs(slope)).unwrap();
        } else {
            write!(&mut text, "{}", self.txt_slope).unwrap();
        }

        // draw labels
        let tf_txt = self.transform_text(slope);
        self.buffer.push_str(&tf_txt);
        let (opt_x, opt_y) = self.options_text();
        if self.flipped {
            if slope < 0.0 {
                write!(
                    &mut self.buffer,
                    "plt.text({},{},'1',ha='center',va='top'{})\n",
                    xc, ym, opt_x
                )
                .unwrap();
            } else {
                write!(
                    &mut self.buffer,
                    "plt.text({},{},'1',ha='center',va='bottom'{})\n",
                    xc, yp, opt_x
                )
                .unwrap();
            }
            write!(
                &mut self.buffer,
                "plt.text({},{},{},ha='right',va='center'{})\n",
                xm, yc, text, opt_y
            )
            .unwrap();
        } else {
            if slope < 0.0 {
                write!(
                    &mut self.buffer,
                    "plt.text({},{},'1',ha='center',va='bottom'{})\n",
                    xc, yr, opt_x
                )
                .unwrap();
            } else {
                write!(
                    &mut self.buffer,
                    "plt.text({},{},'1',ha='center',va='top'{})\n",
                    xc, ys, opt_x
                )
                .unwrap();
            }
            write!(
                &mut self.buffer,
                "plt.text({},{},{},ha='left',va='center'{})\n",
                xp, yc, text, opt_y
            )
            .unwrap();
        }
    }

    /// Sets option to draw flipped indicator
    pub fn set_flipped(&mut self, flag: bool) -> &mut Self {
        self.flipped = flag;
        self
    }

    /// Set option to consider the x axis being scaled using log10
    pub fn set_log_x(&mut self, flag: bool) -> &mut Self {
        self.log_x = flag;
        self
    }

    /// Set option to consider the y axis being scaled using log10
    pub fn set_log_y(&mut self, flag: bool) -> &mut Self {
        self.log_y = flag;
        self
    }

    /// Sets the color of lines and text
    pub fn set_color(&mut self, color: &str) -> &mut Self {
        self.color = String::from(color);
        self
    }

    /// Sets the style of lines
    ///
    /// Options:
    ///
    /// * "`-`", `:`", "`--`", "`-.`", or "`None`"
    /// * As defined in <https://matplotlib.org/stable/gallery/lines_bars_and_markers/linestyles.html>
    pub fn set_line_style(&mut self, style: &str) -> &mut Self {
        self.line_style = String::from(style);
        self
    }

    /// Sets the width of lines
    pub fn set_line_width(&mut self, width: f64) -> &mut Self {
        self.line_width = width;
        self
    }

    /// Sets the whole indicator's offset in normalized axes coordinates in points
    pub fn set_offset_v(&mut self, value: f64) -> &mut Self {
        self.offset_v = value;
        self
    }

    /// Sets option to skip drawing text
    pub fn set_no_text(&mut self, flag: bool) -> &mut Self {
        self.no_text = flag;
        self
    }

    /// Sets the font size
    pub fn set_fontsize(&mut self, fontsize: f64) -> &mut Self {
        self.fontsize = fontsize;
        self
    }

    /// Sets text of slope indicator
    pub fn set_txt_slope(&mut self, slope: &str) -> &mut Self {
        self.txt_slope = String::from(slope);
        self
    }

    /// Sets the horizontal offset for the text in normalized axes coordinates in points
    pub fn set_txt_offset_h(&mut self, value: f64) -> &mut Self {
        self.txt_offset_h = value;
        self
    }

    /// Sets the vertical offset for the text in normalized axes coordinates in points
    pub fn set_txt_offset_v(&mut self, value: f64) -> &mut Self {
        self.txt_offset_v = value;
        self
    }

    /// Returns the indicator's (whole) coordinate transform
    fn transform(&self, slope: f64) -> String {
        let mut opt = String::new();
        if self.offset_v > 0.0 {
            let dv = if self.flipped {
                self.offset_v * f64::signum(slope)
            } else {
                -self.offset_v * f64::signum(slope)
            };
            write!(
                &mut opt,
                "tf=tra.offset_copy(plt.gca().transData,fig=plt.gcf(),x=0,y={},units='points')\n",
                dv,
            )
            .unwrap();
        }
        opt
    }

    /// Returns the coordinate transform for text
    fn transform_text(&self, slope: f64) -> String {
        let mut opt = String::new();
        if self.offset_v > 0.0 || self.txt_offset_v > 0.0 {
            let dv = if self.flipped {
                (self.offset_v + self.txt_offset_v) * f64::signum(slope)
            } else {
                -(self.offset_v + self.txt_offset_v) * f64::signum(slope)
            };
            write!(
                &mut opt,
                "tfx=tra.offset_copy(plt.gca().transData,fig=plt.gcf(),x=0,y={},units='points')\n",
                dv,
            )
            .unwrap();
        }
        if self.offset_v > 0.0 || self.txt_offset_h > 0.0 {
            let dv = if self.flipped {
                self.offset_v * f64::signum(slope)
            } else {
                -self.offset_v * f64::signum(slope)
            };
            let dh = if self.flipped {
                -self.txt_offset_h
            } else {
                self.txt_offset_h
            };
            write!(
                &mut opt,
                "tfy=tra.offset_copy(plt.gca().transData,fig=plt.gcf(),x={},y={},units='points')\n",
                dh, dv,
            )
            .unwrap();
        }
        opt
    }

    /// Returns options for slope indicator
    fn options(&self) -> String {
        let mut opt = String::new();
        if self.color != "" {
            write!(&mut opt, ",color='{}'", self.color).unwrap();
        }
        if self.line_style != "" {
            write!(&mut opt, ",linestyle='{}'", self.line_style).unwrap();
        }
        if self.line_width > 0.0 {
            write!(&mut opt, ",linewidth={}", self.line_width).unwrap();
        }
        if self.offset_v > 0.0 {
            write!(&mut opt, ",transform=tf").unwrap();
        }
        opt
    }

    /// Returns options for text
    fn options_text(&self) -> (String, String) {
        let mut opt_x = String::new();
        let mut opt_y = String::new();
        if self.color != "" {
            write!(&mut opt_x, ",color='{}'", self.color).unwrap();
        }
        if self.fontsize > 0.0 {
            write!(&mut opt_x, ",fontsize={}", self.fontsize).unwrap();
        }
        opt_y.push_str(&opt_x);
        if self.offset_v > 0.0 || self.txt_offset_v > 0.0 {
            write!(&mut opt_x, ",transform=tfx").unwrap();
        }
        if self.offset_v > 0.0 || self.txt_offset_h > 0.0 {
            write!(&mut opt_y, ",transform=tfy").unwrap();
        }
        (opt_x, opt_y)
    }
}

impl GraphMaker for SlopeIndicator {
    fn get_buffer<'a>(&'a self) -> &'a String {
        &self.buffer
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::SlopeIndicator;

    #[test]
    fn new_works() {
        let indicator = SlopeIndicator::new();
        assert_eq!(indicator.flipped, false);
        assert_eq!(indicator.log_x, false);
        assert_eq!(indicator.log_y, false);
        assert_eq!(indicator.color.len(), 7);
        assert_eq!(indicator.line_style.len(), 0);
        assert_eq!(indicator.line_width, 0.0);
        assert_eq!(indicator.offset_v, 5.0);
        assert_eq!(indicator.no_text, false);
        assert_eq!(indicator.fontsize, 0.0);
        assert_eq!(indicator.txt_slope.len(), 0);
        assert_eq!(indicator.txt_offset_h, 3.0);
        assert_eq!(indicator.txt_offset_v, 3.0);
        assert_eq!(indicator.buffer.len(), 0);
    }

    #[test]
    fn transform_works() {
        let mut indicator = SlopeIndicator::new();
        indicator.set_offset_v(7.0);
        indicator.set_flipped(false);
        assert_eq!(
            indicator.transform(1.0),
            "tf=tra.offset_copy(plt.gca().transData,fig=plt.gcf(),x=0,y=-7,units='points')\n"
        );
        indicator.set_flipped(true);
        assert_eq!(
            indicator.transform(1.0),
            "tf=tra.offset_copy(plt.gca().transData,fig=plt.gcf(),x=0,y=7,units='points')\n"
        );
        indicator.set_flipped(false);
        assert_eq!(
            indicator.transform(-1.0),
            "tf=tra.offset_copy(plt.gca().transData,fig=plt.gcf(),x=0,y=7,units='points')\n"
        );
        indicator.set_flipped(true);
        assert_eq!(
            indicator.transform(-1.0),
            "tf=tra.offset_copy(plt.gca().transData,fig=plt.gcf(),x=0,y=-7,units='points')\n"
        );
        indicator.set_offset_v(0.0);
        indicator.set_flipped(false);
        assert_eq!(indicator.transform(-1.0), "");
    }

    #[test]
    fn transform_text_works() {
        let mut indicator = SlopeIndicator::new();

        indicator.set_offset_v(7.0);
        indicator.set_txt_offset_h(1.0);
        indicator.set_txt_offset_v(3.0);
        indicator.set_flipped(false);
        assert_eq!(
            indicator.transform_text(1.0),
            "tfx=tra.offset_copy(plt.gca().transData,fig=plt.gcf(),x=0,y=-10,units='points')\n\
             tfy=tra.offset_copy(plt.gca().transData,fig=plt.gcf(),x=1,y=-7,units='points')\n"
        );
        indicator.set_flipped(true);
        assert_eq!(
            indicator.transform_text(1.0),
            "tfx=tra.offset_copy(plt.gca().transData,fig=plt.gcf(),x=0,y=10,units='points')\n\
             tfy=tra.offset_copy(plt.gca().transData,fig=plt.gcf(),x=-1,y=7,units='points')\n"
        );
        indicator.set_flipped(false);
        assert_eq!(
            indicator.transform_text(-1.0),
            "tfx=tra.offset_copy(plt.gca().transData,fig=plt.gcf(),x=0,y=10,units='points')\n\
             tfy=tra.offset_copy(plt.gca().transData,fig=plt.gcf(),x=1,y=7,units='points')\n"
        );
        indicator.set_flipped(true);
        assert_eq!(
            indicator.transform_text(-1.0),
            "tfx=tra.offset_copy(plt.gca().transData,fig=plt.gcf(),x=0,y=-10,units='points')\n\
             tfy=tra.offset_copy(plt.gca().transData,fig=plt.gcf(),x=-1,y=-7,units='points')\n"
        );

        indicator.set_offset_v(0.0);
        indicator.set_flipped(false);
        assert_eq!(
            indicator.transform_text(1.0),
            "tfx=tra.offset_copy(plt.gca().transData,fig=plt.gcf(),x=0,y=-3,units='points')\n\
             tfy=tra.offset_copy(plt.gca().transData,fig=plt.gcf(),x=1,y=-0,units='points')\n"
        );
        indicator.set_flipped(true);
        assert_eq!(
            indicator.transform_text(1.0),
            "tfx=tra.offset_copy(plt.gca().transData,fig=plt.gcf(),x=0,y=3,units='points')\n\
             tfy=tra.offset_copy(plt.gca().transData,fig=plt.gcf(),x=-1,y=0,units='points')\n"
        );
        indicator.set_flipped(false);
        assert_eq!(
            indicator.transform_text(-1.0),
            "tfx=tra.offset_copy(plt.gca().transData,fig=plt.gcf(),x=0,y=3,units='points')\n\
             tfy=tra.offset_copy(plt.gca().transData,fig=plt.gcf(),x=1,y=0,units='points')\n"
        );
        indicator.set_flipped(true);
        assert_eq!(
            indicator.transform_text(-1.0),
            "tfx=tra.offset_copy(plt.gca().transData,fig=plt.gcf(),x=0,y=-3,units='points')\n\
             tfy=tra.offset_copy(plt.gca().transData,fig=plt.gcf(),x=-1,y=-0,units='points')\n"
        );
    }

    #[test]
    fn options_works() {
        let mut indicator = SlopeIndicator::new();
        indicator
            .set_color("red")
            .set_line_style("--")
            .set_line_width(2.0)
            .set_offset_v(12.0);
        let options = indicator.options();
        assert_eq!(
            options,
            ",color='red'\
             ,linestyle='--'\
             ,linewidth=2\
             ,transform=tf"
        );
    }

    #[test]
    fn draw_works() {
        let mut indicator = SlopeIndicator::new();
        indicator.draw(1.0, 0.5, 0.5, 0.2);
        let b: &str = "x=np.array([0.4,0.6,0.6,0.4,],dtype=float)\n\
                       y=np.array([0.4,0.4,0.6,0.4,],dtype=float)\n\
                       tf=tra.offset_copy(plt.gca().transData,fig=plt.gcf(),x=0,y=-5,units='points')\n\
                       plt.plot(x,y,color='#3f3f3f',transform=tf)\n\
                       tfx=tra.offset_copy(plt.gca().transData,fig=plt.gcf(),x=0,y=-8,units='points')\n\
                       tfy=tra.offset_copy(plt.gca().transData,fig=plt.gcf(),x=3,y=-5,units='points')\n\
                       plt.text(0.5,0.4,'1',ha='center',va='top',color='#3f3f3f',transform=tfx)\n\
                       plt.text(0.6,0.5,'1',ha='left',va='center',color='#3f3f3f',transform=tfy)\n";
        assert_eq!(indicator.buffer, b);
    }
}
