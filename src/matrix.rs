// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::constants::EPSILON;
use crate::tuple::Tuple;

use std::cmp::{Eq, PartialEq};
use std::ops::Mul;

#[derive(Debug, Copy, Clone)]
pub struct Matrix {
    size: usize,
    data: [f64; 16],
}

impl Matrix {
    pub fn new(
        m00: f64,
        m01: f64,
        m02: f64,
        m03: f64,
        m10: f64,
        m11: f64,
        m12: f64,
        m13: f64,
        m20: f64,
        m21: f64,
        m22: f64,
        m23: f64,
        m30: f64,
        m31: f64,
        m32: f64,
        m33: f64,
    ) -> Matrix {
        Self {
            size: 4,
            data: [
                m00, m01, m02, m03, //
                m10, m11, m12, m13, //
                m20, m21, m22, m23, //
                m30, m31, m32, m33, //
            ],
        }
    }

    pub fn new2(m00: f64, m01: f64, m10: f64, m11: f64) -> Matrix {
        Self {
            size: 2,
            data: [
                m00, m01, 0.0, 0.0, //
                m10, m11, 0.0, 0.0, //
                0.0, 0.0, 0.0, 0.0, //
                0.0, 0.0, 0.0, 0.0, //
            ],
        }
    }

    pub fn new3(
        m00: f64,
        m01: f64,
        m02: f64,
        m10: f64,
        m11: f64,
        m12: f64,
        m20: f64,
        m21: f64,
        m22: f64,
    ) -> Matrix {
        Self {
            size: 3,
            data: [
                m00, m01, m02, 0.0, //
                m10, m11, m12, 0.0, //
                m20, m21, m22, 0.0, //
                0.0, 0.0, 0.0, 0.0, //
            ],
        }
    }

    pub fn zero() -> Matrix {
        Self {
            size: 4,
            data: [
                0.0, 0.0, 0.0, 0.0, //
                0.0, 0.0, 0.0, 0.0, //
                0.0, 0.0, 0.0, 0.0, //
                0.0, 0.0, 0.0, 0.0, //
            ],
        }
    }

    pub fn one() -> Matrix {
        Self {
            size: 4,
            data: [
                1.0, 0.0, 0.0, 0.0, //
                0.0, 1.0, 0.0, 0.0, //
                0.0, 0.0, 1.0, 0.0, //
                0.0, 0.0, 0.0, 1.0, //
            ],
        }
    }

    pub fn at(&self, i: usize, j: usize) -> f64 {
        check_bounds(i, j, self.size);
        self.data[idx(i, j)]
    }

    pub fn transposed(&self) -> Matrix {
        let mut res = Matrix::zero();
        for i in 0..4 {
            for j in 0..4 {
                res.data[idx(j, i)] = self.data[idx(i, j)];
            }
        }
        res
    }

    pub fn det(&self) -> f64 {
        if self.size == 2 {
            return self.at(0, 0) * self.at(1, 1) - self.at(0, 1) * self.at(1, 0);
        }
        let mut d = 0.0;
        for i in 0..self.size {
            d += self.data[idx(0, i)] * self.cofactor(0, i);
        }
        d
    }

    /// Determinant of the submatrix with row i and column j removed
    pub fn minor(&self, i: usize, j: usize) -> f64 {
        let m = self.submatrix(i, j);
        m.det()
    }

    pub fn cofactor(&self, i: usize, j: usize) -> f64 {
        let m = self.minor(i, j);
        if (i + j) % 2 == 1 {
            return -m;
        }
        m
    }

    /// Create a copy of the current matrix with row i and column j removed
    pub fn submatrix(&self, i: usize, j: usize) -> Matrix {
        check_bounds(i, j, self.size);
        let mut res = Matrix::zero();
        res.size = self.size - 1;

        let mut i2 = 0_usize;

        for i1 in 0..self.size {
            if i1 == i {
                continue;
            }

            let mut j2 = 0_usize;
            for j1 in 0..self.size {
                if j1 == j {
                    continue;
                }
                res.data[idx(i2, j2)] = self.data[idx(i1, j1)];
                j2 += 1;
            }
            i2 += 1;
        }
        res
    }
}

fn idx(i: usize, j: usize) -> usize {
    j + 4 * i
}

fn check_bounds(i: usize, j: usize, dims: usize) {
    if i >= dims {
        panic!("i is out of bounds: {}", i);
    }
    if j >= dims {
        panic!("j is out of bounds: {}", j);
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..16 {
            if (self.data[i] - other.data[i]).abs() > EPSILON {
                return false;
            }
        }
        true
    }
}

impl Eq for Matrix {}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, other: Matrix) -> Self::Output {
        let mut res = Matrix::zero();
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    res.data[idx(i, j)] += self.data[idx(i, k)] * other.data[idx(k, j)]
                }
            }
        }
        res
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Self::Output {
        let mut res = Tuple::zero_vector();
        for i in 0..4 {
            let mut v = 0.0_f64;
            for j in 0..4 {
                v += self.data[idx(i, j)] * other.at(j)
            }
            res.set(i, v);
        }
        res
    }
}
