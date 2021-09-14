use super::*;
use std::fmt::Write;

/// Generates a contour plot
pub struct Contour {
    /// Colors to be used instead of a pre-defined colormap
    ///
    /// Will use `colormap_index` instead if it empty.
    pub colors: Vec<String>,

    /// Pre-defined levels, otherwise automatically calculate levels
    pub levels: Vec<f64>,

    /// Colormap index
    /// * 0 -- bwr
    /// * 1 -- RdBu
    /// * 2 -- hsv
    /// * 3 -- jet
    /// * 4 -- terrain
    /// * 5 -- pink
    /// * 6 -- Greys
    /// * `>`6 -- starts over from 0
    pub colormap_index: i32,

    /// Colormap name as defined in <https://matplotlib.org/stable/tutorials/colors/colormaps.html>
    ///
    /// Will use `colormap_index` instead if `colormap_name` is empty.
    pub colormap_name: String,

    /// Skip drawing a lines contour on top of the filled contour
    pub no_lines: bool,

    /// Skip adding labels to the lines contour (if enabled)
    pub no_labels: bool,

    /// Do not draw labels inline with the contour lines (if enabled)
    pub no_inline_labels: bool,

    /// Skip drawing a colorbar
    pub no_colorbar: bool,

    /// Colorbar label
    pub colorbar_label: String,

    /// Number format for the labels in lines contour (e.g. "%.2f")
    pub colorbar_number_format: String,

    /// Line color for the lines contour (default is black)
    pub line_color: String,

    /// Line width for the lines contour
    pub line_width: f64,

    /// Font size for labels
    pub font_size_labels: f64,

    /// Draw a line contour with a selected level (e.g., 0.0) on top of everything
    pub with_selected: bool,

    /// Selected level (e.g., 0.0)
    pub selected_level: f64,

    /// Color to mark the selected level
    pub selected_line_color: String,

    /// Line style for the selected level
    pub selected_line_style: String,

    /// Line width for the selected level
    pub selected_line_width: f64,

    // buffer
    pub(crate) buffer: String,
}

impl Contour {
    /// Creates a new Contour object
    pub fn new() -> Self {
        Contour {
            colors: Vec::new(),
            levels: Vec::new(),
            colormap_index: 0,
            colormap_name: String::new(),
            no_lines: false,
            no_labels: false,
            no_inline_labels: false,
            no_colorbar: false,
            colorbar_label: String::new(),
            colorbar_number_format: String::new(),
            line_color: "black".to_string(),
            line_width: 0.0,
            font_size_labels: 0.0,
            with_selected: false,
            selected_level: 0.0,
            selected_line_color: "yellow".to_string(),
            selected_line_style: "-".to_string(),
            selected_line_width: 2.0,
            buffer: String::new(),
        }
    }

