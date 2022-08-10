// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::matrix::Matrix;
use crate::pattern::{LocalPattern, Pattern, PatternImpl};
use crate::pattern_boilerplate_2p;
use crate::pattern_solid::solid_pattern;
use crate::tuple::Tuple;

#[derive(Debug, Clone)]
pub struct GradientPattern {
    pattern1: Box<dyn Pattern>,
    pattern2: Box<dyn Pattern>,
}

impl LocalPattern for GradientPattern {
    fn local_color_at(&self, pt: Tuple) -> Tuple {
        let color1 = self.pattern1.shape_color_at(pt);
        let color2 = self.pattern2.shape_color_at(pt);
        let distance = color2 - color1;
        let mut fraction = pt.x().abs();
        while fraction >= 2.0 {
            fraction -= 2.0;
        }
        if fraction > 1.0 {
            return color1 + distance * (2.0 - fraction);
        }
        color1 + distance * fraction
    }
}

pattern_boilerplate_2p!(
    GradientPattern,
    gradient_pattern_unit,
    gradient_pattern_color,
    gradient_pattern
);
