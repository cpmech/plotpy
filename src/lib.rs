//! Rust plotting library using Python and Matplotlib
//!
//! This library implements high-level functions to generate plots and drawings.
//! Although we use Python/Matplotlib, the goal is to provide a convenient Rust library
//! that is **different** than Matplotlib. The difference happens because we want **convenience**
//! for the Rust developer while getting the **fantastic quality of Matplotlib** 😀.
//!
//! Internally, we use [Matplotlib](https://matplotlib.org/) via a Python 3 script.
//! First, we generate a python code in a directory of your choice (e.g., `/tmp/plotpy`),
//! and then we call **python3** using Rust's [std::process::Command].
//!
//! The Python script has the same name as the figure name given to the [Plot::save] function,
//! but with the `.py` extension. The figure name can have the (png, pdf, or svg) extension
//! (see [Matplotlib](https://matplotlib.org/stable/api/_as_gen/matplotlib.pyplot.savefig.html))
//! for more information regarding file extensions.
//!
//! We generate the Python script with the preamble listed in [PYTHON_HEADER] and the file
//! should be useful for double checking or even directly adding Python/Matplotlib commands,
//! in case the functionality is not implemented here yet.
//!
//! When calling [Plot::save] or [Plot::save_and_show], if an error occurs, we generate a log
//! file in the same output directory with the same filename as the figure (and python script),
//! but with the `.log` extension.
//!
//! The typical use of this library is by allocating structures such as [Canvas], [Curve], [Contour],
//! [Histogram], [Surface], [Text] (and more) and then passing them to [Plot] for the generation
//! of the files mentioned above. The [Plot::save_and_show] function may also be used to immediately
//! see the plot or drawing on the screen.
//!
//! Each structure (e.g. [Curve], [Legend], or [Text]) defines many configuration options
//! that can be set by calling their own `set_...` function. Typically, these structures provide
//! `draw_...` functions to plot/draw features. Afterwards, we call [Plot::add] to add these structures
//! to the [Plot] and then call [Plot::save]. The `draw` method of each object must be called
//! before adding to `Plot`.
//!
//! # Example
//!
//! ```
//! use plotpy::{generate3d, Plot, StrError, Surface};
//!
//! fn main() -> Result<(), StrError> {
//!     let mut surface = Surface::new();
//!     surface
//!         .set_with_wireframe(true)
//!         .set_colormap_name("Pastel1")
//!         .set_with_colorbar(true)
//!         .set_colorbar_label("temperature")
//!         .set_line_color("#1862ab")
//!         .set_line_style(":")
//!         .set_line_width(0.75);
//!
//!     // draw surface
//!     let n = 9;
//!     let (x, y, z) = generate3d(-2.0, 2.0, -2.0, 2.0, n, n, |x, y| x * x + y * y);
//!     surface.draw(&x, &y, &z);
//!
//!     // add surface to plot
//!     let mut plot = Plot::new();
//!     plot.add(&surface);
//!
//!     // save figure
//!     plot.save("/tmp/plotpy/example_main.svg")?;
//!     Ok(())
//! }
//! ```
//!
//! ![example_main.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/example_main.svg)

/// Defines a type alias for the error type as a static string
pub type StrError = &'static str;

// modules ////////////////////////////////////////
mod as_matrix;
mod as_vector;
mod auxiliary;
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
pub use crate::auxiliary::*;
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
