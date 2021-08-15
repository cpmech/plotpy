//! rplotpy - Rust plotting library that calls Python-MatPlotLib
//!
//! # Examples
//!
//! ```
//! use rplotpy::*;
//! let x = &[1.0, 2.0, 3.0, 4.0, 5.0];
//! let y = &[1.0, 4.0, 9.0, 16.0, 25.0];
//! let mut plt = Plot::new();
//! plt.scatter(x, y);
//! plt.save("/tmp/rplotpy", "example", "svg");
//! ```

// modules ////////////////////////////////////////
mod arrays;
mod basic;
mod constants;
mod fileio;
mod plot;
pub use crate::constants::*;
pub use crate::fileio::*;
pub use crate::plot::*;
