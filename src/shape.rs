// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::tuple::Tuple;

pub trait Shape {
    fn intersect(&self, ray: &Ray) -> Vec<f64>;
    fn transform(&mut self, transform: Matrix);
    fn current_transform(&self) -> &Matrix;
    fn normal_at(&self, point_w: Tuple) -> Tuple;
}
