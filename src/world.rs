// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::intersections::{intersect, IntersectionProperties, Intersections};
use crate::light::{point_light, Light};
use crate::material::Material;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::sphere::{sphere, sphere_unit};
use crate::transformations::scaling;
use crate::tuple::{color, point, Tuple};

pub struct World {
    pub shapes: Vec<Box<dyn Shape>>,
    pub lights: Vec<Light>,
    pub shadows: bool,
}

impl World {
    pub fn empty() -> World {
        World {
            shapes: Vec::new(),
            lights: Vec::new(),
            shadows: true,
        }
    }

    pub fn default() -> World {
        let mut w = World {
            shapes: Vec::new(),
            lights: Vec::new(),
            shadows: true,
        };

        let l = point_light(point(-10.0, 10.0, -10.0), color(1.0, 1.0, 1.0));
        w.lights.push(l);

        let mut s1 = sphere_unit();
        let mut m1 = Material::new();
        m1.color = color(0.8, 1.0, 0.6);
        m1.diffuse = 0.7;
        m1.specular = 0.2;
        s1.set_material(&m1);

        let s2 = sphere(scaling(0.5, 0.5, 0.5));

        w.shapes.push(s1);
        w.shapes.push(s2);

        w
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let mut xs = Intersections::new();

        for s in self.shapes.iter() {
            let s_xs = intersect(s.as_ref(), ray);
            xs.append(s_xs);
        }

        xs
    }

    pub fn shade_hit(&self, props: IntersectionProperties, _depth: u8) -> Tuple {
        let mut color = Tuple::zero_color();
        for l in self.lights.iter() {
            let mut shadowed = false;

            if self.shadows {
                // we need to use a point slightly above our point along the normal to
                // account for floating-point inaccuracies
                shadowed = self.is_shadowed(l, props.over_point);
            }

            color = color
                + props.shape.material().lighting(
                    props.shape,
                    l,
                    &props.point,
                    &props.eyev,
                    &props.normalv,
                    shadowed,
                );
        }
        color
    }

    pub fn reflected_color(&self, props: IntersectionProperties, depth: u8) -> Tuple {
        if props.shape.material().reflective == 0.0 || depth == 0 {
            return Tuple::zero_color();
        }

        let reflected_ray = Ray::new(props.over_point, props.reflectv);
        props.shape.material().reflective * self.color_at(&reflected_ray, depth - 1)
    }

    pub fn color_at(&self, ray: &Ray, depth: u8) -> Tuple {
        let xs = self.intersect(ray);
        let hit = xs.hit();

        if hit.is_none() {
            return Tuple::zero_color();
        }

        let h = hit.unwrap();
        let props = h.properties(ray);
        self.shade_hit(props, depth)
    }

    pub fn is_shadowed(&self, light: &Light, pt: Tuple) -> bool {
        let v = light.position - pt;
        let distance = v.norm();
        let direction = v.normalized();

        let r = Ray::new(pt, direction);
        let xs = self.intersect(&r);
        let hit = xs.hit();

        matches!(hit, Some(h) if h.t() < distance)
    }
}
