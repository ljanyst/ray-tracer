// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::constants::EPSILON;
use crate::ray::Ray;

pub fn feq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

pub fn peq<T: ?Sized>(left: &T, right: &T) -> bool {
    let left: *const T = left;
    let right: *const T = right;
    left == right
}

pub fn check_cap(ray: &Ray, limit: f64, radius: f64) -> Option<f64> {
    let t = (limit - ray.origin().y()) / ray.direction().y();
    let x = ray.origin().x() + t * ray.direction().x();
    let z = ray.origin().z() + t * ray.direction().z();
    if x.powi(2) + z.powi(2) <= radius.powi(2) {
        return Some(t);
    }
    None
}

pub fn solve_quadratic(a: f64, b: f64, c: f64) -> Vec<f64> {
    let disc = b.powi(2) - 4.0 * a * c;

    if disc < 0.0 {
        return vec![];
    }

    let disc_sqrt = disc.sqrt();
    let mut t0 = (-b - disc_sqrt) / (2.0 * a);
    let mut t1 = (-b + disc_sqrt) / (2.0 * a);

    if t0 > t1 {
        std::mem::swap(&mut t0, &mut t1);
    }
    vec![t0, t1]
}

pub fn filter_min_max(t: f64, ray: &Ray, min: f64, max: f64) -> bool {
    let y = ray.origin().y() + t * ray.direction().y();
    if min < y && y < max {
        return true;
    }
    false
}

#[macro_export]
macro_rules! pattern_boilerplate_2p {
    ($cls:ident, $unit:ident, $color:ident, $full:ident) => {
        pub fn $unit(color1: Tuple, color2: Tuple) -> Box<dyn Pattern> {
            Box::new(PatternImpl::new($cls {
                pattern1: solid_pattern(color1),
                pattern2: solid_pattern(color2),
            }))
        }

        pub fn $color(color1: Tuple, color2: Tuple, transform: Matrix) -> Box<dyn Pattern> {
            let mut s = $unit(color1, color2);
            s.transform(transform);
            s
        }

        pub fn $full(
            pattern1: Box<dyn Pattern>,
            pattern2: Box<dyn Pattern>,
            transform: Matrix,
        ) -> Box<dyn Pattern> {
            let mut p = Box::new(PatternImpl::new($cls { pattern1, pattern2 }));
            p.transform(transform);
            p
        }

        impl PartialEq for $cls {
            fn eq(&self, other: &Self) -> bool {
                &self.pattern1 == &other.pattern1 && &self.pattern2 == &other.pattern2
            }
        }

        impl Eq for $cls {}
    };
}
