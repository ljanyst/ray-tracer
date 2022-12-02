// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::constants::EPSILON;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::{LocalShape, Shape, ShapeImpl};
use crate::tuple::{vector, Tuple};

pub struct Cylinder {}

impl LocalShape for Cylinder {
    fn local_intersect(&self, ray: &Ray) -> Vec<f64> {
        // The equation of a cylinder of radius 1 infinite height along the
        // y axis and centered at origin is:
        //
        // x^2 + z^2 = 1
        //
        // The parametric equation for a line is:
        //
        // x = x_o + t * x_d
        // y = y_o + t * y_d
        // z = z_o + t * z_d
        //
        // Where (x_o, y_o, z_o) are the coordinates of the origin and
        // (x_d, y_d, z_d) is the normalized direction.
        //
        // We solve for t.

        let a = ray.direction().x().powi(2) + ray.direction().z().powi(2);

        // Ray is parallel to the Y axis
        if a < EPSILON {
            return vec![];
        }

        let b = 2.0 * ray.origin().x() * ray.direction().x()
            + 2.0 * ray.origin().z() * ray.direction().z();
        let c = ray.origin().x().powi(2) + ray.origin().z().powi(2) - 1.0;
        let disc = b.powi(2) - 4.0 * a * c;

        if disc < 0.0 {
            return vec![];
        }

        let disc_sqrt = disc.sqrt();
        let t0 = (-b - disc_sqrt) / (2.0 * a);
        let t1 = (-b + disc_sqrt) / (2.0 * a);

        vec![t0, t1]
    }

    fn local_normal_at(&self, pt: Tuple) -> Tuple {
        vector(pt.x(), 0.0, pt.z())
    }
}

pub fn cylinder_unit() -> Box<dyn Shape> {
    Box::new(ShapeImpl::new(Cylinder {}))
}

pub fn cylinder(transform: Matrix) -> Box<dyn Shape> {
    let mut s = cylinder_unit();
    s.transform(transform);
    s
}
