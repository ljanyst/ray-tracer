// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::tuple::{color, Tuple};
use crate::utils::feq;

use std::cmp::{Eq, PartialEq};

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub color: Tuple,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new() -> Material {
        Material {
            color: color(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
            && feq(self.ambient, other.ambient)
            && feq(self.diffuse, other.diffuse)
            && feq(self.specular, other.specular)
            && feq(self.shininess, other.shininess)
    }
}

impl Eq for Material {}
