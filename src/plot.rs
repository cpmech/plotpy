use super::*;
use std::fmt::Write;
use std::fs::{self, File};
use std::io::Write as IoWrite;
use std::path::Path;

pub trait GraphMaker {
    fn get_buffer<'a>(&'a self) -> &'a String;
}

/// Driver structure that calls Python
///
/// # Example
///
/// ```
/// # fn main() -> Result<(), &'static str> {
/// // import
/// use plotpy::*;
/// use russell_lab::Vector;
/// use std::path::Path;
///
/// // directory to save figures
/// const OUT_DIR: &str = "/tmp/plotpy/doc_tests";
///
/// // generate (x,y) points
/// let n = 11;
/// let x = Vector::linspace(-1.0, 1.0, n);
/// let y1 = x.get_copy();
/// let y2 = x.get_mapped(|v| f64::abs(v));
/// let y3 = x.get_mapped(|v| f64::exp(1.0 + v) - 1.0);
/// let y4 = x.get_mapped(|v| f64::sqrt(1.0 + v));
///
/// // configure and draw curves
/// let mut curve1 = Curve::new();
/// let mut curve2 = Curve::new();
/// let mut curve3 = Curve::new();
/// let mut curve4 = Curve::new();
/// curve1.set_label("y = x");
/// curve2.set_label("y = |x|");
/// curve2.set_line_color("#cd0000");
/// curve3.set_label("y = exp(1+x)-1");
/// curve3.set_line_color("#e79955");
/// curve4.set_label("y = sqrt(1+x)");
/// curve4.set_line_color("#b566ab");
/// curve1.draw(&x, &y1);
/// curve2.draw(&x, &y2);
/// curve3.draw(&x, &y3);
/// curve4.draw(&x, &y4);
///
/// // configure plot
/// let mut plot = Plot::new();
/// plot.set_super_title("FOUR CURVES");
/// plot.set_gaps(0.35, 0.5);
///
/// // add curve to subplot
/// plot.subplot(2, 2, 1);
/// plot.set_title("first");
/// plot.add(&curve1);
/// plot.set_equal_axes();
/// plot.grid_labels_legend("x", "y");
///
/// // add curve to subplot
/// plot.subplot(2, 2, 2);
/// plot.set_title("second");
/// plot.add(&curve2);
/// plot.grid_labels_legend("x", "y");
///
/// // add curve to subplot
/// plot.subplot(2, 2, 3);
/// plot.set_title("third");
/// plot.add(&curve3);
/// plot.set_range(-1.0, 1.0, 0.0, 6.0);
/// plot.grid_labels_legend("x", "y");
///
/// // add curve to subplot
/// plot.subplot(2, 2, 4);
/// plot.set_title("fourth");
/// plot.add(&curve4);
/// plot.grid_labels_legend("x", "y");
///
/// // save figure
/// let path = Path::new(OUT_DIR).join("doc_plot.svg");
/// plot.save(&path)?;
/// # Ok(())
/// # }
/// ```
///
/// ![doc_plot.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/doc_plot.svg)
///
pub struct Plot {
    buffer: String,
}

impl Plot {
    /// Creates new Plot object
    pub fn new() -> Self {
        Plot { buffer: String::new() }
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

    /// Prints the log file created by the save command
    ///
    /// **Note:** the log file is **only** generated when python fails
    pub fn print_log_file(&self, figure_path: &Path) -> Result<(), &'static str> {
        let mut log_path = Path::new(figure_path).to_path_buf();
        log_path.set_extension("log");
        let output = fs::read_to_string(log_path).map_err(|_| "cannot read log file")?;
        println!("{}", output);
        Ok(())
    }

    /// Clears current figure
    pub fn clear_current_figure(&mut self) {
        self.buffer.push_str("plt.clf()\n");
    }

    /// Adds legend to plot (see Legend for further options)
    pub fn legend(&mut self) {
        let mut legend = Legend::new();
        legend.draw();
        self.add(&legend);
    }

