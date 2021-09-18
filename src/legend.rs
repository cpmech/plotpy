use super::*;
use std::fmt::Write;

/// Generates a Legend
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
/// // generate (x,y) points
/// let x = &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
/// let y1 = &[1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5];
/// let y2 = &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
/// let y3 = &[1.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0];
/// let y4 = &[1.0, 4.0, 9.0, 16.0, 25.0, 36.0, 49.0, 64.0];
///
/// // configure curves
/// let mut curve1 = Curve::new();
/// let mut curve2 = Curve::new();
/// let mut curve3 = Curve::new();
/// let mut curve4 = Curve::new();
/// curve1.set_label("first");
/// curve2.set_label("second");
/// curve3.set_label("third");
/// curve4.set_label("fourth");
///
/// // draw curves
/// curve1.draw(x, y1);
/// curve2.draw(x, y2);
/// curve3.draw(x, y3);
/// curve4.draw(x, y4);
///
/// // add curves to plot
/// let mut plot = Plot::new();
/// plot.add(&curve1);
/// plot.add(&curve2);
/// plot.add(&curve3);
/// plot.add(&curve4);
///
/// // configure and draw legend
/// let mut legend = Legend::new();
/// legend.fontsize = 18.0;
/// legend.handle_len = 5.5;
/// legend.num_col = 2;
/// legend.outside = true;
/// legend.show_frame = false;
/// legend.draw();
///
/// // add legend to plot
/// plot.add(&legend);
/// plot.grid_and_labels("x", "y");
///
/// // save figure
/// let path = Path::new(OUT_DIR).join("doc_legend.svg");
/// plot.save(&path)?;
/// # Ok(())
/// # }
/// ```
///
/// ![doc_legend.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/doc_legend.svg)
///
pub struct Legend {
    /// Fontsize
    pub fontsize: f64,

    /// Length of legend's indicator line
    pub handle_len: f64,

    /// Number of columns
    pub num_col: i32,

    /// Location, e.g., "best", "right", "center left"
    ///
    /// Only used if outside == false
    pub location: String,

    /// Put legend outside plot area
    pub outside: bool,

    /// Show frame around legend
    pub show_frame: bool,

    /// Normalized coordinates to put legend outsize
    pub x_coords_outside: Vec<f64>,

    // buffer
    pub(crate) buffer: String,
}

impl Legend {
    /// Creates a new Legend object
    pub fn new() -> Self {
        Legend {
            fontsize: 0.0,
            handle_len: 3.0,
            num_col: 1,
            location: "best".to_string(),
            outside: false,
            show_frame: true,
            x_coords_outside: vec![0.0, 1.02, 1.0, 0.102],
            buffer: String::new(),
        }
    }

    /// Draws legend
    pub fn draw(&mut self) {
        let opt = self.options();
        if self.outside {
            vector_to_numbers(&mut self.buffer, "coo", self.x_coords_outside.as_slice());
        }
        write!(&mut self.buffer, "h,l=plt.gca().get_legend_handles_labels()\n").unwrap();
        write!(&mut self.buffer, "if len(h)>0 and len(l)>0:\n").unwrap();
        write!(&mut self.buffer, "    leg=plt.legend({})\n", &opt).unwrap();
        write!(&mut self.buffer, "    addToEA(leg)\n").unwrap();
        if !self.show_frame {
            write!(&mut self.buffer, "    leg.get_frame().set_linewidth(0.0)\n").unwrap();
        }
    }

    /// Returns options for legend
    pub(crate) fn options(&self) -> String {
        let mut opt = String::new();
        let mut comma = "";
        if self.handle_len > 0.0 {
            write!(&mut opt, "handlelength={}", self.handle_len).unwrap();
            comma = ",";
        }
        if self.fontsize > 0.0 {
            write!(&mut opt, "{}prop={{'size':{}}}", comma, self.fontsize).unwrap();
            comma = ",";
        }
        if self.num_col > 0 {
            write!(&mut opt, "{}ncol={}", comma, self.num_col).unwrap();
            comma = ",";
        }
        if self.outside {
            write!(
                &mut opt,
                "{}loc=3,bbox_to_anchor=coo,mode='expand',borderaxespad=0.0,columnspacing=1,handletextpad=0.05",
                comma
            )
            .unwrap();
        } else {
            if self.location != "" {
                write!(&mut opt, "{}loc='{}'", comma, self.location).unwrap();
            }
        }
        opt
    }
}

impl GraphMaker for Legend {
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
        let legend = Legend::new();
        assert_eq!(legend.fontsize, 0.0);
        assert_eq!(legend.handle_len, 3.0);
        assert_eq!(legend.num_col, 1);
        assert_eq!(legend.location, "best".to_string());
        assert_eq!(legend.outside, false);
        assert_eq!(legend.show_frame, true);
        assert_eq!(legend.x_coords_outside, vec![0.0, 1.02, 1.0, 0.102]);
        assert_eq!(legend.buffer.len(), 0);
    }

    #[test]
    fn options_works() {
        let mut legend = Legend::new();
        legend.handle_len = 6.0;
        let opt = legend.options();
        assert_eq!(opt, "handlelength=6,ncol=1,loc='best'");
    }

    #[test]
    fn draw_works() {
        let mut legend = Legend::new();
        legend.draw();
        let b: &str = "h,l=plt.gca().get_legend_handles_labels()\n\
                       if len(h)>0 and len(l)>0:\n\
                       \x20\x20\x20\x20leg=plt.legend(handlelength=3,ncol=1,loc='best')\n\
                       \x20\x20\x20\x20addToEA(leg)\n";
        assert_eq!(legend.buffer, b);
    }
}
