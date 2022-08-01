// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::intersections::{intersect, Intersections};
use crate::light::{point_light, Light};
use crate::material::Material;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::sphere::Sphere;
use crate::transformations::scaling;
use crate::tuple::{color, point};

pub struct World {
    shapes: Vec<Box<dyn Shape>>,
    lights: Vec<Light>,
}

impl World {
    pub fn empty() -> World {
        World {
            shapes: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn default() -> World {
        let mut w = World {
            shapes: Vec::new(),
            lights: Vec::new(),
        };

        let l = point_light(point(-10.0, 10.0, -10.0), color(1.0, 1.0, 1.0));
        w.add_light(l);

        let mut s1 = Box::new(Sphere::unit()) as Box<dyn Shape>;
        let mut m1 = Material::new();
        m1.color = color(0.9, 1.0, 0.6);
        m1.diffuse = 0.7;
        m1.specular = 0.2;
        s1.set_material(&m1);

        let s2 = Box::new(Sphere::new(scaling(0.5, 0.5, 0.5))) as Box<dyn Shape>;

        w.add_shape(s1);
        w.add_shape(s2);

        w
    }

    pub fn add_shape(&mut self, shape: Box<dyn Shape>) {
        self.shapes.push(shape)
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light)
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let mut xs = Intersections::new();

        for s in self.shapes.iter() {
            let s_xs = intersect(s, ray);
            xs.append(s_xs);
        }

        xs
    }
}
