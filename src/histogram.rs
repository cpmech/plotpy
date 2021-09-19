use super::*;
use std::fmt::Write;

/// Generates a Histogram plot
///
/// # Example
///
/// ```
/// # fn main() -> Result<(), &'static str> {
/// // import
/// use plotpy::*;
/// use std::path::Path;
///
/// // directory to save figures
/// const OUT_DIR: &str = "/tmp/plotpy/doc_tests";
///
/// // set values
/// let values = vec![
///     vec![1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 4, 5, 6], // first series
///     vec![-1, -1, 0, 1, 2, 3],                    // second series
///     vec![5, 6, 7, 8],                            // third series
/// ];
///
/// // set labels
/// let labels = ["first".to_string(), "second".to_string(), "third".to_string()];
///
/// // configure and draw histogram
/// let mut histogram = Histogram::new();
/// histogram.draw(&values, &labels);
///
/// // add histogram to plot
/// let mut plot = Plot::new();
/// plot.add(&histogram);
/// plot.legend();
/// plot.grid_and_labels("values", "count");
///
/// // save figure
/// let path = Path::new(OUT_DIR).join("doc_histogram.svg");
/// plot.save(&path)?;
/// # Ok(())
/// # }
/// ```
///
/// ![doc_histogram.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/doc_histogram.svg)
///
pub struct Histogram {
    colors: Vec<String>, // Colors for each bar
    style: String,       // Type of histogram; e.g. "bar"
    stacked: bool,       // Draws stacked histogram
    no_fill: bool,       // Skip filling bars
    number_bins: i32,    // Number of bins
    buffer: String,      // buffer
}

impl Histogram {
    /// Creates a new Histogram object
    pub fn new() -> Self {
        Histogram {
            colors: Vec::new(),
            style: String::new(),
            stacked: false,
            no_fill: false,
            number_bins: 0,
            buffer: String::new(),
        }
    }

    /// Draws histogram
    ///
    /// # Input
    ///
    /// * `values` - values
    /// * `labels` - labels
    ///
    /// # Notes
    ///
    /// * The type `T` of the input array must be a number.
    ///
    pub fn draw<T>(&mut self, values: &Vec<Vec<T>>, labels: &[String])
    where
        T: std::fmt::Display,
    {
        let opt = self.options();
        matrix_to_list(&mut self.buffer, "values", values);
        vector_to_strings(&mut self.buffer, "labels", labels);
        if self.colors.len() > 0 {
            vector_to_strings(&mut self.buffer, "colors", self.colors.as_slice());
        }
        write!(&mut self.buffer, "plt.hist(values,label=labels{})\n", &opt).unwrap();
    }

    /// Sets the colors for each bar
    pub fn set_colors(&mut self, colors: &[&str]) -> &mut Self {
        self.colors = colors.iter().map(|color| color.to_string()).collect();
        self
    }

    /// Sets the type of histogram
    ///
    /// Options:
    ///
    /// * `bar` is a traditional bar-type histogram. If multiple data are given the bars are arranged side by side.
    /// * `barstacked` is a bar-type histogram where multiple data are stacked on top of each other.
    /// * `step` generates a lineplot that is by default unfilled.
    /// * `stepfilled` generates a lineplot that is by default filled.
    /// * As defined in <https://matplotlib.org/stable/api/_as_gen/matplotlib.pyplot.hist.html>
    pub fn set_style(&mut self, style: &str) -> &mut Self {
        self.style = String::from(style);
        self
    }

    /// Sets option to draw stacked histogram
    pub fn set_stacked(&mut self, flag: bool) -> &mut Self {
        self.stacked = flag;
        self
    }

    /// Sets option to skip filling bars
    pub fn set_no_fill(&mut self, flag: bool) -> &mut Self {
        self.no_fill = flag;
        self
    }

    /// Sets the number of bins
    pub fn set_number_bins(&mut self, bins: i32) -> &mut Self {
        self.number_bins = bins;
        self
    }

    /// Returns options for histogram
    fn options(&self) -> String {
        let mut opt = String::new();
        if self.colors.len() > 0 {
            write!(&mut opt, ",color=colors").unwrap();
        }
        if self.style != "" {
            write!(&mut opt, ",histtype='{}'", self.style).unwrap();
        }
        if self.stacked {
            write!(&mut opt, ",stacked=True").unwrap();
        }
        if self.no_fill {
            write!(&mut opt, ",fill=False").unwrap();
        }
        if self.number_bins > 0 {
            write!(&mut opt, ",bins={}", self.number_bins).unwrap();
        }
        opt
    }
}

impl GraphMaker for Histogram {
    fn get_buffer<'a>(&'a self) -> &'a String {
        &self.buffer
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_works() {
        let histogram = Histogram::new();
        assert_eq!(histogram.colors.len(), 0);
        assert_eq!(histogram.style.len(), 0);
        assert_eq!(histogram.stacked, false);
        assert_eq!(histogram.no_fill, false);
        assert_eq!(histogram.number_bins, 0);
        assert_eq!(histogram.buffer.len(), 0);
    }

    #[test]
    fn options_works() {
        let mut histogram = Histogram::new();
        histogram.set_stacked(true);
        let opt = histogram.options();
        assert_eq!(opt, ",stacked=True");
    }

    #[test]
    fn draw_works() {
        let values = vec![vec![1, 1, 1, 2, 2, 2, 2, 2, 3, 3], vec![5, 6, 7, 8]];
        let labels = ["first".to_string(), "second".to_string()];
        let mut histogram = Histogram::new();
        histogram.set_colors(&vec!["red", "green"]);
        histogram.draw(&values, &labels);
        let b: &str = "values=[[1,1,1,2,2,2,2,2,3,3,],[5,6,7,8,],]\n\
                       labels=['first','second',]\n\
                       colors=['red','green',]\n\
                       plt.hist(values,label=labels,color=colors)\n";
        assert_eq!(histogram.buffer, b);
    }
}
