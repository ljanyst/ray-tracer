// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::tuple::point;

pub struct Sphere {
    transform: Matrix,
}

impl Sphere {
    pub fn unit() -> Sphere {
        Sphere {
            transform: Matrix::one(),
        }
    }

    pub fn new(transform: Matrix) -> Sphere {
        Sphere {
            transform: transform,
        }
    }

    pub fn current_transform(&self) -> &Matrix {
        &self.transform
    }

    pub fn transform(&mut self, transform: Matrix) {
        self.transform = transform * self.transform
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray) -> Vec<f64> {
        // Derivation: https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection

        // For now we assume a unit sphere centered at origin
        let dst = ray.origin() - point(0.0, 0.0, 0.0);
        let a = ray.direction().norm().powi(2);
        let b = 2.0 * ray.direction().dot(&dst);
        let c = dst.norm().powi(2) - 1.0; // r^2 == 1
        let delta = b.powi(2) - 4.0 * a * c;

        if delta < 0.0 {
            return vec![];
        }

        let s_delta = delta.sqrt();
        let d1 = (-b - s_delta) / 2.0 * a;
        let d2 = (-b + s_delta) / 2.0 * a;
        vec![d1, d2]
    }
}
