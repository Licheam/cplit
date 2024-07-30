//! # Competitive Programming Library
//!
//! # Examples
//! 
//! ```no_run
//! use cplit::scanln;
//!
//! fn main() {
//!     let (a, b): (usize, usize);
//!     scanln!(a, b);
//!     println!("{}", a + b);
//! }
//! ```
#![allow(clippy::needless_doctest_main)]

pub mod data_structure;
pub mod graph;
mod io;
pub mod num;
