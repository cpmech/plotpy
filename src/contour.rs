use super::{generate_list_quoted, matrix_to_array, vector_to_array, AsMatrix, GraphMaker};
use num_traits::Num;
use std::fmt::Write;

/// Generates a contour plot
///
/// [See Matplotlib's documentation](https://matplotlib.org/stable/api/_as_gen/matplotlib.pyplot.contour.html)
///
/// # Example
///
/// ```
/// use plotpy::{Contour, Plot, generate3d};
///
/// fn main() -> Result<(), &'static str> {
///     // generate (x,y,z) matrices
///     let n = 21;
///     let (x, y, z) = generate3d(-2.0, 2.0, -2.0, 2.0, n, n, |x, y| x * x - y * y);
///
///     // configure contour
///     let mut contour = Contour::new();
///     contour
///         .set_colorbar_label("temperature")
///         .set_colormap_name("terrain")
///         .set_selected_line_color("#f1eb67")
///         .set_selected_line_width(12.0)
///         .set_selected_level(0.0, true);
///
///     // draw contour
///     contour.draw(&x, &y, &z);
///
///     // add contour to plot
///     let mut plot = Plot::new();
///     plot.add(&contour)
///         .set_labels("x", "y");
///
///     // save figure
///     plot.save("/tmp/plotpy/doc_tests/doc_contour.svg")?;
///     Ok(())
/// }
/// ```
///
/// ![doc_contour.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/doc_contour.svg)
///
/// See also integration tests in the [tests directory](https://github.com/cpmech/plotpy/tree/main/tests)
///
/// Output from some integration tests:
///
/// ![integ_contour.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/integ_contour.svg)
pub struct Contour {
    colors: Vec<String>,         // Colors to be used instead of colormap
    levels: Vec<f64>,            // Pre-defined levels
    colormap_name: String,       // Colormap name
    no_lines: bool,              // Skip drawing a lines contour
    no_labels: bool,             // Skip adding labels to the lines contour
    no_inline_labels: bool,      // Do not draw labels inline
    no_colorbar: bool,           // Skip drawing a colorbar
    colorbar_label: String,      // Colorbar label
    number_format_cb: String,    // Number format for the labels in lines contour
    line_color: String,          // Line color for the lines contour
    line_style: String,          // Line style for the lines contour
    line_width: f64,             // Line width for the lines contour
    fontsize_labels: f64,        // Font size for labels
    with_selected: bool,         // Draw a line contour with a selected level
    selected_level: f64,         // Selected level (e.g., 0.0)
    selected_line_color: String, // Color to mark the selected level
    selected_line_style: String, // Line style for the selected level
    selected_line_width: f64,    // Line width for the selected level
    extra_filled: String,        // Extra commands (comma separated) for the filled contour
    extra_line: String,          // Extra commands (comma separated) for the line contour
    buffer: String,              // buffer
}

impl Contour {
    /// Creates a new Contour object
    pub fn new() -> Self {
        Contour {
            colors: Vec::new(),
            levels: Vec::new(),
            colormap_name: "bwr".to_string(),
            no_lines: false,
            no_labels: false,
            no_inline_labels: false,
            no_colorbar: false,
            colorbar_label: String::new(),
            number_format_cb: String::new(),
            line_color: "black".to_string(),
            line_style: String::new(),
            line_width: 0.0,
            fontsize_labels: 0.0,
            with_selected: false,
            selected_level: 0.0,
            selected_line_color: "yellow".to_string(),
            selected_line_style: "-".to_string(),
            selected_line_width: 2.0,
            extra_filled: String::new(),
            extra_line: String::new(),
            buffer: String::new(),
        }
    }

