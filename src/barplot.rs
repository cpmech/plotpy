use super::{generate_list_quoted, vector_to_array, AsVector, GraphMaker};
use num_traits::Num;
use std::fmt::Write;

/// Generates a Barplot plot
///
/// [See Matplotlib's documentation](https://matplotlib.org/stable/api/_as_gen/matplotlib.pyplot.bar.html)
///
/// # Examples
///
/// ## Basic bar plot
///
/// ```
/// use plotpy::{Barplot, Plot, StrError};
/// use std::collections::HashMap;
///
/// fn main() -> Result<(), StrError> {
///     // data
///     let x = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
///     let y = [5, 4, 3, 2, 1, 0, 1, 2, 3, 4];
///
///     // barplot object and options
///     let mut bar = Barplot::new();
///     bar.draw(&x, &y);
///
///     // save figure
///     let mut plot = Plot::new();
///     plot.add(&bar)
///         .save("/tmp/plotpy/doc_tests/doc_barplot_1.svg")?;
///     Ok(())
/// }
/// ```
///
/// ![doc_barplot_1.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/doc_barplot_1.svg)
///
/// ## Using string as labels
///
/// The code below implements the [Bar Label Demo from Matplotlib documentation](https://matplotlib.org/stable/gallery/lines_bars_and_markers/bar_label_demo.html#sphx-glr-gallery-lines-bars-and-markers-bar-label-demo-py)
///
/// ```
/// use plotpy::{Barplot, Plot, StrError};
/// use std::collections::HashMap;
///
/// fn main() -> Result<(), StrError> {
///     // data
///     let species = ["Adelie", "Chinstrap", "Gentoo"];
///     let sex_counts = HashMap::from([
///         ("Male", [73.0, 34.0, 61.0]), //
///         ("Female", [73.0, 34.0, 58.0]),
///     ]);
///
///    // barplot object and options
///    let mut bar = Barplot::new();
///    bar.set_with_text("center");
///
///    // draw bars
///    let mut bottom = [0.0, 0.0, 0.0];
///    for (sex, sex_count) in &sex_counts {
///        bar.set_label(sex)
///            .set_bottom(&bottom)
///            .draw_with_str(&species, sex_count);
///        for i in 0..sex_count.len() {
///            bottom[i] += sex_count[i];
///        }
///    }
///
///     // add barplot to plot and save figure
///     let mut plot = Plot::new();
///     plot.add(&bar)
///         .set_title("Number of penguins by sex")
///         .legend()
///         .save("/tmp/plotpy/doc_tests/doc_barplot_2.svg")?;
///     Ok(())
/// }
/// ```
///
/// ![doc_barplot_2.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/doc_barplot_2.svg)
///
/// ## Horizontal bars
///
/// ```
/// use plotpy::{Barplot, Plot, StrError};
///
/// fn main() -> Result<(), StrError> {
///     // data
///     let fruits = ["Apple", "Banana", "Orange"];
///     let prices = [10.0, 20.0, 30.0];
///     let errors = [3.0, 2.0, 1.0];
///
///     // barplot object and options
///     let mut bar = Barplot::new();
///     bar.set_errors(&errors)
///         .set_horizontal(true)
///         .set_with_text("edge")
///         .draw_with_str(&fruits, &prices);
///
///     // save figure
///     let mut plot = Plot::new();
///     plot.set_inv_y()
///         .add(&bar)
///         .set_title("Fruits")
///         .set_label_x("price")
///         .save("/tmp/plotpy/doc_tests/doc_barplot_3.svg")?;
///     Ok(())
/// }
/// ```
///
/// ![doc_barplot_3.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/doc_barplot_3.svg)
///
/// ## More examples
///
/// See also integration test in the **tests** directory.
pub struct Barplot {
    label: String,             // Name of this bar in the legend
    colors: Vec<String>,       // Colors for each bar
    width: f64,                // Width of the bars
    bottom: Vec<f64>,          // bottom coordinates to stack bars
    with_text: Option<String>, // Text to be added to each bar (aka, bar_label)
    horizontal: bool,          // Horizontal barplot
    errors: Vec<f64>,          // Shows error icons on bars
    extra: String,             // Extra commands (comma separated)
    buffer: String,            // buffer
}

impl Barplot {
    /// Creates a new Barplot object
    pub fn new() -> Self {
        Barplot {
            label: String::new(),
            colors: Vec::new(),
            width: 0.0,
            bottom: Vec::new(),
            with_text: None,
            horizontal: false,
            errors: Vec::new(),
            extra: String::new(),
            buffer: String::new(),
        }
    }

