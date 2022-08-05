// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::{LocalShape, Shape, ShapeImpl};
use crate::tuple::{point, Tuple};
use crate::utils::feq;

pub struct Plane {}

impl LocalShape for Plane {
    fn local_intersect(&self, ray: &Ray) -> Vec<f64> {
        // A ray that is parallel to the plane has no intersections with it.
        // Even it the ray and the plane are co-planar, the plane is infinitely
        // thin, so we assume no intersections as well for simplicity.
        if feq(ray.direction().y(), 0.0) {
            return vec![];
        }

        // We have an xz plane at origin
        let t = -ray.origin().y() / ray.direction().y();
        vec![t]
    }

    fn local_normal_at(&self, _pt: Tuple) -> Tuple {
        // We have an xz plane at origin, so normal is constant and points towards y
        point(0.0, 1.0, 0.0)
    }
}

pub fn plane_unit() -> Box<dyn Shape> {
    Box::new(ShapeImpl::new(Plane {}))
}

pub fn plane(transform: Matrix) -> Box<dyn Shape> {
    let mut s = plane_unit();
    s.transform(transform);
    s
}
