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
    shape: &'a Box<dyn Shape>,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, shape: &Box<dyn Shape>) -> Intersection {
        Intersection { t: t, shape: shape }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn shape(&self) -> &Box<dyn Shape> {
        self.shape
    }

    pub fn properties(&self, ray: &Ray) -> IntersectionProperties {
        let point = ray.position(self.t);
        let mut normalv = self.shape.normal_at(point);
        let eyev = -ray.direction();
        let mut inside = false;

        if normalv.dot(&eyev) < 0.0 {
            inside = true;
            normalv = -normalv;
        }

        IntersectionProperties {
            t: self.t,
            shape: self.shape,
            point: point,
            eyev: eyev,
            normalv: normalv,
            inside: inside,
            over_point: point + normalv * EPSILON,
        }
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
        let ptr: *const dyn Shape = self.shape.as_ref();
        f.debug_struct("Intersection")
            .field("t", &self.t)
            .field("shape", &ptr)
            .finish()
    }
}

impl<'a> Eq for Intersection<'a> {}

pub struct IntersectionProperties<'a> {
    pub t: f64,
    pub shape: &'a Box<dyn Shape>,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
}

impl<'a> fmt::Debug for IntersectionProperties<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ptr: *const dyn Shape = self.shape.as_ref();
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

pub struct Intersections<'a> {
    xs: Vec<Intersection<'a>>,
}

impl<'a> Intersections<'a> {
    pub fn new() -> Intersections<'a> {
        Intersections { xs: Vec::new() }
    }

    pub fn from_vector(xs: Vec<Intersection>) -> Intersections {
        Intersections { xs: xs }
    }

    pub fn at(&self, i: usize) -> &Intersection {
        &self.xs[i]
    }

    pub fn append(&mut self, mut xs: Vec<Intersection<'a>>) {
        self.xs.append(&mut xs);
        self.xs.sort_by(|a, b| a.t().partial_cmp(&b.t()).unwrap());
    }

    pub fn push(&mut self, x: Intersection<'a>) {
        self.xs.push(x);
        self.xs.sort_by(|a, b| a.t().partial_cmp(&b.t()).unwrap());
    }

    pub fn len(&self) -> usize {
        self.xs.len()
    }

    pub fn hit(&self) -> Option<Intersection> {
        let mut res = None;
        for x in self.xs.iter() {
            if x.t() > 0.0 {
                res = Some(*x);
                break;
            }
        }
        res
    }
}

pub fn intersect<'a>(shape: &'a Box<dyn Shape>, ray: &Ray) -> Vec<Intersection<'a>> {
    let inx = shape.intersect(&ray);
    let mut res = Vec::new();

    for x in inx.iter() {
        res.push(Intersection::new(*x, shape));
    }
    res
}
