use std::ops::{Add, AddAssign, BitXor, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::utils::F64;

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Point {
    pub x: F64,
    pub y: F64,
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}

impl Mul<F64> for Point {
    type Output = Point;

    fn mul(self, rhs: F64) -> Self::Output {
        Self {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
        }
    }
}

impl Div<F64> for Point {
    type Output = Point;

    fn div(self, rhs: F64) -> Self::Output {
        Self {
            x: self.x.div(rhs),
            y: self.y.div(rhs),
        }
    }
}

impl Mul<Point> for Point {
    type Output = F64;

    fn mul(self, rhs: Point) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl BitXor<Point> for Point {
    type Output = F64;

    fn bitxor(self, rhs: Point) -> Self::Output {
        self.x * rhs.y - self.y * rhs.x
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
    }
}

impl MulAssign<F64> for Point {
    fn mul_assign(&mut self, rhs: F64) {
        self.x.mul_assign(rhs);
        self.y.mul_assign(rhs);
    }
}

impl DivAssign<F64> for Point {
    fn div_assign(&mut self, rhs: F64) {
        self.x.div_assign(rhs);
        self.y.div_assign(rhs);
    }
}

type Vector = Point;

impl Vector {
    pub fn length(&self) -> F64 {
        F64((self.x * self.x + self.y * self.y).0.sqrt())
    }

    /// Returns the angle between two vectors in radians.
    pub fn angle(&self, other: &Self) -> F64 {
        F64((*self * *other / self.length() / other.length()).0.acos())
    }

    /// Returns the area^2 of the parallelogram spanned by two vectors.
    pub fn area2(&self, other: &Self) -> F64 {
        *self ^ *other
    }

    /// Rotate the vector by rad radians counter-clockwise.
    pub fn rotate(&self, rad: F64) -> Self {
        Self {
            x: self.x * F64(rad.0.cos()) - self.y * F64(rad.0.sin()),
            y: self.x * F64(rad.0.sin()) + self.y * F64(rad.0.cos()),
        }
    }

    /// Returns the unit normal vector with a 90-degree counter-clockwise rotation.
    pub fn normal(&self) -> Self {
        let len = self.length();
        Self {
            x: -self.y / len,
            y: self.x / len,
        }
    }

    /// Judge whether the vector p is on the left of the vector self.
    pub fn to_left_test(&self, p: &Vector) -> bool {
        *self ^ *p > F64(0.0)
    }
}

pub struct Line {
    pub p1: Point,
    pub p2: Point,
}

impl Line {
    pub fn new(p1: Point, p2: Point) -> Self {
        Self { p1, p2 }
    }

    pub fn at(&self, t: F64) -> Point {
        self.p1 + (self.p2 - self.p1) / (self.p2 - self.p1).length() * t
    }

    pub fn on_line(&self, p: &Point) -> bool {
        (*p - self.p1) ^ (self.p2 - self.p1) == F64(0.0)
    }

    pub fn intersects_with_line(&self, other: &Self) -> bool {
        (self.p2 - self.p1) ^ (other.p2 - other.p1) != F64(0.0)
    }

    pub fn get_intersection(&self, other: &Self) -> Option<Point> {
        if !self.intersects_with_line(other) {
            return None;
        }
        Some(
            self.at((other.p2 - other.p1) ^ (self.p1 - other.p1))
                / ((self.p2 - self.p1) ^ (other.p2 - other.p1)),
        )
    }

    pub fn get_distance(&self, p: &Point) -> F64 {
        F64(((self.p2 - self.p1) ^ (*p - self.p1)).0.abs())
    }

    pub fn get_projection(&self, p: &Point) -> Point {
        self.at((*p - self.p1) * (self.p2 - self.p1))
    }
}

type Segment = Line;

impl Segment {
    /// Judge whether the point p is on the segment self.
    pub fn on_segment(&self, p: &Point) -> bool {
        self.on_line(p) && (*p - self.p1) * (*p - self.p2) <= F64(0.0)
    }

    /// Judge whether the point p is strictly on the segment self.
    pub fn on_segment_strict(&self, p: &Point) -> bool {
        self.on_line(p) && (*p - self.p1) * (*p - self.p2) < F64(0.0)
    }
}
