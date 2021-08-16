use super::*;
use std::path::Path;

pub trait GraphMaker {
    fn get_buffer<'a>(&'a self) -> &'a String;
}

/// Driver structure that calls Python
///
/// ```
/// use plotpy::*;
/// let mut plot = Plot::new();
/// plot.axes_equal();
/// plot.axes_range(-1.0, 1.0, 0.0, 2.0);
/// plot.grid_and_labels("x-label", "y-label");
/// plot.save("/tmp/plotpy", "example_plot", "svg");
/// ```
///
pub struct Plot {
    /// hide bottom frame border
    pub option_hide_bottom_border: bool,

    /// hide left frame border
    pub option_hide_left_border: bool,

    /// hide right frame border
    pub option_hide_right_border: bool,

    /// hide top frame border
    pub option_hide_top_border: bool,

    /// font size for labels
    pub font_size_labels: f64,

    /// font size for legend
    pub font_size_legend: f64,

    /// font size for x-ticks
    pub font_size_x_tick: f64,

    /// font size for y-ticks
    pub font_size_y_tick: f64,

    // buffer
    pub(crate) buffer: String,
}

impl Plot {
    /// Creates new Plot object
    pub fn new() -> Self {
        Plot {
            // options
            option_hide_bottom_border: true,
            option_hide_left_border: true,
            option_hide_right_border: true,
            option_hide_top_border: true,

            // font sizes
            font_size_labels: 0.0,
            font_size_legend: 0.0,
            font_size_x_tick: 0.0,
            font_size_y_tick: 0.0,

            // buffer
            buffer: String::new(),
        }
    }

    /// Adds new graph entity
    pub fn add(&mut self, graph: &dyn GraphMaker) {
        self.buffer.push_str(graph.get_buffer());
    }

    /// Saves figure to disk
    ///
    /// # Arguments
    ///
    /// * `output_dir` - Creates a directory to save the figure, and temporary files
    /// * `filename_key` - The filename without extension
    /// * `filename_ext` - The extension of the filename; e.g., "png" or "svg"
    pub fn save(&self, output_dir: &str, filename_key: &str, filename_ext: &str) -> std::io::Result<String> {
        // filename
        let ext = filename_ext.replace(".", "");
        let filename_py = format!("{}.py", filename_key);
        let filename_fig = format!("{}.{}", filename_key, ext);
        let filepath_fig = Path::new(output_dir).join(filename_fig);

        // update commands
        let path = filepath_fig.to_string_lossy();
        let commands = format!(
            "{}\nfn='{}'\nplt.savefig(fn, bbox_inches='tight', bbox_extra_artists=EXTRA_ARTISTS)\nprint('figure {} created')\n",
            self.buffer,
            path, path,
        );

        // call python
        call_python3(&commands, output_dir, &filename_py)
    }

    /// Configures subplots
    ///
    /// # Arguments
    ///
    /// * `row` - number of rows in the subplot grid
    /// * `col` - number of columns in the subplot grid
    /// * `index` - activate current subplot; indices start at one [1-based]
    ///
    pub fn subplot(&mut self, row: i32, col: i32, index: i32) {
        self.buffer
            .push_str(&format!("plt.subplot({},{},{})\n", row, col, index));
    }

    /// Sets the horizontal gap between subplots
    pub fn subplot_horizontal_gap(&mut self, value: f64) {
        self.buffer
            .push_str(&format!("plt.subplots_adjust(hspace={})\n", value));
    }

    /// Sets the vertical gap between subplots
    pub fn subplot_vertical_gap(&mut self, value: f64) {
        self.buffer
            .push_str(&format!("plt.subplots_adjust(wspace={})\n", value));
    }

    /// Sets same scale for both axes
    pub fn axes_equal(&mut self) {
        self.buffer.push_str("plt.axis('equal')\n");
    }

    /// Hides axes
    pub fn axes_off(&mut self) {
        self.buffer.push_str("plt.axis('off')\n");
    }