    /// Draws the bar plot
    pub fn draw<'a, T, U>(&mut self, x: &'a T, y: &'a T)
    where
        T: AsVector<'a, U>,
        U: 'a + std::fmt::Display + Num,
    {
        vector_to_array(&mut self.buffer, "x", x);
        vector_to_array(&mut self.buffer, "y", y);
        let opt = self.options();
        if self.colors.len() > 0 {
            generate_list_quoted(&mut self.buffer, "colors", self.colors.as_slice());
        }
        if self.bottom.len() > 0 {
            vector_to_array(&mut self.buffer, "bottom", &self.bottom);
        }
        if self.errors.len() > 0 {
            vector_to_array(&mut self.buffer, "err", &self.errors);
        }
        if self.horizontal {
            write!(&mut self.buffer, "p=plt.barh(x,y{})\n", &opt).unwrap();
        } else {
            write!(&mut self.buffer, "p=plt.bar(x,y{})\n", &opt).unwrap();
        }
        if let Some(t) = &self.with_text {
            write!(&mut self.buffer, "plt.gca().bar_label(p,label_type='{}')\n", t).unwrap();
        }
    }

    /// Draws the bar plot with strings
    pub fn draw_with_str<'a, T, U>(&mut self, x: &[&str], y: &'a T)
    where
        T: AsVector<'a, U>,
        U: 'a + std::fmt::Display + Num,
    {
        generate_list_quoted(&mut self.buffer, "x", x);
        vector_to_array(&mut self.buffer, "y", y);
        let opt = self.options();
        if self.colors.len() > 0 {
            generate_list_quoted(&mut self.buffer, "colors", self.colors.as_slice());
        }
        if self.bottom.len() > 0 {
            vector_to_array(&mut self.buffer, "bottom", &self.bottom);
        }
        if self.errors.len() > 0 {
            vector_to_array(&mut self.buffer, "err", &self.errors);
        }
        if self.horizontal {
            write!(&mut self.buffer, "p=plt.barh(x,y{})\n", &opt).unwrap();
        } else {
            write!(&mut self.buffer, "p=plt.bar(x,y{})\n", &opt).unwrap();
        }
        if let Some(t) = &self.with_text {
            write!(&mut self.buffer, "plt.gca().bar_label(p,label_type='{}')\n", t).unwrap();
        }
    }

    /// Sets the name of this bar in the legend
    pub fn set_label(&mut self, label: &str) -> &mut Self {
        self.label = String::from(label);
        self
    }

    /// Sets the colors for each bar
    pub fn set_colors(&mut self, colors: &[&str]) -> &mut Self {
        self.colors = colors.iter().map(|color| color.to_string()).collect();
        self
    }

    /// Sets the width of the bars
    pub fn set_width(&mut self, width: f64) -> &mut Self {
        self.width = width;
        self
    }

    /// Sets the vertical offset to stack bars
    pub fn set_bottom(&mut self, bottom: &[f64]) -> &mut Self {
        self.bottom = Vec::from(bottom);
        self
    }

    /// Sets an option to show the text (labels) of each bar
    ///
    /// # Input
    ///
    /// `position` -- "edge" or "center"; Use "" to remove the label
    ///
    /// See [Matplotlib documentation](https://matplotlib.org/stable/api/_as_gen/matplotlib.axes.Axes.bar_label.html#matplotlib.axes.Axes.bar_label)
    pub fn set_with_text(&mut self, position: &str) -> &mut Self {
        if position == "" {
            self.with_text = None
        } else {
            self.with_text = Some(position.to_string());
        }
        self
    }

    /// Enables drawing horizontal bars
    pub fn set_horizontal(&mut self, flag: bool) -> &mut Self {
        self.horizontal = flag;
        self
    }

    /// Enables error indicators
    pub fn set_errors(&mut self, errors: &[f64]) -> &mut Self {
        self.errors = errors.to_vec();
        self
    }

    /// Sets extra matplotlib commands (comma separated)
    ///
    /// **Important:** The extra commands must be comma separated. For example:
    ///
    /// ```text
    /// param1=123,param2='hello'
    /// ```
    ///
    /// [See Matplotlib's documentation for extra parameters](https://matplotlib.org/stable/api/_as_gen/matplotlib.pyplot.bar.html)
    pub fn set_extra(&mut self, extra: &str) -> &mut Self {
        self.extra = extra.to_string();
        self
    }

    /// Returns options for barplot
    fn options(&self) -> String {
        let mut opt = String::new();
        if self.label != "" {
            write!(&mut opt, ",label=r'{}'", self.label).unwrap();
        }
        if self.colors.len() > 0 {
            write!(&mut opt, ",color=colors").unwrap();
        }
        if self.width > 0.0 {
            write!(&mut opt, ",width={}", self.width).unwrap();
        }
        if self.bottom.len() > 0 {
            write!(&mut opt, ",bottom=bottom").unwrap();
        }
        if self.errors.len() > 0 {
            if self.horizontal {
                write!(&mut opt, ",xerr=err").unwrap();
            } else {
                write!(&mut opt, ",yerr=err").unwrap();
            }
        }
        if self.extra != "" {
            write!(&mut opt, ",{}", self.extra).unwrap();
        }
        opt
    }
}

