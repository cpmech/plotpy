//! Rust plotting library using Python (Matplotlib)
//!
//! # Examples
//!
//! ```
//! use plotpy::*;
//! let x = &[1.0, 2.0, 3.0, 4.0, 5.0];
//! let y = &[1.0, 4.0, 9.0, 16.0, 25.0];
//! let mut plot = Plot::new();
//! let mut curve = Curve::new();
//! let mut scatter = Scatter::new();
//! curve.line_style = "--".to_string();
//! scatter.marker_style = "*".to_string();
//! curve.draw(x, y);
//! scatter.draw(y, x);
//! plot.add(&curve);
//! plot.add(&scatter);
//! plot.add_grid_and_labels("x-label", "y-label");
//! plot.save("/tmp/plotpy", "example", "svg");
//! ```
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
