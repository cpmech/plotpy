//! Rust plotting library using Python (Matplotlib)
//!
//! # Examples
//!
//! ```
//! use plotpy::*;
//! let x = &[1.0, 2.0, 3.0, 4.0, 5.0];
//! let y = &[1.0, 4.0, 9.0, 16.0, 25.0];
//! let mut plt = Plot::new();
//! let args = Arguments::new();
//! plt.scatter(x, y, &args);
//! plt.save("/tmp/plotpy", "example", "svg");
//! ```
//!
//! # Todo
//!
//! - [ ] Test shapes arguments
//! - [ ] Test text arguments
//! - [ ] Test legend arguments
//! - [ ] Test contour arguments
//! - [ ] Test histogram arguments
//! - [ ] Test 3D graphs arguments
//!

// modules ////////////////////////////////////////
mod arguments;
mod arrays;
mod basic;
mod constants;
mod fileio;
mod plot;
pub use crate::arguments::*;
pub use crate::constants::*;
pub use crate::fileio::*;
pub use crate::plot::*;
