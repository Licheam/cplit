//! # Competitive Programming Library
//!
//! This library aims to provide a set of algorithm, data structure and tools for competitive programming.
//!
//! There are several philosophies behind this library:
//! - **Simple**: The library should be simple and fast to use, as time is precious in competitive programming.
//!     - Do not overuse Option, Result to provide unnecessary boundary checks (panic is fine).
//! - **Extensible**: Most of algorithms and data structures should not be limited to a specific type.
//!     - Design genetic traits to allow different types to be used (including potentially user-defined types like matrices).
//! - **Efficient**: The library should be efficient enough for competitive programming problems.
//! - **Comprehensive**: The library should be readable and understandable for educational purposes.
//! - **Tested**: The library should be tested with some problems from online judges to ensure its correctness.
//! - **Indexed from _1_**: The library should use 1-based index for most of the algorithms and data structures while leaving the index 0 for buffer manipulation.
//!     - This is because most of the problem use 1-based index.
//!     - Index 0 should default be set to ZERO.
//!
//!
//! ## Examples
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
pub mod general;
pub mod geometry;
pub mod graph;
pub mod num;
pub mod number_theory;
pub mod utils;

#[macro_use]
mod macros;
