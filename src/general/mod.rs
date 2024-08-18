//! General purpose algorithms.

use crate::num::{Bounded, Numeric, NumericCmpOps, NumericOps};
use std::cmp::{Ord, Ordering};
use std::ops::{Bound, RangeBounds};

/// Finds the smallest number `x` in the specified `bounds` such that `f(x) == true`.
pub fn binary_search<N>(bounds: impl RangeBounds<N>, f: impl Fn(N) -> bool) -> N
where
    N: Numeric + NumericOps + NumericCmpOps + Copy + Clone + Bounded,
{
    let mut left = match bounds.start_bound() {
        Bound::Included(&s) => s,
        Bound::Excluded(&s) => s + N::ONE,
        Bound::Unbounded => N::MIN,
    };
    let mut right = match bounds.end_bound() {
        Bound::Included(&e) => e + N::ONE,
        Bound::Excluded(&e) => e,
        Bound::Unbounded => N::MAX,
    };

    while left < right {
        let mid = (left + right) / (N::ONE + N::ONE);
        if f(mid) {
            right = mid;
        } else {
            left = mid + N::ONE;
        }
    }

    left
}

/// Get the next permutation of the specified slice.
pub fn next_permutation<T>(arr: &mut [T]) -> bool
where
    T: Ord,
{
    if arr.len() == 0 {
        return false;
    }

    if let Some(i) = (0..arr.len() - 1).rev().find(|&i| arr[i] < arr[i + 1]) {
        (i + 1..arr.len())
            .rev()
            .find(|&j| arr[i] < arr[j])
            .map(|j| {
                arr.swap(i, j);
            });
        arr[i + 1..].reverse();
        true
    } else {
        arr.reverse();
        false
    }
}
