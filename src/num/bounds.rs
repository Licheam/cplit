/// Numbers which have lower bounds
pub trait LowerBounded {
    /// Returns the smallest finite number this type can represent
    const MIN: Self;
}

/// Numbers which have upper bounds
pub trait UpperBounded {
    /// Returns the largest finite number this type can represent
    const MAX: Self;
}

pub trait Bounded: LowerBounded + UpperBounded {}

macro_rules! bounded_trait_impl {
    ($t:ty, $min:expr, $max:expr) => {
        impl LowerBounded for $t {
            const MIN: Self = $min;
        }

        impl UpperBounded for $t {
            const MAX: Self = $max;
        }
    };
}

bounded_trait_impl!(usize, usize::MIN, usize::MAX);
bounded_trait_impl!(u8, u8::MIN, u8::MAX);
bounded_trait_impl!(u16, u16::MIN, u16::MAX);
bounded_trait_impl!(u32, u32::MIN, u32::MAX);
bounded_trait_impl!(u64, u64::MIN, u64::MAX);
bounded_trait_impl!(u128, u128::MIN, u128::MAX);

bounded_trait_impl!(isize, isize::MIN, isize::MAX);
bounded_trait_impl!(i8, i8::MIN, i8::MAX);
bounded_trait_impl!(i16, i16::MIN, i16::MAX);
bounded_trait_impl!(i32, i32::MIN, i32::MAX);
bounded_trait_impl!(i64, i64::MIN, i64::MAX);
bounded_trait_impl!(i128, i128::MIN, i128::MAX);

bounded_trait_impl!(f32, f32::MIN, f32::MAX);
bounded_trait_impl!(f64, f64::MIN, f64::MAX);
