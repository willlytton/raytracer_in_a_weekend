use std::f64;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug)]
pub struct Vec3 {
    e: [f64; 3], // an array type of float elements with 3 values
}

#[allow(dead_code)]
impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        // Self refers to the instance of the type Vec3
        Vec3 { e: [e0, e1, e2] }
    }

    fn x(&self) {
        // function parameter &self borrows the struct without mutating its value
        self.e[0];
    }

    fn y(&self) {
        self.e[1];
    }

    fn z(&self) {
        self.e[2];
    }

    fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    fn length_squared(&self) -> f64 {
        (self.e[0] * self.e[0] + self.e[1]) * (self.e[1] + self.e[2] * self.e[2])
    }
}

impl Add<Vec3> for Vec3 {
    // implements the Add trait for Vec3
    type Output = Self; // represents the total output of the operation which will be a Vec3
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ],
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] / rhs.e[0],
                self.e[1] / rhs.e[1],
                self.e[2] / rhs.e[2],
            ],
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}
