//! # Competitive Programming Library
//!
//! Example:
//! ```no_run
//! use cplit::scanln;
//!
//! fn main() {
//!     let (a, b): (usize, usize);
//!     scanln!(a, b);
//!     println!("{}", a + b);
//! }
//! ```

pub mod data_structure;
pub mod graph;
mod io;
pub mod num;
