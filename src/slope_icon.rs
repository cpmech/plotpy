use super::GraphMaker;
use std::fmt::Write;

pub struct SlopeIcon {
    above: bool,        // draw icon above line
    log_x: bool,        // x-axis is logarithm
    log_y: bool,        // y-axis is logarithm
    edge_color: String, // Color of icon lines
    face_color: String, // Color of icon faces
    line_style: String, // Style of lines
    line_width: f64,    // Width of lines
    length: f64,        // horizontal length of icon in Axes coords [0,1]
    offset_v: f64,      // vertical offset in points
    no_text: bool,      // do not draw text
    fontsize: f64,      // text font size
    precision: usize,   // precision of slope number in label
    text_h: String,     // use fixed text for horizontal value
    text_v: String,     // use fixed text for vertical (slope) value
    text_color: String, // Color of text
    text_offset_h: f64, // horizontal offset for text in points
    text_offset_v: f64, // vertical offset for text in points
    buffer: String,     // buffer
}

impl SlopeIcon {
    /// Creates a new SlopeIcon object
    pub fn new() -> Self {
        SlopeIcon {
            above: false,
            log_x: false,
            log_y: false,
            edge_color: "#000000".to_string(),
            face_color: "#f7f7f7".to_string(),
            line_style: String::new(),
            line_width: 0.0,
            length: 0.1,
            offset_v: 5.0,
            no_text: false,
            fontsize: 0.0,
            precision: 0,
            text_h: "1".to_string(),
            text_v: String::new(),
            text_color: "#000000".to_string(),
            text_offset_h: 3.0,
            text_offset_v: 2.0,
            buffer: String::new(),
        }
    }

    /// Draws an icon of line slope
    pub fn draw(&mut self, slope: f64, x_center: f64, y_center: f64) {
        // set flip flag
        let flip = if slope < 0.0 { !self.above } else { self.above };

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
            xc,
            yc,
            xa,
            ya,
            self.length / 2.0,
        )
        .unwrap();

        // set polygon
        if flip {
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
        if self.text_v == "" {
            if self.precision == 0 {
                write!(&mut text, "{}", f64::abs(slope)).unwrap();
            } else {
                write!(&mut text, "{:.1$}", f64::abs(slope), self.precision).unwrap();
            }
        } else {
            write!(&mut text, "{}", self.text_v).unwrap();
        }

        // draw labels
        let tf_txt = self.transform_text(slope);
        self.buffer.push_str(&tf_txt);
        let (opt_x, opt_y) = self.options_text();
        if flip {
            if slope < 0.0 {
                write!(
                    &mut self.buffer,
                    "plt.text(xc,yp,r'{}',ha='center',va='top'{})\n",
                    self.text_h, opt_x
                )
                .unwrap();
            } else {
                write!(
                    &mut self.buffer,
                    "plt.text(xc,yp,r'{}',ha='center',va='bottom'{})\n",
                    self.text_h, opt_x
                )
                .unwrap();
            }
            write!(
                &mut self.buffer,
                "plt.text(xm,yc,r'{}',ha='right',va='center'{})\n",
                text, opt_y
            )
            .unwrap();
        } else {
            if slope < 0.0 {
                write!(
                    &mut self.buffer,
                    "plt.text(xc,ym,r'{}',ha='center',va='bottom'{})\n",
                    self.text_h, opt_x
                )
                .unwrap();
            } else {
                write!(
                    &mut self.buffer,
                    "plt.text(xc,ym,r'{}',ha='center',va='top'{})\n",
                    self.text_h, opt_x
                )
                .unwrap();
            }
            write!(
                &mut self.buffer,
                "plt.text(xp,yc,r'{}',ha='left',va='center'{})\n",
                text, opt_y
            )
            .unwrap();
        }
    }

    /// Sets option to draw icon above line
    pub fn set_above(&mut self, flag: bool) -> &mut Self {
        self.above = flag;
        self
    }

    /// Sets option to consider the x axis being scaled using log10
    pub fn set_log_x(&mut self, flag: bool) -> &mut Self {
        self.log_x = flag;
        self
    }