    /// Draws a fancy contour: filled contour with a line contour and a colorbar
    ///
    /// # Input
    ///
    /// * `x` -- matrix with x values
    /// * `y` -- matrix with y values
    /// * `z` -- matrix with z values
    ///
    /// # Flags
    ///
    /// The following flags control what features are not to be drawn:
    ///
    /// * `no_lines` -- skip drawing a lines contour on top of the filled contour
    /// * `no_labels` -- skip adding labels to the lines contour (if enabled)
    /// * `no_colorbar` -- skip drawing a colorbar
    /// * `with_selected` -- draw a line contour with a selected level (e.g., 0.0) on top of everything
    pub fn draw<'a, T, U>(&mut self, x: &'a T, y: &'a T, z: &'a T)
    where
        T: AsMatrix<'a, U>,
        U: 'a + std::fmt::Display + Num,
    {
        matrix_to_array(&mut self.buffer, "x", x);
        matrix_to_array(&mut self.buffer, "y", y);
        matrix_to_array(&mut self.buffer, "z", z);
        if self.colors.len() > 0 {
            generate_list_quoted(&mut self.buffer, "colors", &self.colors);
        }
        if self.levels.len() > 0 {
            vector_to_array(&mut self.buffer, "levels", &self.levels);
        }
        let opt = self.options_filled();
        write!(&mut self.buffer, "cf=plt.contourf(x,y,z{})\n", &opt).unwrap();
        if !self.no_lines {
            let opt_line = self.options_line();
            write!(&mut self.buffer, "cl=plt.contour(x,y,z{})\n", &opt_line).unwrap();
            if !self.no_labels {
                let opt_label = self.options_label();
                write!(&mut self.buffer, "plt.clabel(cl{})\n", &opt_label).unwrap();
            }
        }
        if !self.no_colorbar {
            let opt_colorbar = self.options_colorbar();
            write!(&mut self.buffer, "cb=plt.colorbar(cf{})\n", &opt_colorbar).unwrap();
            if self.colorbar_label != "" {
                write!(&mut self.buffer, "cb.ax.set_ylabel(r'{}')\n", self.colorbar_label).unwrap();
            }
        }
        if self.with_selected {
            let opt_selected = self.options_selected();
            write!(&mut self.buffer, "plt.contour(x,y,z{})\n", &opt_selected).unwrap();
        }
    }

    /// Sets the colors to be used instead of a pre-defined colormap
    ///
    /// Will use `colormap_index` instead if its empty.
    pub fn set_colors(&mut self, colors: &[&str]) -> &mut Self {
        self.colors = colors.iter().map(|color| color.to_string()).collect();
        self
    }

    /// Sets pre-defined levels, otherwise automatically calculate levels
    pub fn set_levels(&mut self, levels: &[f64]) -> &mut Self {
        self.levels = levels.to_vec();
        self
    }

    /// Sets the colormap index
    ///
    /// Options:
    ///
    /// * 0 -- bwr
    /// * 1 -- RdBu
    /// * 2 -- hsv
    /// * 3 -- jet
    /// * 4 -- terrain
    /// * 5 -- pink
    /// * 6 -- Greys
    /// * `>`6 -- starts over from 0
    pub fn set_colormap_index(&mut self, index: usize) -> &mut Self {
        const CMAP: [&str; 7] = ["bwr", "RdBu", "hsv", "jet", "terrain", "pink", "Greys"];
        self.colormap_name = CMAP[index % 7].to_string();
        self.colors = Vec::new();
        self
    }

    /// Sets the colormap name
    ///
    /// Colormap names:
    ///
    /// * see <https://matplotlib.org/stable/tutorials/colors/colormaps.html>
    ///
    /// Will use `colormap_index` instead if `colormap_name` is empty.
    pub fn set_colormap_name(&mut self, name: &str) -> &mut Self {
        self.colormap_name = String::from(name);
        self.colors = Vec::new();
        self
    }

    /// Sets option to skip drawing a lines contour on top of the filled contour
    pub fn set_no_lines(&mut self, flag: bool) -> &mut Self {
        self.no_lines = flag;
        self
    }

    /// Sets option to skip adding labels to the lines contour (if enabled)
    pub fn set_no_labels(&mut self, flag: bool) -> &mut Self {
        self.no_labels = flag;
        self
    }

    /// Sets option to skip drawing labels inline with the contour lines (if enabled)
    pub fn set_no_inline_labels(&mut self, flag: bool) -> &mut Self {
        self.no_inline_labels = flag;
        self
    }

    /// Sets option to skip drawing a colorbar
    pub fn set_no_colorbar(&mut self, flag: bool) -> &mut Self {
        self.no_colorbar = flag;
        self
    }

    /// Sets the colorbar label
    pub fn set_colorbar_label(&mut self, label: &str) -> &mut Self {
        self.colorbar_label = String::from(label);
        self
    }

    /// Sets the number format for the labels in the colorbar (cb)
    pub fn set_number_format_cb(&mut self, format: &str) -> &mut Self {
        self.number_format_cb = String::from(format);
        self
    }

    /// Sets the line color for the lines contour (default is black)
    pub fn set_line_color(&mut self, color: &str) -> &mut Self {
        self.line_color = String::from(color);
        self
    }

    /// Sets the line style for the lines contour
    ///
    /// Options:
    ///
    /// * "`-`", "`:`", "`--`", "`-.`"
    pub fn set_line_style(&mut self, style: &str) -> &mut Self {
        self.line_style = String::from(style);
        self
    }

    /// Sets the line width for the lines contour
    pub fn set_line_width(&mut self, width: f64) -> &mut Self {
        self.line_width = width;
        self
    }

    /// Sets the font size for labels
    pub fn set_fontsize_labels(&mut self, fontsize: f64) -> &mut Self {
        self.fontsize_labels = fontsize;
        self
    }

    /// Sets option to draw a line contour with a selected level (e.g., 0.0)
    ///
    /// Will draw the selected level (e.g., 0.0) on top of everything
    pub fn set_selected_level(&mut self, level: f64, enabled: bool) -> &mut Self {
        self.selected_level = level;
        self.with_selected = enabled;
        self
    }

    /// Sets the color to mark the selected level
    pub fn set_selected_line_color(&mut self, color: &str) -> &mut Self {
        self.selected_line_color = String::from(color);
        self
    }

    /// Sets the line style for the selected level
    ///
    /// Options:
    ///
    /// * "`-`", "`:`", "`--`", "`-.`"
    pub fn set_selected_line_style(&mut self, style: &str) -> &mut Self {
        self.selected_line_style = String::from(style);
        self
    }

    /// Sets the line width for the selected level
    pub fn set_selected_line_width(&mut self, width: f64) -> &mut Self {
        self.selected_line_width = width;
        self
    }

    /// Sets extra matplotlib commands (comma separated) for the filled contour
    ///
    /// **Important:** The extra commands must be comma separated. For example:
    ///
    /// ```text
    /// param1=123,param2='hello'
    /// ```
    ///
    /// [See Matplotlib's documentation for extra parameters](https://matplotlib.org/stable/api/_as_gen/matplotlib.pyplot.contour.html)
    pub fn set_extra_filled(&mut self, extra: &str) -> &mut Self {
        self.extra_filled = extra.to_string();
        self
    }

    /// Sets extra matplotlib commands (comma separated) for the line contour
    ///
    /// **Important:** The extra commands must be comma separated. For example:
    ///
    /// ```text
    /// param1=123,param2='hello'
    /// ```
    ///
    /// [See Matplotlib's documentation for extra parameters](https://matplotlib.org/stable/api/_as_gen/matplotlib.pyplot.contour.html)
    pub fn set_extra_line(&mut self, extra: &str) -> &mut Self {
        self.extra_line = extra.to_string();
        self
    }

    /// Returns options for filled contour
    fn options_filled(&self) -> String {
        let mut opt = String::new();
        if self.colors.len() > 0 {
            write!(&mut opt, ",colors=colors",).unwrap();
        } else {
            if self.colormap_name != "" {
                write!(&mut opt, ",cmap=plt.get_cmap('{}')", self.colormap_name).unwrap();
            }
        }
        if self.levels.len() > 0 {
            write!(&mut opt, ",levels=levels").unwrap();
        }
        if self.extra_filled != "" {
            write!(&mut opt, ",{}", self.extra_filled).unwrap();
        }
        opt
    }

    /// Returns options for line contour
    fn options_line(&self) -> String {
        let mut opt = String::new();
        if self.line_color != "" {
            write!(&mut opt, ",colors=['{}']", self.line_color).unwrap();
        }
        if self.levels.len() > 0 {
            write!(&mut opt, ",levels=levels").unwrap();
        }
        if self.line_style != "" {
            write!(&mut opt, ",linestyles=['{}']", self.line_style).unwrap();
        }
        if self.line_width > 0.0 {
            write!(&mut opt, ",linewidths=[{}]", self.line_width).unwrap();
        }
        if self.extra_line != "" {
            write!(&mut opt, ",{}", self.extra_line).unwrap();
        }
        opt
    }

    /// Returns options for labels
    fn options_label(&self) -> String {
        let mut opt = String::new();
        if self.no_inline_labels {
            write!(&mut opt, ",inline=False").unwrap();
        } else {
            write!(&mut opt, ",inline=True").unwrap();
        }
        if self.fontsize_labels > 0.0 {
            write!(&mut opt, ",fontsize={}", self.fontsize_labels).unwrap();
        }
        opt
    }

    /// Returns options for colorbar
    fn options_colorbar(&self) -> String {
        let mut opt = String::new();
        if self.number_format_cb != "" {
            write!(&mut opt, ",format='{}'", self.number_format_cb).unwrap();
        }
        opt
    }

    /// Returns options for selected line contour
    fn options_selected(&self) -> String {
        let mut opt = String::new();
        if self.selected_line_color != "" {
            write!(&mut opt, ",colors=['{}']", self.selected_line_color).unwrap();
        }
        write!(&mut opt, ",levels=[{}]", self.selected_level).unwrap();
        if self.selected_line_style != "" {
            write!(&mut opt, ",linestyles=['{}']", self.selected_line_style).unwrap();
        }
        if self.selected_line_width > 0.0 {
            write!(&mut opt, ",linewidths=[{}]", self.selected_line_width).unwrap();
        }
        opt
    }
}

