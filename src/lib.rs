//! Rust plotting library using Python (Matplotlib)
//!
//! This library generates plots by calling **python3** after generating a python script.
//! The name of the python script is based on the name of the figure (png, svg, ...).
//! If an error occurs, a log file is created (also named as the figure).
//!
//! The main idea here is to create objects such as `Curve`, `Contour`, `Histogram`,
//! or `Surface` and add it to a plot using the `add` command. The plot may
//! also configured using the methods of `Plot`.
//!
//! Each object (e.g. `Curve`, `Legend`, `Text`) defines a number of configuration options
//! that can be directly set on the object. Then, the `draw` method of each object must
//! be called before adding to `Plot`.
//!
//! # Example
//!
//! ```
//! # use plotpy::*;
//! # use std::path::Path;
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let x = &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
//! let y = &[1.0, 4.0, 9.0, 16.0, 25.0, 36.0, 49.0, 64.0];
//! let mut curve = Curve::new();
//! curve.draw(x, y);
//!
//! let mut plot = Plot::new();
//! plot.add(&curve);
//! plot.grid_and_labels("x", "y");
//!
//! plot.save(Path::new("/tmp/plotpy/example_main.svg"))?;
//! # Ok(())
//! # }
//! ```
//!
//! ![example_main.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/example_main.svg)
//!

// modules ////////////////////////////////////////
mod as_matrix;
mod as_vector;
mod constants;
mod contour;
mod conversions;
mod curve;
mod fileio;
mod histogram;
mod legend;
mod plot;
mod shapes;
mod surface;
mod text;
pub use crate::as_matrix::*;
pub use crate::as_vector::*;
pub use crate::constants::*;
pub use crate::contour::*;
use crate::conversions::*;
pub use crate::curve::*;
use crate::fileio::*;
pub use crate::histogram::*;
pub use crate::legend::*;
pub use crate::plot::*;
pub use crate::shapes::*;
pub use crate::surface::*;
pub use crate::text::*;

// run code from README file
#[cfg(doctest)]
mod test_readme {
    macro_rules! external_doc_test {
        ($x:expr) => {
            #[doc = $x]
            extern "C" {}
        };
    }
    external_doc_test!(include_str!("../README.md"));
}
