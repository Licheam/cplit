use crate::num::{Numeric, NumericOps};
use std::convert::TryFrom;
use std::fmt::Debug;
use std::marker::PhantomData;

pub trait Operation<V, T>
where
    V: Clone + Copy,
    T: Clone + Copy,
{
    /// Combines two values from left and right child.
    const COMBINE: fn(left_val: V, right_val: V) -> V;

    /// Pushes the tag down to the left or right child.
    const PUSH_VAL: fn(val: V, tag: T, len: usize) -> V;

    /// Pushes the tag down to the child tag.
    const PUSH_TAG: fn(child_tag: T, tag: T) -> T;

    /// Identity value for the buffer.
    const VAL_IDENTITY: V;

    /// Identity value for the tag.
    const TAG_IDENTITY: T;
}

#[derive(Debug)]
pub struct AddSum;

impl<V> Operation<V, V> for AddSum
where
    V: Numeric + NumericOps + Clone + Copy + TryFrom<usize>,
    <V as TryFrom<usize>>::Error: Debug,
{
    const COMBINE: fn(V, V) -> V = |left_val, right_val| left_val + right_val;
    const PUSH_VAL: fn(V, V, usize) -> V =
        |val, tag, len| val + tag * V::try_from(len).unwrap();
    const PUSH_TAG: fn(V, V) -> V = |child_tag, tag| child_tag + tag;
    const TAG_IDENTITY: V = V::ZERO;
    const VAL_IDENTITY: V = V::ZERO;
}

pub struct OperationPair<V1, V2, T1, T2, O1, O2>
where
    V1: Clone + Copy,
    V2: Clone + Copy,
    T1: Clone + Copy,
    T2: Clone + Copy,
    O1: Operation<V1, T1>,
    O2: Operation<V2, T2>,
{
    _phantoms: (
        PhantomData<V1>,
        PhantomData<V2>,
        PhantomData<T1>,
        PhantomData<T2>,
        PhantomData<O1>,
        PhantomData<O2>,
    ),
}

// impl<N1, N2, O1, O2> Operation<(N1, N2)> for OperationPair<N1, N2, O1, O2>
// where
//     N1: Clone + Copy,
//     N2: Clone + Copy,
//     O1: Operation<N1>,
//     O2: Operation<N2>,
// {
//     const COMBINE: fn((N1, N2), (N1, N2)) -> (N1, N2) =
//         |(left_child1, left_child2), (right_child1, right_child2)| {
//             (
//                 O1::COMBINE(left_child1, right_child1),
//                 O2::COMBINE(left_child2, right_child2),
//             )
//         };
//     const PUSH_VAL: fn((N1, N2), (N1, N2), usize) -> (N1, N2) =
//         |(child1, child2), (tag1, tag2), len| {
//             (
//                 O1::PUSH_VAL(child1, tag1, len),
//                 O2::PUSH_VAL(child2, tag2, len),
//             )
//         };
//     const PUSH_TAG: fn((N1, N2), (N1, N2)) -> (N1, N2) =
//         |(child_tag1, child_tag2), (tag1, tag2)| {
//             (
//                 O1::PUSH_TAG(child_tag1, tag1),
//                 O2::PUSH_TAG(child_tag2, tag2),
//             )
//         };
//     const TAG_IDENTITY: (N1, N2) = (O1::TAG_IDENTITY, O2::TAG_IDENTITY);
//     const VAL_IDENTITY: (N1, N2) = (O1::VAL_IDENTITY, O2::VAL_IDENTITY);
// }
