pub mod ops;
#[doc(inline)]
pub use self::ops::{AddSum, Operation, OperationPair};

use std::fmt::Debug;
use std::marker::PhantomData;

/// Segment tree for range queries and point updates.
///
/// For more information, see [Segment tree](https://cp-algorithms.com/data_structures/segment_tree.html).
///
/// The interface of the segment tree is a bit redundant for now, may fix it later.
/// 
/// # Example 1
/// ```no_run
/// use cplit::data_structure::{SegmentTree, segment_tree::AddSum};
/// use cplit::scanln;
///
/// fn main() {
///     let (n, m): (usize, usize);
///     scanln!(n, m);
///     let v: Vec<isize>;
///     scanln!(v; n);
///     let mut st = SegmentTree::<isize, isize, AddSum>::from(v);
///     let mut ans = String::new();
///     for _ in 0..m {
///         let (op, x, y, k): (usize, usize, usize, isize);
///         scanln!(op, x, y, k, ?);
///         match op {
///             1 => st.modify(1, 1, n, x, y, k),
///             2 => println!("{}", st.query(1, 1, n, x, y)),
///             _ => unreachable!(),
///         }
///     }
/// }
/// ```
/// # Example 2
/// ```no_run
/// use cplit::data_structure::{SegmentTree, segment_tree::{Operation, AddSum}};
/// use cplit::scanln;
/// 
/// #[derive(Debug)]
/// struct AddMulSum {}
///
/// impl AddMulSum {
///     const MOD_BASE: isize = 571373;
/// }
/// 
/// impl Operation<isize, (isize, isize)> for AddMulSum {
///     const COMBINE: fn(isize, isize) -> isize =
///         |left_val, right_val| (left_val + right_val) % Self::MOD_BASE;
///     const PUSH_VAL: fn(isize, (isize, isize), usize) -> isize = |val, (add, mul), len| {
///         (val * mul % Self::MOD_BASE + add * len as isize % Self::MOD_BASE) % Self::MOD_BASE
///     };
///     const PUSH_TAG: fn((isize, isize), (isize, isize)) -> (isize, isize) =
///         |(child_add, child_mul), (add, mul)| {
///             (
///                 (child_add * mul % Self::MOD_BASE + add) % Self::MOD_BASE,
///                 child_mul * mul % Self::MOD_BASE,
///             )
///         };
///     const TAG_IDENTITY: (isize, isize) = (0, 1);
///     const VAL_IDENTITY: isize = 0;
/// }
///
/// fn main() {
///     let (n, m): (usize, usize);
///     scanln!(n, m);
///     let v: Vec<isize>;
///     scanln!(v; n);
///     let mut st = SegmentTree::<_, _, AddMulSum>::from(v);
///     for _ in 0..m {
///         let (op, x, y, k): (usize, usize, usize, isize);
///         scanln!(op, x, y, k, ?);
///         match op {
///             1 => st.modify(1, 1, n, x, y, (0, k)), // mul
///             2 => st.modify(1, 1, n, x, y, (k, 1)), // add
///             3 => println!("{}", st.query(1, 1, n, x, y)),
///             _ => unreachable!(),
///         }
///     }
/// }
/// ```
#[derive(Debug)]
pub struct SegmentTree<V, T, O>
where
    V: Clone + Copy,
    T: Clone + Copy,
    O: Operation<V, T>,
{
    val: Vec<V>,
    tag: Vec<T>,
    len: usize,
    phantom: PhantomData<O>,
}

impl<V, T, O> SegmentTree<V, T, O>
where
    V: Clone + Copy,
    T: Clone + Copy,
    O: Operation<V, T>,
{
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn pushup(&mut self, x: usize) {
        self.val[x] = O::COMBINE(self.val[x << 1], self.val[x << 1 | 1]);
    }

    fn pushdown(&mut self, x: usize, l: usize, r: usize) {
        let m = (l + r) >> 1;
        self.val[x << 1] = O::PUSH_VAL(self.val[x << 1], self.tag[x], m - l + 1);
        self.val[x << 1 | 1] = O::PUSH_VAL(self.val[x << 1 | 1], self.tag[x], r - m);
        self.tag[x << 1] = O::PUSH_TAG(self.tag[x << 1], self.tag[x]);
        self.tag[x << 1 | 1] = O::PUSH_TAG(self.tag[x << 1 | 1], self.tag[x]);
        self.tag[x] = O::TAG_IDENTITY;
    }

    pub fn modify(&mut self, x: usize, l: usize, r: usize, ql: usize, qr: usize, delta: T) {
        if ql <= l && r <= qr {
            self.val[x] = O::PUSH_VAL(self.val[x], delta, r - l + 1);
            self.tag[x] = O::PUSH_TAG(self.tag[x], delta);
        } else {
            self.pushdown(x, l, r);
            let m = (l + r) >> 1;
            if ql <= m {
                self.modify(x << 1, l, m, ql, qr, delta)
            }
            if m < qr {
                self.modify(x << 1 | 1, m + 1, r, ql, qr, delta)
            }
            self.pushup(x);
        }
    }

    pub fn query(&mut self, x: usize, l: usize, r: usize, ql: usize, qr: usize) -> V {
        if ql <= l && r <= qr {
            self.val[x]
        } else {
            self.pushdown(x, l, r);
            let m = (l + r) >> 1;
            O::COMBINE(
                if ql <= m {
                    self.query(x << 1, l, m, ql, qr)
                } else {
                    O::VAL_IDENTITY
                },
                if m < qr {
                    self.query(x << 1 | 1, m + 1, r, ql, qr)
                } else {
                    O::VAL_IDENTITY
                },
            )
        }
    }

    fn init(&mut self, x: usize, l: usize, r: usize, a: &Vec<V>) {
        if l == r {
            self.val[x] = a[l]
        } else {
            let m = (l + r) >> 1;
            self.init(x << 1, l, m, a);
            self.init(x << 1 | 1, m + 1, r, a);
            self.pushup(x);
        }
    }
}

