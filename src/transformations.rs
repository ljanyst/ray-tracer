// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::matrix::{Matrix, MatrixBuilder};
use crate::tuple::Tuple;

pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
    let mut res = Matrix::one();
    res.set(0, 3, x);
    res.set(1, 3, y);
    res.set(2, 3, z);
    res
}

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
    let mut res = Matrix::one();
    res.set(0, 0, x);
    res.set(1, 1, y);
    res.set(2, 2, z);
    res
}

/// Left-handed rotation along the x-axis
pub fn rotation_x(r: f64) -> Matrix {
    let mut res = Matrix::one();
    res.set(1, 1, r.cos());
    res.set(1, 2, -r.sin());
    res.set(2, 1, r.sin());
    res.set(2, 2, r.cos());
    res
}

/// Left-handed rotation along the y-axis
pub fn rotation_y(r: f64) -> Matrix {
    let mut res = Matrix::one();
    res.set(0, 0, r.cos());
    res.set(0, 2, r.sin());
    res.set(2, 0, -r.sin());
    res.set(2, 2, r.cos());
    res
}

/// Left-handed rotation along the z-axis
pub fn rotation_z(r: f64) -> Matrix {
    let mut res = Matrix::one();
    res.set(0, 0, r.cos());
    res.set(0, 1, -r.sin());
    res.set(1, 0, r.sin());
    res.set(1, 1, r.cos());
    res
}

pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
    let mut res = Matrix::one();
    res.set(0, 1, xy);
    res.set(0, 2, xz);
    res.set(1, 0, yx);
    res.set(1, 2, yz);
    res.set(2, 0, zx);
    res.set(2, 1, zy);
    res
}

pub fn view_transform(from: Tuple, to: Tuple, up: Tuple) -> Matrix {
    let forward = (to - from).normalized();
    let left = forward.cross(&up.normalized());
    let true_up = left.cross(&forward);
    MatrixBuilder::new(4)
        .row(&[left.x(), left.y(), left.z(), 0.0])
        .row(&[true_up.x(), true_up.y(), true_up.z(), 0.0])
        .row(&[-forward.x(), -forward.y(), -forward.z(), 0.0])
        .row(&[0.0, 0.0, 0.0, 1.0])
        .matrix()
        * translation(-from.x(), -from.y(), -from.z())
}
