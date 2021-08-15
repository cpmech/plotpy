//! Rust plotting library using Python (Matplotlib)
//!
//! # Examples
//!
//! ```
//! use plotpy::*;
//! let x = &[1.0, 2.0, 3.0, 4.0, 5.0];
//! let y = &[1.0, 4.0, 9.0, 16.0, 25.0];
//! let mut plt = Plot::new();
//! plt.scatter(x, y);
//! plt.save("/tmp/plotpy", "example", "svg");
//! ```

// modules ////////////////////////////////////////
mod arrays;
mod basic;
mod constants;
mod curve;
mod curve_style;
mod fileio;
mod plot;
pub use crate::constants::*;
pub use crate::curve::*;
pub use crate::curve_style::*;
pub use crate::fileio::*;
pub use crate::plot::*;
