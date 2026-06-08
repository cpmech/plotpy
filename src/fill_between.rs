use super::{vector_to_array, AsVector, GraphMaker};
use num_traits::Num;
use std::fmt::Write;

/// Fills the area between two curves
///
/// When `y2` is `None`, fills the area between `y1` and the x-axis (`y = 0`).
///
/// **Important:** The generated Python script uses `y1` and `y2` as variable names.
/// Any `where_condition` (e.g., for two-color filling) must reference these names.
///
/// # Examples
///
/// ```
/// use plotpy::{Curve, FillBetween, Plot, StrError, linspace};
///
/// fn main() -> Result<(), StrError> {
///     // data and curve
///     let x = linspace(-1.0, 2.0, 21);
///     let y: Vec<_> = x.iter().map(|&x| x * x).collect();
///     let mut curve = Curve::new();
///     curve.set_line_color("black").draw(&x, &y);
///
///     // draw area between curve and x-axis
///     // (note that we have to use "y1" as variable name for the curve)
///     let mut fb = FillBetween::new();
///     fb.set_where("y1>=0.5").set_extra("alpha=0.5").draw(&x, &y, None);
///
///     // add curve and fb to plot
///     let mut plot = Plot::new();
///     plot.add(&curve).add(&fb);
///
///     // save figure
///     plot.save("/tmp/plotpy/doc_tests/doc_fill_between.svg")?;
///     Ok(())
/// }
/// ```
///
/// ![doc_fill_between.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/doc_fill_between.svg)
pub struct FillBetween {
    where_condition: String,
    facecolor: String,
    interpolate: bool,
    extra: String,
    buffer: String,
}

impl FillBetween {
    /// Allocates a new instance
    pub fn new() -> Self {
        FillBetween {
            where_condition: String::new(),
            facecolor: String::new(),
            interpolate: false,
            extra: String::new(),
            buffer: String::new(),
        }
    }

    /// Draws the filled area between two curves (or between a curve and the x-axis)
    ///
    /// # Input
    ///
    /// * `x` -- shared x-axis values
    /// * `y1` -- y values of the first curve (variable name: `y1`)
    /// * `y2` -- optional y values of the second curve (variable name: `y2`).
    ///   If `None`, fills the area between `y1` and the x-axis (`y = 0`).
    ///
    /// # Note
    ///
    /// For two-color filling (above/below a threshold), use [`set_where`](Self::set_where)
    /// with a condition referencing `y1` and `y2`.
    pub fn draw<'a, T, U>(&mut self, x: &'a T, y1: &'a T, y2: Option<&'a T>)
    where
        T: AsVector<'a, U>,
        U: 'a + std::fmt::Display + Num,
    {
        let opt = self.options();
        vector_to_array(&mut self.buffer, "x", x);
        vector_to_array(&mut self.buffer, "y1", y1);
        match y2 {
            Some(y2) => {
                vector_to_array(&mut self.buffer, "y2", y2);
                write!(&mut self.buffer, "plt.fill_between(x,y1,y2{})\n", &opt).unwrap();
            }
            None => {
                write!(&mut self.buffer, "plt.fill_between(x,y1{})\n", &opt).unwrap();
            }
        }
    }

    /// Sets the condition that selects which region to fill
    ///
    /// **The condition must use `y1` and `y2` as variable names.**
    ///
    /// Examples:
    /// - `"y1>=0.5"` -- fill where the curve is above 0.5 on the y-axis
    /// - `"y2>=y1"` -- fill where the second curve is above the first
    pub fn set_where(&mut self, condition: &str) -> &mut Self {
        self.where_condition = condition.to_string();
        self
    }

    /// Sets the fill color of the shaded area
    pub fn set_facecolor(&mut self, color: &str) -> &mut Self {
        self.facecolor = color.to_string();
        self
    }

    /// When `true`, calculates the actual intersection point of crossing curves
    /// and extends the filled region up to that point for a clean edge
    ///
    /// This only matters when using [`set_where`](Self::set_where) with crossing curves.
    /// Default is `false`.
    ///
    /// From the [Matplotlib docs](https://matplotlib.org/stable/api/_as_gen/matplotlib.pyplot.fill_between.html):
    ///
    /// "Semantically, `where` is often used for `y1 > y2` or similar. By default,
    /// the nodes of the polygon defining the filled region will only be placed at the
    /// positions in the x array. Such a polygon cannot describe the above semantics
    /// close to the intersection. The x-sections containing the intersection are
    /// simply clipped."
    pub fn set_interpolate(&mut self, interpolate: bool) -> &mut Self {
        self.interpolate = interpolate;
        self
    }

    /// Sets extra matplotlib commands (comma separated) for `fill_between`
    ///
    /// **Important:** The extra commands must be comma separated. For example:
    ///
    /// ```text
    /// alpha=0.5,color='#ffaabb'
    /// ```
    ///
    /// [See Matplotlib's documentation](https://matplotlib.org/stable/api/_as_gen/matplotlib.pyplot.fill_between.html)
    pub fn set_extra(&mut self, extra: &str) -> &mut Self {
        self.extra = extra.to_string();
        self
    }

    /// Returns the options
    fn options(&self) -> String {
        let mut opt = String::new();
        if self.facecolor != "" {
            write!(&mut opt, ",facecolor='{}'", self.facecolor).unwrap();
        }
        if self.where_condition != "" {
            write!(&mut opt, ",where={}", self.where_condition).unwrap();
        }
        if self.interpolate {
            write!(&mut opt, ",interpolate=True").unwrap();
        }
        if self.extra != "" {
            write!(&mut opt, ",{}", self.extra).unwrap();
        }
        opt
    }
}

impl GraphMaker for FillBetween {
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
    use super::FillBetween;

    #[test]
    fn new_works() {
        let fill_between = FillBetween::new();
        assert_eq!(fill_between.where_condition, "");
        assert_eq!(fill_between.facecolor, "");
        assert_eq!(fill_between.interpolate, false);
        assert_eq!(fill_between.extra, "");
        assert_eq!(fill_between.buffer.len(), 0);
    }
}
