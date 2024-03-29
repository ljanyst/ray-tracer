// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::constants::EPSILON;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::{LocalShape, Shape, ShapeImpl};
use crate::tuple::{vector, Tuple};
use crate::utils::{check_cap, filter_min_max, solve_quadratic};

pub struct Cylinder {
    minimum: f64,
    maximum: f64,
    closed: bool,
}

impl Cylinder {
    fn intersect_caps(&self, ray: &Ray) -> Vec<f64> {
        if !self.closed || ray.direction().y().abs() < EPSILON {
            return vec![];
        }

        let mut xs = Vec::new();
        for val in [self.minimum, self.maximum].iter() {
            if let Some(t) = check_cap(ray, *val, 1.0) {
                xs.push(t)
            }
        }
        xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        xs
    }
}

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
            return self.intersect_caps(ray);
        }

        let b = 2.0 * ray.origin().x() * ray.direction().x()
            + 2.0 * ray.origin().z() * ray.direction().z();
        let c = ray.origin().x().powi(2) + ray.origin().z().powi(2) - 1.0;

        let mut xs = Vec::new();
        for t in solve_quadratic(a, b, c).iter() {
            if filter_min_max(*t, ray, self.minimum, self.maximum) {
                xs.push(*t)
            }
        }

        xs.append(&mut self.intersect_caps(ray));
        xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        xs
    }

    fn local_normal_at(&self, pt: Tuple) -> Tuple {
        let dist = pt.x().powi(2) + pt.z().powi(2);
        if dist < 1.0 {
            if pt.y() > self.maximum - EPSILON {
                return vector(0.0, 1.0, 0.0);
            }
            if pt.y() < self.minimum + EPSILON {
                return vector(0.0, -1.0, 0.0);
            }
        }
        vector(pt.x(), 0.0, pt.z())
    }
}

pub fn cylinder_unit() -> Box<dyn Shape> {
    Box::new(ShapeImpl::new(Cylinder {
        minimum: f64::NEG_INFINITY,
        maximum: f64::INFINITY,
        closed: false,
    }))
}

pub fn cylinder_min_max(minimum: f64, maximum: f64, closed: bool) -> Box<dyn Shape> {
    Box::new(ShapeImpl::new(Cylinder {
        minimum,
        maximum,
        closed,
    }))
}

pub fn cylinder(transform: Matrix) -> Box<dyn Shape> {
    let mut s = cylinder_unit();
    s.transform(transform);
    s
}
