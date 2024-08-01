use crate::num::Numeric;
// Distance trait for graph algorithms, which is used to calculate the distance defined by the edge.
pub trait Distance<N>
where
    N: Numeric + Copy,
{
    fn dist(&self) -> N;
}

impl<N> Distance<N> for N
where
    N: Numeric + Copy,
{
    fn dist(&self) -> N {
        *self
    }
}
