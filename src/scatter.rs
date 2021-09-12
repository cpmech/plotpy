use super::*;

/// Generates scatter plot given two arrays (x,y)
///
/// # Examples
///
/// ```
/// use plotpy::*;
/// let x = &[1.0, 2.0, 3.0, 4.0, 5.0];
/// let y = &[1.0, 4.0, 9.0, 16.0, 25.0];
/// let mut plot = Plot::new();
/// let mut scatter = Scatter::new();
/// scatter.marker_style = "*".to_string();
/// scatter.draw(x, y);
/// plot.add(&scatter);
/// plot.save("/tmp/plotpy", "example_scatter", "svg");
/// ```
///
pub struct Scatter {
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

impl Scatter {
    /// Creates new Scatter object
    pub fn new() -> Self {
        Scatter {
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

    /// Draw scatter graph
    ///
    /// # Arguments
    /// * `x` - abscissa array
    /// * `y` - ordinate array
    ///
    pub fn draw(&mut self, x: &[f64], y: &[f64]) {
        vec_to_numpy_array(&mut self.buffer, "x", x);
        vec_to_numpy_array(&mut self.buffer, "y", y);
        let command = format!("plt.scatter(x,y{})\n", self.options());
        self.buffer.push_str(&command);
    }

    pub(crate) fn options(&self) -> String {
        let mut options = String::new();
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

impl GraphMaker for Scatter {
    fn get_buffer<'a>(&'a self) -> &'a String {
        &self.buffer
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string_works() {
        let mut scatter = Scatter::new();
        scatter.marker_alpha = 0.5;
        scatter.marker_color = "#4c4deb".to_string();
        scatter.marker_every = 2;
        scatter.marker_is_void = false;
        scatter.marker_line_color = "blue".to_string();
        scatter.marker_line_style = "--".to_string();
        scatter.marker_line_width = 1.5;
        scatter.marker_size = 8.0;
        scatter.marker_style = "o".to_string();
        let options = scatter.options();
        assert_eq!(
            options,
            "\
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
        let mut scatter = Scatter::new();
        scatter.draw(x, y);
        let correct: &str = "x=np.array([1,2,3,4,5,],dtype=float)\n\
                             y=np.array([1,4,9,16,25,],dtype=float)\n\
                             plt.scatter(x,y)\n";
        assert_eq!(scatter.buffer, correct);
    }
}
