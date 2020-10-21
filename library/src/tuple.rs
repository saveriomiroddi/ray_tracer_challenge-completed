use std::ops::{Add, Div, Mul, Neg, Sub};

pub const POINT_TYPE: f64 = 1.0;
pub const VECTOR_TYPE: f64 = 0.0;

pub const EPSILON: f64 = 1e-6;

// At this stage, is not clear is direct operation will be carried also on the type field. If so, using
// an enum may make things more complicated, so conservatively, a float is used.
//
#[derive(Debug)]
pub struct Tuple(pub f64, pub f64, pub f64, pub f64);

impl Tuple {
    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple(x, y, z, POINT_TYPE)
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple(x, y, z, VECTOR_TYPE)
    }

    pub fn magnitude(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2) + self.3.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();

        Self::vector(self.0 / magnitude, self.1 / magnitude, self.2 / magnitude)
    }

    pub fn dot_product(&self, rhs: Tuple) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2 + self.3 * rhs.3
    }

    pub fn cross_product(&self, rhs: Tuple) -> Tuple {
        Tuple::vector(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }
}

impl PartialEq for Tuple {
    // Values are considered as equals if within EPSILON.
    //
    fn eq(&self, rhs: &Self) -> bool {
        ((self.0 - rhs.0).abs() < EPSILON)
            && ((self.1 - rhs.1).abs() < EPSILON)
            && ((self.2 - rhs.2).abs() < EPSILON)
            && ((self.3 - rhs.3).abs() < EPSILON)
    }
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Self) -> Self::Output {
        Tuple(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Self) -> Self::Output {
        Tuple(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Tuple(-self.0, -self.1, -self.2, -self.3)
    }
}

impl Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Self::Output {
        Tuple(self.0 * rhs, self.1 * rhs, self.2 * rhs, self.3 * rhs)
    }
}

impl Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: f64) -> Self::Output {
        Tuple(self.0 / rhs, self.1 / rhs, self.2 / rhs, self.3 / rhs)
    }
}