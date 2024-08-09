use super::{matrix_to_array, AsMatrix, GraphMaker};
use std::fmt::Write;

/// Generates an image plot (imshow)
///
/// # Examples
///
/// ```
/// use plotpy::{Image, Plot, StrError};
///
/// fn main() -> Result<(), StrError> {
///     // set values
///     let data = [
///         [0.8, 2.4, 2.5, 3.9, 0.0, 4.0, 0.0],
///         [2.4, 0.0, 4.0, 1.0, 2.7, 0.0, 0.0],
///         [1.1, 2.4, 0.8, 4.3, 1.9, 4.4, 0.0],
///         [0.6, 0.0, 0.3, 0.0, 3.1, 0.0, 0.0],
///         [0.7, 1.7, 0.6, 2.6, 2.2, 6.2, 0.0],
///         [1.3, 1.2, 0.0, 0.0, 0.0, 3.2, 5.1],
///         [0.1, 2.0, 0.0, 1.4, 0.0, 1.9, 6.3],
///     ];
///
///     // image plot and options
///     let mut img = Image::new();
///     img.set_colormap_name("hsv").draw(&data);
///
///     // save figure
///     let mut plot = Plot::new();
///     plot.add(&img);
///     plot.save("/tmp/plotpy/doc_tests/doc_image_1.svg")?;
///     Ok(())
/// }
/// ```
///
/// ![doc_image_1.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/doc_image_1.svg)
///
/// See also integration test in the **tests** directory.
pub struct Image {
    colormap_name: String, // Colormap name
    extra: String,         // Extra commands (comma separated)
    buffer: String,        // buffer
}

impl Image {
    /// Creates a new Image object
    pub fn new() -> Self {
        Image {
            colormap_name: String::new(),
            extra: String::new(),
            buffer: String::new(),
        }
    }

    /// (imshow) Displays data as an image
    ///
    /// # Notes
    ///
    /// * The type `U` of the input array must be a number.
    pub fn draw<'a, T, U>(&mut self, data: &'a T)
    where
        T: AsMatrix<'a, U>,
        U: 'a + std::fmt::Display,
    {
        matrix_to_array(&mut self.buffer, "data", data);
        let opt = self.options();
        write!(&mut self.buffer, "plt.imshow(data{})\n", &opt).unwrap();
    }

    /// Sets the colormap index
    ///
    /// Options:
    ///
    /// * 0 -- bwr
    /// * 1 -- RdBu
    /// * 2 -- hsv
    /// * 3 -- jet
    /// * 4 -- terrain
    /// * 5 -- pink
    /// * 6 -- Greys
    /// * `>`6 -- starts over from 0
    pub fn set_colormap_index(&mut self, index: usize) -> &mut Self {
        const CMAP: [&str; 7] = ["bwr", "RdBu", "hsv", "jet", "terrain", "pink", "Greys"];
        self.colormap_name = CMAP[index % 7].to_string();
        self
    }

    /// Sets the colormap name
    ///
    /// Colormap names:
    ///
    /// * see <https://matplotlib.org/stable/tutorials/colors/colormaps.html>
    ///
    /// Will use `colormap_index` instead if `colormap_name` is empty.
    pub fn set_colormap_name(&mut self, name: &str) -> &mut Self {
        self.colormap_name = String::from(name);
        self
    }

    // Sets extra python/matplotlib commands (comma separated)
    pub fn set_extra(&mut self, extra: &str) -> &mut Self {
        self.extra = extra.to_string();
        self
    }

    /// Returns options for barplot
    fn options(&self) -> String {
        let mut opt = String::new();
        if self.colormap_name != "" {
            write!(&mut opt, ",cmap=plt.get_cmap('{}')", self.colormap_name).unwrap();
        }
        if self.extra != "" {
            write!(&mut opt, ",{}", self.extra).unwrap();
        }
        opt
    }
}

impl GraphMaker for Image {
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
    use super::Image;
    use crate::GraphMaker;

    #[test]
    fn new_works() {
        let img = Image::new();
        assert_eq!(img.colormap_name.len(), 0);
        assert_eq!(img.extra.len(), 0);
        assert_eq!(img.buffer.len(), 0);
    }

    #[test]
    fn draw_works_1() {
        let xx = [[1, 2], [3, 2]];
        let mut img = Image::new();
        img.set_colormap_index(0).set_colormap_name("terrain").draw(&xx);
        let b: &str = "data=np.array([[1,2,],[3,2,],],dtype=float)\n\
                       plt.imshow(data,cmap=plt.get_cmap('terrain'))\n";
        assert_eq!(img.buffer, b);
        img.clear_buffer();
        assert_eq!(img.buffer, "");
    }
}
