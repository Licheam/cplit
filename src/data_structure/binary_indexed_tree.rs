use crate::num::{Numeric, NumericAssOps, NumericOps};
use std::ops::{Bound, RangeBounds};
use std::vec;

macro_rules! low_bit {
    ($index: expr) => {
        ($index & (!$index + 1))
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
    body: Vec<N>,
}

impl<N> BinaryIndexedTree<N>
where
    N: Numeric + NumericOps + NumericAssOps + Clone + Copy,
{
    /// Constructs an empty binary indexed tree.
    pub fn new() -> Self {
        Self {
            body: vec![N::ZERO],
        }
    }

    /// Constructs an empty binary indexed tree with the specified `capacity`.
    pub fn with_capacity(capacity: usize) -> Self {
        let mut body = Vec::with_capacity(capacity + 1);
        body.push(N::ZERO);
        Self { body }
    }

    /// The length of the binary indexed tree.
    pub fn len(&self) -> usize {
        self.body.len() - 1
    }

    /// Returns `true` if the binary indexed tree is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Updates the value of the element at `index` by adding `delta`.
    /// Complexity: _O(log n)_.
    pub fn add(&mut self, mut index: usize, delta: N) {
        if !(1..=self.len()).contains(&index) {
            panic!(
                "Index out of bounds: the range is 1..={} but the index is {}",
                self.len(),
                index
            );
        }
        while index <= self.len() {
            self.body[index] += delta;
            index += low_bit!(index);
        }
    }

    /// Returns the sum of the elements in the range `bounds`.
    /// Complexity: _O(log n)_.
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

        if !(1..=self.len() + 1).contains(&start) || !(1..=self.len() + 1).contains(&end) {
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

    /// Pushes a new element into the binary indexed tree.
    /// Complexity: _O(log n)_.
    pub fn push(&mut self, value: N) {
        let len = self.len() + 1;
        let sum = self.sum(len - low_bit!(len) + 1..len);
        self.body.push(value + sum);
    }

    /// Pops the last element from the binary indexed tree.
    /// Complexity: _O(1)_.
    pub fn pop(&mut self) -> Option<N> {
        self.body.pop()
    }

    // Initializes the binary indexed tree.
    fn init(&mut self) {
        for i in 1..=self.len() {
            let j = i + low_bit!(i);
            if j <= self.len() {
                let t = self.body[i];
                self.body[j] += t;
            }
        }
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

impl<N> Default for BinaryIndexedTree<N>
where
    N: Numeric + NumericOps + NumericAssOps + Clone + Copy,
{
    fn default() -> Self {
        Self::new()
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
        println!("{:?}", bit);
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
