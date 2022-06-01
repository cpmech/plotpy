//! Rust plotting library using Python (Matplotlib)
//!
//! This library generates plots by calling **python3** after generating a python script.
//! The name of the python script is based on the name of the figure (png, svg, ...).
//! If an error occurs, a log file is created (also named as the figure).
//!
//! The main idea here is to create objects such as `Curve`, `Contour`, `Histogram`,
//! or `Surface` and add them to a plot using the `add` command. The plot may
//! also configured using the methods of `Plot`.
//!
//! Each object (e.g. `Curve`, `Legend`, `Text`) defines a number of configuration options
//! that can be directly set on the object. Then, the `draw` method of each object must
//! be called before adding to `Plot`.
//!
//! # Example
//!
//! ```
//! # use plotpy::{Plot, StrError, Surface};
//! # use std::path::Path;
//! # fn main() -> Result<(), StrError> {
//! use russell_lab::generate3d;
//! let mut surface = Surface::new();
//! surface
//!     .set_with_wireframe(true)
//!     .set_colormap_name("Pastel1")
//!     .set_with_colorbar(true)
//!     .set_colorbar_label("temperature")
//!     .set_line_color("#1862ab")
//!     .set_line_style(":")
//!     .set_line_width(0.75);
//!
//! // draw surface
//! let n = 9;
//! let (x, y, z) = generate3d(-2.0, 2.0, -2.0, 2.0, n, n, |x, y| x * x + y * y);
//! surface.draw(&x, &y, &z);
//!
//! // add surface to plot
//! let mut plot = Plot::new();
//! plot.add(&surface);
//!
//! // save figure
//! plot.save(Path::new("/tmp/plotpy/example_main.svg"))?;
//! # Ok(())
//! # }
//! ```
//!
//! ![example_main.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/example_main.svg)
//!

/// Defines a type alias for the error type as a static string
pub type StrError = &'static str;

// modules ////////////////////////////////////////
mod as_matrix;
mod as_vector;
mod canvas;
mod constants;
mod contour;
mod conversions;
mod curve;
mod fileio;
mod histogram;
mod legend;
mod plot;
mod slope_icon;
mod surface;
mod surface_geometry;
mod text;
pub use crate::as_matrix::*;
pub use crate::as_vector::*;
pub use crate::canvas::*;
pub use crate::constants::*;
pub use crate::contour::*;
use crate::conversions::*;
pub use crate::curve::*;
use crate::fileio::*;
pub use crate::histogram::*;
pub use crate::legend::*;
pub use crate::plot::*;
pub use crate::slope_icon::*;
pub use crate::surface::*;
pub use crate::surface_geometry::*;
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
