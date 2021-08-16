use super::*;

/// Generates a contour plot
pub struct Contour {
    pub colors: Vec<String>,     // colors
    pub levels: Vec<f64>,        // levels (may be nil)
    pub colormap_index: i32,     // colormap index
    pub number_format: String,   // number format
    pub no_lines: bool,          // no lines on top of filled contour
    pub no_labels: bool,         // no labels
    pub no_inline: bool,         // no labels 'inline'
    pub no_colorbar: bool,       // no colorbar
    pub colorbar_label: String,  // colorbar label
    pub selected_value: f64,     // selected value
    pub selected_color: String,  // color to mark selected level
    pub selected_linewidth: f64, // zero level linewidth

    // buffer
    pub(crate) buffer: String,
}

impl Contour {
    pub fn new() -> Self {
        Contour {
            colors: Vec::new(),
            levels: Vec::new(),
            colormap_index: 0,
            number_format: String::new(),
            no_lines: false,
            no_labels: false,
            no_inline: false,
            no_colorbar: false,
            colorbar_label: String::new(),
            selected_value: 0.0,
            selected_color: String::new(),
            selected_linewidth: 0.0,
            buffer: String::new(),
        }
    }

    pub(crate) fn options(&self) -> String {
        let mut options = String::new();
        if self.colors.len() > 0 {
            options.push_str(&format!(",colors={}", array2list(&self.colors)));
        }
        if self.levels.len() > 0 {
            options.push_str(&format!(",levels={}", array2list(&self.levels)));
        }
        options
    }
}

impl GraphMaker for Contour {
    fn get_buffer<'a>(&'a self) -> &'a String {
        &self.buffer
    }
}
