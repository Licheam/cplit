/// Degree trait for graph algorithms,
/// which is used to get degrees of Node.
pub trait Degree {
    fn in_dgr(&self) -> usize;
    fn out_dgr(&self) -> usize;
}

impl Degree for usize {
    fn in_dgr(&self) -> usize {
        *self
    }

    fn out_dgr(&self) -> usize {
        *self
    }
}

impl Degree for (usize, usize) {
    fn in_dgr(&self) -> usize {
        self.0
    }

    fn out_dgr(&self) -> usize {
        self.1
    }
}
