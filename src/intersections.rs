// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::ray::Ray;
use crate::shape::Shape;

pub struct Intersection<'a> {
    t: f64,
    shape: &'a Box<dyn Shape>,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, shape: &'a Box<dyn Shape>) -> Intersection<'a> {
        Intersection { t: t, shape: shape }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn shape(&self) -> &'a Box<dyn Shape> {
        self.shape
    }
}

pub struct Intersections<'a> {
    ints: Vec<Intersection<'a>>,
}

impl<'a> Intersections<'a> {
    pub fn new(ints: Vec<Intersection<'a>>) -> Intersections<'a> {
        Intersections { ints: ints }
    }

    pub fn at(&self, i: usize) -> &Intersection<'a> {
        &self.ints[i]
    }

    pub fn len(&self) -> usize {
        self.ints.len()
    }
}

pub fn intersect<'a>(shape: &'a Box<dyn Shape>, ray: &Ray) -> Vec<Intersection<'a>> {
    let inx = shape.intersect(&ray);
    let mut res = Vec::new();

    for i in inx.iter() {
        res.push(Intersection::new(*i, shape));
    }
    res
}
