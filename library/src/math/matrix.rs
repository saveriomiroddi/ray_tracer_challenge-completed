use super::Tuple;
use crate::{lang::ApproximateFloat64Ops, Axis};

use std::ops::{Index, IndexMut, Mul};

#[derive(Debug)]
pub struct Matrix {
    pub values: Vec<Vec<f64>>,
}

impl Matrix {
    // There isn't an entirely clean structure for the values to pass:
    //
    // - an array of arrays doesn't work, since the size must be known at compile time;
    // - a flat list could work with by appending an empty comment to each line, but as soon as a method
    //   is invoked on an entry (e.g. into()), it alignes vertically.
    // - using slices works, although it's quite ugly.
    //
    // So, screw rustfmt, and just use `#[rustfmt::skip]`.
    //
    pub fn new<T: Copy + Into<f64>>(source_values: &[T]) -> Self {
        let order = (source_values.len() as f64).sqrt() as usize;

        if source_values.len() != order.pow(2) {
            panic!("Number of source values is not a square value");
        }

        let mut values = Vec::with_capacity(order);

        for source_row in source_values.chunks_exact(order) {
            values.push(
                source_row
                    .iter()
                    .map(|value| (*value).into())
                    .collect::<Vec<_>>(),
            );
        }

        Self { values }
    }

    pub fn translation<T: Into<f64>>(x: T, y: T, z: T) -> Self {
        let (x, y, z) = (x.into(), y.into(), z.into());

        #[rustfmt::skip]
        let transformation_values = [
            1.0, 0.0, 0.0, x,
            0.0, 1.0, 0.0, y,
            0.0, 0.0, 1.0, z,
            0.0, 0.0, 0.0, 1.0,
        ];

        Self::new(&transformation_values)
    }

    pub fn scaling<T: Into<f64>>(x: T, y: T, z: T) -> Self {
        let (x, y, z) = (x.into(), y.into(), z.into());

        #[rustfmt::skip]
        let transformation_values = [
            x,   0.0, 0.0, 0.0,
            0.0, y,   0.0, 0.0,
            0.0, 0.0, z,   0.0,
            0.0, 0.0, 0.0, 1.0,
        ];

        Self::new(&transformation_values)
    }

    // r: radians.
    //
    pub fn rotation(axis: Axis, r: f64) -> Self {
        let (cos_r, sin_r) = (r.cos(), r.sin());

        #[rustfmt::skip]
        let transformation_values = match axis {
            Axis::X => [
                1.0, 0.0,   0.0,    0.0,
                0.0, cos_r, -sin_r, 0.0,
                0.0, sin_r, cos_r,  0.0,
                0.0, 0.0,   0.0,    1.0,
            ],
            Axis::Y => [
                cos_r,  0.0, sin_r,  0.0,
                0.0,    1.0, 0.0,    0.0,
                -sin_r, 0.0, cos_r,  0.0,
                0.0,    0.0, 0.0,    1.0,
            ],
            Axis::Z => [
                cos_r,  -sin_r, 0.0, 0.0,
                sin_r,  cos_r,  0.0, 0.0,
                0.0,    0.0,    1.0, 0.0,
                0.0,    0.0,    0.0, 1.0,
            ]
        };

        Self::new(&transformation_values)
    }

    pub fn shearing<T: Into<f64>>(x_py: T, x_pz: T, y_px: T, y_pz: T, z_px: T, z_py: T) -> Self {
        let (x_py, x_pz, y_px, y_pz, z_px, z_py) = (
            x_py.into(),
            x_pz.into(),
            y_px.into(),
            y_pz.into(),
            z_px.into(),
            z_py.into(),
        );

        #[rustfmt::skip]
        let transformation_values = [
            1.0,  x_py, x_pz, 0.0,
            y_px, 1.0,  y_pz, 0.0,
            z_px, z_py, 1.0,  0.0,
            0.0,  0.0,  0.0,  1.0,
        ];

        Self::new(&transformation_values)
    }

    pub fn identity(order: usize) -> Self {
        let mut source_values = vec![0; order.pow(2)];

        for i in 0..order {
            source_values[(order + 1) * i] = 1;
        }

        Self::new(&source_values)
    }

    pub fn view_transform(from: &Tuple, to: &Tuple, up: &Tuple) -> Self {
        let forward = (*to - from).normalize();
        let normalized_up = up.normalize();
        let left = forward.cross_product(normalized_up);
        let true_up = left.cross_product(forward);

        #[rustfmt::skip]
        let orientation_values = [
            left.x,     left.y,     left.z,     0.0,
            true_up.x,  true_up.y,  true_up.z,  0.0,
            -forward.x, -forward.y, -forward.z, 0.0,
            0.0,        0.0,        0.0,        1.0,
        ];

        Self::new(&orientation_values) * &Matrix::translation(-from.x, -from.y, -from.z)
    }

    // Builder APIs.
    //
    // WATCH OUT! The transformation applied in the intuitive, not mathematical, fashion - the transform
    // applied is the left multiplication operand.
    //
    // The methods could be a Shape trait (e.g. `ShapeBuilder: Shape + Sized`), but they'd be less generic,
    // e.g. they couldn't be applied to Camera; both designs are valid.

