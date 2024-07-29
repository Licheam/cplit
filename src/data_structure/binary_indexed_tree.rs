#[derive(Debug)]
pub struct BinaryIndexedTree {
    b: Vec<isize>,
    n: usize,
}

impl BinaryIndexedTree {
    pub fn new(n: usize) -> Self {
        Self {
            b: vec![0; n+1],
            n
        }
    }

    fn lowbit(x: usize) -> usize {
        x & (!x + 1)
    }

    pub fn change(&mut self, mut x: usize, y: isize) {
        let BinaryIndexedTree { b, n } = self;
        while x <= *n {
            b[x] += y;
            x += Self::lowbit(x);
        }
    }

    pub fn sum(&self, mut x: usize) -> isize {
        let BinaryIndexedTree { b, .. } = &self;
        let mut s: isize = 0;
        while x > 0 {
            s += b[x];
            x -= Self::lowbit(x);
        }
        s
    }

    pub fn build(&mut self, a: &Vec<isize>) {
        let BinaryIndexedTree { b, n } = self;
        for i in 1..=*n {
            b[i] = a[i-1];
        }
        let mut x = 1;
        while x << 1 <= *n {
            for i in (x..=*n-x).step_by(x << 1) {
                b[i+x] += b[i];
            }
            x <<= 1;
        }
    }
}