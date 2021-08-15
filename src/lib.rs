//! rplotpy - Rust plotting library that calls Python-MatPlotLib

// modules ////////////////////////////////////////
mod arrays;
mod basic;
mod constants;
mod fileio;
mod plot;
pub use crate::constants::*;
pub use crate::fileio::*;
pub use crate::plot::*;
