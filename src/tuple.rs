// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::utils::feq;

use std::cmp::{Eq, PartialEq};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone, Default)]
pub struct Tuple {
    data: [f64; 4],
}

impl Tuple {
    pub const fn zero_point() -> Tuple {
        Self {
            data: [0.0, 0.0, 0.0, 1.0],
        }
    }

    pub const fn zero_vector() -> Tuple {
        Self {
            data: [0.0, 0.0, 0.0, 0.0],
        }
    }

    pub const fn zero_color() -> Tuple {
        Tuple::zero_vector()
    }

    pub const fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Self { data: [x, y, z, w] }
    }

    pub fn x(&self) -> f64 {
        self.data[0]
    }

    pub fn y(&self) -> f64 {
        self.data[1]
    }

    pub fn z(&self) -> f64 {
        self.data[2]
    }

    pub fn w(&self) -> f64 {
        self.data[3]
    }

    pub fn r(&self) -> f64 {
        self.data[0]
    }

    pub fn g(&self) -> f64 {
        self.data[1]
    }

    pub fn b(&self) -> f64 {
        self.data[2]
    }

    pub fn is_point(&self) -> bool {
        self.data[3] == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.data[3] == 0.0
    }

    pub fn norm(&self) -> f64 {
        let mut norm = 0.0;
        for i in 0..4 {
            norm += self.data[i] * self.data[i];
        }
        norm.sqrt()
    }

    pub fn normalized(&self) -> Tuple {
        let norm = self.norm();
        if norm == 0.0 {
            return *self;
        }
        let mut n = *self;
        for i in 0..4 {
            n.data[i] /= norm
        }
        n
    }

    pub fn dot(&self, other: &Tuple) -> f64 {
        let mut res = 0.0_f64;
        for i in 0..4 {
            res += self.data[i] * other.data[i];
        }
        res
    }

    pub fn cross(&self, other: &Tuple) -> Tuple {
        let mut res = Tuple::zero_vector();
        res.data[0] = self.y() * other.z() - self.z() * other.y();
        res.data[1] = self.z() * other.x() - self.x() * other.z();
        res.data[2] = self.x() * other.y() - self.y() * other.x();
        res
    }

    pub fn hadamard(&self, other: &Tuple) -> Tuple {
        let mut res = Tuple::zero_vector();
        for i in 0..4 {
            res.data[i] = self.data[i] * other.data[i];
        }
        res
    }

    pub fn set(&mut self, i: usize, v: f64) {
        self.data[i] = v;
    }

    pub fn at(&self, i: usize) -> f64 {
        self.data[i]
    }

    pub fn reflected(&self, normal: &Tuple) -> Tuple {
        if !feq(self.data[3], 0.0) {
            panic!("Can only compute a normal of a vector");
        }

        if !normal.is_vector() {
            panic!("Normal must be a vector");
        }

        *self - (2.0 * self.dot(normal)) * *normal
    }
}

pub const fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::new(x, y, z, 1.0)
}

pub const fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::new(x, y, z, 0.0)
}

pub const fn color(r: f64, g: f64, b: f64) -> Tuple {
    vector(r, g, b)
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..4 {
            if !feq(self.data[i], other.data[i]) {
                return false;
            }
        }
        true
    }
}

impl Eq for Tuple {}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut res = Self::zero_vector();
        for i in 0..4 {
            res.data[i] = self.data[i] + other.data[i];
        }
        res
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut res = Self::zero_vector();
        for i in 0..4 {
            res.data[i] = self.data[i] - other.data[i];
        }
        res
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut res = Self::zero_vector();
        for i in 0..4 {
            res.data[i] = -self.data[i];
        }
        res
    }
}

impl Mul<Tuple> for f64 {
    type Output = Tuple;

    fn mul(self, mut other: Tuple) -> Self::Output {
        for i in 0..4 {
            other.data[i] *= self;
        }
        other
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(mut self, other: f64) -> Self::Output {
        for i in 0..4 {
            self.data[i] *= other;
        }
        self
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(mut self, other: f64) -> Self::Output {
        for i in 0..4 {
            self.data[i] /= other;
        }
        self
    }
}