    /// Sets option to consider the y axis being scaled using log10
    pub fn set_log_y(&mut self, flag: bool) -> &mut Self {
        self.log_y = flag;
        self
    }

    /// Sets the color of icon lines
    pub fn set_edge_color(&mut self, color: &str) -> &mut Self {
        self.edge_color = String::from(color);
        self
    }

    /// Sets the color of icon face
    pub fn set_face_color(&mut self, color: &str) -> &mut Self {
        self.face_color = String::from(color);
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

    /// Sets the (horizontal) length of the icon in Axes coordinates [0, 1]
    pub fn set_length(&mut self, value: f64) -> &mut Self {
        self.length = value;
        self
    }

    /// Sets the whole icon's offset in normalized axes coordinates in points
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

    /// Sets text of horizontal value (== 1)
    pub fn set_text_h(&mut self, one: &str) -> &mut Self {
        self.text_h = String::from(one);
        self
    }

    /// Sets text of vertical value (slope)
    pub fn set_text_v(&mut self, slope: &str) -> &mut Self {
        self.text_v = String::from(slope);
        self
    }

    /// Sets the color of text
    pub fn set_text_color(&mut self, color: &str) -> &mut Self {
        self.text_color = String::from(color);
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

    /// Returns the icon's (whole) coordinate transform
    fn transform(&self, slope: f64) -> String {
        let flip = if slope < 0.0 { !self.above } else { self.above };
        let mut opt = String::new();
        if self.offset_v > 0.0 {
            let dv = if flip {
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
        let flip = if slope < 0.0 { !self.above } else { self.above };
        let mut opt = String::new();
        if self.offset_v > 0.0 || self.text_offset_v > 0.0 {
            let dv = if flip {
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
            let dv = if flip {
                self.offset_v * f64::signum(slope)
            } else {
                -self.offset_v * f64::signum(slope)
            };
            let dh = if flip { -self.text_offset_h } else { self.text_offset_h };
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

    /// Returns options for slope icon
    fn options(&self) -> String {
        let mut opt = String::from(",transform=tf");
        if self.edge_color != "" {
            write!(&mut opt, ",edgecolor='{}'", self.edge_color).unwrap();
        }
        if self.face_color != "" {
            write!(&mut opt, ",facecolor='{}'", self.face_color).unwrap();
        }
        if self.line_style != "" {
            write!(&mut opt, ",linestyle='{}'", self.line_style).unwrap();
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

impl GraphMaker for SlopeIcon {
    fn get_buffer<'a>(&'a self) -> &'a String {
        &self.buffer
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::SlopeIcon;

    #[test]
    fn new_works() {
        let icon = SlopeIcon::new();
        assert_eq!(icon.above, false);
        assert_eq!(icon.log_x, false);
        assert_eq!(icon.log_y, false);
        assert_eq!(icon.edge_color.len(), 7);
        assert_eq!(icon.line_style.len(), 0);
        assert_eq!(icon.line_width, 0.0);
        assert_eq!(icon.length, 0.1);
        assert_eq!(icon.offset_v, 5.0);
        assert_eq!(icon.no_text, false);
        assert_eq!(icon.fontsize, 0.0);
        assert_eq!(icon.precision, 0);
        assert_eq!(icon.text_h.len(), 1);
        assert_eq!(icon.text_v.len(), 0);
        assert_eq!(icon.text_color.len(), 7);
        assert_eq!(icon.text_offset_h, 3.0);
        assert_eq!(icon.text_offset_v, 2.0);
        assert_eq!(icon.buffer.len(), 0);
    }

    #[test]
    fn transform_works() {
        let mut icon = SlopeIcon::new();
        icon.set_offset_v(7.0);
        icon.set_above(false);
        assert_eq!(
            icon.transform(1.0),
            "tf=tra.offset_copy(plt.gca().transAxes,fig=plt.gcf(),x=0,y=-7,units='points')\n"
        );
        icon.set_above(true);
        assert_eq!(
            icon.transform(1.0),
            "tf=tra.offset_copy(plt.gca().transAxes,fig=plt.gcf(),x=0,y=7,units='points')\n"
        );
        icon.set_above(false);
        assert_eq!(
            icon.transform(-1.0),
            "tf=tra.offset_copy(plt.gca().transAxes,fig=plt.gcf(),x=0,y=-7,units='points')\n"
        );
        icon.set_above(true);
        assert_eq!(
            icon.transform(-1.0),
            "tf=tra.offset_copy(plt.gca().transAxes,fig=plt.gcf(),x=0,y=7,units='points')\n"
        );
        icon.set_offset_v(0.0);
        icon.set_above(false);
        assert_eq!(icon.transform(-1.0), "tf=plt.gca().transAxes\n");
    }

    #[test]
    fn transform_text_works() {
        let mut icon = SlopeIcon::new();

        icon.set_offset_v(7.0);
        icon.set_text_offset_h(1.0);
        icon.set_text_offset_v(3.0);
        icon.set_above(false);
        assert_eq!(
            icon.transform_text(1.0),
            "tfx=tra.offset_copy(plt.gca().transAxes,fig=plt.gcf(),x=0,y=-10,units='points')\n\
             tfy=tra.offset_copy(plt.gca().transAxes,fig=plt.gcf(),x=1,y=-7,units='points')\n"
        );
        icon.set_above(true);
        assert_eq!(
            icon.transform_text(1.0),
            "tfx=tra.offset_copy(plt.gca().transAxes,fig=plt.gcf(),x=0,y=10,units='points')\n\
             tfy=tra.offset_copy(plt.gca().transAxes,fig=plt.gcf(),x=-1,y=7,units='points')\n"
        );
        icon.set_above(false);
        assert_eq!(
            icon.transform_text(-1.0),
            "tfx=tra.offset_copy(plt.gca().transAxes,fig=plt.gcf(),x=0,y=-10,units='points')\n\
             tfy=tra.offset_copy(plt.gca().transAxes,fig=plt.gcf(),x=-1,y=-7,units='points')\n"
        );
        icon.set_above(true);
        assert_eq!(
            icon.transform_text(-1.0),
            "tfx=tra.offset_copy(plt.gca().transAxes,fig=plt.gcf(),x=0,y=10,units='points')\n\
             tfy=tra.offset_copy(plt.gca().transAxes,fig=plt.gcf(),x=1,y=7,units='points')\n"
        );

        icon.set_offset_v(0.0);
        icon.set_above(false);
        assert_eq!(
            icon.transform_text(1.0),
            "tfx=tra.offset_copy(plt.gca().transAxes,fig=plt.gcf(),x=0,y=-3,units='points')\n\
             tfy=tra.offset_copy(plt.gca().transAxes,fig=plt.gcf(),x=1,y=-0,units='points')\n"
        );
        icon.set_above(true);
        assert_eq!(
            icon.transform_text(1.0),
            "tfx=tra.offset_copy(plt.gca().transAxes,fig=plt.gcf(),x=0,y=3,units='points')\n\
             tfy=tra.offset_copy(plt.gca().transAxes,fig=plt.gcf(),x=-1,y=0,units='points')\n"
        );
        icon.set_above(false);
        assert_eq!(
            icon.transform_text(-1.0),
            "tfx=tra.offset_copy(plt.gca().transAxes,fig=plt.gcf(),x=0,y=-3,units='points')\n\
             tfy=tra.offset_copy(plt.gca().transAxes,fig=plt.gcf(),x=-1,y=-0,units='points')\n"
        );
        icon.set_above(true);
        assert_eq!(
            icon.transform_text(-1.0),
            "tfx=tra.offset_copy(plt.gca().transAxes,fig=plt.gcf(),x=0,y=3,units='points')\n\
             tfy=tra.offset_copy(plt.gca().transAxes,fig=plt.gcf(),x=1,y=0,units='points')\n"
        );

        icon.set_offset_v(0.0);
        icon.set_text_offset_v(0.0);
        icon.set_text_offset_h(0.0);
        icon.set_above(false);
        assert_eq!(
            icon.transform_text(1.0),
            "tfx=plt.gca().transAxes\n\
             tfy=plt.gca().transAxes\n"
        );
        assert_eq!(
            icon.transform_text(-1.0),
            "tfx=plt.gca().transAxes\n\
             tfy=plt.gca().transAxes\n"
        );
        icon.set_above(true);
        assert_eq!(
            icon.transform_text(1.0),
            "tfx=plt.gca().transAxes\n\
             tfy=plt.gca().transAxes\n"
        );
        assert_eq!(
            icon.transform_text(-1.0),
            "tfx=plt.gca().transAxes\n\
             tfy=plt.gca().transAxes\n"
        );
    }

    #[test]
    fn options_works() {
        let mut icon = SlopeIcon::new();
        icon.set_edge_color("red")
            .set_face_color("gold")
            .set_line_style("--")
            .set_line_width(2.0);
        let options = icon.options();
        assert_eq!(
            options,
            ",transform=tf\
             ,edgecolor='red'\
             ,facecolor='gold'\
             ,linestyle='--'\
             ,linewidth=2"
        );
    }

    #[test]
    fn options_text_works() {
        let mut icon = SlopeIcon::new();
        icon.set_text_color("red").set_fontsize(12.0);
        let (opt_x, opt_y) = icon.options_text();
        assert_eq!(
            opt_x,
            ",transform=tfx\
             ,color='red'\
             ,fontsize=12"
        );
        assert_eq!(
            opt_y,
            ",transform=tfy\
             ,color='red'\
             ,fontsize=12"
        );
    }

    #[test]
    fn draw_works() {
        let mut icon = SlopeIcon::new();
        icon.set_above(true)
            .set_log_x(false)
            .set_log_y(false)
            .set_edge_color("red")
            .set_face_color("blue")
            .set_line_style(":")
            .set_line_width(1.1)
            .set_length(0.2)
            .set_offset_v(3.0)
            .set_no_text(false)
            .set_fontsize(4.0)
            .set_precision(5)
            .set_text_h("one")
            .set_text_v("lambda")
            .set_text_color("gold")
            .set_text_offset_h(6.0)
            .set_text_offset_v(7.0)
            .draw(10.0, 0.5, 0.1);
        let b: &str = "xc,yc=dataToAxis((0.5,0.1))\n\
                       xa,ya=dataToAxis((1.5,10.1))\n\
                       m,l=(ya-yc)/(xa-xc),0.1\n\
                       dat=[[pth.Path.MOVETO,(xc-l,yc-m*l)],[pth.Path.LINETO,(xc-l,yc+m*l)],[pth.Path.LINETO,(xc+l,yc+m*l)],[pth.Path.CLOSEPOLY,(None,None)]]\n\
                       tf=tra.offset_copy(plt.gca().transAxes,fig=plt.gcf(),x=0,y=3,units='points')\n\
                       cmd,pts=zip(*dat)\n\
                       h=pth.Path(pts,cmd)\n\
                       p=pat.PathPatch(h,transform=tf,edgecolor='red',facecolor='blue',linestyle=':',linewidth=1.1)\n\
                       plt.gca().add_patch(p)\n\
                       xm,ym=xc-l,yc-m*l\n\
                       xp,yp=xc+l,yc+m*l\n\
                       tfx=tra.offset_copy(plt.gca().transAxes,fig=plt.gcf(),x=0,y=10,units='points')\n\
                       tfy=tra.offset_copy(plt.gca().transAxes,fig=plt.gcf(),x=-6,y=3,units='points')\n\
                       plt.text(xc,yp,r'one',ha='center',va='bottom',transform=tfx,color='gold',fontsize=4)\n\
                       plt.text(xm,yc,r'lambda',ha='right',va='center',transform=tfy,color='gold',fontsize=4)\n";
        assert_eq!(icon.buffer, b);
    }
}
