use super::*;
use std::fmt::Write;
use std::fs::File;
use std::io::Write as IoWrite;
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
    pub(crate) buffer: String,
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

    /// Configures subplots
    ///
    /// # Arguments
    ///
    /// * `row` - number of rows in the subplot grid
    /// * `col` - number of columns in the subplot grid
    /// * `index` - activate current subplot; indices start at one [1-based]
    ///
    pub fn subplot(&mut self, row: i32, col: i32, index: i32) {
        assert!(index > 0);
        write!(&mut self.buffer, "\nplt.subplot({},{},{})\n", row, col, index).unwrap();
    }

    /// Sets the horizontal gap between subplots
    pub fn subplot_horizontal_gap(&mut self, value: f64) {
        write!(&mut self.buffer, "plt.subplots_adjust(hspace={})\n", value).unwrap();
    }

    /// Sets the vertical gap between subplots
    pub fn subplot_vertical_gap(&mut self, value: f64) {
        write!(&mut self.buffer, "plt.subplots_adjust(wspace={})\n", value).unwrap();
    }

    /// Sets the horizontal and vertical gap between subplots
    pub fn subplot_gap(&mut self, horizontal: f64, vertical: f64) {
        write!(
            &mut self.buffer,
            "plt.subplots_adjust(hspace={},wspace={})\n",
            horizontal, vertical
        )
        .unwrap();
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
        write!(&mut self.buffer, "plt.axis([{},{},{},{}])\n", xmin, xmax, ymin, ymax).unwrap();
    }

    /// Sets x and y limits
    pub fn range_vec(&mut self, limits: &[f64]) {
        write!(
            &mut self.buffer,
            "plt.axis([{},{},{},{}])\n",
            limits[0], limits[1], limits[2], limits[3]
        )
        .unwrap();
    }

    /// Sets minimum x
    pub fn xmin(&mut self, xmin: f64) {
        write!(
            &mut self.buffer,
            "plt.axis([{},plt.axis()[1],plt.axis()[2],plt.axis()[3]])\n",
            xmin
        )
        .unwrap();
    }

    /// Sets maximum x
    pub fn xmax(&mut self, xmax: f64) {
        write!(
            &mut self.buffer,
            "plt.axis([plt.axis()[0],{},plt.axis()[2],plt.axis()[3]])\n",
            xmax
        )
        .unwrap();
    }

    /// Sets minimum y
    pub fn ymin(&mut self, ymin: f64) {
        write!(
            &mut self.buffer,
            "plt.axis([plt.axis()[0],plt.axis()[1],{},plt.axis()[3]])\n",
            ymin
        )
        .unwrap();
    }

    /// Sets maximum y
    pub fn ymax(&mut self, ymax: f64) {
        write!(
            &mut self.buffer,
            "plt.axis([plt.axis()[0],plt.axis()[1],plt.axis()[2],{}])\n",
            ymax
        )
        .unwrap();
    }

    /// Sets x-range (i.e. limits)
    pub fn xrange(&mut self, xmin: f64, xmax: f64) {
        write!(
            &mut self.buffer,
            "plt.axis([{},{},plt.axis()[2],plt.axis()[3]])\n",
            xmin, xmax
        )
        .unwrap();
    }

    /// Sets y-range (i.e. limits)
    pub fn yrange(&mut self, ymin: f64, ymax: f64) {
        write!(
            &mut self.buffer,
            "plt.axis([plt.axis()[0],plt.axis()[1],{},{}])\n",
            ymin, ymax
        )
        .unwrap();
    }

    // Sets number of ticks along x
    pub fn xnticks(&mut self, num: i32) {
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
    pub fn ynticks(&mut self, num: i32) {
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

    /// Adds x-label
    pub fn xlabel(&mut self, label: &str) {
        write!(&mut self.buffer, "plt.xlabel(r'{}')\n", label).unwrap();
    }

    /// Adds y-label
    pub fn ylabel(&mut self, label: &str) {
        write!(&mut self.buffer, "plt.ylabel(r'{}')\n", label).unwrap();
    }

    /// Adds labels
    pub fn labels(&mut self, xlabel: &str, ylabel: &str) {
        write!(
            &mut self.buffer,
            "plt.xlabel(r'{}')\nplt.ylabel(r'{}')\n",
            xlabel, ylabel
        )
        .unwrap();
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
        assert_eq!(lines_iter.count(), 33);
        Ok(())
    }

    #[test]
    fn subplot_functions_work() {
        let mut plot = Plot::new();
        plot.subplot(2, 2, 1);
        plot.subplot_horizontal_gap(0.1);
        plot.subplot_vertical_gap(0.2);
        plot.subplot_gap(0.3, 0.4);
        let correct: &str = "\nplt.subplot(2,2,1)\n\
                               plt.subplots_adjust(hspace=0.1)\n\
                               plt.subplots_adjust(wspace=0.2)\n\
                               plt.subplots_adjust(hspace=0.3,wspace=0.4)\n";
        assert_eq!(plot.buffer, correct);
    }

    #[test]
    fn axes_functions_work() {
        let mut plot = Plot::new();
        plot.equal();
        plot.hide_axes();
        plot.range(-1.0, 1.0, -1.0, 1.0);
        plot.range_vec(&[0.0, 1.0, 0.0, 1.0]);
        plot.xmin(0.0);
        plot.xmax(1.0);
        plot.ymin(0.0);
        plot.ymax(1.0);
        plot.xrange(0.0, 1.0);
        plot.yrange(0.0, 1.0);
        plot.xnticks(0);
        plot.xnticks(8);
        plot.ynticks(0);
        plot.ynticks(5);
        plot.xlabel("x-label");
        plot.ylabel("y-label");
        plot.labels("x", "y");
        plot.grid_and_labels("xx", "yy");
        plot.clear_current_figure();
        plot.legend();
        let correct: &str = "plt.axis('equal')\n\
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
                             \x20\x20\x20\x20addToEA(leg)\n";
        assert_eq!(plot.buffer, correct);
    }
}
