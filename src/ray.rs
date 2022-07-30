// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::matrix::Matrix;
use crate::tuple::Tuple;

#[derive(Debug)]
pub struct Ray {
    origin: Tuple,
    direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn origin(&self) -> Tuple {
        self.origin
    }

    pub fn direction(&self) -> Tuple {
        self.direction
    }

    pub fn position(&self, t: f64) -> Tuple {
        self.origin + (t * self.direction)
    }

    pub fn transformed(&self, transform: Matrix) -> Ray {
        Ray {
            origin: transform * self.origin,
            direction: transform * self.direction,
        }
    }
}
