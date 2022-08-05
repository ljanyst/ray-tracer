// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::{LocalShape, Shape, ShapeImpl};
use crate::tuple::{point, Tuple};

pub struct Sphere {}

impl LocalShape for Sphere {
    fn local_intersect(&self, ray: &Ray) -> Vec<f64> {
        // Derivation: https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection

        // We are at a unit sphere centered at origin and the ray is expressed in the
        // local frame
        let dst = ray.origin() - point(0.0, 0.0, 0.0);
        let a = ray.direction().norm().powi(2);
        let b = 2.0 * ray.direction().dot(&dst);
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

    fn local_normal_at(&self, pt: Tuple) -> Tuple {
        // We get the normal in the local frame by subtracting the origin since
        // the point is on a unit sphere at origin.
        pt - point(0.0, 0.0, 0.0)
    }
}

pub fn sphere_unit() -> Box<dyn Shape> {
    Box::new(ShapeImpl::new(Sphere {}))
}

pub fn sphere(transform: Matrix) -> Box<dyn Shape> {
    let mut s = sphere_unit();
    s.transform(transform);
    s
}