    pub fn apply_transformation(&self, transform: Matrix) -> Self {
        transform * self
    }

    pub fn scale<T: Into<f64>>(&self, x: T, y: T, z: T) -> Self {
        Matrix::scaling(x, y, z) * self
    }

    pub fn equiscale<T: Into<f64> + Copy>(&self, s: T) -> Self {
        Matrix::scaling(s, s, s) * self
    }

    pub fn translate<T: Into<f64>>(&self, x: T, y: T, z: T) -> Self {
        Matrix::translation(x, y, z) * self
    }

    pub fn rotate(&self, axis: Axis, r: f64) -> Self {
        Matrix::rotation(axis, r) * self
    }

    // Mathematical APIs

    pub fn transpose(&self) -> Self {
        let order = self.values.len();
        let mut result = vec![Vec::with_capacity(order); order];

        for (y, result_row) in result.iter_mut().enumerate() {
            for x in 0..order {
                result_row.push(self[x][y]);
            }
        }

        Self { values: result }
    }

    pub fn determinant(&self) -> f64 {
        if self.values.len() == 2 {
            self[0][0] * self[1][1] - self[0][1] * self[1][0]
        } else {
            self[0]
                .iter()
                .enumerate()
                .map(|(x, value)| value * self.cofactor(0, x))
                .sum()
        }
    }

    pub fn submatrix(&self, y: usize, x: usize) -> Self {
        let order = self.values.len();

        let mut result = Vec::with_capacity(order - 1);

        for current_y in 0..order {
            if current_y != y {
                let mut result_row = Vec::with_capacity(order - 1);

                for current_x in 0..order {
                    if current_x != x {
                        result_row.push(self[current_y][current_x]);
                    }
                }

                result.push(result_row);
            }
        }

        Self { values: result }
    }

    pub fn minor(&self, y: usize, x: usize) -> f64 {
        self.submatrix(y, x).determinant()
    }

    // Mad lulz here. Note that for portability, the bit shift should change depending on the arch.
    //
    pub fn cofactor(&self, y: usize, x: usize) -> f64 {
        let minor = self.minor(y, x);

        // The data type is irrelevant here, as long as it supports bit shifts (float doesn't).
        // usize is used for convenience on the next operation.
        //
        let minor_bits = minor.to_bits();

        // This is (0 for even/1 for odd), shifted to be the leftmost bit, so that it's in the sign position
        // of f64 values.
        //
        let sign_bits = ((x + y) << 63) as u64;

        // Xor keeps the <destination sign> if the <sign operand> is 0, and changes it, if the <sign operand> is 1.
        //
        f64::from_bits(minor_bits ^ sign_bits)
    }

    pub fn inverse(&self) -> Self {
        let determinant = self.determinant();

        if determinant == 0.0 {
            panic!("The matrix has zero determinant!")
        } else {
            let order = self.values.len();

            let result = (0..order)
                .map(|y| {
                    (0..order)
                        // WATCH OUT! row/col inversion here.
                        //
                        .map(|x| self.cofactor(x, y) / determinant)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            Self { values: result }
        }
    }
}

impl Index<usize> for Matrix {
    type Output = Vec<f64>;

    fn index(&self, y: usize) -> &Self::Output {
        &self.values[y]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, y: usize) -> &mut Vec<f64> {
        &mut self.values[y]
    }
}

// Due to the epsilon handling, we can't use a direct/bitwise comparison.
//
impl PartialEq for Matrix {
    fn eq(&self, rhs: &Self) -> bool {
        self.values
            .iter()
            .zip(rhs.values.iter())
            .all(|(row, rhs_row)| {
                row.iter()
                    .zip(rhs_row.iter())
                    .all(|(value, rhs_value)| value.approximate_equals(*rhs_value))
            })
    }
}

// It would be more readable just to have the owned version of the operations, with a borrowed one for
// the exceptional cases.

impl Mul<&Matrix> for Matrix {
    type Output = Self;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        &self * rhs
    }
}

impl Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        let order = self.values.len();

        let values = (0..order)
            .map(|y| {
                //
                (0..order)
                    .map(|x| {
                        //
                        (0..order).map(|k| self[y][k] * rhs[k][x]).sum()
                    })
                    .collect::<Vec<f64>>()
                //
            })
            .collect::<Vec<Vec<f64>>>();

        Self::Output { values }
    }
}

impl Mul<&Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, rhs: &Tuple) -> Self::Output {
        &self * rhs
    }
}

impl Mul<&Tuple> for &Matrix {
    type Output = Tuple;

    fn mul(self, rhs: &Tuple) -> Self::Output {
        let order = self.values.len();

        if order != 4 {
            panic!("Only matrices of order 4 are allowed to be multiplied by a Tuple");
        }

        let mut result = Tuple {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };

        for y in 0..order {
            result[y] = (0..order).map(|k| self[y][k] * rhs[k]).sum();
        }

        result
    }
}
