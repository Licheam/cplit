use crate::num::{Numeric, NumericAssOps};

#[derive(Debug)]
pub struct BinaryIndexedTree<N>
where
    N: Numeric + NumericAssOps,
{
    b: Vec<N>,
    n: usize,
}

impl<N> BinaryIndexedTree<N>
where
    N: Numeric + NumericAssOps,
{
    pub fn new(n: usize) -> Self {
        Self {
            b: vec![N::ZERO; n + 1],
            n,
        }
    }

    fn lowbit(x: usize) -> usize {
        x & (!x + 1)
    }

    pub fn change(&mut self, mut x: usize, y: N) {
        while x <= self.n {
            self.b[x] += y;
            x += Self::lowbit(x);
        }
    }

    pub fn sum(&self, mut x: usize) -> N {
        let mut s = N::ZERO;
        while x > 0 {
            s += self.b[x];
            x -= Self::lowbit(x);
        }
        s
    }

    pub fn build(&mut self, a: &[N]) {
        for i in 1..=self.n {
            self.b[i] = a[i - 1];
        }
        let mut x = 1;
        while x << 1 <= self.n {
            for i in (x..=self.n - x).step_by(x << 1) {
                let t = self.b[i];
                self.b[i + x] += t;
            }
            x <<= 1;
        }
    }
}
