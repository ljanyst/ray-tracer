// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::matrix::Matrix;
use crate::shape::Shape;
use crate::tuple::Tuple;

use std::any::Any;
use std::fmt;

pub trait Pattern: fmt::Debug {
    fn transform(&mut self, transform: Matrix);
    fn current_transform(&self) -> &Matrix;

    /// Returns the color for a point in the worlds's frame of reference
    ///
    /// # Arguments
    ///
    /// * `pt_w` - A a point in the world's frame of reference
    fn color_at(&self, shape: &Box<dyn Shape>, pt_w: Tuple) -> Tuple;

    /// Returns the color for a point in the shape's frame of reference
    ///
    /// # Arguments
    ///
    /// * `pt_s` - A a point in the shape's frame of reference
    fn shape_color_at(&self, pt_s: Tuple) -> Tuple;

    fn dyn_clone(&self) -> Box<dyn Pattern>;
    fn dyn_eq(&self, other: &Box<dyn Pattern>) -> bool;
    fn as_any(&self) -> &dyn Any;
}

pub trait LocalPattern: Clone + fmt::Debug + PartialEq + Eq {
    /// Returns the color for a point in the pattern's frame of reference
    ///
    /// # Arguments
    ///
    /// * `pt_p` - A a point in the pattern's frame of reference
    fn local_color_at(&self, pt_p: Tuple) -> Tuple;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternImpl<T: LocalPattern> {
    transform: Matrix,
    transform_inv: Matrix,
    pattern: T,
}

impl Clone for Box<dyn Pattern> {
    fn clone(&self) -> Self {
        self.dyn_clone()
    }
}

impl PartialEq for Box<dyn Pattern> {
    fn eq(&self, other: &Self) -> bool {
        self.dyn_eq(other)
    }
}

impl Eq for Box<dyn Pattern> {}

impl<T> PatternImpl<T>
where
    T: LocalPattern + 'static,
{
    pub fn new(pattern: T) -> PatternImpl<T> {
        PatternImpl {
            transform: Matrix::one(),
            transform_inv: Matrix::one(),
            pattern: pattern,
        }
    }
}

impl<T> Pattern for PatternImpl<T>
where
    T: LocalPattern + 'static,
{
    fn color_at(&self, shape: &Box<dyn Shape>, pt: Tuple) -> Tuple {
        let pt_s = *shape.current_inverse_transform() * pt;
        self.shape_color_at(pt_s)
    }

    fn shape_color_at(&self, pt_o: Tuple) -> Tuple {
        let pt_p = self.transform_inv * pt_o;
        self.pattern.local_color_at(pt_p)
    }

    fn transform(&mut self, transform: Matrix) {
        self.transform = transform * self.transform;
        self.transform_inv = self.transform.inverted();
    }

    fn current_transform(&self) -> &Matrix {
        &self.transform
    }

    fn dyn_clone(&self) -> Box<dyn Pattern> {
        Box::new(self.clone())
    }

    fn dyn_eq(&self, other: &Box<dyn Pattern>) -> bool {
        match other.as_any().downcast_ref::<Self>() {
            None => false,
            Some(other) => self == other,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
