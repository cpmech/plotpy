use super::{generate_list, matrix_to_array, AsMatrix, GraphMaker};
use std::fmt::Write;

/// Draw a box and whisker plot
///
/// [See Matplotlib's documentation](https://matplotlib.org/3.6.3/api/_as_gen/matplotlib.pyplot.boxplot.html)
///
/// # Examples
///
/// ## Data as a 2D array
///
/// ```
/// use plotpy::{Boxplot, Plot, StrError};
///
/// fn main() -> Result<(), StrError> {
///     // data (as a 2D array/matrix)
///     let data = vec![
///         //   A  B  C  D  E
///         vec![1, 2, 3, 4, 5],
///         vec![2, 3, 4, 5, 6],
///         vec![3, 4, 5, 6, 7],
///         vec![4, 5, 6, 7, 8],
///         vec![5, 6, 7, 8, 9],
///         vec![6, 7, 8, 9, 10],
///         vec![14, 14, 14, 14, 14], // outliers
///     ];
///
///     // x ticks and labels
///     let ncol = data[0].len();
///     let ticks: Vec<_> = (1..(ncol + 1)).into_iter().collect();
///     let labels = ["A", "B", "C", "D", "E"];
///
///     // boxplot object and options
///     let mut boxes = Boxplot::new();
///     boxes.draw_mat(&data);
///
///     // save figure
///     let mut plot = Plot::new();
///     plot.add(&boxes)
///         .set_title("boxplot documentation test")
///         .set_ticks_x_labels(&ticks, &labels)
///         .save("/tmp/plotpy/doc_tests/doc_boxplot_1.svg")?;
///     Ok(())
/// }
/// ```
///
/// ![doc_boxplot_1.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/doc_boxplot_1.svg)
///
/// ## More examples
///
/// See also integration test in the **tests** directory.
pub struct Boxplot {
    symbol: String,       // The default symbol for flier (outlier) points.
    horizontal: bool,     // Horizontal boxplot (default is false)
    whisker: Option<f64>, // The position of the whiskers
    positions: Vec<f64>,  // The positions of the boxes
    width: Option<f64>,   // The width of the boxes
    extra: String,        // Extra commands (comma separated)
    buffer: String,       // Buffer
}

impl Boxplot {
    /// Creates a new Boxplot object
    pub fn new() -> Self {
        Boxplot {
            symbol: String::new(),
            horizontal: false,
            whisker: None,
            positions: Vec::new(),
            width: None,
            extra: String::new(),
            buffer: String::new(),
        }
    }

    /// Draws the box plot given a 2D array (Matrix)
    ///
    /// # Notes
    ///
    /// * The type `U` must be a number.
    pub fn draw_mat<'a, T, U>(&mut self, x: &'a T)
    where
        T: AsMatrix<'a, U>,
        U: 'a + std::fmt::Display,
    {
        matrix_to_array(&mut self.buffer, "x", x);
        if self.positions.len() > 0 {
            generate_list(&mut self.buffer, "positions", self.positions.as_slice());
        }
        let opt = self.options(); // Optional parameters
        write!(&mut self.buffer, "p=plt.boxplot(x{})\n", &opt).unwrap();
    }

    /// Sets the symbol for the outliers
    pub fn set_symbol(&mut self, symbol: &str) -> &mut Self {
        self.symbol = symbol.to_string();
        self
    }

    /// Enables drawing horizontal boxes
    pub fn set_horizontal(&mut self, flag: bool) -> &mut Self {
        self.horizontal = flag;
        self
    }

    /// Sets the position of the whiskers
    ///
    /// The default value of whisker = 1.5 corresponds to Tukey's original definition of boxplots.
    ///
    /// [See Matplotlib's documentation](https://matplotlib.org/3.6.3/api/_as_gen/matplotlib.pyplot.boxplot.html)
    pub fn set_whisker(&mut self, whisker: f64) -> &mut Self {
        self.whisker = Some(whisker);
        self
    }

    /// Sets the positions of the boxes
    pub fn set_positions(&mut self, positions: &[f64]) -> &mut Self {
        self.positions = positions.to_vec();
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
    ///
    /// [See Matplotlib's documentation for extra parameters](https://matplotlib.org/3.6.3/api/_as_gen/matplotlib.pyplot.boxplot.html)
    pub fn set_extra(&mut self, extra: &str) -> &mut Self {
        self.extra = extra.to_string();
        self
    }

    /// Returns options (optional parameters) for boxplot
    fn options(&self) -> String {
        let mut opt = String::new();
        if self.symbol != "" {
            write!(&mut opt, ",sym=r'{}'", self.symbol).unwrap();
        }
        if self.horizontal {
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
        assert_eq!(boxes.symbol.len(), 0);
        assert_eq!(boxes.horizontal, false);
        assert_eq!(boxes.whisker, None);
        assert_eq!(boxes.positions.len(), 0);
        assert_eq!(boxes.width, None);
        assert_eq!(boxes.extra.len(), 0);
        assert_eq!(boxes.buffer.len(), 0);
    }

    #[test]
    fn draw_works_1() {
        let x = vec![
            vec![1, 2, 3, 4, 5],
            vec![2, 3, 4, 5, 6],
            vec![3, 4, 5, 6, 7],
            vec![4, 5, 6, 7, 8],
            vec![5, 6, 7, 8, 9],
            vec![6, 7, 8, 9, 10],
        ];
        let mut boxes = Boxplot::new();
        boxes.draw_mat(&x);
        let b: &str = "x=np.array([[1,2,3,4,5,],[2,3,4,5,6,],[3,4,5,6,7,],[4,5,6,7,8,],[5,6,7,8,9,],[6,7,8,9,10,],],dtype=float)\n\
                       p=plt.boxplot(x)\n";
        assert_eq!(boxes.buffer, b);
        boxes.clear_buffer();
        assert_eq!(boxes.buffer, "");
    }

    #[test]
    fn draw_works_2() {
        let x = vec![
            vec![1, 2, 3, 4, 5],
            vec![2, 3, 4, 5, 6],
            vec![3, 4, 5, 6, 7],
            vec![4, 5, 6, 7, 8],
            vec![5, 6, 7, 8, 9],
            vec![6, 7, 8, 9, 10],
        ];
        let mut boxes = Boxplot::new();
        boxes
            .set_symbol("b+")
            .set_horizontal(true)
            .set_whisker(1.5)
            .set_positions(&[1.0, 2.0, 3.0, 4.0, 5.0])
            .set_width(0.5)
            .draw_mat(&x);
        let b: &str = "x=np.array([[1,2,3,4,5,],[2,3,4,5,6,],[3,4,5,6,7,],[4,5,6,7,8,],[5,6,7,8,9,],[6,7,8,9,10,],],dtype=float)\n\
                       positions=[1,2,3,4,5,]\n\
                       p=plt.boxplot(x,sym=r'b+',vert=False,whis=1.5,positions=positions,widths=0.5)\n";
        assert_eq!(boxes.buffer, b);
        boxes.clear_buffer();
        assert_eq!(boxes.buffer, "");
    }
}