impl GraphMaker for Barplot {
    fn get_buffer<'a>(&'a self) -> &'a String {
        &self.buffer
    }
    fn clear_buffer(&mut self) {
        self.buffer.clear();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Barplot;
    use crate::GraphMaker;

    #[test]
    fn new_works() {
        let barplot = Barplot::new();
        assert_eq!(barplot.label.len(), 0);
        assert_eq!(barplot.colors.len(), 0);
        assert_eq!(barplot.width, 0.0);
        assert_eq!(barplot.bottom.len(), 0);
        assert_eq!(barplot.with_text, None);
        assert_eq!(barplot.buffer.len(), 0);
    }

    #[test]
    fn draw_works_1() {
        let xx = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let yy = [5, 4, 3, 2, 1, 0, 1, 2, 3, 4];
        let mut bar = Barplot::new();
        bar.draw(&xx, &yy);
        let b: &str = "x=np.array([0,1,2,3,4,5,6,7,8,9,],dtype=float)\n\
                       y=np.array([5,4,3,2,1,0,1,2,3,4,],dtype=float)\n\
                       p=plt.bar(x,y)\n";
        assert_eq!(bar.buffer, b);
        bar.clear_buffer();
        assert_eq!(bar.buffer, "");
    }

    #[test]
    fn draw_works_2() {
        let xx = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let yy = [5, 4, 3, 2, 1, 0, 1, 2, 3, 4];
        let mut bar = Barplot::new();
        bar.set_label("LABEL")
            .set_colors(&vec!["red", "green"])
            .set_width(10.0)
            .set_bottom(&[1.0, 2.0, 3.0])
            .set_with_text("center")
            .set_extra("edgecolor='black'")
            .draw(&xx, &yy);
        let b: &str = "x=np.array([0,1,2,3,4,5,6,7,8,9,],dtype=float)\n\
                       y=np.array([5,4,3,2,1,0,1,2,3,4,],dtype=float)\n\
                       colors=['red','green',]\n\
                       bottom=np.array([1,2,3,],dtype=float)\n\
                       p=plt.bar(x,y\
                       ,label=r'LABEL'\
                       ,color=colors\
                       ,width=10\
                       ,bottom=bottom\
                       ,edgecolor='black')\n\
                       plt.gca().bar_label(p,label_type='center')\n";
        assert_eq!(bar.buffer, b);
        bar.clear_buffer();
        bar.set_with_text("");
        assert_eq!(bar.buffer, "");
    }

    #[test]
    fn draw_with_str_works_1() {
        let xx = ["one", "two", "three"];
        let yy = [1, 2, 3];
        let mut bar = Barplot::new();
        bar.draw_with_str(&xx, &yy);
        let b: &str = "x=['one','two','three',]\n\
                       y=np.array([1,2,3,],dtype=float)\n\
                       p=plt.bar(x,y)\n";
        assert_eq!(bar.buffer, b);
    }

    #[test]
    fn draw_with_str_works_2() {
        let xx = ["one", "two", "three"];
        let yy = [1, 2, 3];
        let mut bar = Barplot::new();
        bar.set_label("LABEL")
            .set_colors(&vec!["red", "green"])
            .set_width(10.0)
            .set_bottom(&[1.0, 2.0, 3.0])
            .set_with_text("center")
            .set_extra("edgecolor='black'")
            .draw_with_str(&xx, &yy);
        let b: &str = "x=['one','two','three',]\n\
                       y=np.array([1,2,3,],dtype=float)\n\
                       colors=['red','green',]\n\
                       bottom=np.array([1,2,3,],dtype=float)\n\
                       p=plt.bar(x,y\
                       ,label=r'LABEL'\
                       ,color=colors\
                       ,width=10\
                       ,bottom=bottom\
                       ,edgecolor='black')\n\
                       plt.gca().bar_label(p,label_type='center')\n";
        assert_eq!(bar.buffer, b);
    }
}
