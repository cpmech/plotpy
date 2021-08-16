//! Rust plotting library using Python (Matplotlib)
//!
//! # Examples
//!
//! ```
//! use plotpy::*;
//! let x = &[1.0, 2.0, 3.0, 4.0, 5.0];
//! let y = &[1.0, 4.0, 9.0, 16.0, 25.0];
//! let mut plt = Plot::new();
//! let options = Options::new();
//! plt.scatter(x, y, &options);
//! plt.save("/tmp/plotpy", "example", "svg");
//! ```
//!
//! # Todo
//!
//! - [ ] Test shapes options
//! - [ ] Test text options
//! - [ ] Test legend options
//! - [ ] Test contour options
//! - [ ] Test histogram options
//! - [ ] Test 3D graphs options
//!

// modules ////////////////////////////////////////
mod arrays;
mod basic;
mod constants;
mod fileio;
mod options;
mod plot;
pub use crate::constants::*;
pub use crate::fileio::*;
pub use crate::options::*;
pub use crate::plot::*;
