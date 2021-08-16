//! Rust plotting library using Python (Matplotlib)
//!
//! # Examples
//!
//! ```
//! use plotpy::*;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let x = &[1.0, 2.0, 3.0, 4.0, 5.0];
//!     let y = &[1.0, 4.0, 9.0, 16.0, 25.0];
//!     let mut curve = Curve::new();
//!     curve.draw(x, y);
//!
//!     let mut plot = Plot::new();
//!     plot.subplot(2, 2, 1);
//!     plot.add(&curve);
//!
//!     plot.subplot(2, 2, 2);
//!     plot.add(&curve);
//!
//!     plot.subplot(2, 2, 3);
//!     plot.add(&curve);
//!
//!     plot.subplot(2, 2, 4);
//!     plot.add(&curve);
//!     plot.grid_and_labels("x", "y");
//!
//!     let message = plot.save("/tmp/plotpy", "example_main", "svg")?;
//!     println!("{}", message);
//!     Ok(())
//! }
//! ```
//!
//! ![example_main.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/example_main.svg)
//!
//! # Todo
//!
//! - [ ] Test Contour
//! - [ ] Test Graphs3d
//! - [ ] Test Histogram
//! - [ ] Test Legend
//! - [ ] Test Shapes
//! - [ ] Test Text
//!

// modules ////////////////////////////////////////
mod constants;
mod contour;
mod curve;
mod fileio;
mod graph3d;
mod histogram;
mod legend;
mod plot;
mod scatter;
mod shapes;
mod text;
mod util;
pub use crate::constants::*;
pub use crate::contour::*;
pub use crate::curve::*;
use crate::fileio::*;
pub use crate::graph3d::*;
pub use crate::histogram::*;
pub use crate::legend::*;
pub use crate::plot::*;
pub use crate::scatter::*;
pub use crate::shapes::*;
pub use crate::text::*;
use crate::util::*;
