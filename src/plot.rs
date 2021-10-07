use super::{call_python3, Legend};
use std::ffi::OsStr;
use std::fmt::Write;
use std::fs::{self, File};
use std::io::Write as IoWrite;
use std::path::Path;

/// Defines the trait used by Plot to add graph entities
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
/// use plotpy::{Curve, Plot};
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
/// curve2.set_label("y = |x|").set_line_color("#cd0000");
/// curve3.set_label("y = exp(1+x)-1").set_line_color("#e79955");
/// curve4.set_label("y = sqrt(1+x)").set_line_color("#b566ab");
/// curve1.draw(&x, &y1);
/// curve2.draw(&x, &y2);
/// curve3.draw(&x, &y3);
/// curve4.draw(&x, &y4);
///
/// // configure plot
/// let mut plot = Plot::new();
/// plot.set_super_title("FOUR CURVES").set_gaps(0.35, 0.5);
///
/// // add curve to subplot
/// plot.set_subplot(2, 2, 1)
///     .set_title("first")
///     .add(&curve1)
///     .grid_labels_legend("x", "y")
///     .set_equal_axes(true);
///
/// // add curve to subplot
/// plot.set_subplot(2, 2, 2)
///     .set_title("second")
///     .add(&curve2)
///     .grid_labels_legend("x", "y");
///
/// // add curve to subplot
/// plot.set_subplot(2, 2, 3)
///     .set_title("third")
///     .add(&curve3)
///     .set_range(-1.0, 1.0, 0.0, 6.0)
///     .grid_labels_legend("x", "y");
///
/// // add curve to subplot
/// plot.set_subplot(2, 2, 4)
///     .set_title("fourth")
///     .add(&curve4)
///     .grid_labels_legend("x", "y");
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
    pub fn add(&mut self, graph: &dyn GraphMaker) -> &mut Self {
        self.buffer.push_str(graph.get_buffer());
        self
    }

    /// Calls python3 and saves the python script and figure
    ///
    /// # Input
    ///
    /// * `figure_path` -- may be a String, &str, or Path
    pub fn save<S>(&self, figure_path: &S) -> Result<(), &'static str>
    where
        S: AsRef<OsStr> + ?Sized,
    {
        // update commands
        let fig_path = Path::new(figure_path);
        let commands = format!(
            "{}\nfn='{}'\nplt.savefig(fn, bbox_inches='tight', bbox_extra_artists=EXTRA_ARTISTS)\n",
            self.buffer,
            fig_path.to_string_lossy(),
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

            // TODO: if not CI, do not print the log file by default
            // self.print_log_file(&figure_path)?;

            return Err("python3 failed; please see the log file");
        }
        Ok(())
    }

    /// Prints the log file created by the save command
    ///
    /// **Note:** the log file is **only** generated when python fails
    pub fn print_log_file<S>(&self, figure_path: &S) -> Result<(), &'static str>
    where
        S: AsRef<OsStr> + ?Sized,
    {
        let mut log_path = Path::new(figure_path).to_path_buf();
        log_path.set_extension("log");
        let output = fs::read_to_string(log_path).map_err(|_| "cannot read log file")?;
        println!("{}", output);
        Ok(())
    }

    /// Clears current figure
    pub fn clear_current_figure(&mut self) -> &mut Self {
        self.buffer.push_str("plt.clf()\n");
        self
    }

    /// Adds legend to plot (see Legend for further options)
    pub fn legend(&mut self) -> &mut Self {
        let mut legend = Legend::new();
        legend.draw();
        self.add(&legend)
    }

    /// Adds grid and labels
    pub fn grid_and_labels(&mut self, xlabel: &str, ylabel: &str) -> &mut Self {
        write!(
            &mut self.buffer,
            "plt.gca().set_axisbelow(True)\n\
             plt.grid(linestyle='--',color='grey',zorder=-1000)\n\
             plt.xlabel(r'{}')\n\
             plt.ylabel(r'{}')\n",
            xlabel, ylabel
        )
        .unwrap();
        self
    }

    /// Adds grid, labels, and legend
    pub fn grid_labels_legend(&mut self, xlabel: &str, ylabel: &str) -> &mut Self {
        write!(
            &mut self.buffer,
            "plt.gca().set_axisbelow(True)\n\
             plt.grid(linestyle='--',color='grey',zorder=-1000)\n\
             plt.xlabel(r'{}')\n\
             plt.ylabel(r'{}')\n",
            xlabel, ylabel
        )
        .unwrap();
        self.legend()
    }

    /// Writes extra Python/Matplotlib commands to buffer
    ///
    /// # Note
    ///
    /// The order of commands matter (as usual in a Python/Matplotlib script).
    pub fn write_extra(&mut self, commands: &str) -> &mut Self {
        self.buffer.push_str(commands);
        self
    }

    /// Configures subplots
    ///
    /// # Arguments
    ///
    /// * `row` - number of rows in the subplot grid
    /// * `col` - number of columns in the subplot grid
    /// * `index` - activate current subplot; **indices start at one** (1-based)
    ///
    pub fn set_subplot(&mut self, row: usize, col: usize, index: usize) -> &mut Self {
        write!(&mut self.buffer, "\nplt.subplot({},{},{})\n", row, col, index).unwrap();
        self
    }

    /// Adds a title to the plot or sub-plot
    pub fn set_title(&mut self, title: &str) -> &mut Self {
        write!(&mut self.buffer, "plt.title(r'{}')\n", title).unwrap();
        self
    }

    /// Adds a title to all sub-plots
    pub fn set_super_title(&mut self, title: &str) -> &mut Self {
        write!(&mut self.buffer, "st=plt.suptitle(r'{}')\naddToEA(st)\n", title).unwrap();
        self
    }

    /// Sets the horizontal gap between subplots
    pub fn set_horizontal_gap(&mut self, value: f64) -> &mut Self {
        write!(&mut self.buffer, "plt.subplots_adjust(wspace={})\n", value).unwrap();
        self
    }

    /// Sets the vertical gap between subplots
    pub fn set_vertical_gap(&mut self, value: f64) -> &mut Self {
        write!(&mut self.buffer, "plt.subplots_adjust(hspace={})\n", value).unwrap();
        self
    }

    /// Sets the horizontal and vertical gap between subplots
    pub fn set_gaps(&mut self, horizontal: f64, vertical: f64) -> &mut Self {
        write!(
            &mut self.buffer,
            "plt.subplots_adjust(wspace={},hspace={})\n",
            horizontal, vertical
        )
        .unwrap();
        self
    }

    /// Sets same scale for both axes
    pub fn set_equal_axes(&mut self, equal: bool) -> &mut Self {
        if equal {
            self.buffer.push_str("setEqualAspect()\n");
        } else {
            self.buffer.push_str("plt.gca().axes.set_aspect('auto')\n");
        }
        self
    }

    /// Set option to hide axes
    pub fn set_hide_axes(&mut self, hide: bool) -> &mut Self {
        let option = if hide { "off" } else { "on" };
        write!(&mut self.buffer, "plt.axis('{}')\n", option).unwrap();
        self
    }

    /// Sets axes limits
    pub fn set_range(&mut self, xmin: f64, xmax: f64, ymin: f64, ymax: f64) -> &mut Self {
        write!(&mut self.buffer, "plt.axis([{},{},{},{}])\n", xmin, xmax, ymin, ymax).unwrap();
        self
    }

    /// Sets axes limits from vector
    pub fn set_range_from_vec(&mut self, limits: &[f64]) -> &mut Self {
        write!(
            &mut self.buffer,
            "plt.axis([{},{},{},{}])\n",
            limits[0], limits[1], limits[2], limits[3]
        )
        .unwrap();
        self
    }

    /// Sets minimum x
    pub fn set_xmin(&mut self, xmin: f64) -> &mut Self {
        write!(
            &mut self.buffer,
            "plt.axis([{},plt.axis()[1],plt.axis()[2],plt.axis()[3]])\n",
            xmin
        )
        .unwrap();
        self
    }

    /// Sets maximum x
    pub fn set_xmax(&mut self, xmax: f64) -> &mut Self {
        write!(
            &mut self.buffer,
            "plt.axis([plt.axis()[0],{},plt.axis()[2],plt.axis()[3]])\n",
            xmax
        )
        .unwrap();
        self
    }

    /// Sets minimum y
    pub fn set_ymin(&mut self, ymin: f64) -> &mut Self {
        write!(
            &mut self.buffer,
            "plt.axis([plt.axis()[0],plt.axis()[1],{},plt.axis()[3]])\n",
            ymin
        )
        .unwrap();
        self
    }

    /// Sets maximum y
    pub fn set_ymax(&mut self, ymax: f64) -> &mut Self {
        write!(
            &mut self.buffer,
            "plt.axis([plt.axis()[0],plt.axis()[1],plt.axis()[2],{}])\n",
            ymax
        )
        .unwrap();
        self
    }

    /// Sets x-range (i.e. limits)
    pub fn set_xrange(&mut self, xmin: f64, xmax: f64) -> &mut Self {
        write!(
            &mut self.buffer,
            "plt.axis([{},{},plt.axis()[2],plt.axis()[3]])\n",
            xmin, xmax
        )
        .unwrap();
        self
    }

    /// Sets y-range (i.e. limits)
    pub fn set_yrange(&mut self, ymin: f64, ymax: f64) -> &mut Self {
        write!(
            &mut self.buffer,
            "plt.axis([plt.axis()[0],plt.axis()[1],{},{}])\n",
            ymin, ymax
        )
        .unwrap();
        self
    }

    /// Sets number of ticks along x
    pub fn set_num_ticks_x(&mut self, num: usize) -> &mut Self {
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
        self
    }

    /// Sets number of ticks along y
    pub fn set_num_ticks_y(&mut self, num: usize) -> &mut Self {
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
        self
    }

    /// Sets a log10 x-scale
    ///
    /// # Note
    ///
    /// `set_log_x(true)` must be called before adding curves.
    pub fn set_log_x(&mut self, log: bool) -> &mut Self {
        if log {
            self.buffer.push_str("plt.gca().set_xscale('log')\n");
        } else {
            self.buffer.push_str("plt.gca().set_xscale('linear')\n");
        }
        self
    }

    /// Sets a log10 y-scale
    ///
    /// # Note
    ///
    /// `set_log_y(true)` must be called before adding curves.
    pub fn set_log_y(&mut self, log: bool) -> &mut Self {
        if log {
            self.buffer.push_str("plt.gca().set_yscale('log')\n");
        } else {
            self.buffer.push_str("plt.gca().set_yscale('linear')\n");
        }
        self
    }

    /// Sets the label for the x-axis
    pub fn set_label_x(&mut self, label: &str) -> &mut Self {
        write!(&mut self.buffer, "plt.xlabel(r'{}')\n", label).unwrap();
        self
    }

    /// Sets the label for the y-axis
    pub fn set_label_y(&mut self, label: &str) -> &mut Self {
        write!(&mut self.buffer, "plt.ylabel(r'{}')\n", label).unwrap();
        self
    }

    /// Sets the labels of x and y axis
    pub fn set_labels(&mut self, xlabel: &str, ylabel: &str) -> &mut Self {
        write!(
            &mut self.buffer,
            "plt.xlabel(r'{}')\nplt.ylabel(r'{}')\n",
            xlabel, ylabel
        )
        .unwrap();
        self
    }

    /// Sets camera in 3d graph. Sets the elevation and azimuth of the axes.
    ///
    /// # Input
    ///
    /// * `elev` -- is the elevation angle in the z plane
    /// * `azimuth` -- is the azimuth angle in the x,y plane
    pub fn set_camera(&mut self, elev: f64, azimuth: f64) -> &mut Self {
        write!(
            &mut self.buffer,
            "plt.gca().view_init(elev={},azim={})\n",
            elev, azimuth
        )
        .unwrap();
        self
    }

    /// Sets option to hide (or show) frame borders
    pub fn set_frame_border(&mut self, left: bool, right: bool, bottom: bool, top: bool) -> &mut Self {
        if left {
            self.buffer.push_str("plt.gca().spines['left'].set_visible(True)\n");
        } else {
            self.buffer.push_str("plt.gca().spines['left'].set_visible(False)\n");
        }
        if right {
            self.buffer.push_str("plt.gca().spines['right'].set_visible(True)\n");
        } else {
            self.buffer.push_str("plt.gca().spines['right'].set_visible(False)\n");
        }
        if bottom {
            self.buffer.push_str("plt.gca().spines['bottom'].set_visible(True)\n");
        } else {
            self.buffer.push_str("plt.gca().spines['bottom'].set_visible(False)\n");
        }
        if top {
            self.buffer.push_str("plt.gca().spines['top'].set_visible(True)\n");
        } else {
            self.buffer.push_str("plt.gca().spines['top'].set_visible(False)\n");
        }
        self
    }

    /// Sets visibility of all frame borders
    pub fn set_frame_borders(&mut self, show_all: bool) -> &mut Self {
        self.set_frame_border(show_all, show_all, show_all, show_all)
    }

    /// Sets the figure size
    ///
    /// # Note
    ///
    /// This function must be called right at the beginning.
    pub fn set_figure_size(&mut self, width_points: f64, proportion: f64) -> &mut Self {
        let w = width_points / 72.27; // width in inches
        let h = w * proportion;
        write!(
            &mut self.buffer,
            "plt.rcParams.update({{'figure.figsize':[{},{}]}})\n",
            w as i32, h as i32
        )
        .unwrap();
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Plot;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::Path;

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
        let file = File::open(&path).map_err(|_| "cannot open file")?;
        let buffered = BufReader::new(file);
        let lines_iter = buffered.lines();
        assert!(lines_iter.count() > 20);
        Ok(())
    }

    #[test]
    fn save_str_works() -> Result<(), &'static str> {
        let plot = Plot::new();
        assert_eq!(plot.buffer.len(), 0);
        let path = "/tmp/plotpy/unit_tests/save_str_works.svg";
        plot.save(&path)?;
        let file = File::open(&path).map_err(|_| "cannot open file")?;
        let buffered = BufReader::new(file);
        let lines_iter = buffered.lines();
        assert!(lines_iter.count() > 20);
        Ok(())
    }

    #[test]
    fn print_log_file_works() -> Result<(), &'static str> {
        const WRONG: usize = 0;
        let mut plot = Plot::new();
        plot.set_subplot(1, 1, WRONG);
        let path = Path::new(OUT_DIR).join("print_log_file_works.svg");
        assert_eq!(plot.save(&path).err(), Some("python3 failed; please see the log file"));
        plot.print_log_file(&path)?;
        Ok(())
    }

    #[test]
    fn basic_functions_work() -> Result<(), &'static str> {
        let mut plot = Plot::new();
        plot.legend().write_extra("print('Hello World')\n");
        let b: &str = "h,l=plt.gca().get_legend_handles_labels()\n\
                       if len(h)>0 and len(l)>0:\n\
                       \x20\x20\x20\x20leg=plt.legend(handlelength=3,ncol=1,loc='best')\n\
                       \x20\x20\x20\x20addToEA(leg)\n\
                       print('Hello World')\n";
        assert_eq!(plot.buffer, b);
        Ok(())
    }

    #[test]
    fn subplot_functions_work() {
        let mut plot = Plot::new();
        plot.set_super_title("all subplots")
            .set_subplot(2, 2, 1)
            .set_horizontal_gap(0.1)
            .set_vertical_gap(0.2)
            .set_gaps(0.3, 0.4);
        let b: &str = "st=plt.suptitle(r'all subplots')\n\
                       addToEA(st)\n\
                       \nplt.subplot(2,2,1)\n\
                         plt.subplots_adjust(wspace=0.1)\n\
                         plt.subplots_adjust(hspace=0.2)\n\
                         plt.subplots_adjust(wspace=0.3,hspace=0.4)\n";
        assert_eq!(plot.buffer, b);
    }

    #[test]
    fn grid_functions_work() {
        let mut plot = Plot::new();
        plot.grid_and_labels("xx", "yy").grid_labels_legend("xx", "yy").legend();
        let b: &str = "plt.gca().set_axisbelow(True)\n\
                       plt.grid(linestyle='--',color='grey',zorder=-1000)\n\
                       plt.xlabel(r'xx')\n\
                       plt.ylabel(r'yy')\n\
                       plt.gca().set_axisbelow(True)\n\
                       plt.grid(linestyle='--',color='grey',zorder=-1000)\n\
                       plt.xlabel(r'xx')\n\
                       plt.ylabel(r'yy')\n\
                       h,l=plt.gca().get_legend_handles_labels()\n\
                       if len(h)>0 and len(l)>0:\n\
                       \x20\x20\x20\x20leg=plt.legend(handlelength=3,ncol=1,loc='best')\n\
                       \x20\x20\x20\x20addToEA(leg)\n\
                       h,l=plt.gca().get_legend_handles_labels()\n\
                       if len(h)>0 and len(l)>0:\n\
                       \x20\x20\x20\x20leg=plt.legend(handlelength=3,ncol=1,loc='best')\n\
                       \x20\x20\x20\x20addToEA(leg)\n";
        assert_eq!(plot.buffer, b);
    }

    #[test]
    fn set_functions_work() {
        let mut plot = Plot::new();
        plot.set_figure_size(400.0, 0.5);
        plot.set_title("my plot")
            .set_equal_axes(true)
            .set_equal_axes(false)
            .set_hide_axes(true)
            .set_range(-1.0, 1.0, -1.0, 1.0)
            .set_range_from_vec(&[0.0, 1.0, 0.0, 1.0])
            .set_xmin(0.0)
            .set_xmax(1.0)
            .set_ymin(0.0)
            .set_ymax(1.0)
            .set_xrange(0.0, 1.0)
            .set_yrange(0.0, 1.0)
            .set_num_ticks_x(0)
            .set_num_ticks_x(8)
            .set_num_ticks_y(0)
            .set_num_ticks_y(5)
            .set_log_x(true)
            .set_log_y(true)
            .set_log_x(false)
            .set_log_y(false)
            .set_label_x("x-label")
            .set_label_y("y-label")
            .set_labels("x", "y")
            .set_camera(1.0, 10.0)
            .clear_current_figure();
        let b: &str = "plt.rcParams.update({'figure.figsize':[5,2]})\n\
                       plt.title(r'my plot')\n\
                       setEqualAspect()\n\
                       plt.gca().axes.set_aspect('auto')\n\
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
                       plt.gca().set_xscale('log')\n\
                       plt.gca().set_yscale('log')\n\
                       plt.gca().set_xscale('linear')\n\
                       plt.gca().set_yscale('linear')\n\
                       plt.xlabel(r'x-label')\n\
                       plt.ylabel(r'y-label')\n\
                       plt.xlabel(r'x')\n\
                       plt.ylabel(r'y')\n\
                       plt.gca().view_init(elev=1,azim=10)\n\
                       plt.clf()\n";
        assert_eq!(plot.buffer, b);
    }

    #[test]
    fn set_frame_functions_work() {
        let mut plot = Plot::new();
        plot.set_frame_border(false, false, false, false)
            .set_frame_border(true, true, true, true)
            .set_frame_borders(false);
        let b: &str = "plt.gca().spines['left'].set_visible(False)\n\
                       plt.gca().spines['right'].set_visible(False)\n\
                       plt.gca().spines['bottom'].set_visible(False)\n\
                       plt.gca().spines['top'].set_visible(False)\n\
                       plt.gca().spines['left'].set_visible(True)\n\
                       plt.gca().spines['right'].set_visible(True)\n\
                       plt.gca().spines['bottom'].set_visible(True)\n\
                       plt.gca().spines['top'].set_visible(True)\n\
                       plt.gca().spines['left'].set_visible(False)\n\
                       plt.gca().spines['right'].set_visible(False)\n\
                       plt.gca().spines['bottom'].set_visible(False)\n\
                       plt.gca().spines['top'].set_visible(False)\n";
        assert_eq!(plot.buffer, b);
    }
}
