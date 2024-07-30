//! Data structures.
//! 
//! This module contains data structures that are useful in competitive programming.
//! 
//! For now, the following data structures are available:
//! 
//! [`DisjointSetUnion`], [`SegmentTree`], [BinaryIndexedTree]

pub mod disjoint_set_union;
pub mod segment_tree;
pub mod binary_indexed_tree;

#[doc(inline)]
pub use self::disjoint_set_union::DisjointSetUnion;
#[doc(inline)]
pub use self::segment_tree::SegmentTree;
#[doc(inline)]
pub use self::binary_indexed_tree::BinaryIndexedTree;