    /// Adds grid and labels
    pub fn grid_and_labels(&mut self, xlabel: &str, ylabel: &str) {
        write!(
            &mut self.buffer,
            "plt.grid(linestyle='--',color='grey',zorder=-1000)\nplt.xlabel(r'{}')\nplt.ylabel(r'{}')\n",
            xlabel, ylabel
        )
        .unwrap();
    }

    /// Adds grid, labels, and legend
    pub fn grid_labels_legend(&mut self, xlabel: &str, ylabel: &str) {
        write!(
            &mut self.buffer,
            "plt.grid(linestyle='--',color='grey',zorder=-1000)\nplt.xlabel(r'{}')\nplt.ylabel(r'{}')\n",
            xlabel, ylabel
        )
        .unwrap();
        self.legend();
    }

    /// Configures subplots
    ///
    /// # Arguments
    ///
    /// * `row` - number of rows in the subplot grid
    /// * `col` - number of columns in the subplot grid
    /// * `index` - activate current subplot; **indices start at one** (1-based)
    ///
    pub fn subplot(&mut self, row: usize, col: usize, index: usize) {
        write!(&mut self.buffer, "\nplt.subplot({},{},{})\n", row, col, index).unwrap();
    }

    /// Adds a title to the plot or sub-plot
    pub fn set_title(&mut self, title: &str) {
        write!(&mut self.buffer, "plt.title(r'{}')\n", title).unwrap();
    }

    /// Adds a title to all sub-plots
    pub fn set_super_title(&mut self, title: &str) {
        write!(&mut self.buffer, "st=plt.suptitle(r'{}')\naddToEA(st)\n", title).unwrap();
    }

    /// Sets the horizontal gap between subplots
    pub fn set_horizontal_gap(&mut self, value: f64) {
        write!(&mut self.buffer, "plt.subplots_adjust(wspace={})\n", value).unwrap();
    }

    /// Sets the vertical gap between subplots
    pub fn set_vertical_gap(&mut self, value: f64) {
        write!(&mut self.buffer, "plt.subplots_adjust(hspace={})\n", value).unwrap();
    }

    /// Sets the horizontal and vertical gap between subplots
    pub fn set_gaps(&mut self, horizontal: f64, vertical: f64) {
        write!(
            &mut self.buffer,
            "plt.subplots_adjust(wspace={},hspace={})\n",
            horizontal, vertical
        )
        .unwrap();
    }

    /// Sets same scale for both axes
    pub fn set_equal_axes(&mut self) {
        self.buffer.push_str("plt.axis('equal')\n");
    }

    /// Hides axes
    pub fn hide_axes(&mut self) {
        self.buffer.push_str("plt.axis('off')\n");
    }

    /// Sets axes limits
    pub fn set_range(&mut self, xmin: f64, xmax: f64, ymin: f64, ymax: f64) {
        write!(&mut self.buffer, "plt.axis([{},{},{},{}])\n", xmin, xmax, ymin, ymax).unwrap();
    }

    /// Sets axes limits from vector
    pub fn set_range_from_vec(&mut self, limits: &[f64]) {
        write!(
            &mut self.buffer,
            "plt.axis([{},{},{},{}])\n",
            limits[0], limits[1], limits[2], limits[3]
        )
        .unwrap();
    }

    /// Sets minimum x
    pub fn set_xmin(&mut self, xmin: f64) {
        write!(
            &mut self.buffer,
            "plt.axis([{},plt.axis()[1],plt.axis()[2],plt.axis()[3]])\n",
            xmin
        )
        .unwrap();
    }

    /// Sets maximum x
    pub fn set_xmax(&mut self, xmax: f64) {
        write!(
            &mut self.buffer,
            "plt.axis([plt.axis()[0],{},plt.axis()[2],plt.axis()[3]])\n",
            xmax
        )
        .unwrap();
    }

    /// Sets minimum y
    pub fn set_ymin(&mut self, ymin: f64) {
        write!(
            &mut self.buffer,
            "plt.axis([plt.axis()[0],plt.axis()[1],{},plt.axis()[3]])\n",
            ymin
        )
        .unwrap();
    }

