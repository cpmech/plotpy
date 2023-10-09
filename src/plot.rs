use super::{call_python3, Legend, StrError};
use std::ffi::OsStr;
use std::fmt::Write;
use std::fs::File;
use std::io::Write as IoWrite;
use std::path::Path;

/// Defines the trait used by Plot to add graph entities
pub trait GraphMaker {
    /// Returns the text buffer with Python3 commands
    fn get_buffer<'a>(&'a self) -> &'a String;

    /// Clear the text buffer with Python commands
    fn clear_buffer(&mut self);
}

/// Driver structure that calls Python
///
/// # Example
///
/// ```
/// use plotpy::{linspace, Curve, Plot, StrError};
///
/// fn main() -> Result<(), StrError> {
///     // generate (x,y) points
///     let n = 11;
///     let x = linspace(-1.0, 1.0, n);
///     let y1 = x.clone();
///     let y2: Vec<_> = x.iter().map(|v| f64::abs(*v)).collect();
///     let y3: Vec<_> = x.iter().map(|v| f64::exp(1.0 + *v) - 1.0).collect();
///     let y4: Vec<_> = x.iter().map(|v| f64::sqrt(1.0 + *v)).collect();
///
///     // configure and draw curves
///     let mut curve1 = Curve::new();
///     let mut curve2 = Curve::new();
///     let mut curve3 = Curve::new();
///     let mut curve4 = Curve::new();
///     curve1.set_label("y = x");
///     curve2.set_label("y = |x|").set_line_color("#cd0000");
///     curve3.set_label("y = exp(1+x)-1").set_line_color("#e79955");
///     curve4.set_label("y = sqrt(1+x)").set_line_color("#b566ab");
///     curve1.draw(&x, &y1);
///     curve2.draw(&x, &y2);
///     curve3.draw(&x, &y3);
///     curve4.draw(&x, &y4);
///
///     // configure plot
///     let mut plot = Plot::new();
///     plot.set_super_title("FOUR CURVES").set_gaps(0.35, 0.5);
///
///     // add curve to subplot
///     plot.set_subplot(2, 2, 1)
///         .set_title("first")
///         .add(&curve1)
///         .grid_labels_legend("x", "y")
///         .set_equal_axes(true);
///
///     // add curve to subplot
///     plot.set_subplot(2, 2, 2)
///         .set_title("second")
///         .add(&curve2)
///         .grid_labels_legend("x", "y");
///
///     // add curve to subplot
///     plot.set_subplot(2, 2, 3)
///         .set_title("third")
///         .add(&curve3)
///         .set_range(-1.0, 1.0, 0.0, 6.0)
///         .grid_labels_legend("x", "y");
///
///     // add curve to subplot
///     plot.set_subplot(2, 2, 4)
///         .set_title("fourth")
///         .add(&curve4)
///         .grid_labels_legend("x", "y");
///
///     // save figure
///     plot.save("/tmp/plotpy/doc_tests/doc_plot.svg")?;
///     Ok(())
/// }
/// ```
///
/// ![doc_plot.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/doc_plot.svg)
///
/// See also integration tests in the [tests directory](https://github.com/cpmech/plotpy/tree/main/tests)
pub struct Plot {
    show_errors: bool, // show python errors, if any
    buffer: String,    // buffer
}