    /// Draws a fancy contour: filled contour with a line contour and a colorbar
    ///
    /// # Flags
    ///
    /// The following flags control what features are not to be drawn:
    ///
    /// * `no_lines` -- skip drawing a lines contour on top of the filled contour
    /// * `no_labels` -- skip adding labels to the lines contour (if enabled)
    /// * `no_colorbar` -- skip drawing a colorbar
    /// * `with_selected` -- draw a line contour with a selected level (e.g., 0.0) on top of everything
    ///
    /// # Notes
    ///
    /// * The type `T` of the input matrices must be a number.
    ///
    pub fn draw<T>(&mut self, x: &Vec<Vec<T>>, y: &Vec<Vec<T>>, z: &Vec<Vec<T>>)
    where
        T: std::fmt::Display,
    {
        matrix_to_array(&mut self.buffer, "x", x);
        matrix_to_array(&mut self.buffer, "y", y);
        matrix_to_array(&mut self.buffer, "z", z);
        if self.colors.len() > 0 {
            vector_to_strings(&mut self.buffer, "colors", &self.colors);
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

    /// Returns options for filled contour
    pub(crate) fn options_filled(&self) -> String {
        let mut opt = String::new();
        if self.colors.len() > 0 {
            write!(&mut opt, ",colors=colors",).unwrap();
        } else {
            if self.colormap_name != "" {
                write!(&mut opt, ",cmap=plt.get_cmap('{}')", self.colormap_name).unwrap();
            } else {
                write!(&mut opt, ",cmap=getColormap({})", self.colormap_index).unwrap();
            }
        }
        if self.levels.len() > 0 {
            write!(&mut opt, ",levels=levels").unwrap();
        }
        opt
    }

    /// Returns options for line contour
    pub(crate) fn options_line(&self) -> String {
        let mut opt = String::new();
        if self.line_color != "" {
            write!(&mut opt, ",colors=['{}']", self.line_color).unwrap();
        }
        if self.levels.len() > 0 {
            write!(&mut opt, ",levels=levels").unwrap();
        }
        if self.line_width > 0.0 {
            write!(&mut opt, ",linewidths[{}]", self.line_width).unwrap();
        }
        opt
    }

    /// Returns options for labels
    pub(crate) fn options_label(&self) -> String {
        let mut opt = String::new();
        if self.no_inline_labels {
            write!(&mut opt, ",inline=False").unwrap();
        } else {
            write!(&mut opt, ",inline=True").unwrap();
        }
        if self.font_size_labels > 0.0 {
            write!(&mut opt, ",fontsize={}", self.font_size_labels).unwrap();
        }
        opt
    }

    /// Returns options for colorbar
    pub(crate) fn options_colorbar(&self) -> String {
        let mut opt = String::new();
        if self.colorbar_number_format != "" {
            write!(&mut opt, ",format='{}'", self.colorbar_number_format).unwrap();
        }
        opt
    }

    /// Returns options for selected line contour
    pub(crate) fn options_selected(&self) -> String {
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
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_xyz() -> (Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<Vec<f64>>) {
        let x = vec![
            vec![-1.0, -0.5, 0.0, 0.5],
            vec![-1.0, -0.5, 0.0, 0.5],
            vec![-1.0, -0.5, 0.0, 0.5],
            vec![-1.0, -0.5, 0.0, 0.5],
        ];
        let y = vec![
            vec![-1.0, -1.0, -1.0, -1.0],
            vec![-0.5, -0.5, -0.5, -0.5],
            vec![0.0, 0.0, 0.0, 0.0],
            vec![0.5, 0.5, 0.5, 0.5],
        ];
        let z = vec![
            vec![2.00, 1.25, 1.00, 1.25],
            vec![1.25, 0.50, 0.25, 0.50],
            vec![1.00, 0.25, 0.00, 0.25],
            vec![1.25, 0.50, 0.25, 0.50],
        ];
        (x, y, z)
    }

    #[test]
    fn new_works() {
        let contour = Contour::new();
        assert_eq!(contour.colors.len(), 0);
        assert_eq!(contour.levels.len(), 0);
        assert_eq!(contour.colormap_index, 0);
        assert_eq!(contour.colormap_name, "".to_string());
        assert_eq!(contour.no_lines, false);
        assert_eq!(contour.no_labels, false);
        assert_eq!(contour.no_inline_labels, false);
        assert_eq!(contour.no_colorbar, false);
        assert_eq!(contour.colorbar_label, "".to_string());
        assert_eq!(contour.colorbar_number_format, "".to_string());
        assert_eq!(contour.line_color, "black".to_string());
        assert_eq!(contour.line_width, 0.0);
        assert_eq!(contour.font_size_labels, 0.0);
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
        contour.colors = vec!["#f00".to_string(), "#0f0".to_string(), "#00f".to_string()];
        contour.levels = vec![0.25, 0.5, 1.0];
        let opt = contour.options_filled();
        assert_eq!(
            opt,
            ",colors=colors\
             ,levels=levels"
        );
        contour.colors = Vec::new();
        contour.colormap_index = 4;
        let opt = contour.options_filled();
        assert_eq!(
            opt,
            ",cmap=getColormap(4)\
             ,levels=levels"
        );
    }

    #[test]
    fn options_line_works() {
        let mut contour = Contour::new();
        contour.levels = vec![0.25, 0.5, 1.0];
        contour.line_color = "red".to_string();
        contour.line_width = 3.0;
        let opt = contour.options_line();
        assert_eq!(
            opt,
            ",colors=['red']\
             ,levels=levels\
             ,linewidths[3]"
        );
    }

    #[test]
    fn options_label_works() {
        let mut contour = Contour::new();
        contour.no_inline_labels = false;
        contour.font_size_labels = 5.0;
        let opt = contour.options_label();
        assert_eq!(
            opt,
            ",inline=True\
             ,fontsize=5"
        );
        contour.no_inline_labels = true;
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
        contour.colorbar_number_format = "%.4f".to_string();
        let opt = contour.options_colorbar();
        assert_eq!(opt, ",format='%.4f'");
    }

    #[test]
    fn options_selected_works() {
        let mut contour = Contour::new();
        contour.with_selected = true;
        contour.selected_level = 0.75;
        contour.selected_line_color = "blue".to_string();
        contour.selected_line_style = "--".to_string();
        contour.selected_line_width = 2.5;
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
        contour.colors = vec!["#f00".to_string(), "#0f0".to_string(), "#00f".to_string()];
        contour.levels = vec![0.25, 0.5, 1.0];
        contour.colorbar_label = "temperature".to_string();
        contour.with_selected = true;
        let (x, y, z) = gen_xyz();
        contour.draw(&x, &y, &z);
        let b: &str = "x=np.array([[-1,-0.5,0,0.5,],[-1,-0.5,0,0.5,],[-1,-0.5,0,0.5,],[-1,-0.5,0,0.5,],],dtype=float)\n\
                       y=np.array([[-1,-1,-1,-1,],[-0.5,-0.5,-0.5,-0.5,],[0,0,0,0,],[0.5,0.5,0.5,0.5,],],dtype=float)\n\
                       z=np.array([[2,1.25,1,1.25,],[1.25,0.5,0.25,0.5,],[1,0.25,0,0.25,],[1.25,0.5,0.25,0.5,],],dtype=float)\n\
                       colors=['#f00','#0f0','#00f',]\n\
                       levels=np.array([0.25,0.5,1,],dtype=float)\n\
                       cf=plt.contourf(x,y,z,colors=colors,levels=levels)\n\
                       cl=plt.contour(x,y,z,colors=['black'],levels=levels)\n\
                       plt.clabel(cl,inline=True)\n\
                       cb=plt.colorbar(cf)\n\
                       cb.ax.set_ylabel(r'temperature')\n\
                       plt.contour(x,y,z,colors=['yellow'],levels=[0],linestyles=['-'],linewidths=[2])\n";
        assert_eq!(contour.buffer, b);
    }
}
