// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::tuple::point;
use crate::tuple::Tuple;

pub struct Sphere {
    transform: Matrix,
    transform_inv: Matrix,
}

impl Sphere {
    pub fn unit() -> Sphere {
        Sphere {
            transform: Matrix::one(),
            transform_inv: Matrix::one(),
        }
    }

    pub fn new(transform: Matrix) -> Sphere {
        Sphere {
            transform: transform,
            transform_inv: transform.inverted(),
        }
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray) -> Vec<f64> {
        // Derivation: https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection

        // We assume a unit sphere centered at origin and transform the ray to the
        // object frame
        let r = ray.transformed(self.transform_inv);
        let dst = r.origin() - point(0.0, 0.0, 0.0);
        let a = r.direction().norm().powi(2);
        let b = 2.0 * r.direction().dot(&dst);
        let c = dst.norm().powi(2) - 1.0; // r^2 == 1
        let delta = b.powi(2) - 4.0 * a * c;

        if delta < 0.0 {
            return vec![];
        }

        let s_delta = delta.sqrt();
        let d1 = (-b - s_delta) / (2.0 * a);
        let d2 = (-b + s_delta) / (2.0 * a);
        vec![d1, d2]
    }

    fn current_transform(&self) -> &Matrix {
        &self.transform
    }

    fn transform(&mut self, transform: Matrix) {
        self.transform = transform * self.transform;
        self.transform_inv = self.transform.inverted();
    }

    fn normal_at(&self, point_w: Tuple) -> Tuple {
        let point_o = self.transform_inv * point_w;

        // We get the normal in the object space by subtracting the origin since
        // we assume the point is on the unit sphere at origin.
        let normal_o = point_o - point(0.0, 0.0, 0.0);

        // Technically we should invert and transpose the submatrix(3, 3), but
        // we can clean up the mess by zeroing the w component of the result.
        let mut normal_w = self.transform_inv.transposed() * normal_o;
        normal_w.set(3, 0.0);

        normal_w.normalized()
    }
}
