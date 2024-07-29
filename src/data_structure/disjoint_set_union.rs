//! A disjoint-set data structure, also called a union–find data structure or merge–find set.

#[derive(Debug)]
pub struct DisjointSetUnion {
    fa: Vec<usize>,
}

impl DisjointSetUnion {
    pub fn new(n: usize) -> Self {
        let fa = (0..=n).collect();
        DisjointSetUnion { fa }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.fa[x] == x {
            x
        } else {
            self.fa[x] = self.find(self.fa[x]);
            self.fa[x]
        }
    }

    pub fn join(&mut self, x: usize, y: usize) {
        let fx = self.find(x);
        self.fa[fx] = self.find(y);
    }
}
