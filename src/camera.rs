// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::canvas::Canvas;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::tuple::point;
use crate::world::World;

pub struct Camera {
    hres: usize,
    vres: usize,
    fov: f64,
    transform: Matrix,
    transform_inv: Matrix,
    pixel_size: f64,
    half_width: f64,
    half_height: f64,
}

impl Camera {
    pub fn new(hres: usize, vres: usize, fov: f64) -> Camera {
        let mut c = Camera {
            hres,
            vres,
            fov,
            transform: Matrix::one(),
            transform_inv: Matrix::one(),
            pixel_size: 0.0,
            half_width: 0.0,
            half_height: 0.0,
        };

        let aspect = hres as f64 / vres as f64;
        let half_view = (fov / 2.0).tan();

        if aspect >= 1.0 {
            c.half_width = half_view;
            c.half_height = half_view / aspect;
        } else {
            c.half_width = half_view * aspect;
            c.half_height = half_view;
        }
        c.pixel_size = (2.0 * c.half_width) / c.hres as f64;

        c
    }

    pub fn hres(&self) -> usize {
        self.hres
    }

    pub fn vres(&self) -> usize {
        self.vres
    }

    pub fn field_of_view(&self) -> f64 {
        self.fov
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
        self.transform_inv = self.transform.inverted();
    }

    pub fn pixel_size(&self) -> f64 {
        self.pixel_size
    }

    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        // we aix at the middle of the pixel
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;

        // camera looks towards -z, so +x is on the left
        let x_w = self.half_width - xoffset;
        let y_w = self.half_height - yoffset;

        // transform the canvas point and the origin; canvas is at z = -1
        let pixel = self.transform_inv * point(x_w, y_w, -1.0);
        let origin = self.transform_inv * point(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalized();

        Ray::new(origin, direction)
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut img = Canvas::new(self.hres, self.vres);

        for x in 0..self.hres {
            for y in 0..self.vres {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray);
                img.set(x, y, &color);
            }
        }

        img
    }
}