    /// Sets axes limits
    pub fn axes_range(&mut self, xmin: f64, xmax: f64, ymin: f64, ymax: f64) {
        self.buffer
            .push_str(&format!("plt.axis([{},{},{},{}])\n", xmin, xmax, ymin, ymax));
    }

    /// Sets x and y limits
    pub fn axes_range_vec(&mut self, lims: &[f64]) {
        self.buffer.push_str(&format!(
            "plt.axis([{},{},{},{}])\n",
            lims[0], lims[1], lims[2], lims[3]
        ));
    }

    /// Sets minimum x
    pub fn axes_xmin(&mut self, xmin: f64) {
        self.buffer.push_str(&format!(
            "plt.axis([{},plt.axis()[1],plt.axis()[2],plt.axis()[3]])\n",
            xmin
        ));
    }

    /// Sets maximum x
    pub fn axes_xmax(&mut self, xmax: f64) {
        self.buffer.push_str(&format!(
            "plt.axis([plt.axis()[0],{},plt.axis()[2],plt.axis()[3]])\n",
            xmax
        ));
    }

    /// Sets minimum y
    pub fn axes_ymin(&mut self, ymin: f64) {
        self.buffer.push_str(&format!(
            "plt.axis([plt.axis()[0],plt.axis()[1],{},plt.axis()[3]])\n",
            ymin
        ));
    }

    /// Sets maximum y
    pub fn axes_ymax(&mut self, ymax: f64) {
        self.buffer.push_str(&format!(
            "plt.axis([plt.axis()[0],plt.axis()[1],plt.axis()[2],{}])\n",
            ymax
        ));
    }

    /// Sets x-range (i.e. limits)
    pub fn axes_xrange(&mut self, xmin: f64, xmax: f64) {
        self.buffer
            .push_str(&format!("plt.axis([{},{},plt.axis()[2],plt.axis()[3]])\n", xmin, xmax));
    }

    /// Sets y-range (i.e. limits)
    pub fn axes_yrange(&mut self, ymin: f64, ymax: f64) {
        self.buffer
            .push_str(&format!("plt.axis([plt.axis()[0],plt.axis()[1],{},{}])\n", ymin, ymax));
    }

    /// Adds grid, labels, and legend
    pub fn grid_and_labels(&mut self, xlabel: &str, ylabel: &str) {
        self.buffer.push_str(&format!(
            "plt.grid(linestyle='--',color='grey',zorder=-1000)\nplt.xlabel(r'{}')\nplt.ylabel(r'{}')\n",
            xlabel, ylabel,
        ));
    }

    /// Clears current figure
    pub fn clear_current_figure(&mut self) {
        self.buffer.push_str("plt.clf()\n");
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn new_plot_works() -> Result<(), Box<dyn std::error::Error>> {
        let plot = Plot::new();
        assert_eq!(plot.buffer.len(), 0);
        plot.save("/tmp/plotpy", "test", "svg")?;
        let svg = fs::read_to_string("/tmp/plotpy/test.svg")?;
        let lines = svg.lines().collect::<Vec<_>>();
        assert_eq!(lines.len(), 33);
        Ok(())
    }

    #[test]
    fn subplot_functions_work() {
        let mut plot = Plot::new();
        plot.subplot(2, 2, 0);
        plot.subplot_horizontal_gap(0.1);
        plot.subplot_vertical_gap(0.2);
        let correct = "plt.subplot(2,2,0)
plt.subplots_adjust(hspace=0.1)
plt.subplots_adjust(wspace=0.2)
";
        assert_eq!(plot.buffer, correct);
    }

    #[test]
    fn axes_functions_work() {
        let mut plot = Plot::new();
        plot.axes_equal();
        plot.axes_off();
        plot.axes_range(-1.0, 1.0, -1.0, 1.0);
        let correct = "plt.axis('equal')
plt.axis('off')
plt.axis([-1,1,-1,1])
";
        assert_eq!(plot.buffer, correct);
    }
}
