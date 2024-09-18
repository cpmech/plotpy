use super::{AsMatrix, matrix_to_array, generate_list, GraphMaker};
use std::fmt::Write;

/// Draw a box and whisker plot
///
/// [See Matplotlib's documentation](https://matplotlib.org/stable/api/_as_gen/matplotlib.pyplot.boxplot.html)
///
/// # Examples
///
/// ## Basic box plot
///
/// ```
/// use plotpy::{Boxplot, AsMatrix, Plot, StrError};
///
/// fn main() -> Result<(), StrError> {
///     // data
///     let x = vec![vec![1,2,3,4,5],
///                  vec![2,3,4,5,6],
///                  vec![3,4,5,6,7],
///                  vec![4,5,6,7,8],
///                  vec![5,6,7,8,9],
///                  vec![6,7,8,9,10]];
/// 
///     // x ticks and labels
///     let ticks: Vec<_> = (1..=x.size().1).into_iter().collect();
///     let labels = ["x1", "x2", "x3", "x4", "x5"];
/// 
///     // boxplot object and options
///     let mut boxes = Boxplot::new();
///     boxes.set_vertical(true)
///          .draw(&x);
///
///     // save figure
///     let mut plot = Plot::new();
///     plot.add(&boxes).set_title("boxplot")
///         .set_ticks_x_labels(&ticks, &labels)
///         .set_labels("x", "y")  // x-axis and y-axis label
///         .save("/tmp/plotpy/doc_tests/doc_boxplot_1.svg")?;
///     Ok(())
/// }
/// ```
/// 
/// ![doc_boxplot_1.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/doc_boxplot_1.svg)
/// 
/// 
/// ## More examples
/// 
/// See also integration test in the **tests** directory.
pub struct Boxplot {
    label: String,              // Name of this box in the legend
    symbol: Option<String>,     // The default symbol for flier (outlier) points.
    vertical: Option<bool>,     // Vertical boxplot
    whisker: Option<f64>,       // The position of the whiskers
    positions: Vec<f64>,        // The positions of the boxes
    width: Option<f64>,         // The width of the boxes
    extra: String,              // Extra commands (comma separated)
    buffer: String,             // Buffer
}

impl Boxplot {
    /// Creates a new Boxplot object
    pub fn new() -> Self {
        Boxplot {
            label: String::new(),
            symbol: None,
            vertical: None,
            whisker: None,
            positions: Vec::new(),
            width: None,
            extra: String::new(),
            buffer: String::new(),
        }
    }

    /// Draws the box plot
    /// 
    /// # Notes
    /// 
    /// * The type `U` must be a number.
    pub fn draw<'a, T, U>(&mut self, x: &'a T)
    where
        T: AsMatrix<'a, U>,
        U: 'a + std::fmt::Display,
    {
        matrix_to_array(&mut self.buffer, "x", x);
        if self.positions.len() > 0 {
            generate_list(&mut self.buffer, "positions", self.positions.as_slice());
        }
        let opt = self.options();  // Optional parameters
        write!(&mut self.buffer, "p=plt.boxplot(x{})\n", &opt).unwrap();
    }

    /// Sets the name of this boxplot in the legend
    pub fn set_label(&mut self, label: &str) -> &mut Self {
        self.label = String::from(label);
        self
    }

    /// Sets the symbol for the boxplot
    pub fn set_symbol(&mut self, symbol: &str) -> &mut Self {
        self.symbol = Some(symbol.to_string());
        self
    }

    /// Enables drawing vertical boxplot
    pub fn set_vertical(&mut self, flag: bool) -> &mut Self {
        self.vertical = Some(flag);
        self
    }

    /// Sets the whisker of this boxplot
    pub fn set_whisker(&mut self, whisker: f64) -> &mut Self {
        self.whisker = Some(whisker);
        self
    }

    /// Sets the positions of the boxes
    pub fn set_positions(&mut self, positions: Vec<f64>) -> &mut Self {
        self.positions = positions;
        self
    }

    /// Sets the width of the boxes
    pub fn set_width(&mut self, width: f64) -> &mut Self {
        self.width = Some(width);
        self
    }

    /// Sets extra matplotlib commands (comma separated)
    /// 
    /// **Important:** The extra commands must be comma separated. For example:
    /// 
    /// ```text
    /// param1=123,param2='hello'
    /// ```
    /// [See Matplotlib's documentation for extra parameters](https://matplotlib.org/stable/api/_as_gen/matplotlib.pyplot.boxplot.html)
    pub fn set_extra(&mut self, extra: &str) -> &mut Self {
        self.extra = extra.to_string();
        self
    }

    /// Returns options (optional parameters) for boxplot
    fn options(&self) -> String {
        let mut opt = String::new();
        if self.label != "" {
            write!(&mut opt, ",label=r'{}'", self.label).unwrap();
        }
        if self.symbol != None {
            write!(&mut opt, ",sym=r'{}'", self.symbol.clone().unwrap()).unwrap();
        }
        if self.vertical == Some(true) {
            write!(&mut opt, ",vert=True").unwrap();
        } else if self.vertical == Some(false) {
            write!(&mut opt, ",vert=False").unwrap();
        }
        if self.whisker != None {
            write!(&mut opt, ",whis={}", self.whisker.unwrap()).unwrap();
        }
        if self.positions.len() > 0 {
            write!(&mut opt, ",positions=positions").unwrap();
        }
        if self.width != None {
            write!(&mut opt, ",widths={}", self.width.unwrap()).unwrap();
        }
        if self.extra != "" {
            write!(&mut opt, ",{}", self.extra).unwrap();
        }
        opt
    }
}

