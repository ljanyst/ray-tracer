// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::matrix::Matrix;
use crate::noise::Noise;
use crate::pattern::{LocalPattern, Pattern, PatternImpl};
use crate::tuple::{vector, Tuple};

#[derive(Debug, Clone)]
pub struct NoisePattern {
    pattern: Box<dyn Pattern>,
    noise: Noise,
}

impl LocalPattern for NoisePattern {
    fn local_color_at(&self, pt: Tuple) -> Tuple {
        let nx = self.noise.octave_noise(pt + vector(10.0, 0.0, 0.0), 6, 0.5) * 0.5;
        let ny = self.noise.octave_noise(pt + vector(0.0, 10.0, 0.0), 6, 0.5) * 0.5;
        let nz = self.noise.octave_noise(pt + vector(0.0, 0.0, 10.0), 6, 0.5) * 0.5;

        let pt_noised = pt + vector(nx, ny, nz);
        self.pattern.shape_color_at(pt_noised)
    }
}

pub fn noise_pattern(pattern: Box<dyn Pattern>, transform: Matrix) -> Box<dyn Pattern> {
    let mut p = noise_pattern_unit(pattern);
    p.transform(transform);
    p
}

pub fn noise_pattern_unit(pattern: Box<dyn Pattern>) -> Box<dyn Pattern> {
    Box::new(PatternImpl::new(NoisePattern {
        pattern: pattern,
        noise: Noise::new(),
    }))
}

impl PartialEq for NoisePattern {
    fn eq(&self, other: &Self) -> bool {
        &self.pattern == &other.pattern
    }
}

impl Eq for NoisePattern {}
