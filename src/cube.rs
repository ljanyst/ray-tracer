// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::{LocalShape, Shape, ShapeImpl};
use crate::tuple::{vector, Tuple};

pub struct Cube {}

fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
    let tmin_numerator = -1.0 - origin;
    let tmax_numerator = 1.0 - origin;
    let mut tmin = tmin_numerator / direction;
    let mut tmax = tmax_numerator / direction;
    if tmin > tmax {
        std::mem::swap(&mut tmin, &mut tmax);
    }
    (tmin, tmax)
}

impl LocalShape for Cube {
    fn local_intersect(&self, ray: &Ray) -> Vec<f64> {
        let (xtmin, xtmax) = check_axis(ray.origin().x(), ray.direction().x());
        let (ytmin, ytmax) = check_axis(ray.origin().y(), ray.direction().y());
        let (ztmin, ztmax) = check_axis(ray.origin().z(), ray.direction().z());
        let tmin = xtmin.max(ytmin).max(ztmin);
        let tmax = xtmax.min(ytmax).min(ztmax);

        if tmin > tmax {
            return Vec::<f64>::new();
        }

        vec![tmin, tmax]
    }

    fn local_normal_at(&self, pt: Tuple) -> Tuple {
        let max = pt.x().abs().max(pt.y().abs()).max(pt.z().abs());
        if max == pt.x().abs() {
            return vector(pt.x(), 0.0, 0.0);
        } else if max == pt.y().abs() {
            return vector(0.0, pt.y(), 0.0);
        }
        vector(0.0, 0.0, pt.z())
    }
}

pub fn cube_unit() -> Box<dyn Shape> {
    Box::new(ShapeImpl::new(Cube {}))
}

pub fn cube(transform: Matrix) -> Box<dyn Shape> {
    let mut c = cube_unit();
    c.transform(transform);
    c
}
