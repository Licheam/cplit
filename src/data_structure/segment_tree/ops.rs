use crate::num::{Numeric, NumericOps};
use std::convert::TryFrom;
use std::fmt::Debug;
use std::marker::PhantomData;

/// The abstract operation for segment tree.
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
    const PUSH_VAL: fn(V, V, usize) -> V = |val, tag, len| val + tag * V::try_from(len).unwrap();
    const PUSH_TAG: fn(V, V) -> V = |child_tag, tag| child_tag + tag;
    const TAG_IDENTITY: V = V::ZERO;
    const VAL_IDENTITY: V = V::ZERO;
}

/// Cartesian product of two operations.
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

impl<V1, V2, T1, T2, O1, O2> OperationPair<V1, V2, T1, T2, O1, O2>
where
    V1: Clone + Copy,
    V2: Clone + Copy,
    T1: Clone + Copy,
    T2: Clone + Copy,
    O1: Operation<V1, T1>,
    O2: Operation<V2, T2>,
{
    const COMBINE: fn(left_val: (V1, V2), right_val: (V1, V2)) -> (V1, V2) =
        |left_val, right_val| {
            (
                O1::COMBINE(left_val.0, right_val.0),
                O2::COMBINE(left_val.1, right_val.1),
            )
        };

    const PUSH_VAL: fn((V1, V2), (T1, T2), usize) -> (V1, V2) = |val, tag, len| {
        (
            O1::PUSH_VAL(val.0, tag.0, len),
            O2::PUSH_VAL(val.1, tag.1, len),
        )
    };

    const PUSH_TAG: fn((T1, T2), (T1, T2)) -> (T1, T2) = |child_tag, tag| {
        (
            O1::PUSH_TAG(child_tag.0, tag.0),
            O2::PUSH_TAG(child_tag.1, tag.1),
        )
    };

    const TAG_IDENTITY: (T1, T2) = (O1::TAG_IDENTITY, O2::TAG_IDENTITY);

    const VAL_IDENTITY: (V1, V2) = (O1::VAL_IDENTITY, O2::VAL_IDENTITY);
}
