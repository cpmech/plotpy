//! Rust plotting library using Python (Matplotlib)
//!
//! # Examples
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
