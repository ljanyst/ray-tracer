// Copyright 2023 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::constants::EPSILON;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::{LocalShape, Shape, ShapeImpl};
use crate::tuple::{vector, Tuple};
use crate::utils::{check_cap, filter_min_max, solve_quadratic};

pub struct Cone {
    minimum: f64,
    maximum: f64,
    closed: bool,
}

impl Cone {
    fn intersect_caps(&self, ray: &Ray) -> Vec<f64> {
        if !self.closed || ray.direction().y().abs() < EPSILON {
            return vec![];
        }

        let mut xs = Vec::new();
        for val in [self.minimum, self.maximum].iter() {
            if let Some(t) = check_cap(ray, *val, *val) {
                xs.push(t)
            }
        }
        xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        xs
    }
}

impl LocalShape for Cone {
    fn local_intersect(&self, ray: &Ray) -> Vec<f64> {
        // The equation for a cone if infinite height along the y axis and
        // centered at origin is:
        //
        // x^2 + z^2 = y^2
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

        let a =
            ray.direction().x().powi(2) + ray.direction().z().powi(2) - ray.direction().y().powi(2);
        let b = 2.0
            * (ray.origin().x() * ray.direction().x() + ray.origin().z() * ray.direction().z()
                - ray.origin().y() * ray.direction().y());
        let c = ray.origin().x().powi(2) + ray.origin().z().powi(2) - ray.origin().y().powi(2);

        // Ray is parallel to the wall of the cone, it intersects with a wall in a single place
        // if b is non-zero and potentially with the caps
        if a.abs() < EPSILON {
            if b.abs() < EPSILON {
                return vec![];
            }
            let mut xs = vec![-c / b];
            xs.append(&mut self.intersect_caps(ray));
            xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
            return xs;
        }

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
        if dist < pt.y().powi(2) {
            if pt.y() > self.maximum - EPSILON {
                return vector(0.0, 1.0, 0.0);
            }
            if pt.y() < self.minimum + EPSILON {
                return vector(0.0, -1.0, 0.0);
            }
        }

        let mut y = dist.sqrt();
        if pt.y() > 0.0 {
            y = -y;
        }
        let ret = vector(pt.x(), y, pt.z());
        println!("{ret:?} - {pt:?}");
        ret
    }
}

pub fn cone_unit() -> Box<dyn Shape> {
    Box::new(ShapeImpl::new(Cone {
        minimum: f64::NEG_INFINITY,
        maximum: f64::INFINITY,
        closed: false,
    }))
}

pub fn cone_min_max(minimum: f64, maximum: f64, closed: bool) -> Box<dyn Shape> {
    Box::new(ShapeImpl::new(Cone {
        minimum,
        maximum,
        closed,
    }))
}

pub fn cone(transform: Matrix) -> Box<dyn Shape> {
    let mut s = cone_unit();
    s.transform(transform);
    s
}
