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

        xs.sort();
        xs
    }

    pub fn shade_hit(&self, props: &IntersectionProperties, depth: u8) -> Tuple {
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

        let reflected = self.reflected_color(props, depth);
        let refracted = self.refracted_color(props, depth);

        let material = props.shape.material();
        if material.reflective > 0.0 && material.transparency > 0.0 {
            let reflectance = props.schlick();
            return color + reflected * reflectance + refracted * (1.0 - reflectance);
        }

        color + reflected + refracted
    }

    pub fn reflected_color(&self, props: &IntersectionProperties, depth: u8) -> Tuple {
        if props.shape.material().reflective == 0.0 || depth == 0 {
            return Tuple::zero_color();
        }

        let reflected_ray = Ray::new(props.over_point, props.reflectv);
        props.shape.material().reflective * self.color_at(&reflected_ray, depth - 1)
    }

    pub fn refracted_color(&self, props: &IntersectionProperties, depth: u8) -> Tuple {
        if props.shape.material().transparency == 0.0 || depth == 0 {
            return Tuple::zero_color();
        }

        // Snell's law: https://en.wikipedia.org/wiki/Snell's_law
        // Vector form: https://physics.stackexchange.com/questions/435512/snells-law-in-vector-form
        let (n1, n2) = props.refraction_indices;
        let cos_theta_1 = props.eyev.dot(&props.normalv);
        let ratio = n1 / n2;
        let sinsq_theta_2 = ratio.powi(2) * (1.0 - cos_theta_1.powi(2));
        let cos_theta_2 = (1.0 - sinsq_theta_2).sqrt();

        // Total internal reflection happens when sin(theta_2) would be larger
        // than 1 which is impossible to satisfy
        if sinsq_theta_2 > 1.0 {
            return Tuple::zero_color();
        }

        // Spawn the refracted ray and compute it's color
        let direction = (ratio * cos_theta_1 - cos_theta_2) * props.normalv - ratio * props.eyev;
        let refracted_ray = Ray::new(props.under_point, direction);

        props.shape.material().transparency * self.color_at(&refracted_ray, depth - 1)
    }

    pub fn color_at(&self, ray: &Ray, depth: u8) -> Tuple {
        let xs = self.intersect(ray);
        let hit = xs.hit();

        if hit.is_none() {
            return Tuple::zero_color();
        }

        let h = hit.unwrap();
        let props = h.properties(ray, &xs);
        self.shade_hit(&props, depth)
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
