pub mod ops;
#[doc(inline)]
pub use self::ops::{Operation, OperationPair, Sum};

use crate::num::{Numeric, NumericAssOps, NumericOps, Zero};
use std::fmt::Debug;
use std::marker::PhantomData;

/// Segment tree for range queries and point updates. (Quite early version, certain interfaces may be revised.)
///
/// For more information, see [Segment tree](https://cp-algorithms.com/data_structures/segment_tree.html).
#[derive(Debug)]
pub struct SegmentTree<N, O>
where
    N: Zero + Clone + Copy,
    O: Operation<N>,
{
    buf: Vec<N>,
    tag: Vec<N>,
    len: usize,
    phantom: PhantomData<O>,
}

impl<N, O> SegmentTree<N, O>
where
    N: Zero + Clone + Copy,
    O: Operation<N>,
{
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn pushup(&mut self, x: usize) {
        self.buf[x] = O::COMBINE(self.buf[x << 1], self.buf[x << 1 | 1]);
    }

    fn pushdown(&mut self, x: usize, l: usize, r: usize) {
        let m = (l + r) >> 1;
        self.buf[x << 1] = O::PUSH_BUF(self.buf[x << 1], self.tag[x], m - l + 1);
        self.tag[x << 1] = O::PUSH_TAG(self.tag[x << 1], self.tag[x]);
        self.buf[x << 1 | 1] = O::PUSH_BUF(self.buf[x << 1 | 1], self.tag[x], r - m);
        self.tag[x << 1 | 1] = O::PUSH_TAG(self.tag[x << 1 | 1], self.tag[x]);
        self.tag[x] = N::ZERO;
    }

    pub fn modify(&mut self, x: usize, l: usize, r: usize, ql: usize, qr: usize, delta: N) {
        if ql <= l && r <= qr {
            self.buf[x] = O::PUSH_BUF(self.buf[x], delta, r - l + 1);
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

    pub fn query(&mut self, x: usize, l: usize, r: usize, ql: usize, qr: usize) -> N {
        if ql <= l && r <= qr {
            self.buf[x]
        } else {
            self.pushdown(x, l, r);
            let m = (l + r) >> 1;
            O::COMBINE(
                if ql <= m {
                    self.query(x << 1, l, m, ql, qr)
                } else {
                    N::ZERO
                },
                if m < qr {
                    self.query(x << 1 | 1, m + 1, r, ql, qr)
                } else {
                    N::ZERO
                },
            )
        }
    }

    fn init(&mut self, x: usize, l: usize, r: usize, a: &Vec<N>) {
        if l == r {
            self.buf[x] = a[l]
        } else {
            let m = (l + r) >> 1;
            self.init(x << 1, l, m, a);
            self.init(x << 1 | 1, m + 1, r, a);
            self.pushup(x);
        }
    }
}

impl<N, O, T> From<T> for SegmentTree<N, O>
where
    N: Numeric + NumericOps + NumericAssOps + Clone + Copy,
    O: Operation<N>,
    T: Into<Vec<N>>,
{
    /// Constructs a new binary indexed tree.
    ///
    /// Complexity: _O(n)_.
    fn from(a: T) -> Self {
        let v = a.into();
        let len = v.len() - 1;
        let mut st = SegmentTree {
            buf: vec![N::ZERO; 1 + (len << 2)],
            tag: vec![N::ZERO; 1 + (len << 2)],
            len,
            phantom: PhantomData,
        };
        st.init(1, 1, len, &v);
        st
    }
}

#[cfg(test)]
mod tests {
    use crate::data_structure::{SegmentTree, Sum};
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
        let mut st = SegmentTree::<isize, Sum>::from(v);
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
}
