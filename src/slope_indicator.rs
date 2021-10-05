use super::GraphMaker;
use std::fmt::Write;

pub struct SlopeIndicator {
    flipped: bool,      // indicator is flipped
    log_x: bool,        // x-axis is logarithm
    log_y: bool,        // y-axis is logarithm
    edge_color: String, // Color of indicator lines
    face_color: String, // Color of indicator faces
    line_style: String, // Style of lines
    line_width: f64,    // Width of lines
    length: f64,        // horizontal length of indicator in Axes coords
    offset_v: f64,      // vertical offset in points
    no_text: bool,      // do not draw text
    fontsize: f64,      // text font size
    precision: usize,   // precision of slope number in label
    text_color: String, // Color of text
    text_slope: String, // use fixed text for slope value
    text_offset_h: f64, // horizontal offset for text in points
    text_offset_v: f64, // vertical offset for text in points
    buffer: String,     // buffer
}

impl SlopeIndicator {
    /// Creates a new SlopeIndicator object
    pub fn new() -> Self {
        SlopeIndicator {
            flipped: false,
            log_x: false,
            log_y: false,
            edge_color: "black".to_string(),
            face_color: "#f3f3f3".to_string(),
            line_style: String::new(),
            line_width: 0.0,
            length: 0.1,
            offset_v: 5.0,
            no_text: false,
            fontsize: 0.0,
            precision: 0,
            text_color: "black".to_string(),
            text_slope: String::new(),
            text_offset_h: 3.0,
            text_offset_v: 3.0,
            buffer: String::new(),
        }
    }

