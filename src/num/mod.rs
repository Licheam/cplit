//! Numeric traits for generic algorithms.
//!
//!
use core::cmp::{Eq, Ord, PartialEq, PartialOrd};
use core::ops::{Add, Div, Mul, Rem, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
pub mod bounds;
#[doc(inline)]
pub use bounds::{Bounded, LowerBounded, UpperBounded};

/// The base trait for numeric types
pub trait Numeric: Default + Copy + Clone + PartialEq + Zero + One {}

impl<T> Numeric for T where T: Default + Copy + Clone + PartialEq + Zero + One {}

/// Generic trait for types implementing basic numeric operations
///
/// This is automatically implemented for types which implement the operators.
pub trait NumericOps<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output>
    + Sub<Rhs, Output = Output>
    + Mul<Rhs, Output = Output>
    + Div<Rhs, Output = Output>
    + Rem<Rhs, Output = Output>
{
}

impl<T, Rhs, Output> NumericOps<Rhs, Output> for T where
    T: Add<Rhs, Output = Output>
        + Sub<Rhs, Output = Output>
        + Mul<Rhs, Output = Output>
        + Div<Rhs, Output = Output>
        + Rem<Rhs, Output = Output>
{
}

/// Generic trait for types implementing numeric assignment operators (like `+=`).
///
/// This is automatically implemented for types which implement the operators.
pub trait NumericAssOps<Rhs = Self>:
    AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs> + RemAssign<Rhs>
{
}

impl<T, Rhs> NumericAssOps<Rhs> for T where
    T: AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs> + RemAssign<Rhs>
{
}

pub trait NumericCmpOps<Rhs = Self>: PartialEq<Rhs> + PartialOrd<Rhs> + Eq + Ord {}

impl<T, Rhs> NumericCmpOps<Rhs> for T where T: PartialEq<Rhs> + PartialOrd<Rhs> + Eq + Ord {}

pub trait Zero {
    const ZERO: Self;
}

pub trait One {
    const ONE: Self;
}

macro_rules! zero_trait_impl {
    ($name:ident for $($t:ty)*) => ($(
        impl $name for $t {
            const ZERO: Self = 0 as $t;
        }
    )*)
}

zero_trait_impl!(Zero for usize u8 u16 u32 u64 u128);
zero_trait_impl!(Zero for isize i8 i16 i32 i64 i128);
zero_trait_impl!(Zero for f32 f64);

macro_rules! one_trait_impl {
    ($name:ident for $($t:ty)*) => ($(
        impl $name for $t {
            const ONE: Self = 1 as $t;
        }
    )*)
}

one_trait_impl!(One for usize u8 u16 u32 u64 u128);
one_trait_impl!(One for isize i8 i16 i32 i64 i128);
one_trait_impl!(One for f32 f64);
