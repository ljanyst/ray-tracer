// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::tuple::Tuple;

use std::ops::Deref;

pub trait Shape: Deref<Target = dyn LocalShape> {
    fn intersect(&self, ray: &Ray) -> Vec<f64>;
    fn transform(&mut self, transform: Matrix);
    fn current_transform(&self) -> &Matrix;
    fn current_inverse_transform(&self) -> &Matrix;
    fn normal_at(&self, pt: Tuple) -> Tuple;
    fn material(&self) -> &Material;
    fn material_mut(&mut self) -> &mut Material;
    fn set_material(&mut self, material: &Material);
}

pub trait LocalShape {
    fn local_intersect(&self, ray: &Ray) -> Vec<f64>;
    fn local_normal_at(&self, pt: Tuple) -> Tuple;
}

pub struct ShapeImpl<T: LocalShape> {
    transform: Matrix,
    transform_inv: Matrix,
    material: Material,
    shape: T,
}

impl<T> ShapeImpl<T>
where
    T: LocalShape,
{
    pub fn new(shape: T) -> ShapeImpl<T> {
        ShapeImpl {
            transform: Matrix::one(),
            transform_inv: Matrix::one(),
            material: Material::new(),
            shape,
        }
    }
}

impl<T> Shape for ShapeImpl<T>
where
    T: LocalShape + 'static,
{
    fn intersect(&self, ray: &Ray) -> Vec<f64> {
        let r = ray.transformed(self.transform_inv);
        self.shape.local_intersect(&r)
    }

    fn transform(&mut self, transform: Matrix) {
        self.transform = transform * self.transform;
        self.transform_inv = self.transform.inverted();
    }

    fn current_transform(&self) -> &Matrix {
        &self.transform
    }

    fn current_inverse_transform(&self) -> &Matrix {
        &self.transform_inv
    }

    fn normal_at(&self, pt: Tuple) -> Tuple {
        let pt_o = self.transform_inv * pt;

        let normal_o = self.shape.local_normal_at(pt_o);

        // Technically we should invert and transpose the submatrix(3, 3), but
        // we can clean up the mess by zeroing the w component of the result.
        let mut normal_w = self.transform_inv.transposed() * normal_o;
        normal_w.set(3, 0.0);

        normal_w.normalized()
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    fn set_material(&mut self, material: &Material) {
        self.material = material.clone();
    }
}

impl<T> Deref for ShapeImpl<T>
where
    T: LocalShape + 'static,
{
    type Target = dyn LocalShape;
    fn deref(&self) -> &Self::Target {
        &self.shape
    }
}
