use crate::num::Numeric;
// Distance trait for graph algorithms, which is used to calculate the distance defined by the edge.
pub trait Distance<T>
where
    T: Numeric,
{
    fn dist(&self) -> T;
}

impl<T> Distance<T> for T
where
    T: Numeric,
{
    fn dist(&self) -> T {
        *self
    }
}