    /// Draws an indicator of line slope
    pub fn draw(&mut self, slope: f64, x_center: f64, y_center: f64) {
        // compute axis (normalized) coordinates and slope
        let (mut xc, mut yc) = (x_center, y_center);
        let (mut xa, mut ya) = (xc + 1.0, yc + slope);
        if self.log_x {
            xc = f64::log10(xc);
            xa = xc + 1.0;
        }
        if self.log_y {
            yc = f64::log10(yc);
            ya = yc + slope;
        }
        write!(
            &mut self.buffer,
            "xc,yc=dataToAxis(({},{}))\n\
             xa,ya=dataToAxis(({},{}))\n\
             m,l=(ya-yc)/(xa-xc),{}\n",
            xc, yc, xa, ya, self.length,
        )
        .unwrap();

        // set polygon
        if self.flipped {
            self.buffer.push_str(
                "dat=[[pth.Path.MOVETO,(xc-l,yc-m*l)],\
                      [pth.Path.LINETO,(xc-l,yc+m*l)],\
                      [pth.Path.LINETO,(xc+l,yc+m*l)],\
                      [pth.Path.CLOSEPOLY,(None,None)]]\n",
            );
        } else {
            self.buffer.push_str(
                "dat=[[pth.Path.MOVETO,(xc-l,yc-m*l)],\
                      [pth.Path.LINETO,(xc+l,yc-m*l)],\
                      [pth.Path.LINETO,(xc+l,yc+m*l)],\
                      [pth.Path.CLOSEPOLY,(None,None)]]\n",
            );
        }

        // draw
        let tf = self.transform(slope);
        let opt = self.options();
        write!(
            &mut self.buffer,
            "{}cmd,pts=zip(*dat)\n\
             h=pth.Path(pts,cmd)\n\
             p=pat.PathPatch(h{})\n\
             plt.gca().add_patch(p)\n",
            tf, opt,
        )
        .unwrap();

        // skip text
        if self.no_text {
            return;
        }

        // coordinates for labels
        self.buffer.push_str(
            "xm,ym=xc-l,yc-m*l\n\
             xp,yp=xc+l,yc+m*l\n",
        );

        // slope text
        let mut text = String::new();
        if self.text_slope == "" {
            if self.precision == 0 {
                write!(&mut text, "'{}'", f64::abs(slope)).unwrap();
            } else {
                write!(&mut text, "'{:.1$}'", f64::abs(slope), self.precision).unwrap();
            }
        } else {
            write!(&mut text, "{}", self.text_slope).unwrap();
        }

        // draw labels
        let tf_txt = self.transform_text(slope);
        self.buffer.push_str(&tf_txt);
        let (opt_x, opt_y) = self.options_text();
        if self.flipped {
            if slope < 0.0 {
                write!(&mut self.buffer, "plt.text(xc,yp,'1',ha='center',va='top'{})\n", opt_x).unwrap();
            } else {
                write!(
                    &mut self.buffer,
                    "plt.text(xc,yp,'1',ha='center',va='bottom'{})\n",
                    opt_x
                )
                .unwrap();
            }
            write!(
                &mut self.buffer,
                "plt.text(xm,yc,{},ha='right',va='center'{})\n",
                text, opt_y
            )
            .unwrap();
        } else {
            if slope < 0.0 {
                write!(
                    &mut self.buffer,
                    "plt.text(xc,ym,'1',ha='center',va='bottom'{})\n",
                    opt_x
                )
                .unwrap();
            } else {
                write!(&mut self.buffer, "plt.text(xc,ym,'1',ha='center',va='top'{})\n", opt_x).unwrap();
            }
            write!(
                &mut self.buffer,
                "plt.text(xp,yc,{},ha='left',va='center'{})\n",
                text, opt_y
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

    /// Sets the color of indicator lines
    pub fn set_edge_color(&mut self, color: &str) -> &mut Self {
        self.edge_color = String::from(color);
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

    /// Sets the the precision of slope number in label
    pub fn set_precision(&mut self, value: usize) -> &mut Self {
        self.precision = value;
        self
    }

    /// Sets text of slope indicator
    pub fn set_text_slope(&mut self, slope: &str) -> &mut Self {
        self.text_slope = String::from(slope);
        self
    }

    /// Sets the horizontal offset for the text in normalized axes coordinates in points
    pub fn set_text_offset_h(&mut self, value: f64) -> &mut Self {
        self.text_offset_h = value;
        self
    }

    /// Sets the vertical offset for the text in normalized axes coordinates in points
    pub fn set_text_offset_v(&mut self, value: f64) -> &mut Self {
        self.text_offset_v = value;
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
                "tf=tra.offset_copy(plt.gca().transAxes,fig=plt.gcf(),x=0,y={},units='points')\n",
                dv,
            )
            .unwrap();
        } else {
            opt.push_str("tf=plt.gca().transAxes\n");
        }
        opt
    }

    /// Returns the coordinate transform for text
    fn transform_text(&self, slope: f64) -> String {
        let mut opt = String::new();
        if self.offset_v > 0.0 || self.text_offset_v > 0.0 {
            let dv = if self.flipped {
                (self.offset_v + self.text_offset_v) * f64::signum(slope)
            } else {
                -(self.offset_v + self.text_offset_v) * f64::signum(slope)
            };
            write!(
                &mut opt,
                "tfx=tra.offset_copy(plt.gca().transAxes,fig=plt.gcf(),x=0,y={},units='points')\n",
                dv,
            )
            .unwrap();
        } else {
            opt.push_str("tfx=plt.gca().transAxes\n");
        }
        if self.offset_v > 0.0 || self.text_offset_h > 0.0 {
            let dv = if self.flipped {
                self.offset_v * f64::signum(slope)
            } else {
                -self.offset_v * f64::signum(slope)
            };
            let dh = if self.flipped {
                -self.text_offset_h
            } else {
                self.text_offset_h
            };
            write!(
                &mut opt,
                "tfy=tra.offset_copy(plt.gca().transAxes,fig=plt.gcf(),x={},y={},units='points')\n",
                dh, dv,
            )
            .unwrap();
        } else {
            opt.push_str("tfy=plt.gca().transAxes\n");
        }
        opt
    }

    /// Returns options for slope indicator
    fn options(&self) -> String {
        let mut opt = String::from(",transform=tf");
        if self.edge_color != "" {
            write!(&mut opt, ",edgecolor='{}'", self.edge_color).unwrap();
        }
        if self.face_color != "" {
            write!(&mut opt, ",facecolor='{}'", self.face_color).unwrap();
        }
        if self.line_width > 0.0 {
            write!(&mut opt, ",linewidth={}", self.line_width).unwrap();
        }
        opt
    }

    /// Returns options for text
    fn options_text(&self) -> (String, String) {
        let mut opt_x = String::from(",transform=tfx");
        let mut opt_y = String::from(",transform=tfy");
        if self.text_color != "" {
            write!(&mut opt_x, ",color='{}'", self.text_color).unwrap();
            write!(&mut opt_y, ",color='{}'", self.text_color).unwrap();
        }
        if self.fontsize > 0.0 {
            write!(&mut opt_x, ",fontsize={}", self.fontsize).unwrap();
            write!(&mut opt_y, ",fontsize={}", self.fontsize).unwrap();
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
        assert_eq!(indicator.edge_color.len(), 7);
        assert_eq!(indicator.line_style.len(), 0);
        assert_eq!(indicator.line_width, 0.0);
        assert_eq!(indicator.offset_v, 5.0);
        assert_eq!(indicator.no_text, false);
        assert_eq!(indicator.fontsize, 0.0);
        assert_eq!(indicator.text_slope.len(), 0);
        assert_eq!(indicator.text_offset_h, 3.0);
        assert_eq!(indicator.text_offset_v, 3.0);
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
        indicator.set_text_offset_h(1.0);
        indicator.set_text_offset_v(3.0);
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
            .set_edge_color("red")
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
        indicator.draw(1.0, 0.5, 0.5);
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