    /// Sets maximum y
    pub fn set_ymax(&mut self, ymax: f64) {
        write!(
            &mut self.buffer,
            "plt.axis([plt.axis()[0],plt.axis()[1],plt.axis()[2],{}])\n",
            ymax
        )
        .unwrap();
    }

    /// Sets x-range (i.e. limits)
    pub fn set_xrange(&mut self, xmin: f64, xmax: f64) {
        write!(
            &mut self.buffer,
            "plt.axis([{},{},plt.axis()[2],plt.axis()[3]])\n",
            xmin, xmax
        )
        .unwrap();
    }

    /// Sets y-range (i.e. limits)
    pub fn set_yrange(&mut self, ymin: f64, ymax: f64) {
        write!(
            &mut self.buffer,
            "plt.axis([plt.axis()[0],plt.axis()[1],{},{}])\n",
            ymin, ymax
        )
        .unwrap();
    }

    // Sets number of ticks along x
    pub fn set_num_ticks_x(&mut self, num: usize) {
        if num == 0 {
            self.buffer.push_str("plt.gca().get_xaxis().set_ticks([])\n");
        } else {
            write!(
                &mut self.buffer,
                "plt.gca().get_xaxis().set_major_locator(tck.MaxNLocator({}))\n",
                num
            )
            .unwrap();
        }
    }

    // Sets number of ticks along y
    pub fn set_num_ticks_y(&mut self, num: usize) {
        if num == 0 {
            self.buffer.push_str("plt.gca().get_yaxis().set_ticks([])\n");
        } else {
            write!(
                &mut self.buffer,
                "plt.gca().get_yaxis().set_major_locator(tck.MaxNLocator({}))\n",
                num
            )
            .unwrap();
        }
    }

    /// Sets the label for the x-axis
    pub fn set_label_x(&mut self, label: &str) {
        write!(&mut self.buffer, "plt.xlabel(r'{}')\n", label).unwrap();
    }

    /// Sets the label for the y-axis
    pub fn set_label_y(&mut self, label: &str) {
        write!(&mut self.buffer, "plt.ylabel(r'{}')\n", label).unwrap();
    }

    /// Sets the labels of x and y axis
    pub fn set_labels(&mut self, xlabel: &str, ylabel: &str) {
        write!(
            &mut self.buffer,
            "plt.xlabel(r'{}')\nplt.ylabel(r'{}')\n",
            xlabel, ylabel
        )
        .unwrap();
    }

    /// Sets camera in 3d graph. Sets the elevation and azimuth of the axes.
    ///
    /// # Input
    ///
    /// * `elev` -- is the elevation angle in the z plane
    /// * `azimuth` -- is the azimuth angle in the x,y plane
    pub fn set_camera(&mut self, elev: f64, azimuth: f64) {
        write!(
            &mut self.buffer,
            "plt.gca().view_init(elev={},azim={})\n",
            elev, azimuth
        )
        .unwrap();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, BufReader};

    const OUT_DIR: &str = "/tmp/plotpy/unit_tests";

    #[test]
    fn new_plot_works() {
        let plot = Plot::new();
        assert_eq!(plot.buffer.len(), 0);
    }

