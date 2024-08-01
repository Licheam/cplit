use crate::num::{Numeric, NumericOps, Zero};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::convert::TryFrom;

pub trait Operation<N>
where
    N: Zero + Clone + Copy,
{
    /// Combines two values from left and right child.
    const COMBINE: fn(left_child: N, right_child: N) -> N;

    /// Pushes the tag down to the left or right child.
    const PUSH_BUF: fn(child: N, tag: N, len: usize) -> N;
    const PUSH_TAG: fn(child_tag: N, tag: N) -> N;
}

pub struct Sum;

impl<N> Operation<N> for Sum
where
    N: Numeric + NumericOps + Clone + Copy + TryFrom<usize>,
    <N as TryFrom<usize>>::Error: Debug,
{
    const COMBINE: fn(N, N) -> N = |left_child, right_child| left_child + right_child;
    const PUSH_BUF: fn(N, N, usize) -> N =
        |child, tag, len| child + tag * N::try_from(len).unwrap();
    const PUSH_TAG: fn(N, N) -> N = |child_tag, tag| child_tag + tag;
}

pub struct OperationPair<N1, N2, O1, O2>
where
    N1: Zero + Clone + Copy,
    N2: Zero + Clone + Copy,
    O1: Operation<N1>,
    O2: Operation<N2>,
{
    _phantoms: (
        PhantomData<N1>,
        PhantomData<N2>,
        PhantomData<O1>,
        PhantomData<O2>,
    ),
}

impl<N1, N2, O1, O2> Operation<(N1, N2)> for OperationPair<N1, N2, O1, O2>
where
    N1: Zero + Clone + Copy,
    N2: Zero + Clone + Copy,
    O1: Operation<N1>,
    O2: Operation<N2>,
{
    const COMBINE: fn((N1, N2), (N1, N2)) -> (N1, N2) =
        |(left_child1, left_child2), (right_child1, right_child2)| {
            (
                O1::COMBINE(left_child1, right_child1),
                O2::COMBINE(left_child2, right_child2),
            )
        };
    const PUSH_BUF: fn((N1, N2), (N1, N2), usize) -> (N1, N2) =
        |(child1, child2), (tag1, tag2), len| {
            (
                O1::PUSH_BUF(child1, tag1, len),
                O2::PUSH_BUF(child2, tag2, len),
            )
        };
    const PUSH_TAG: fn((N1, N2), (N1, N2)) -> (N1, N2) =
        |(child_tag1, child_tag2), (tag1, tag2)| {
            (
                O1::PUSH_TAG(child_tag1, tag1),
                O2::PUSH_TAG(child_tag2, tag2),
            )
        };
}
