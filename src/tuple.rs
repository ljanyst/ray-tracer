// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::constants::EPSILON;

use std::cmp::{Eq, PartialEq};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    data: [f64; 4],
}

impl Tuple {
    pub fn zero_point() -> Tuple {
        Self {
            data: [0.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn zero_vector() -> Tuple {
        Self {
            data: [0.0, 0.0, 0.0, 0.0],
        }
    }

    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
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
        let mut n = *self;
        for i in 0..4 {
            n.data[i] /= norm
        }
        n
    }
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::new(x, y, z, 1.0)
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple::new(x, y, z, 0.0)
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..4 {
            if (self.data[i] - other.data[i]).abs() > EPSILON {
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