impl<V, T, O, Q> From<Q> for SegmentTree<V, T, O>
where
    V: Clone + Copy,
    T: Clone + Copy,
    O: Operation<V, T>,
    Q: Into<Vec<V>>,
{
    /// Constructs a new binary indexed tree.
    ///
    /// Complexity: _O(n)_.
    fn from(a: Q) -> Self {
        let v = a.into();
        let len = v.len() - 1;
        let mut st = SegmentTree {
            val: vec![O::VAL_IDENTITY; 1 + (len << 2)],
            tag: vec![O::TAG_IDENTITY; 1 + (len << 2)],
            len,
            phantom: PhantomData,
        };
        st.init(1, 1, len, &v);
        st
    }
}

#[cfg(test)]
mod tests {
    use crate::data_structure::segment_tree::{AddSum, Operation};
    use crate::data_structure::SegmentTree;
    use crate::fscanln;
    use std::io::{BufReader, Cursor};

    #[test]
    fn luogu_p3372() {
        let mut reader = BufReader::new(Cursor::new(
            r#"
5 5
1 5 4 2 3
2 2 4
1 2 3 2
2 3 4
1 1 5 1
2 1 4
"#,
        ));

        let (n, m): (usize, usize);
        fscanln!(reader, n, m);
        let v: Vec<isize>;
        fscanln!(reader, v; n);
        let mut st = SegmentTree::<isize, isize, AddSum>::from(v);
        let mut ans = String::new();
        for _ in 0..m {
            let (op, x, y, k): (usize, usize, usize, isize);
            fscanln!(reader, op, x, y, k, ?);
            match op {
                1 => st.modify(1, 1, n, x, y, k),
                2 => ans.push_str(&format!("{}\n", st.query(1, 1, n, x, y))),
                _ => unreachable!(),
            }
        }
        assert_eq!(ans, "11\n8\n20\n".to_string());
    }

    #[test]
    fn luogu_p3373() {
        #[derive(Debug)]
        struct AddMulSum {}

        impl AddMulSum {
            const MOD_BASE: isize = 571373;
        }

        impl Operation<isize, (isize, isize)> for AddMulSum {
            const COMBINE: fn(isize, isize) -> isize =
                |left_val, right_val| (left_val + right_val) % Self::MOD_BASE;
            const PUSH_VAL: fn(isize, (isize, isize), usize) -> isize = |val, (add, mul), len| {
                (val * mul % Self::MOD_BASE + add * len as isize % Self::MOD_BASE) % Self::MOD_BASE
            };
            const PUSH_TAG: fn((isize, isize), (isize, isize)) -> (isize, isize) =
                |(child_add, child_mul), (add, mul)| {
                    (
                        (child_add * mul % Self::MOD_BASE + add) % Self::MOD_BASE,
                        child_mul * mul % Self::MOD_BASE,
                    )
                };
            const TAG_IDENTITY: (isize, isize) = (0, 1);
            const VAL_IDENTITY: isize = 0;
        }

        let mut reader = BufReader::new(Cursor::new(
            r#"
5 5
1 5 4 2 3
2 1 4 1
3 2 5
1 2 4 2
2 3 5 5
3 1 4
"#,
        ));

        let (n, m): (usize, usize);
        fscanln!(reader, n, m);
        let v: Vec<isize>;
        fscanln!(reader, v; n);
        let mut st = SegmentTree::<_, _, AddMulSum>::from(v);
        let mut ans = String::new();
        for _ in 0..m {
            let (op, x, y, k): (usize, usize, usize, isize);
            fscanln!(reader, op, x, y, k, ?);
            match op {
                1 => st.modify(1, 1, n, x, y, (0, k)), // mul
                2 => st.modify(1, 1, n, x, y, (k, 1)), // add
                3 => ans.push_str(&format!("{}\n", st.query(1, 1, n, x, y))),
                _ => unreachable!(),
            }
        }
        assert_eq!(ans, "17\n40\n".to_string());
    }
}
