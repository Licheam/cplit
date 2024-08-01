use crate::num::{Numeric, NumericAssOps, NumericOps};
use std::collections::binary_heap::Iter;
use std::iter::successors;
use std::ops::{Bound, RangeBounds};

macro_rules! low_bit {
    ($idx: expr) => {
        ($idx & (!$idx + 1))
    };
}

/// Binary indexed tree (Fenwick tree) for range sum queries and point updates.
///
/// For more information, see [Fenwick tree](https://cp-algorithms.com/data_structures/fenwick.html).
#[derive(Debug)]
pub struct BinaryIndexedTree<N>
where
    N: Numeric + NumericOps + NumericAssOps + Clone + Copy,
{
    // TODO: Use Box<[N]> instead of Vec<N> to reduce memory usage.
    // However, some boxed slices functions are still in nightly.
    // Consider using Vec<N> for now.
    body: Vec<N>,
}

impl<N> BinaryIndexedTree<N>
where
    N: Numeric + NumericOps + NumericAssOps + Clone + Copy,
{
    /// Constructs a new binary indexed tree with the specified `len` with each element set as N::ZERO.
    pub fn with_len(n: usize) -> Self {
        Self {
            body: vec![N::ZERO; n + 1],
        }
    }

    /// The length of the binary indexed tree.
    pub fn len(&self) -> usize {
        self.body.len() - 1
    }

    /// Updates the value of the element at index `idx` by adding `delta`.
    pub fn add(&mut self, mut idx: usize, delta: N) {
        if (1..=self.len()).contains(&idx) {
            panic!(
                "Index out of bounds: the range is 1..={} but the index is {}",
                self.len(),
                idx
            );
        }
        while idx <= self.len() {
            self.body[idx] += delta;
            idx += low_bit!(idx);
        }
    }

    pub fn sum(&self, bounds: impl RangeBounds<usize>) -> N {
        let mut start = match bounds.start_bound() {
            Bound::Included(&s) => s,
            Bound::Excluded(&s) => s + 1,
            Bound::Unbounded => 1,
        };
        let mut end = match bounds.end_bound() {
            Bound::Included(&e) => e + 1,
            Bound::Excluded(&e) => e,
            Bound::Unbounded => self.len() + 1,
        };

        if !(1..=self.len()).contains(&start) || !(1..=self.len() + 1).contains(&end) {
            panic!(
                "Query out of bounds: the range is 1..={} but the query is {}..{}",
                self.len(),
                start,
                end,
            );
        }
        if start >= end {
            return N::ZERO;
        }

        start -= 1;
        end -= 1;

        let mut s = N::ZERO;
        while end > start {
            s += self.body[end];
            end -= low_bit!(end);
        }
        while start > end {
            s -= self.body[start];
            start -= low_bit!(start);
        }
        s
    }

    fn init(&mut self) {
        let len = self.len();
        successors(Some(1), |&step| Some(step << 1))
            .take_while(|&step| step << 1 <= len)
            .for_each(|step: usize| {
                successors(Some(step), |idx| Some(idx + (step << 1)))
                    .take_while(|&idx| idx + step <= len)
                    .for_each(|idx| {
                        let t = self.body[idx];
                        self.body[idx + step] += t;
                    });
            });
    }
}

impl<N, T> From<T> for BinaryIndexedTree<N>
where
    N: Numeric + NumericOps + NumericAssOps + Clone + Copy,
    T: Into<Vec<N>>,
{
    /// Constructs a new binary indexed tree.
    ///
    /// Complexity: _O(n)_.
    fn from(a: T) -> Self {
        let mut bit = BinaryIndexedTree { body: a.into() };
        bit.init();
        bit
    }
}

#[cfg(test)]
mod tests {
    use crate::data_structure::BinaryIndexedTree;
    use crate::fscanln;
    use std::io::{BufReader, Cursor};

    #[test]
    fn luogu_p3374() {
        let mut reader = BufReader::new(Cursor::new(
            r#"
5 5
1 5 4 2 3
1 1 3
2 2 5
1 3 -1
1 4 2
2 1 4
"#,
        ));

        let (n, m): (usize, usize);
        fscanln!(reader, n, m);
        let v: Vec<isize>;
        fscanln!(reader, v; n);
        let mut bit = BinaryIndexedTree::from(v);
        let mut ans: Vec<_> = vec![];
        for _ in 0..m {
            let (op, x, y): (usize, usize, isize);
            fscanln!(reader, op, x, y);
            match op {
                1 => {
                    bit.add(x, y);
                }
                2 => {
                    ans.push(bit.sum(x..=y as usize));
                }
                _ => unreachable!(),
            }
        }
        assert_eq!(ans, vec![14, 16]);
    }
}
