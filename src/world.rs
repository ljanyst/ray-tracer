// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::intersections::{intersect, IntersectionProperties, Intersections};
use crate::light::{point_light, Light};
use crate::material::Material;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::sphere::Sphere;
use crate::transformations::scaling;
use crate::tuple::{color, point, Tuple};

pub struct World {
    pub shapes: Vec<Box<dyn Shape>>,
    pub lights: Vec<Light>,
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
        w.lights.push(l);

        let mut s1 = Box::new(Sphere::unit()) as Box<dyn Shape>;
        let mut m1 = Material::new();
        m1.color = color(0.8, 1.0, 0.6);
        m1.diffuse = 0.7;
        m1.specular = 0.2;
        s1.set_material(&m1);

        let s2 = Box::new(Sphere::new(scaling(0.5, 0.5, 0.5))) as Box<dyn Shape>;

        w.shapes.push(s1);
        w.shapes.push(s2);

        w
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let mut xs = Intersections::new();

        for s in self.shapes.iter() {
            let s_xs = intersect(s, ray);
            xs.append(s_xs);
        }

        xs
    }

    pub fn shade_hit(&self, props: IntersectionProperties) -> Tuple {
        let mut color = Tuple::zero_color();
        for l in self.lights.iter() {
            color = color
                + props.shape.material().lighting(
                    l,
                    &props.point,
                    &props.eyev,
                    &props.normalv,
                    false,
                );
        }
        color
    }

    pub fn color_at(&self, ray: &Ray) -> Tuple {
        let xs = self.intersect(ray);
        let hit = xs.hit();

        if hit == None {
            return Tuple::zero_color();
        }

        let h = hit.unwrap();
        let props = h.properties(ray);
        self.shade_hit(props)
    }
}
