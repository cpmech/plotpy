use super::GraphMaker;
use std::fmt::Write;

/// Implements extra utilities such as wrapping Matplotlib's Cookbook functions
pub struct Util {
    buffer: String, // buffer
}

impl Util {
    /// Creates a new `Util` object with an empty buffer.
    ///
    /// # Returns
    ///
    /// A new instance of `Util`.
    pub fn new() -> Self {
        Self { buffer: String::new() }
    }

    /// Wraps Matplotlib's `cbook.get_sample_data` function.
    ///
    /// This function generates a command to retrieve a sample data file from Matplotlib's
    /// `mpl-data/sample_data` directory. The command is stored in the buffer.
    ///
    /// See <https://matplotlib.org/stable/api/cbook_api.html#matplotlib.cbook.get_sample_data>
    ///
    /// # Arguments
    ///
    /// * `handle` - The name of the **Python variable** to assign the result of `cbook.get_sample_data`
    /// * `matplotlib_fname` - The filename of the sample data, relative to the `mpl-data/sample_data` directory.
    /// * `as_file_obj` - If `true`, returns a file object; otherwise, returns a file path.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `Util` instance, allowing for method chaining.
    ///
    /// # Example
    ///
    /// ```
    /// let mut util = Util::new();
    /// util.get_sample_data("data", "example.csv", false);
    /// ```
    pub fn get_sample_data(&mut self, handle: &str, matplotlib_fname: &str, as_file_obj: bool) -> &mut Self {
        write!(
            &mut self.buffer,
            "{} = cbook.get_sample_data('{}', as_file_obj={})",
            handle, matplotlib_fname, as_file_obj
        )
        .unwrap();
        self
    }
}

impl GraphMaker for Util {
    /// Returns a reference to the buffer containing the generated commands.
    ///
    /// # Returns
    ///
    /// A reference to the buffer as a `String`.
    fn get_buffer<'a>(&'a self) -> &'a String {
        &self.buffer
    }

    /// Clears the buffer, removing all stored commands.
    fn clear_buffer(&mut self) {
        self.buffer.clear();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Util;
    use crate::GraphMaker;

    #[test]
    fn new_works() {
        let util = Util::new();
        assert_eq!(util.get_buffer(), "");
    }

    #[test]
    fn get_sample_data_works() {
        let mut util = Util::new();
        util.get_sample_data("data", "example.csv", false);
        assert_eq!(
            util.get_buffer(),
            "data = cbook.get_sample_data('example.csv', as_file_obj=false)"
        );
    }
}
