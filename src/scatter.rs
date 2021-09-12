use super::*;
use std::fmt::Write;

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
/// ```
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
    pub fn draw(&mut self, x: &[f64], y: &[f64]) -> Result<(), &'static str> {
        vec_to_numpy_array(&mut self.buffer, "x", x);
        vec_to_numpy_array(&mut self.buffer, "y", y);
        let opt = self.options();
        write!(&mut self.buffer, "plt.scatter(x,y{})\n", &opt).unwrap();
        Ok(())
    }

    pub(crate) fn options(&self) -> String {
        let mut opt = String::new();
        if self.marker_alpha > 0.0 {
            write!(&mut opt, ",markeralpha={}", self.marker_alpha).unwrap();
        }
        if self.marker_color != "" {
            write!(&mut opt, ",markerfacecolor='{}'", self.marker_color).unwrap();
        }
        if self.marker_every > 0 {
            write!(&mut opt, ",markevery={}", self.marker_every).unwrap();
        }
        if self.marker_is_void {
            write!(&mut opt, ",markerfacecolor='none'").unwrap();
        }
        if self.marker_line_color != "" {
            write!(&mut opt, ",markeredgecolor='{}'", self.marker_line_color).unwrap();
        }
        if self.marker_line_style != "" {
            write!(&mut opt, ",markerlinestyle='{}'", self.marker_line_style).unwrap();
        }
        if self.marker_line_width > 0.0 {
            write!(&mut opt, ",markeredgewidth={}", self.marker_line_width).unwrap();
        }
        if self.marker_size > 0.0 {
            write!(&mut opt, ",markersize={}", self.marker_size).unwrap();
        }
        if self.marker_style != "" {
            write!(&mut opt, ",marker='{}'", self.marker_style).unwrap();
        }
        opt
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
    fn options_works() {
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
        let opt = scatter.options();
        assert_eq!(
            opt,
            ",markeralpha=0.5\
             ,markerfacecolor='#4c4deb'\
             ,markevery=2\
             ,markeredgecolor='blue'\
             ,markerlinestyle='--'\
             ,markeredgewidth=1.5\
             ,markersize=8\
             ,marker='o'"
        );
    }

    #[test]
    fn draw_works() -> Result<(), &'static str> {
        let x = &[1.0, 2.0, 3.0, 4.0, 5.0];
        let y = &[1.0, 4.0, 9.0, 16.0, 25.0];
        let mut scatter = Scatter::new();
        scatter.draw(x, y)?;
        let correct: &str = "x=np.array([1,2,3,4,5,],dtype=float)\n\
                             y=np.array([1,4,9,16,25,],dtype=float)\n\
                             plt.scatter(x,y)\n";
        assert_eq!(scatter.buffer, correct);
        Ok(())
    }
}
