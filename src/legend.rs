use super::*;
use std::fmt::Write;

/// Generates a Legend
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

    /// Normalized coordinates to put legend outsize
    pub out_coords: Vec<f64>,

    /// Show frame around legend
    pub show_frame: bool,

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
            out_coords: vec![0.0, 1.02, 1.0, 0.102],
            show_frame: true,
            buffer: String::new(),
        }
    }

    /// Draws legend
    pub fn draw(&mut self) {
        let opt = self.options();
        if self.outside {
            vector_to_numbers(&mut self.buffer, "coo", self.out_coords.as_slice());
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
        assert_eq!(legend.out_coords, vec![0.0, 1.02, 1.0, 0.102]);
        assert_eq!(legend.show_frame, true);
        assert_eq!(legend.buffer.len(), 0);
    }

    #[test]
    fn options_works() {
        let mut legend = Legend::new();
        legend.handle_len = 6.0;
        let opt = legend.options();
        assert_eq!(opt, "");
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
