// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::matrix::Matrix;
use crate::pattern::{LocalPattern, Pattern, PatternImpl};
use crate::pattern_boilerplate_2p;
use crate::pattern_solid::solid_pattern;
use crate::tuple::Tuple;

#[derive(Debug, Clone)]
pub struct RingPattern {
    pattern1: Box<dyn Pattern>,
    pattern2: Box<dyn Pattern>,
}

impl LocalPattern for RingPattern {
    fn local_color_at(&self, pt: Tuple) -> Tuple {
        let color1 = self.pattern1.shape_color_at(pt);
        let color2 = self.pattern2.shape_color_at(pt);
        if (pt.x().powi(2) + pt.z().powi(2)).sqrt() as isize % 2 == 0 {
            return color1;
        }
        color2
    }
}

pattern_boilerplate_2p!(
    RingPattern,
    ring_pattern_unit,
    ring_pattern_color,
    ring_pattern
);
