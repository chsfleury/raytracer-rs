use std::ops::{Add, Sub};
use crate::linalg::math;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Tuple4(pub f64, pub f64, pub f64, pub f64);

impl Tuple4 {
    pub fn is_point(&self) -> bool {
        self.3 == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.3 == 0.0
    }

    pub fn equals(self, other: Tuple4, epsilon: f64) -> bool {
        math::equals(self.0, other.0, epsilon)
            && math::equals(self.1, other.1, epsilon)
            && math::equals(self.2, other.2, epsilon)
            && math::equals(self.3, other.3, epsilon)
    }
}

impl Add for Tuple4 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Tuple4(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
        )
    }
}

impl Sub for Tuple4 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Tuple4(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
            self.3 - other.3,
        )
    }
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple4 {
    Tuple4(x, y, z, 1.0)
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple4 {
    Tuple4(x, y, z, 0.0)
}