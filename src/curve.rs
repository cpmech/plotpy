use super::*;

/// Generates a curve (aka line-plot) given two arrays (x,y)
///
/// # Note
///
/// This struct corresponds to the **plot** function of Matplotlib.
///
/// # Examples
///
/// ```
/// use plotpy::*;
/// let x = &[1.0, 2.0, 3.0, 4.0, 5.0];
/// let y = &[1.0, 4.0, 9.0, 16.0, 25.0];
/// let mut plot = Plot::new();
/// let mut curve = Curve::new();
/// curve.line_style = "-".to_string();
/// curve.marker_style = "*".to_string();
/// curve.draw(x, y);
/// plot.add(&curve);
/// plot.save("/tmp/plotpy", "example_curve", "svg");
/// ```
///
pub struct Curve {
    /// alpha (0, 1]. A<1e-14 => A=1.0
    pub line_alpha: f64,

    /// color
    pub line_color: String,

    /// style
    pub line_style: String,

    /// width
    pub line_width: f64,

    /// alpha (0, 1]
    pub marker_alpha: f64,

    /// color
    pub marker_color: String,

    /// mark-every
    pub marker_every: i32,

    /// void marker (draw edge only)
    pub marker_is_void: bool,

    /// edge color
    pub marker_line_color: String,

    /// edge style
    pub marker_line_style: String,

    /// edge width
    pub marker_line_width: f64,

    /// size
    pub marker_size: f64,

    /// type, e.g., "o", "+"
    pub marker_style: String,

    // buffer
    pub(crate) buffer: String,
}

impl Curve {
    /// Creates new Curve object
    pub fn new() -> Self {
        Curve {
            line_alpha: 0.0,
            line_color: String::new(),
            line_style: String::new(),
            line_width: 0.0,
            marker_alpha: 0.0,
            marker_color: String::new(),
            marker_every: 0,
            marker_is_void: false,
            marker_line_color: String::new(),
            marker_line_style: String::new(),
            marker_line_width: 0.0,
            marker_size: 0.0,
            marker_style: String::new(),
            buffer: String::new(),
        }
    }

    /// Draws curve
    ///
    /// # Arguments
    /// * `x` - abscissa array
    /// * `y` - ordinate array
    ///
    pub fn draw(&mut self, x: &[f64], y: &[f64]) {
        vec_to_numpy_array(&mut self.buffer, "x", x);
        vec_to_numpy_array(&mut self.buffer, "y", y);
        let command = format!("plt.plot(x,y{})\n", self.options());
        self.buffer.push_str(&command);
    }

    pub(crate) fn options(&self) -> String {
        // fix color if marker is void
        let line_color = if self.marker_is_void && self.line_color == "" {
            "red"
        } else {
            &self.line_color
        };

        // output
        let mut options = String::new();

        // lines
        if self.line_alpha > 0.0 {
            options.push_str(&format!(",alpha={}", self.line_alpha));
        }
        if line_color != "" {
            options.push_str(&format!(",color='{}'", line_color));
        }
        if self.line_style != "" {
            options.push_str(&format!(",linestyle='{}'", self.line_style));
        }
        if self.line_width > 0.0 {
            options.push_str(&format!(",linewidth={}", self.line_width));
        }

        // markers
        if self.marker_alpha > 0.0 {
            options.push_str(&format!(",markeralpha={}", self.marker_alpha));
        }
        if self.marker_color != "" {
            options.push_str(&format!(",markerfacecolor='{}'", self.marker_color));
        }
        if self.marker_every > 0 {
            options.push_str(&format!(",markevery={}", self.marker_every));
        }
        if self.marker_is_void {
            options.push_str(",markerfacecolor='none'");
        }
        if self.marker_line_color != "" {
            options.push_str(&format!(",markeredgecolor='{}'", self.marker_line_color));
        }
        if self.marker_line_style != "" {
            options.push_str(&format!(",markerlinestyle='{}'", self.marker_line_style));
        }
        if self.marker_line_width > 0.0 {
            options.push_str(&format!(",markeredgewidth={}", self.marker_line_width));
        }
        if self.marker_size > 0.0 {
            options.push_str(&format!(",markersize={}", self.marker_size));
        }
        if self.marker_style != "" {
            options.push_str(&format!(",marker='{}'", self.marker_style));
        }

        options
    }
}

impl GraphMaker for Curve {
    fn get_buffer<'a>(&'a self) -> &'a String {
        &self.buffer
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn options_works() {
        let mut curve = Curve::new();
        curve.line_alpha = 0.7;
        curve.line_color = "#b33434".to_string();
        curve.line_style = "-".to_string();
        curve.line_width = 3.0;
        curve.marker_alpha = 0.5;
        curve.marker_color = "#4c4deb".to_string();
        curve.marker_every = 2;
        curve.marker_is_void = false;
        curve.marker_line_color = "blue".to_string();
        curve.marker_line_style = "--".to_string();
        curve.marker_line_width = 1.5;
        curve.marker_size = 8.0;
        curve.marker_style = "o".to_string();
        let options = curve.options();
        assert_eq!(
            options,
            "\
            ,alpha=0.7\
            ,color='#b33434'\
            ,linestyle='-'\
            ,linewidth=3\
            ,markeralpha=0.5\
            ,markerfacecolor='#4c4deb'\
            ,markevery=2\
            ,markeredgecolor='blue'\
            ,markerlinestyle='--'\
            ,markeredgewidth=1.5\
            ,markersize=8\
            ,marker='o'\
            "
        );
    }

    #[test]
    fn draw_works() {
        let x = &[1.0, 2.0, 3.0, 4.0, 5.0];
        let y = &[1.0, 4.0, 9.0, 16.0, 25.0];
        let mut curve = Curve::new();
        curve.draw(x, y);
        let correct: &str = "x=np.array([1,2,3,4,5,],dtype=float)\n\
                             y=np.array([1,4,9,16,25,],dtype=float)\n\
                             plt.plot(x,y)\n";
        assert_eq!(curve.buffer, correct);
    }
}