impl GraphMaker for Boxplot {
    fn get_buffer<'a>(&'a self) -> &'a String {
        &self.buffer
    }
    fn clear_buffer(&mut self) {
        self.buffer.clear();
    }
}

/////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Boxplot;
    use crate::GraphMaker;

    #[test]
    fn new_works() {
        let boxes = Boxplot::new();
        assert_eq!(boxes.label.len(), 0);
        assert_eq!(boxes.symbol, None);
        assert_eq!(boxes.vertical, None);
        assert_eq!(boxes.whisker, None);
        assert_eq!(boxes.positions.len(), 0);
        assert_eq!(boxes.width, None);
        assert_eq!(boxes.extra.len(), 0);
        assert_eq!(boxes.buffer.len(), 0);
    }

    #[test]
    fn draw_works_1() {
        let x = vec![vec![1,2,3,4,5],
                 vec![2,3,4,5,6],
                 vec![3,4,5,6,7],
                 vec![4,5,6,7,8],
                 vec![5,6,7,8,9],
                 vec![6,7,8,9,10]];
        let mut boxes = Boxplot::new();
        boxes.draw(&x);
        let b: &str = "x=np.array([[1,2,3,4,5,],[2,3,4,5,6,],[3,4,5,6,7,],[4,5,6,7,8,],[5,6,7,8,9,],[6,7,8,9,10,],],dtype=float)\n\
                       p=plt.boxplot(x)\n";
        assert_eq!(boxes.buffer, b);
        boxes.clear_buffer();
        assert_eq!(boxes.buffer, "");
    }

    #[test]
    fn draw_works_2() {
        let x = vec![vec![1,2,3,4,5],
                 vec![2,3,4,5,6],
                 vec![3,4,5,6,7],
                 vec![4,5,6,7,8],
                 vec![5,6,7,8,9],
                 vec![6,7,8,9,10]];
        let mut boxes = Boxplot::new();
        boxes.set_label("LABEL")
             .set_symbol("b+")
             .set_vertical(true)
             .set_whisker(1.5)
             .set_positions(vec![1.0, 2.0, 3.0, 4.0, 5.0])
             .set_width(0.5)
             .draw(&x);
        let b: &str = "x=np.array([[1,2,3,4,5,],[2,3,4,5,6,],[3,4,5,6,7,],[4,5,6,7,8,],[5,6,7,8,9,],[6,7,8,9,10,],],dtype=float)\n\
                       positions=[1,2,3,4,5,]\n\
                       p=plt.boxplot(x,label=r'LABEL',sym=r'b+',vert=True,whis=1.5,positions=positions,widths=0.5)\n";
        assert_eq!(boxes.buffer, b);
        boxes.clear_buffer();
        assert_eq!(boxes.buffer, "");
    }
}