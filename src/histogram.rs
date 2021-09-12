use super::*;
use std::fmt::Write;

/// Generates a Histogram plot
pub struct Histogram {
    pub colors: Vec<String>, // colors
    pub style: String,       // type; e.g. "bar"
    pub stacked: bool,       // stacked
    pub no_fill: bool,       // do not fill bars
    pub number_bins: i32,    // number of bins
    pub normalized: bool,    // normed

    // buffer
    pub(crate) buffer: String,
}

impl Histogram {
    pub fn new() -> Self {
        Histogram {
            colors: Vec::new(),
            style: String::new(),
            stacked: false,
            no_fill: false,
            number_bins: 0,
            normalized: false,
            buffer: String::new(),
        }
    }

    pub(crate) fn options(&self) -> String {
        let mut opt = String::new();
        if self.colors.len() > 0 {
            write!(&mut opt, ",color={}", vec_to_py_list_str(&self.colors)).unwrap();
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
        if self.normalized {
            write!(&mut opt, ",normed=True").unwrap();
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
    }

    #[test]
    fn options_works() {
        let mut histogram = Histogram::new();
        histogram.stacked = true;
        let opt = histogram.options();
        assert_eq!(opt, ",stacked=True");
    }
}