impl GraphMaker for Contour {
    fn get_buffer<'a>(&'a self) -> &'a String {
        &self.buffer
    }
    fn clear_buffer(&mut self) {
        self.buffer.clear();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Contour;
    use crate::GraphMaker;

    #[test]
    fn new_works() {
        let contour = Contour::new();
        assert_eq!(contour.colors.len(), 0);
        assert_eq!(contour.levels.len(), 0);
        assert_eq!(contour.colormap_name, "bwr");
        assert_eq!(contour.no_lines, false);
        assert_eq!(contour.no_labels, false);
        assert_eq!(contour.no_inline_labels, false);
        assert_eq!(contour.no_colorbar, false);
        assert_eq!(contour.colorbar_label.len(), 0);
        assert_eq!(contour.number_format_cb.len(), 0);
        assert_eq!(contour.line_color, "black".to_string());
        assert_eq!(contour.line_style.len(), 0);
        assert_eq!(contour.line_width, 0.0);
        assert_eq!(contour.fontsize_labels, 0.0);
        assert_eq!(contour.with_selected, false);
        assert_eq!(contour.selected_level, 0.0);
        assert_eq!(contour.selected_line_color, "yellow".to_string());
        assert_eq!(contour.selected_line_style, "-".to_string());
        assert_eq!(contour.selected_line_width, 2.0);
        assert_eq!(contour.buffer.len(), 0);
    }

    #[test]
    fn options_filled_works() {
        let mut contour = Contour::new();
        contour
            .set_colors(&vec!["#f00", "#0f0", "#00f"])
            .set_levels(&vec![0.25, 0.5, 1.0]);
        let opt = contour.options_filled();
        assert_eq!(
            opt,
            ",colors=colors\
             ,levels=levels"
        );
        contour.set_colormap_index(4);
        let opt = contour.options_filled();
        assert_eq!(
            opt,
            ",cmap=plt.get_cmap('terrain')\
             ,levels=levels"
        );
    }

    #[test]
    fn options_line_works() {
        let mut contour = Contour::new();
        contour
            .set_levels(&vec![0.25, 0.5, 1.0])
            .set_line_color("red")
            .set_line_style(":")
            .set_line_width(3.0);
        let opt = contour.options_line();
        assert_eq!(
            opt,
            ",colors=['red']\
             ,levels=levels\
             ,linestyles=[':']\
             ,linewidths=[3]"
        );
    }

    #[test]
    fn options_label_works() {
        let mut contour = Contour::new();
        contour.set_no_inline_labels(false).set_fontsize_labels(5.0);
        let opt = contour.options_label();
        assert_eq!(
            opt,
            ",inline=True\
             ,fontsize=5"
        );
        contour.set_no_inline_labels(true);
        let opt = contour.options_label();
        assert_eq!(
            opt,
            ",inline=False\
             ,fontsize=5"
        );
    }

    #[test]
    fn options_colorbar_works() {
        let mut contour = Contour::new();
        contour.set_number_format_cb("%.4f");
        let opt = contour.options_colorbar();
        assert_eq!(opt, ",format='%.4f'");
    }

    #[test]
    fn options_selected_works() {
        let mut contour = Contour::new();
        contour
            .set_selected_level(0.75, true)
            .set_selected_line_color("blue")
            .set_selected_line_style("--")
            .set_selected_line_width(2.5);
        let opt = contour.options_selected();
        assert_eq!(
            opt,
            ",colors=['blue']\
             ,levels=[0.75]\
             ,linestyles=['--']\
             ,linewidths=[2.5]"
        );
    }

    #[test]
    fn draw_works() {
        let mut contour = Contour::new();
        contour
            .set_colors(&vec!["#f00", "#0f0", "#00f"])
            .set_levels(&vec![0.25, 0.5, 1.0])
            .set_colorbar_label("temperature")
            .set_selected_level(0.0, true);
        let x = vec![vec![-0.5, 0.0, 0.5], vec![-0.5, 0.0, 0.5], vec![-0.5, 0.0, 0.5]];
        let y = vec![vec![-0.5, -0.5, -0.5], vec![0.0, 0.0, 0.0], vec![0.5, 0.5, 0.5]];
        let z = vec![vec![0.50, 0.25, 0.50], vec![0.25, 0.00, 0.25], vec![0.50, 0.25, 0.50]];
        contour.draw(&x, &y, &z);
        let b: &str = "x=np.array([[-0.5,0,0.5,],[-0.5,0,0.5,],[-0.5,0,0.5,],])\n\
                       y=np.array([[-0.5,-0.5,-0.5,],[0,0,0,],[0.5,0.5,0.5,],])\n\
                       z=np.array([[0.5,0.25,0.5,],[0.25,0,0.25,],[0.5,0.25,0.5,],])\n\
                       colors=['#f00','#0f0','#00f',]\n\
                       levels=np.array([0.25,0.5,1,])\n\
                       cf=plt.contourf(x,y,z,colors=colors,levels=levels)\n\
                       cl=plt.contour(x,y,z,colors=['black'],levels=levels)\n\
                       plt.clabel(cl,inline=True)\n\
                       cb=plt.colorbar(cf)\n\
                       cb.ax.set_ylabel(r'temperature')\n\
                       plt.contour(x,y,z,colors=['yellow'],levels=[0],linestyles=['-'],linewidths=[2])\n";
        assert_eq!(contour.buffer, b);
        contour.clear_buffer();
        assert_eq!(contour.buffer, "");
    }
}