impl Plot {
    /// Creates new Plot object
    pub fn new() -> Self {
        Plot {
            show_errors: false,
            buffer: String::new(),
        }
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
    ///
    /// # Note
    ///
    /// Call `set_show_errors` to configure how the errors (if any) are printed.
    pub fn save<S>(&self, figure_path: &S) -> Result<(), StrError>
    where
        S: AsRef<OsStr> + ?Sized,
    {
        self.run(figure_path, false)
    }

    /// Calls python3, saves the python script and figure, and show the plot window
    ///
    /// # Input
    ///
    /// * `figure_path` -- may be a String, &str, or Path
    ///
    /// # Note
    ///
    /// Call `set_show_errors` to configure how the errors (if any) are printed.
    pub fn save_and_show<S>(&self, figure_path: &S) -> Result<(), StrError>
    where
        S: AsRef<OsStr> + ?Sized,
    {
        self.run(figure_path, true)
    }

    /// Clears the current axes
    pub fn clear_current_axes(&mut self) -> &mut Self {
        self.buffer.push_str("plt.gca().cla()\n");
        self
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

    /// Sets flag to print python errors (if any) when calling save
    pub fn set_show_errors(&mut self, option: bool) -> &mut Self {
        self.show_errors = option;
        self
    }

    /// Configures subplots
    ///
    /// # Input
    ///
    /// * `row` -- number of rows in the subplot grid
    /// * `col` -- number of columns in the subplot grid
    /// * `index` -- activate current subplot; **indices start at one** (1-based)
    pub fn set_subplot(&mut self, row: usize, col: usize, index: usize) -> &mut Self {
        write!(&mut self.buffer, "\nplt.subplot({},{},{})\n", row, col, index).unwrap();
        self
    }

    /// Configures subplots using GridSpec
    ///
    /// # Input
    ///
    /// * `grid_handle` -- an identifier for GridSpec to be used later with [Plot::set_subplot_grid]
    /// * `row` -- number of rows in the grid
    /// * `col` -- number of columns in the grid
    /// * `options` -- (may be empty) Comma separated options. Example `"wspace=0,hspace=0.35"`.
    ///    See <https://matplotlib.org/stable/api/_as_gen/matplotlib.gridspec.GridSpec.html>
    pub fn set_gridspec(&mut self, grid_handle: &str, row: usize, col: usize, options: &str) -> &mut Self {
        write!(
            &mut self.buffer,
            "grid_{}=plt.GridSpec({},{},{})\n",
            grid_handle, row, col, options
        )
        .unwrap();
        self
    }

    /// Sets a subplot configured via GridSpec
    ///
    /// See function [Plot::set_gridspec]
    ///
    /// # Input
    ///
    /// * `grid_handle` -- an identifier for GridSpec defined by [Plot::set_gridspec]
    /// * `i_range` -- the **zero-based** row index or range such as "0" or "0:2"
    /// * `j_range` -- the **zero-based** column index or range such as "0" or "0:2"
    pub fn set_subplot_grid(&mut self, grid_handle: &str, i_range: &str, j_range: &str) -> &mut Self {
        write!(
            &mut self.buffer,
            "\nplt.subplot(grid_{}[{},{}])\n",
            grid_handle, i_range, j_range
        )
        .unwrap();
        self
    }

    /// Sets the rotation of ticks along the x-axis
    pub fn set_rotation_ticks_x(&mut self, rotation: f64) -> &mut Self {
        write!(
            &mut self.buffer,
            "plt.gca().tick_params(axis='x',rotation={})\n",
            rotation
        )
        .unwrap();
        self
    }

    /// Sets the rotation of ticks along the y-axis
    pub fn set_rotation_ticks_y(&mut self, rotation: f64) -> &mut Self {
        write!(
            &mut self.buffer,
            "plt.gca().tick_params(axis='y',rotation={})\n",
            rotation
        )
        .unwrap();
        self
    }

    /// Aligns the labels when using subplots
    pub fn set_align_labels(&mut self) -> &mut Self {
        write!(&mut self.buffer, "plt.gcf().align_labels()\n").unwrap();
        self
    }

    /// Adds a title to the plot or sub-plot
    pub fn set_title(&mut self, title: &str) -> &mut Self {
        write!(&mut self.buffer, "plt.title(r'{}')\n", title).unwrap();
        self
    }

    /// Adds a title to all sub-plots
    pub fn set_super_title(&mut self, title: &str) -> &mut Self {
        write!(&mut self.buffer, "st=plt.suptitle(r'{}')\nadd_to_ea(st)\n", title).unwrap();
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
            self.buffer.push_str("set_equal_axes()\n");
        } else {
            self.buffer.push_str("plt.gca().axes.set_aspect('auto')\n");
        }
        self
    }

    /// Sets the figure size in inches
    pub fn set_figure_size_inches(&mut self, width: f64, height: f64) -> &mut Self {
        write!(&mut self.buffer, "plt.gcf().set_size_inches({},{})\n", width, height).unwrap();
        self
    }

    /// Sets the figure size in points
    #[rustfmt::skip]
    pub fn set_figure_size_points(&mut self, width: f64, height: f64) -> &mut Self {
        const FACTOR: f64 = 72.27;
        write!(&mut self.buffer, "plt.gcf().set_size_inches({},{})\n", width / FACTOR, height / FACTOR).unwrap();
        self
    }

    /// Set option to hide axes
    pub fn set_hide_axes(&mut self, hide: bool) -> &mut Self {
        let option = if hide { "off" } else { "on" };
        write!(&mut self.buffer, "plt.axis('{}')\n", option).unwrap();
        self
    }

    /// Sets axes limits
    pub fn set_range_3d(&mut self, xmin: f64, xmax: f64, ymin: f64, ymax: f64, zmin: f64, zmax: f64) -> &mut Self {
        write!(
            &mut self.buffer,
            "maybe_create_ax3d()\n\
             AX3D.set_xlim3d({},{})\n\
             AX3D.set_ylim3d({},{})\n\
             AX3D.set_zlim3d({},{})\n",
            xmin, xmax, ymin, ymax, zmin, zmax,
        )
        .unwrap();
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

    /// Sets the number and format of x-ticks
    ///
    /// # Input
    ///
    /// * `major_every` -- step for major ticks (ignored if ≤ 0.0)
    /// * `minor_every` -- step for major ticks (ignored if ≤ 0.0)
    /// * `major_number_format` -- C-style number format for major ticks; e.g. "%.2f" (ignored if empty "")
    ///    See [matplotlib FormatStrFormatter](https://matplotlib.org/stable/api/ticker_api.html#matplotlib.ticker.FormatStrFormatter)
    #[rustfmt::skip]
    pub fn set_ticks_x(&mut self, major_every: f64, minor_every: f64, major_number_format: &str) -> &mut Self {
        if major_every > 0.0 {
            write!(&mut self.buffer, "major_locator = tck.MultipleLocator({})\n", major_every).unwrap();
            write!(&mut self.buffer, "n_ticks = (plt.gca().axis()[1] - plt.gca().axis()[0]) / {}\n", major_every).unwrap();
            write!(&mut self.buffer, "if n_ticks < major_locator.MAXTICKS * 0.9:\n").unwrap();
            write!(&mut self.buffer, "    plt.gca().xaxis.set_major_locator(major_locator)\n").unwrap();
        }
        if minor_every > 0.0 {
            write!(&mut self.buffer, "minor_locator = tck.MultipleLocator({})\n", minor_every).unwrap();
            write!(&mut self.buffer, "n_ticks = (plt.gca().axis()[1] - plt.gca().axis()[0]) / {}\n", minor_every).unwrap();
            write!(&mut self.buffer, "if n_ticks < minor_locator.MAXTICKS * 0.9:\n").unwrap();
            write!(&mut self.buffer, "    plt.gca().xaxis.set_minor_locator(minor_locator)\n").unwrap();
        }
        if major_number_format != "" {
            write!(&mut self.buffer, "major_formatter = tck.FormatStrFormatter(r'{}')\n", major_number_format).unwrap();
            write!(&mut self.buffer, "plt.gca().xaxis.set_major_formatter(major_formatter)\n").unwrap();
        }
        self
    }

    /// Sets the number and format of y-ticks
    ///
    /// # Input
    ///
    /// * `major_every` -- step for major ticks (ignored if ≤ 0.0)
    /// * `minor_every` -- step for major ticks (ignored if ≤ 0.0)
    /// * `major_number_format` -- C-style number format for major ticks; e.g. "%.2f" (ignored if empty "")
    ///    See [matplotlib FormatStrFormatter](https://matplotlib.org/stable/api/ticker_api.html#matplotlib.ticker.FormatStrFormatter)
    #[rustfmt::skip]
    pub fn set_ticks_y(&mut self, major_every: f64, minor_every: f64, major_number_format: &str) -> &mut Self {
        if major_every > 0.0 {
            write!(&mut self.buffer, "major_locator = tck.MultipleLocator({})\n", major_every).unwrap();
            write!(&mut self.buffer, "n_ticks = (plt.gca().axis()[3] - plt.gca().axis()[2]) / {}\n", major_every).unwrap();
            write!(&mut self.buffer, "if n_ticks < major_locator.MAXTICKS * 0.9:\n").unwrap();
            write!(&mut self.buffer, "    plt.gca().yaxis.set_major_locator(major_locator)\n").unwrap();
        }
        if minor_every > 0.0 {
            write!(&mut self.buffer, "minor_locator = tck.MultipleLocator({})\n", minor_every).unwrap();
            write!(&mut self.buffer, "n_ticks = (plt.gca().axis()[3] - plt.gca().axis()[2]) / {}\n", minor_every).unwrap();
            write!(&mut self.buffer, "if n_ticks < minor_locator.MAXTICKS * 0.9:\n").unwrap();
            write!(&mut self.buffer, "    plt.gca().yaxis.set_minor_locator(minor_locator)\n").unwrap();
        }
        if major_number_format != "" {
            write!(&mut self.buffer, "major_formatter = tck.FormatStrFormatter(r'{}')\n", major_number_format).unwrap();
            write!(&mut self.buffer, "plt.gca().yaxis.set_major_formatter(major_formatter)\n").unwrap();
        }
        self
    }

    /// Writes the function multiple_of_pi_formatter to buffer
    #[inline]
    fn write_multiple_of_pi_formatter(&mut self) {
        write!(
            &mut self.buffer,
            "def multiple_of_pi_formatter(x, pos):\n\
             \x20\x20\x20\x20den = 2\n\
             \x20\x20\x20\x20num = int(np.rint(den*x/np.pi))\n\
             \x20\x20\x20\x20com = np.gcd(num,den)\n\
             \x20\x20\x20\x20(num,den) = (int(num/com),int(den/com))\n\
             \x20\x20\x20\x20if den==1:\n\
             \x20\x20\x20\x20\x20\x20\x20\x20if num==0: return r'$0$'\n\
             \x20\x20\x20\x20\x20\x20\x20\x20if num==1: return r'$\\pi$'\n\
             \x20\x20\x20\x20\x20\x20\x20\x20elif num==-1: return r'$-\\pi$'\n\
             \x20\x20\x20\x20\x20\x20\x20\x20else: return r'$%s\\pi$'%num\n\
             \x20\x20\x20\x20else:\n\
             \x20\x20\x20\x20\x20\x20\x20\x20if num==1: return r'$\\frac{{\\pi}}{{%s}}$'%den\n\
             \x20\x20\x20\x20\x20\x20\x20\x20elif num==-1: return r'$\\frac{{-\\pi}}{{%s}}$'%den\n\
             \x20\x20\x20\x20\x20\x20\x20\x20else: return r'$\\frac{{%s\\pi}}{{%s}}$'%(num,den)\n"
        )
        .unwrap();
    }

    /// Sets the x-ticks to multiples of pi
    ///
    /// # Input
    ///
    /// * `minor_every` -- step for major ticks (ignored if ≤ 0.0). Example `PI / 12.0`
    ///
    /// **Note:** This function sets the major ticks as `PI / 2.0`.
    #[rustfmt::skip]
    pub fn set_ticks_x_multiple_of_pi(&mut self, minor_every: f64) -> &mut Self {
        write!(&mut self.buffer, "major_locator = tck.MultipleLocator(np.pi/2.0)\n").unwrap();
        write!(&mut self.buffer, "n_ticks = (plt.gca().axis()[1] - plt.gca().axis()[0]) / (np.pi/2.0)\n").unwrap();
        write!(&mut self.buffer, "if n_ticks < major_locator.MAXTICKS * 0.9:\n").unwrap();
        write!(&mut self.buffer, "    plt.gca().xaxis.set_major_locator(major_locator)\n").unwrap();
        if minor_every > 0.0 {
            write!(&mut self.buffer, "minor_locator = tck.MultipleLocator({})\n", minor_every).unwrap();
            write!(&mut self.buffer, "n_ticks = (plt.gca().axis()[1] - plt.gca().axis()[0]) / {}\n", minor_every).unwrap();
            write!(&mut self.buffer, "if n_ticks < minor_locator.MAXTICKS * 0.9:\n").unwrap();
            write!(&mut self.buffer, "    plt.gca().xaxis.set_minor_locator(minor_locator)\n").unwrap();
        }
        self.write_multiple_of_pi_formatter();
        write!(&mut self.buffer, "major_formatter = tck.FuncFormatter(multiple_of_pi_formatter)\n").unwrap();
        write!(&mut self.buffer, "plt.gca().xaxis.set_major_formatter(major_formatter)\n").unwrap();
        self
    }

    /// Sets the y-ticks to multiples of pi
    ///
    /// # Input
    ///
    /// * `minor_every` -- step for major ticks (ignored if ≤ 0.0). Example `PI / 12.0`
    ///
    /// **Note:** This function sets the major ticks as `PI / 2.0`.
    #[rustfmt::skip]
    pub fn set_ticks_y_multiple_of_pi(&mut self, minor_every: f64) -> &mut Self {
        write!(&mut self.buffer, "major_locator = tck.MultipleLocator(np.pi/2.0)\n").unwrap();
        write!(&mut self.buffer, "n_ticks = (plt.gca().axis()[3] - plt.gca().axis()[2]) / (np.pi/2.0)\n").unwrap();
        write!(&mut self.buffer, "if n_ticks < major_locator.MAXTICKS * 0.9:\n").unwrap();
        write!(&mut self.buffer, "    plt.gca().yaxis.set_major_locator(major_locator)\n").unwrap();
        if minor_every > 0.0 {
            write!(&mut self.buffer, "minor_locator = tck.MultipleLocator({})\n", minor_every).unwrap();
            write!(&mut self.buffer, "n_ticks = (plt.gca().axis()[3] - plt.gca().axis()[2]) / {}\n", minor_every).unwrap();
            write!(&mut self.buffer, "if n_ticks < minor_locator.MAXTICKS * 0.9:\n").unwrap();
            write!(&mut self.buffer, "    plt.gca().yaxis.set_minor_locator(minor_locator)\n").unwrap();
        }
        self.write_multiple_of_pi_formatter();
        write!(&mut self.buffer, "major_formatter = tck.FuncFormatter(multiple_of_pi_formatter)\n").unwrap();
        write!(&mut self.buffer, "plt.gca().yaxis.set_major_formatter(major_formatter)\n").unwrap();
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

    /// Sets inverted x-axis
    pub fn set_inv_x(&mut self) -> &mut Self {
        write!(&mut self.buffer, "plt.gca().invert_xaxis()\n").unwrap();
        self
    }

    /// Sets inverted y-axis
    pub fn set_inv_y(&mut self) -> &mut Self {
        write!(&mut self.buffer, "plt.gca().invert_yaxis()\n").unwrap();
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

    /// Writes extra python commands
    pub fn extra(&mut self, commands: &str) -> &mut Self {
        self.buffer.write_str(commands).unwrap();
        self
    }

    /// Run python
    fn run<S>(&self, figure_path: &S, show: bool) -> Result<(), StrError>
    where
        S: AsRef<OsStr> + ?Sized,
    {
        // update commands
        let fig_path = Path::new(figure_path);
        let txt = if show {
            "plt.savefig(fn,bbox_inches='tight',bbox_extra_artists=EXTRA_ARTISTS)\nplt.show()\n"
        } else {
            "plt.savefig(fn,bbox_inches='tight',bbox_extra_artists=EXTRA_ARTISTS)\n"
        };
        let commands = format!("{}\nfn=r'{}'\n{}", self.buffer, fig_path.to_string_lossy(), txt);

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
            if self.show_errors {
                println!("{}", output);
            }
            return Err("python3 failed; please see the log file");
        }
        Ok(())
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
    fn save_works() {
        let plot = Plot::new();
        assert_eq!(plot.buffer.len(), 0);
        let path = Path::new(OUT_DIR).join("save_works.svg");
        plot.save(&path).unwrap();
        let file = File::open(&path).map_err(|_| "cannot open file").unwrap();
        let buffered = BufReader::new(file);
        let lines_iter = buffered.lines();
        assert!(lines_iter.count() > 20);
    }

    #[test]
    fn save_str_works() {
        let plot = Plot::new();
        assert_eq!(plot.buffer.len(), 0);
        let path = "/tmp/plotpy/unit_tests/save_str_works.svg";
        plot.save(&path).unwrap();
        let file = File::open(&path).map_err(|_| "cannot open file").unwrap();
        let buffered = BufReader::new(file);
        let lines_iter = buffered.lines();
        assert!(lines_iter.count() > 20);
    }

    #[test]
    fn show_errors_works() {
        const WRONG: usize = 0;
        let mut plot = Plot::new();
        plot.set_show_errors(true);
        plot.set_subplot(1, 1, WRONG);
        let path = Path::new(OUT_DIR).join("show_errors_works.svg");
        assert_eq!(plot.save(&path).err(), Some("python3 failed; please see the log file"));
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
                       add_to_ea(st)\n\
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
                       \x20\x20\x20\x20add_to_ea(leg)\n\
                       h,l=plt.gca().get_legend_handles_labels()\n\
                       if len(h)>0 and len(l)>0:\n\
                       \x20\x20\x20\x20leg=plt.legend(handlelength=3,ncol=1,loc='best')\n\
                       \x20\x20\x20\x20add_to_ea(leg)\n";
        assert_eq!(plot.buffer, b);
    }

    #[test]
    fn set_functions_work() {
        let mut plot = Plot::new();
        plot.set_show_errors(true)
            .set_title("my plot")
            .set_equal_axes(true)
            .set_equal_axes(false)
            .set_hide_axes(true)
            .set_range_3d(-1.0, 1.0, -1.0, 1.0, -1.0, 1.0)
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
            .set_ticks_x(1.5, 0.5, "%.2f")
            .set_ticks_y(0.5, 0.1, "%g")
            .set_figure_size_inches(2.0, 2.0)
            .set_figure_size_points(7227.0, 7227.0)
            .clear_current_axes()
            .clear_current_figure();
        let b: &str = "plt.title(r'my plot')\n\
                       set_equal_axes()\n\
                       plt.gca().axes.set_aspect('auto')\n\
                       plt.axis('off')\n\
                       maybe_create_ax3d()\n\
                       AX3D.set_xlim3d(-1,1)\n\
                       AX3D.set_ylim3d(-1,1)\n\
                       AX3D.set_zlim3d(-1,1)\n\
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
                       major_locator = tck.MultipleLocator(1.5)\n\
                       n_ticks = (plt.gca().axis()[1] - plt.gca().axis()[0]) / 1.5\n\
                       if n_ticks < major_locator.MAXTICKS * 0.9:\n\
                       \x20\x20\x20\x20plt.gca().xaxis.set_major_locator(major_locator)\n\
                       minor_locator = tck.MultipleLocator(0.5)\n\
                       n_ticks = (plt.gca().axis()[1] - plt.gca().axis()[0]) / 0.5\n\
                       if n_ticks < minor_locator.MAXTICKS * 0.9:\n\
                       \x20\x20\x20\x20plt.gca().xaxis.set_minor_locator(minor_locator)\n\
                       major_formatter = tck.FormatStrFormatter(r'%.2f')\n\
                       plt.gca().xaxis.set_major_formatter(major_formatter)\n\
                       major_locator = tck.MultipleLocator(0.5)\n\
                       n_ticks = (plt.gca().axis()[3] - plt.gca().axis()[2]) / 0.5\n\
                       if n_ticks < major_locator.MAXTICKS * 0.9:\n\
                       \x20\x20\x20\x20plt.gca().yaxis.set_major_locator(major_locator)\n\
                       minor_locator = tck.MultipleLocator(0.1)\n\
                       n_ticks = (plt.gca().axis()[3] - plt.gca().axis()[2]) / 0.1\n\
                       if n_ticks < minor_locator.MAXTICKS * 0.9:\n\
                       \x20\x20\x20\x20plt.gca().yaxis.set_minor_locator(minor_locator)\n\
                       major_formatter = tck.FormatStrFormatter(r'%g')\n\
                       plt.gca().yaxis.set_major_formatter(major_formatter)\n\
                       plt.gcf().set_size_inches(2,2)\n\
                       plt.gcf().set_size_inches(100,100)\n\
                       plt.gca().cla()\n\
                       plt.clf()\n";
        assert_eq!(plot.buffer, b);
        assert_eq!(plot.show_errors, true);
    }

    #[test]
    fn set_functions_work_2() {
        let mut plot = Plot::new();
        plot.set_ticks_x_multiple_of_pi(0.0);
        let b: &str = "major_locator = tck.MultipleLocator(np.pi/2.0)\n\
                       n_ticks = (plt.gca().axis()[1] - plt.gca().axis()[0]) / (np.pi/2.0)\n\
                       if n_ticks < major_locator.MAXTICKS * 0.9:\n\
                       \x20\x20\x20\x20plt.gca().xaxis.set_major_locator(major_locator)\n\
                       def multiple_of_pi_formatter(x, pos):\n\
                       \x20\x20\x20\x20den = 2\n\
                       \x20\x20\x20\x20num = int(np.rint(den*x/np.pi))\n\
                       \x20\x20\x20\x20com = np.gcd(num,den)\n\
                       \x20\x20\x20\x20(num,den) = (int(num/com),int(den/com))\n\
                       \x20\x20\x20\x20if den==1:\n\
                       \x20\x20\x20\x20\x20\x20\x20\x20if num==0: return r'$0$'\n\
                       \x20\x20\x20\x20\x20\x20\x20\x20if num==1: return r'$\\pi$'\n\
                       \x20\x20\x20\x20\x20\x20\x20\x20elif num==-1: return r'$-\\pi$'\n\
                       \x20\x20\x20\x20\x20\x20\x20\x20else: return r'$%s\\pi$'%num\n\
                       \x20\x20\x20\x20else:\n\
                       \x20\x20\x20\x20\x20\x20\x20\x20if num==1: return r'$\\frac{\\pi}{%s}$'%den\n\
                       \x20\x20\x20\x20\x20\x20\x20\x20elif num==-1: return r'$\\frac{-\\pi}{%s}$'%den\n\
                       \x20\x20\x20\x20\x20\x20\x20\x20else: return r'$\\frac{%s\\pi}{%s}$'%(num,den)\n\
                       major_formatter = tck.FuncFormatter(multiple_of_pi_formatter)\n\
                       plt.gca().xaxis.set_major_formatter(major_formatter)\n";
        assert_eq!(plot.buffer, b);

        let mut plot = Plot::new();
        plot.set_ticks_y_multiple_of_pi(0.0);
        let b: &str = "major_locator = tck.MultipleLocator(np.pi/2.0)\n\
                       n_ticks = (plt.gca().axis()[3] - plt.gca().axis()[2]) / (np.pi/2.0)\n\
                       if n_ticks < major_locator.MAXTICKS * 0.9:\n\
                       \x20\x20\x20\x20plt.gca().yaxis.set_major_locator(major_locator)\n\
                       def multiple_of_pi_formatter(x, pos):\n\
                       \x20\x20\x20\x20den = 2\n\
                       \x20\x20\x20\x20num = int(np.rint(den*x/np.pi))\n\
                       \x20\x20\x20\x20com = np.gcd(num,den)\n\
                       \x20\x20\x20\x20(num,den) = (int(num/com),int(den/com))\n\
                       \x20\x20\x20\x20if den==1:\n\
                       \x20\x20\x20\x20\x20\x20\x20\x20if num==0: return r'$0$'\n\
                       \x20\x20\x20\x20\x20\x20\x20\x20if num==1: return r'$\\pi$'\n\
                       \x20\x20\x20\x20\x20\x20\x20\x20elif num==-1: return r'$-\\pi$'\n\
                       \x20\x20\x20\x20\x20\x20\x20\x20else: return r'$%s\\pi$'%num\n\
                       \x20\x20\x20\x20else:\n\
                       \x20\x20\x20\x20\x20\x20\x20\x20if num==1: return r'$\\frac{\\pi}{%s}$'%den\n\
                       \x20\x20\x20\x20\x20\x20\x20\x20elif num==-1: return r'$\\frac{-\\pi}{%s}$'%den\n\
                       \x20\x20\x20\x20\x20\x20\x20\x20else: return r'$\\frac{%s\\pi}{%s}$'%(num,den)\n\
                       major_formatter = tck.FuncFormatter(multiple_of_pi_formatter)\n\
                       plt.gca().yaxis.set_major_formatter(major_formatter)\n";
        assert_eq!(plot.buffer, b);

        let mut plot = Plot::new();
        plot.set_ticks_x_multiple_of_pi(1.0);
        let b: &str = "major_locator = tck.MultipleLocator(np.pi/2.0)\n\
                       n_ticks = (plt.gca().axis()[1] - plt.gca().axis()[0]) / (np.pi/2.0)\n\
                       if n_ticks < major_locator.MAXTICKS * 0.9:\n\
                       \x20\x20\x20\x20plt.gca().xaxis.set_major_locator(major_locator)\n\
                       minor_locator = tck.MultipleLocator(1)\n\
                       n_ticks = (plt.gca().axis()[1] - plt.gca().axis()[0]) / 1\n\
                       if n_ticks < minor_locator.MAXTICKS * 0.9:\n\
                       \x20\x20\x20\x20plt.gca().xaxis.set_minor_locator(minor_locator)\n\
                       def multiple_of_pi_formatter(x, pos):\n\
                       \x20\x20\x20\x20den = 2\n\
                       \x20\x20\x20\x20num = int(np.rint(den*x/np.pi))\n\
                       \x20\x20\x20\x20com = np.gcd(num,den)\n\
                       \x20\x20\x20\x20(num,den) = (int(num/com),int(den/com))\n\
                       \x20\x20\x20\x20if den==1:\n\
                       \x20\x20\x20\x20\x20\x20\x20\x20if num==0: return r'$0$'\n\
                       \x20\x20\x20\x20\x20\x20\x20\x20if num==1: return r'$\\pi$'\n\
                       \x20\x20\x20\x20\x20\x20\x20\x20elif num==-1: return r'$-\\pi$'\n\
                       \x20\x20\x20\x20\x20\x20\x20\x20else: return r'$%s\\pi$'%num\n\
                       \x20\x20\x20\x20else:\n\
                       \x20\x20\x20\x20\x20\x20\x20\x20if num==1: return r'$\\frac{\\pi}{%s}$'%den\n\
                       \x20\x20\x20\x20\x20\x20\x20\x20elif num==-1: return r'$\\frac{-\\pi}{%s}$'%den\n\
                       \x20\x20\x20\x20\x20\x20\x20\x20else: return r'$\\frac{%s\\pi}{%s}$'%(num,den)\n\
                       major_formatter = tck.FuncFormatter(multiple_of_pi_formatter)\n\
                       plt.gca().xaxis.set_major_formatter(major_formatter)\n";
        assert_eq!(plot.buffer, b);

        let mut plot = Plot::new();
        plot.set_ticks_y_multiple_of_pi(1.0);
        let b: &str = "major_locator = tck.MultipleLocator(np.pi/2.0)\n\
                       n_ticks = (plt.gca().axis()[3] - plt.gca().axis()[2]) / (np.pi/2.0)\n\
                       if n_ticks < major_locator.MAXTICKS * 0.9:\n\
                       \x20\x20\x20\x20plt.gca().yaxis.set_major_locator(major_locator)\n\
                       minor_locator = tck.MultipleLocator(1)\n\
                       n_ticks = (plt.gca().axis()[3] - plt.gca().axis()[2]) / 1\n\
                       if n_ticks < minor_locator.MAXTICKS * 0.9:\n\
                       \x20\x20\x20\x20plt.gca().yaxis.set_minor_locator(minor_locator)\n\
                       def multiple_of_pi_formatter(x, pos):\n\
                       \x20\x20\x20\x20den = 2\n\
                       \x20\x20\x20\x20num = int(np.rint(den*x/np.pi))\n\
                       \x20\x20\x20\x20com = np.gcd(num,den)\n\
                       \x20\x20\x20\x20(num,den) = (int(num/com),int(den/com))\n\
                       \x20\x20\x20\x20if den==1:\n\
                       \x20\x20\x20\x20\x20\x20\x20\x20if num==0: return r'$0$'\n\
                       \x20\x20\x20\x20\x20\x20\x20\x20if num==1: return r'$\\pi$'\n\
                       \x20\x20\x20\x20\x20\x20\x20\x20elif num==-1: return r'$-\\pi$'\n\
                       \x20\x20\x20\x20\x20\x20\x20\x20else: return r'$%s\\pi$'%num\n\
                       \x20\x20\x20\x20else:\n\
                       \x20\x20\x20\x20\x20\x20\x20\x20if num==1: return r'$\\frac{\\pi}{%s}$'%den\n\
                       \x20\x20\x20\x20\x20\x20\x20\x20elif num==-1: return r'$\\frac{-\\pi}{%s}$'%den\n\
                       \x20\x20\x20\x20\x20\x20\x20\x20else: return r'$\\frac{%s\\pi}{%s}$'%(num,den)\n\
                       major_formatter = tck.FuncFormatter(multiple_of_pi_formatter)\n\
                       plt.gca().yaxis.set_major_formatter(major_formatter)\n";
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

    #[test]
    fn additional_features_work() {
        let mut plot = Plot::new();
        plot.set_inv_x().set_inv_y().extra("plt.show()\n");
        let b: &str = "plt.gca().invert_xaxis()\n\
                       plt.gca().invert_yaxis()\n\
                       plt.show()\n";
        assert_eq!(plot.buffer, b);
    }

    #[test]
    fn gridspec_functions_work() {
        let mut plot = Plot::new();
        plot.set_gridspec("the_grid", 2, 2, "wspace=0.1,hspace=0.2")
            .set_subplot_grid("the_grid", "0:2", "0")
            .set_rotation_ticks_x(55.0)
            .set_rotation_ticks_y(45.0)
            .set_align_labels();
        let b: &str = "grid_the_grid=plt.GridSpec(2,2,wspace=0.1,hspace=0.2)\n\
                       \nplt.subplot(grid_the_grid[0:2,0])\n\
                       plt.gca().tick_params(axis='x',rotation=55)\n\
                       plt.gca().tick_params(axis='y',rotation=45)\n\
                       plt.gcf().align_labels()\n";
        assert_eq!(plot.buffer, b);
    }
}
