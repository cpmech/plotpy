use super::*;

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
        let mut options = String::new();
        if self.colors.len() > 0 {
            options.push_str(&format!(",color={}", array2list(&self.colors)));
        }
        if self.style != "" {
            options.push_str(&format!(",histtype='{}'", self.style));
        }
        if self.stacked {
            options.push_str(",stacked=True");
        }
        if self.no_fill {
            options.push_str(",fill=False");
        }
        if self.number_bins > 0 {
            options.push_str(&format!(",bins={}", self.number_bins));
        }
        if self.normalized {
            options.push_str(",normed=True");
        }
        options
    }
}

impl GraphMaker for Histogram {
    fn get_buffer<'a>(&'a self) -> &'a String {
        &self.buffer
    }
}
