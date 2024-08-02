use crate::num::{Bounded, Numeric, NumericCmpOps, NumericOps};
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