    #[test]
    fn save_works() -> Result<(), &'static str> {
        let plot = Plot::new();
        assert_eq!(plot.buffer.len(), 0);
        let path = Path::new(OUT_DIR).join("save_works.svg");
        plot.save(&path)?;
        let file = File::open(path).map_err(|_| "cannot open file")?;
        let buffered = BufReader::new(file);
        let lines_iter = buffered.lines();
        assert!(lines_iter.count() > 20);
        Ok(())
    }

    #[test]
    fn print_log_file_works() -> Result<(), &'static str> {
        const WRONG: usize = 0;
        let mut plot = Plot::new();
        plot.subplot(1, 1, WRONG);
        let path = Path::new(OUT_DIR).join("print_log_file_works.svg");
        assert_eq!(plot.save(&path).err(), Some("python3 failed; please see the log file"));
        plot.print_log_file(&path)?;
        Ok(())
    }

    #[test]
    fn subplot_functions_work() {
        let mut plot = Plot::new();
        plot.set_super_title("all subplots");
        plot.subplot(2, 2, 1);
        plot.set_horizontal_gap(0.1);
        plot.set_vertical_gap(0.2);
        plot.set_gaps(0.3, 0.4);
        let correct: &str = "st=plt.suptitle(r'all subplots')\n\
                             addToEA(st)\n\
                             \nplt.subplot(2,2,1)\n\
                               plt.subplots_adjust(wspace=0.1)\n\
                               plt.subplots_adjust(hspace=0.2)\n\
                               plt.subplots_adjust(wspace=0.3,hspace=0.4)\n";
        assert_eq!(plot.buffer, correct);
    }

    #[test]
    fn axes_functions_work() {
        let mut plot = Plot::new();
        plot.set_title(&"my plot".to_string());
        plot.set_equal_axes();
        plot.hide_axes();
        plot.set_range(-1.0, 1.0, -1.0, 1.0);
        plot.set_range_from_vec(&[0.0, 1.0, 0.0, 1.0]);
        plot.set_xmin(0.0);
        plot.set_xmax(1.0);
        plot.set_ymin(0.0);
        plot.set_ymax(1.0);
        plot.set_xrange(0.0, 1.0);
        plot.set_yrange(0.0, 1.0);
        plot.set_num_ticks_x(0);
        plot.set_num_ticks_x(8);
        plot.set_num_ticks_y(0);
        plot.set_num_ticks_y(5);
        plot.set_label_x("x-label");
        plot.set_label_y("y-label");
        plot.set_labels("x", "y");
        plot.grid_and_labels("xx", "yy");
        plot.clear_current_figure();
        plot.legend();
        plot.set_camera(1.0, 10.0);
        let correct: &str = "plt.title(r'my plot')\n\
                             plt.axis('equal')\n\
                             plt.axis('off')\n\
                             plt.axis([-1,1,-1,1])\n\
                             plt.axis([0,1,0,1])\n\
                             plt.axis([0,plt.axis()[1],plt.axis()[2],plt.axis()[3]])\n\
                             plt.axis([plt.axis()[0],1,plt.axis()[2],plt.axis()[3]])\n\
                             plt.axis([plt.axis()[0],plt.axis()[1],0,plt.axis()[3]])\n\
                             plt.axis([plt.axis()[0],plt.axis()[1],plt.axis()[2],1])\n\
                             plt.axis([0,1,plt.axis()[2],plt.axis()[3]])\n\
                             plt.axis([plt.axis()[0],plt.axis()[1],0,1])\n\
                             plt.gca().get_xaxis().set_ticks([])\n\
                             plt.gca().get_xaxis().set_major_locator(tck.MaxNLocator(8))\n\
                             plt.gca().get_yaxis().set_ticks([])\n\
                             plt.gca().get_yaxis().set_major_locator(tck.MaxNLocator(5))\n\
                             plt.xlabel(r'x-label')\n\
                             plt.ylabel(r'y-label')\n\
                             plt.xlabel(r'x')\n\
                             plt.ylabel(r'y')\n\
                             plt.grid(linestyle='--',color='grey',zorder=-1000)\n\
                             plt.xlabel(r'xx')\n\
                             plt.ylabel(r'yy')\n\
                             plt.clf()\n\
                             h,l=plt.gca().get_legend_handles_labels()\n\
                             if len(h)>0 and len(l)>0:\n\
                             \x20\x20\x20\x20leg=plt.legend(handlelength=3,ncol=1,loc='best')\n\
                             \x20\x20\x20\x20addToEA(leg)\n\
                             plt.gca().view_init(elev=1,azim=10)\n";
        assert_eq!(plot.buffer, correct);
    }
}
