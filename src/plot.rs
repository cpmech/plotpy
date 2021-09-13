use super::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub trait GraphMaker {
    fn get_buffer<'a>(&'a self) -> &'a String;
}

/// Driver structure that calls Python
///
/// ```
/// use plotpy::*;
/// let mut plot = Plot::new();
/// plot.equal();
/// plot.range(-1.0, 1.0, 0.0, 2.0);
/// plot.grid_and_labels("x-label", "y-label");
/// ```
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

    /// Calls python3 and saves the python script and figure
    pub fn save(&self, figure_path: &Path) -> Result<(), &'static str> {
        // update commands
        let commands = format!(
            "{}\nfn='{}'\nplt.savefig(fn, bbox_inches='tight', bbox_extra_artists=EXTRA_ARTISTS)\n",
            self.buffer,
            figure_path.to_string_lossy(),
        );

        // call python
        let mut path = Path::new(figure_path).to_path_buf();
        path.set_extension("py");
        let output = call_python3(&commands, &path)?;

        // handle error => write log file
        if output != "" {
            let mut log_path = Path::new(figure_path).to_path_buf();
            log_path.set_extension("log");
            let mut log_file = File::create(log_path).map_err(|_| "cannot create log file")?;
            log_file
                .write_all(output.as_bytes())
                .map_err(|_| "cannot write to log file")?;
            return Err("python3 failed; please see the log file");
        }

        Ok(())
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
            .push_str(&format!("\nplt.subplot({},{},{})\n", row, col, index));
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

    /// Sets the horizontal and vertical gap between subplots
    pub fn subplot_gap(&mut self, horizontal: f64, vertical: f64) {
        self.buffer.push_str(&format!(
            "plt.subplots_adjust(hspace={},wspace={})\n",
            horizontal, vertical
        ));
    }

    /// Sets same scale for both axes
    pub fn equal(&mut self) {
        self.buffer.push_str("plt.axis('equal')\n");
    }

    /// Hides axes
    pub fn hide_axes(&mut self) {
        self.buffer.push_str("plt.axis('off')\n");
    }

    /// Sets axes limits
    pub fn range(&mut self, xmin: f64, xmax: f64, ymin: f64, ymax: f64) {
        self.buffer
            .push_str(&format!("plt.axis([{},{},{},{}])\n", xmin, xmax, ymin, ymax));
    }

    /// Sets x and y limits
    pub fn range_vec(&mut self, lims: &[f64]) {
        self.buffer.push_str(&format!(
            "plt.axis([{},{},{},{}])\n",
            lims[0], lims[1], lims[2], lims[3]
        ));
    }

    /// Sets minimum x
    pub fn xmin(&mut self, xmin: f64) {
        self.buffer.push_str(&format!(
            "plt.axis([{},plt.axis()[1],plt.axis()[2],plt.axis()[3]])\n",
            xmin
        ));
    }

    /// Sets maximum x
    pub fn xmax(&mut self, xmax: f64) {
        self.buffer.push_str(&format!(
            "plt.axis([plt.axis()[0],{},plt.axis()[2],plt.axis()[3]])\n",
            xmax
        ));
    }

    /// Sets minimum y
    pub fn ymin(&mut self, ymin: f64) {
        self.buffer.push_str(&format!(
            "plt.axis([plt.axis()[0],plt.axis()[1],{},plt.axis()[3]])\n",
            ymin
        ));
    }

    /// Sets maximum y
    pub fn ymax(&mut self, ymax: f64) {
        self.buffer.push_str(&format!(
            "plt.axis([plt.axis()[0],plt.axis()[1],plt.axis()[2],{}])\n",
            ymax
        ));
    }

    /// Sets x-range (i.e. limits)
    pub fn xrange(&mut self, xmin: f64, xmax: f64) {
        self.buffer
            .push_str(&format!("plt.axis([{},{},plt.axis()[2],plt.axis()[3]])\n", xmin, xmax));
    }

    /// Sets y-range (i.e. limits)
    pub fn yrange(&mut self, ymin: f64, ymax: f64) {
        self.buffer
            .push_str(&format!("plt.axis([plt.axis()[0],plt.axis()[1],{},{}])\n", ymin, ymax));
    }

    // Sets number of ticks along x
    pub fn xnticks(&mut self, num: i32) {
        if num == 0 {
            self.buffer.push_str("plt.gca().get_xaxis().set_ticks([])\n");
        } else {
            self.buffer.push_str(&format!(
                "plt.gca().get_xaxis().set_major_locator(tck.MaxNLocator({}))\n",
                num
            ));
        }
    }

    // Sets number of ticks along y
    pub fn ynticks(&mut self, num: i32) {
        if num == 0 {
            self.buffer.push_str("plt.gca().get_yaxis().set_ticks([])\n");
        } else {
            self.buffer.push_str(&format!(
                "plt.gca().get_yaxis().set_major_locator(tck.MaxNLocator({}))\n",
                num
            ));
        }
    }

    /// Adds x-label
    pub fn xlabel(&mut self, xlabel: &str) {
        self.buffer.push_str(&format!("plt.xlabel(r'{}')\n", xlabel));
    }

    /// Adds y-label
    pub fn ylabel(&mut self, ylabel: &str) {
        self.buffer.push_str(&format!("plt.ylabel(r'{}')\n", ylabel));
    }

    /// Adds labels
    pub fn labels(&mut self, xlabel: &str, ylabel: &str) {
        self.buffer
            .push_str(&format!("plt.xlabel(r'{}')\nplt.ylabel(r'{}')\n", xlabel, ylabel));
    }

    /// Adds grid and labels
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

    const OUT_DIR: &str = "/tmp/plotpy/unit_tests";

    #[test]
    fn new_plot_works() {
        let plot = Plot::new();
        assert_eq!(plot.option_hide_bottom_border, true);
        assert_eq!(plot.option_hide_left_border, true);
        assert_eq!(plot.option_hide_right_border, true);
        assert_eq!(plot.option_hide_top_border, true);
        assert_eq!(plot.font_size_labels, 0.0);
        assert_eq!(plot.font_size_legend, 0.0);
        assert_eq!(plot.font_size_x_tick, 0.0);
        assert_eq!(plot.font_size_y_tick, 0.0);
        assert_eq!(plot.buffer.len(), 0);
    }

    #[test]
    fn save_works() -> Result<(), &'static str> {
        let plot = Plot::new();
        assert_eq!(plot.buffer.len(), 0);
        let path = Path::new(OUT_DIR).join("test.svg");
        plot.save(&path)?;
        let svg = fs::read_to_string(&path).map_err(|_| "cannot read file")?;
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
        let correct: &str = "\nplt.subplot(2,2,0)\n\
                               plt.subplots_adjust(hspace=0.1)\n\
                               plt.subplots_adjust(wspace=0.2)\n";
        assert_eq!(plot.buffer, correct);
    }

    #[test]
    fn axes_functions_work() {
        let mut plot = Plot::new();
        plot.equal();
        plot.hide_axes();
        plot.range(-1.0, 1.0, -1.0, 1.0);
        let correct: &str = "plt.axis('equal')\n\
                             plt.axis('off')\n\
                             plt.axis([-1,1,-1,1])\n";
        assert_eq!(plot.buffer, correct);
    }
}
