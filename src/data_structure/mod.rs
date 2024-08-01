//! Data Structures
//!
//! This module contains data structures.
//!
//! For now, the following data structures are available:
//!
//! [`DisjointSetUnion`], [`SegmentTree`], [`BinaryIndexedTree`]

pub mod binary_indexed_tree;
pub mod disjoint_set_union;
pub mod segment_tree;

#[doc(inline)]
pub use self::binary_indexed_tree::BinaryIndexedTree;
#[doc(inline)]
pub use self::disjoint_set_union::DisjointSetUnion;
#[doc(inline)]
pub use self::segment_tree::{SegmentTree, Operation, OperationPair, Sum};
