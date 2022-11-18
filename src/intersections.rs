// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::constants::EPSILON;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::tuple::Tuple;
use crate::utils::{feq, peq};

use std::cmp::{Eq, PartialEq};
use std::fmt;

#[derive(Copy, Clone)]
pub struct Intersection<'a> {
    t: f64,
    shape: &'a dyn Shape,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, shape: &dyn Shape) -> Intersection {
        Intersection { t, shape }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn shape(&self) -> &dyn Shape {
        self.shape
    }

    pub fn properties(&self, ray: &Ray, xs: &Intersections) -> IntersectionProperties {
        let point = ray.position(self.t);
        let mut normalv = self.shape.normal_at(point);
        let eyev = -ray.direction();
        let mut inside = false;

        if normalv.dot(&eyev) < 0.0 {
            inside = true;
            normalv = -normalv;
        }

        let (n1, n2) = self.compute_refraction_indices(xs);

        IntersectionProperties {
            t: self.t,
            shape: self.shape,
            point,
            eyev,
            normalv,
            inside,
            over_point: point + normalv * EPSILON,
            reflectv: ray.direction().reflected(&normalv),
            refraction_indices: (n1, n2),
        }
    }

    fn compute_refraction_indices(&self, xs: &Intersections) -> (f64, f64) {
        let mut n1 = 1.0;
        let mut n2 = 1.0;
        let mut containers = Vec::<&dyn Shape>::new();

        for i in 0..xs.len() {
            let x = xs.at(i);
            if x == self {
                if let Some(last) = containers.last() {
                    n1 = last.material().refractive_index;
                }
            }

            if let Some(index) = containers.iter().position(|xx| peq(*xx, x.shape)) {
                containers.remove(index);
            } else {
                containers.push(x.shape);
            }

            if x == self {
                if let Some(last) = containers.last() {
                    n2 = last.material().refractive_index;
                }
                break;
            }
        }
        (n1, n2)
    }
}

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        if !feq(self.t, other.t) {
            return false;
        }
        if !peq(self.shape, other.shape) {
            return false;
        }
        true
    }
}

impl<'a> fmt::Debug for Intersection<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ptr: *const dyn Shape = self.shape;
        f.debug_struct("Intersection")
            .field("t", &self.t)
            .field("shape", &ptr)
            .finish()
    }
}

impl<'a> Eq for Intersection<'a> {}

pub struct IntersectionProperties<'a> {
    pub t: f64,
    pub shape: &'a dyn Shape,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
    pub reflectv: Tuple,
    pub refraction_indices: (f64, f64),
}

impl<'a> fmt::Debug for IntersectionProperties<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ptr = self.shape as *const dyn Shape;
        f.debug_struct("IntersectionProperties")
            .field("t", &self.t)
            .field("shape", &ptr)
            .field("point", &self.point)
            .field("eyev", &self.eyev)
            .field("normalv", &self.normalv)
            .field("inside", &self.inside)
            .field("over_point", &self.over_point)
            .finish()
    }
}

#[derive(Default)]
pub struct Intersections<'a> {
    xs: Vec<Intersection<'a>>,
}

impl<'a> Intersections<'a> {
    pub fn new() -> Intersections<'a> {
        Intersections { xs: Vec::new() }
    }

    pub fn from_vector(xs: Vec<Intersection>) -> Intersections {
        Intersections { xs }
    }

    pub fn at(&self, i: usize) -> &Intersection {
        &self.xs[i]
    }

    pub fn append(&mut self, mut xs: Vec<Intersection<'a>>) {
        self.xs.append(&mut xs);
    }

    pub fn push(&mut self, x: Intersection<'a>) {
        self.xs.push(x);
    }

    pub fn sort(&mut self) {
        self.xs.sort_by(|a, b| a.t().partial_cmp(&b.t()).unwrap());
    }

    pub fn len(&self) -> usize {
        self.xs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.xs.is_empty()
    }

    pub fn hit(&self) -> Option<&Intersection> {
        let mut res = None;
        for x in self.xs.iter() {
            if x.t() > 0.0 {
                res = Some(x);
                break;
            }
        }
        res
    }
}

pub fn intersect<'a>(shape: &'a dyn Shape, ray: &Ray) -> Vec<Intersection<'a>> {
    let inx = shape.intersect(ray);
    let mut res = Vec::new();

    for x in inx.iter() {
        res.push(Intersection::new(*x, shape));
    }
    res
}
