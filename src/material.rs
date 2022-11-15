// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::light::Light;
use crate::pattern::Pattern;
use crate::tuple::{color, Tuple};
use crate::utils::feq;
use crate::Shape;

use std::cmp::{Eq, PartialEq};

#[derive(Debug, Clone, Default)]
pub struct Material {
    pub color: Tuple,
    pub pattern: Option<Box<dyn Pattern>>,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflective: f64,
}

impl Material {
    pub fn new() -> Material {
        Material {
            color: color(1.0, 1.0, 1.0),
            pattern: None,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
        }
    }

    /// Shade the material according to the Phong reflection model
    pub fn lighting(
        &self,
        shape: &dyn Shape,
        light: &Light,
        pt: &Tuple,
        eyev: &Tuple,
        normalv: &Tuple,
        in_shadow: bool,
    ) -> Tuple {
        // See: https://en.wikipedia.org/wiki/Phong_reflection_model

        let c = match self.pattern.as_ref() {
            None => self.color,
            Some(pattern) => pattern.color_at(shape, *pt),
        };
        let effective_color = c.hadamard(&light.intensity);

        // Direction to the light source
        let lightv = (light.position - *pt).normalized();

        // Ambient contribution
        let ambient = self.ambient * effective_color;

        if in_shadow {
            return ambient;
        }

        // Defuse and specular contributions are black by default
        let mut diffuse = color(0.0, 0.0, 0.0);
        let mut specular = color(0.0, 0.0, 0.0);

        // Cosine of the angle between the ligt and normal vector
        let ln_cos = lightv.dot(normalv);

        // Nevative cosine between the light and normal vectors means the light
        // source is behind the surface
        if ln_cos > 0.0 {
            diffuse = self.diffuse * ln_cos * effective_color;

            // Cosine between the light reflection vector and and the eve vector
            let reflectedv = (-lightv).reflected(normalv);
            let re_cos = reflectedv.dot(eyev);

            // Negative cosinus means the light reflects away from the eye
            if re_cos > 0.0 {
                specular = re_cos.powf(self.shininess) * self.specular * light.intensity;
            }
        }

        ambient + diffuse + specular
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
            && feq(self.ambient, other.ambient)
            && feq(self.diffuse, other.diffuse)
            && feq(self.specular, other.specular)
            && feq(self.shininess, other.shininess)
            && self.pattern == other.pattern
    }
}

impl Eq for Material {}
