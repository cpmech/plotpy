use super::*;
use std::fmt::Write;

/// Generates a Histogram plot
pub struct Histogram {
    /// Colors for each bar
    pub colors: Vec<String>,

    /// Type of histogram; e.g. "bar"
    ///
    /// * `bar` is a traditional bar-type histogram. If multiple data are given the bars are arranged side by side.
    /// * `barstacked` is a bar-type histogram where multiple data are stacked on top of each other.
    /// * `step` generates a lineplot that is by default unfilled.
    /// * `stepfilled` generates a lineplot that is by default filled.
    ///
    /// As defined in <https://matplotlib.org/stable/api/_as_gen/matplotlib.pyplot.hist.html>
    pub style: String,

    /// Draws stacked histogram
    pub stacked: bool,

    /// Skip filling bars
    pub no_fill: bool,

    /// Number of bins
    pub number_bins: i32,

    // buffer
    pub(crate) buffer: String,
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

    /// Returns options for histogram
    pub(crate) fn options(&self) -> String {
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
        histogram.stacked = true;
        let opt = histogram.options();
        assert_eq!(opt, ",stacked=True");
    }

    #[test]
    fn draw_works() {
        let values = vec![vec![1, 1, 1, 2, 2, 2, 2, 2, 3, 3], vec![5, 6, 7, 8]];
        let labels = ["first".to_string(), "second".to_string()];
        let mut histogram = Histogram::new();
        histogram.colors = vec!["red".to_string(), "green".to_string()];
        histogram.draw(&values, &labels);
        let b: &str = "values=[[1,1,1,2,2,2,2,2,3,3,],[5,6,7,8,],]\n\
                       labels=['first','second',]\n\
                       colors=['red','green',]\n\
                       plt.hist(values,label=labels,color=colors)\n";
        assert_eq!(histogram.buffer, b);
    }
}
