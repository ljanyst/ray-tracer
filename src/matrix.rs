// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::constants::EPSILON;

use std::cmp::{Eq, PartialEq};

#[derive(Debug, Copy, Clone)]
pub struct Matrix {
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
            data: [
                m00, m01, 0.0, 0.0, //
                m10, m11, 0.0, 0.0, //
                0.0, 0.0, 1.0, 0.0, //
                0.0, 0.0, 0.0, 1.0, //
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
            data: [
                m00, m01, m02, 0.0, //
                m10, m11, m12, 0.0, //
                m20, m21, m22, 0.0, //
                0.0, 0.0, 0.0, 1.0, //
            ],
        }
    }

    pub fn at(&self, i: usize, j: usize) -> f64 {
        check_bounds(i, j);
        self.data[idx(i, j)]
    }
}

fn idx(i: usize, j: usize) -> usize {
    j + 4 * i
}

fn check_bounds(i: usize, j: usize) {
    if i >= 4 {
        panic!("i is out of bounds: {}", i);
    }
    if j >= 4 {
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
