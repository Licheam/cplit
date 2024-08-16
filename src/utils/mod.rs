use crate::num::{One, Zero};
use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait Flag {
    fn set(&mut self, val: bool);
    fn get(&self) -> bool;
}

impl Flag for bool {
    fn set(&mut self, val: bool) {
        *self = val;
    }

    fn get(&self) -> bool {
        *self
    }
}

static mut EPSILON: F64 = F64(1e-7);

pub fn set_epsilon(val: F64) {
    unsafe {
        EPSILON = val;
    }
}

pub fn get_epsilon() -> F64 {
    unsafe { EPSILON }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct F64(pub f64);

macro_rules! ops_trait_impl {
    ($($name:ident $fun:tt),* for $t:ty) => ($(
        impl $name for $t {
            type Output = Self;

            fn $fun(self, rhs: Self) -> Self::Output {
                Self(self.0.$fun(rhs.0))
            }
        }
    )*)
}

ops_trait_impl!(Add add, Sub sub, Mul mul, Div div for F64);

macro_rules! ass_ops_trait_impl {
    ($($name:ident $fun:tt),* for $t:ty) => ($(
        impl $name for $t {
            fn $fun(&mut self, rhs: Self) {
                self.0.$fun(rhs.0)
            }
        }
    )*)
}

ass_ops_trait_impl!(AddAssign add_assign, SubAssign sub_assign, MulAssign mul_assign, DivAssign div_assign for F64);

impl Neg for F64 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl PartialEq for F64 {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < get_epsilon().0
    }
}

impl PartialOrd for F64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.0.partial_cmp(&other.0) {
            None => None,
            Some(Ordering::Equal) => Some(Ordering::Equal),
            Some(order) => {
                if *self == *other {
                    Some(Ordering::Equal)
                } else {
                    Some(order)
                }
            }
        }
    }
}

impl Zero for F64 {
    const ZERO: Self = F64(0.0_f64);
}

impl One for F64 {
    const ONE: Self = F64(1.0_f64);
}

